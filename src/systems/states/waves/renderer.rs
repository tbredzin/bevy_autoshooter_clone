use crate::systems::animations::animator::SpriteAnimator;
use crate::systems::constants::{tiles_to_pixels, TILES_X, TILES_Y};
use crate::systems::game::MarkedForDespawn;
use crate::systems::states::waves::components::Action::IDLE;
use crate::systems::states::waves::components::Direction::EAST;
use crate::systems::states::waves::components::{
    Action, Direction, Dying, LevelBackground, LevelOverlay,
};
use crate::systems::states::waves::enemy::components::Enemy;
use crate::systems::states::waves::enemy::resources::EnemyAnimations;
use crate::systems::states::waves::player::components::{Player, PlayerBundle};
use crate::systems::states::waves::player::resources::PlayerAnimations;
use crate::systems::states::waves::resources::TilesTextureAtlas;
use crate::systems::states::waves::weapons::components::{Bullet, WeaponArea, WeaponCooldown};
use crate::systems::states::waves::weapons::resources::WeaponsLibrary;
use bevy::camera::Camera2d;
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::image::TextureAtlas;
use bevy::math::Vec3;
use bevy::prelude::TimerMode::Repeating;
use bevy::prelude::*;
use rand::RngExt;
use std::f32::consts;

pub fn spawn_background(mut commands: Commands, atlas: Res<TilesTextureAtlas>) {
    let mut rng = rand::rng();
    for x in 0..TILES_X + 1 {
        for y in 0..TILES_Y + 1 {
            let pos_x = tiles_to_pixels(x as f32 - TILES_X as f32 / 2.0);
            let pos_y = tiles_to_pixels(y as f32 - TILES_Y as f32 / 2.0);

            // Use different tile indices for variety
            let tile_index = rng.random_range(0..4);

            commands.spawn((
                LevelBackground {},
                Sprite::from_atlas_image(
                    atlas.texture.clone(),
                    TextureAtlas {
                        layout: atlas.layout.clone(),
                        index: tile_index,
                    },
                ),
                Transform::from_xyz(pos_x, pos_y, -10.0),
            ));
            commands.spawn((
                LevelOverlay,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ZIndex(50),
            ));
        }
    }
}
pub fn despawn_background(
    mut commands: Commands,
    background: Single<Entity, With<LevelBackground>>,
) {
    commands.entity(background.entity()).despawn();
}

pub fn spawn_entities(
    mut commands: Commands,
    camera: Option<Single<&Camera2d>>,
    player: Option<Single<&Player>>,
    player_animations: Res<PlayerAnimations>,
    weapons_resource: Res<WeaponsLibrary>,
    mut atlas: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) -> Result {
    // Camera
    if camera.is_none() {
        commands.spawn((Camera2d, Msaa::Sample4));
    }
    if player.is_none() {
        let player_entity = commands
            .spawn((
                PlayerBundle::default(),
                Sprite {
                    image: PlayerAnimations::get_image_handle(assets.as_ref(), IDLE),
                    texture_atlas: Some(TextureAtlas {
                        layout: PlayerAnimations::get_layout(atlas.as_mut(), IDLE),
                        index: 0,
                    }),
                    ..default()
                },
                SpriteAnimator::new(player_animations.get(IDLE, EAST).unwrap()),
                Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(2.0)),
                children![(
                    Sprite::from_image(player_animations.shadow_texture.clone()),
                    Transform::from_xyz(0.0, -2.0, -1.0),
                )],
            ))
            .id();

        // // Give all weapons available to player
        let total_weapons = weapons_resource.weapons.len();
        let orbit_radius = (12.0 * weapons_resource.weapons.len() as f32).max(30.0); // Distance from player center
        let sector_arc = consts::TAU / (total_weapons as f32) * 0.8; // 80% of full sector

        for (index, weapon) in weapons_resource.weapons.iter().enumerate() {
            let angle = consts::TAU * (index as f32) / (total_weapons as f32);
            commands.entity(player_entity).with_child((
                weapon.clone(),
                WeaponCooldown {
                    timer: Timer::from_seconds(weapon.base_cooldown, Repeating),
                },
                Transform::from_xyz(angle.cos() * orbit_radius, angle.sin() * orbit_radius, 0.0),
                WeaponArea {
                    orbit_radius: orbit_radius.max(10.0),
                    center_arc: angle,
                    sector_arc,
                },
            ));
        }
    }
    Ok(())
}

pub fn despawn_entities(
    mut commands: Commands,
    entities: Query<Entity, With<Enemy>>,
    overlay: Query<Entity, With<LevelOverlay>>,
    bullets: Query<Entity, With<Bullet>>,
) {
    for entity in entities {
        commands.entity(entity).insert(MarkedForDespawn);
    }
    for entity in &overlay {
        commands.entity(entity).insert(MarkedForDespawn);
    }
    for entity in &bullets {
        commands.entity(entity).insert(MarkedForDespawn);
    }
}

const DYING_OVERLAY_TARGET_ALPHA: f32 = 0.80;
const DYING_OVERLAY_FADE_SPEED_PER_SEC: f32 = 1.4;

pub fn animate_game_over(
    time: Res<Time>,
    player: Single<Entity, (With<Dying>, With<Player>)>,
    mut background: Query<&mut BackgroundColor, With<LevelOverlay>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if player.is_empty() {
        return;
    }
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        camera_transform.translation.y += 1.;
    }

    for mut bg in background.iter_mut() {
        let alpha = bg.0.alpha();
        if alpha < DYING_OVERLAY_TARGET_ALPHA {
            bg.0.set_alpha(
                (alpha + time.delta_secs() * DYING_OVERLAY_FADE_SPEED_PER_SEC)
                    .min(DYING_OVERLAY_TARGET_ALPHA),
            );
        }
        return;
    }
}

pub fn animate_player(
    player_anims: Res<PlayerAnimations>,
    mut query: Query<
        (&Action, &Direction, &mut SpriteAnimator),
        (With<Player>, Or<(Changed<Action>, Changed<Direction>)>),
    >,
) {
    for (action, direction, mut animator) in &mut query {
        if let Some(handle) = player_anims.get(*action, *direction) {
            animator.switch(handle);
        }
    }
}

pub fn animate_enemy(
    enemy_animations: Res<EnemyAnimations>,
    mut query: Query<(&Direction, &Enemy, &mut SpriteAnimator), Changed<Direction>>,
) {
    for (direction, enemy, mut animator) in &mut query {
        if let Some(handle) = enemy_animations.get(enemy.kind, *direction) {
            animator.switch(handle);
        }
    }
}
