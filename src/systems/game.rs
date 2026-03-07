use crate::systems::animations::animation::SpriteAnimation;
use crate::systems::animations::animator::SpriteAnimator;
use crate::systems::constants::{tiles_to_pixels, GAME_AREA};
use crate::systems::states::waves::components::Action::IDLE;
use crate::systems::states::waves::components::Direction::EAST;
use crate::systems::states::waves::player::components::{Player, PlayerBundle};
use crate::systems::states::waves::player::resources::PlayerAnimations;
use crate::systems::states::waves::weapons::components::WeaponBundle;
use crate::systems::states::waves::weapons::messages::WeaponSpawnedMessage;
use crate::systems::states::waves::weapons::resources::WeaponsLibrary;
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InWave,
    UpgradeSelection,
    Shopping,
    GameOver,
}
#[derive(Resource, Default)]
pub struct GameOverStats {
    pub wave_reached: u32,
    pub level_reached: u32,
    pub experience_total: u32,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct MusicVolume(pub u32);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct SoundEffectVolume(pub u32);

#[derive(Component)]
pub struct MarkedForDespawn;

pub fn out_of_bounds_system(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<MarkedForDespawn>>,
) {
    const MARGIN: f32 = tiles_to_pixels(2.0);

    for (entity, transform) in &query {
        let entity_pos = transform.translation().truncate();

        if entity_pos.x < GAME_AREA.min.x - MARGIN
            || entity_pos.x > GAME_AREA.max.x + MARGIN
            || entity_pos.y < GAME_AREA.min.y - MARGIN
            || entity_pos.y > GAME_AREA.max.y + MARGIN
        {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_marked_entities(
    mut commands: Commands,
    query: Query<Entity, With<MarkedForDespawn>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Sample4));
}

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
}

impl TextBundle {
    pub fn new(text: impl Into<String>, font_size: f32, color: Color) -> Self {
        Self {
            text: Text::new(text),
            font: TextFont {
                font_size,
                ..default()
            },
            color: TextColor::from(color),
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    player_query: Option<Single<Entity, With<Player>>>,
    player_animations: Res<PlayerAnimations>,
    animations: Res<Assets<SpriteAnimation>>,
    weapons_resource: Res<WeaponsLibrary>,
    mut events: MessageWriter<WeaponSpawnedMessage>,
) {
    if player_query.is_none() {
        let player_anim = player_animations.get(IDLE, EAST).unwrap();
        let sprite = animations.get(player_anim.id()).unwrap().to_sprite();

        let player = commands
            .spawn((
                PlayerBundle::default(),
                sprite,
                SpriteAnimator::new(player_anim),
                Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(2.0)),
                children![(
                    Sprite::from_image(player_animations.shadow_texture.clone()),
                    Transform::from_xyz(0.0, -2.0, -1.0),
                )],
            ))
            .id();

        // Spawn with 1  weapon
        let weapon = weapons_resource.weapons.get(0).unwrap();
        let weapon_bundle = WeaponBundle::new(
            format!("{:?}-{}", weapon.kind, 0),
            weapon.clone(),
            weapon.base_cooldown,
        );
        let weapon_entity = commands.spawn(weapon_bundle.clone()).id();
        commands.entity(player).add_child(weapon_entity);
        events.write(WeaponSpawnedMessage {
            name: weapon_bundle.name,
            weapon: weapon_bundle.weapon,
            entity: weapon_entity,
            player,
        });
    }
}
