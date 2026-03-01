use crate::systems::animations::components::{PlayerAnimationBundle, ShadowSprite};
use crate::systems::animations::resources::AnimationAssets;
use crate::systems::constants::{tiles_to_pixels, TILES_X, TILES_Y};
use crate::systems::game::MarkedForDespawn;
use crate::systems::states::waves::components::LevelBackground;
use crate::systems::states::waves::enemy::components::Enemy;
use crate::systems::states::waves::player::components::{Player, PlayerBundle};
use crate::systems::states::waves::resources::TilesTextureAtlas;
use crate::systems::states::waves::weapons::components::{WeaponArea, WeaponCooldown};
use crate::systems::states::waves::weapons::resources::WeaponsLibrary;
use bevy::camera::Camera2d;
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
    animations: Res<AnimationAssets>,
    weapons_resource: Res<WeaponsLibrary>,
) -> Result {
    // Camera
    if camera.is_none() {
        commands.spawn((Camera2d, Msaa::Sample4));
    }
    if player.is_none() {
        let position = Vec3::ZERO;
        let player_entity = commands
            .spawn((
                PlayerBundle::default(),
                PlayerAnimationBundle::new(
                    animations.idle_texture.clone(),
                    animations.idle_layout.clone(),
                    0,
                    7,
                ),
                Transform::from_translation(position).with_scale(Vec3::splat(2.0)),
                children![(
                    Sprite::from_image(animations.shadow_texture.clone()),
                    Transform::from_translation(position - Vec3::new(0.0, 2.0, -1.0)), // Slightly below player
                    ShadowSprite,
                )],
            ))
            .id();

        // Give all weapons available to player
        let total_weapons = weapons_resource.weapons.len();
        let orbit_radius = 12.0 * weapons_resource.weapons.len() as f32; // Distance from player center
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

pub fn despawn_entities(mut commands: Commands, entities: Query<Entity, With<Enemy>>) {
    for entity in entities {
        commands.entity(entity).insert(MarkedForDespawn);
    }
}
