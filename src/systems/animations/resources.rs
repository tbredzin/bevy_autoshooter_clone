use crate::systems::animations::components::Animation;
use crate::systems::states::waves::player::components::{Direction, PlayerAction};
use bevy::prelude::*;

const IDLE_SPRITESHEET_PATH: &'static str = "spritesheet/player/Idle_spritesheet_8x6.png";
const WALK_SPRITESHEET_PATH: &'static str = "spritesheet/player/walk_spritesheet_8x6.png";
const DASH_SPRITESHEET_PATH: &'static str = "spritesheet/player/dash_spritesheet_8x6.png";
const DASH_DUST_SPRITESHEET_PATH: &'static str = "spritesheet/player/dash_dust_spritesheet_8x6.png";
const DEATH_SPRITESHEET_PATH: &'static str = "spritesheet/player/death_spritesheet_8x6.png";
const SHADOW_SPRITE_PATH: &'static str = "spritesheet/player/shadow_sprite.png";
const DEATH_SHADOW_SPRITESHEET_PATH: &'static str =
    "spritesheet/player/death_shadow_spritesheet_8x6.png";

const IDLE_ANIMATION_IN_SEC: f32 = 0.12;
const WALK_ANIMATION_IN_SEC: f32 = 0.2;
const DASH_ANIMATION_IN_SEC: f32 = 0.12;
const DYING_ANIMATION_IN_SEC: f32 = 0.50;

#[derive(Resource)]
pub struct AnimationAssets {
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub idle_texture: Handle<Image>,

    pub walk_layout: Handle<TextureAtlasLayout>,
    pub walk_texture: Handle<Image>,

    pub dash_layout: Handle<TextureAtlasLayout>,
    pub dash_texture: Handle<Image>,
    pub dash_dust_layout: Handle<TextureAtlasLayout>,
    pub dash_dust_texture: Handle<Image>,

    pub death_layout: Handle<TextureAtlasLayout>,
    pub death_texture: Handle<Image>,

    pub death_shdw_layout: Handle<TextureAtlasLayout>,
    pub death_shadow_texture: Handle<Image>,

    pub shadow_texture: Handle<Image>,
}

impl FromWorld for AnimationAssets {
    fn from_world(world: &mut World) -> Self {
        // let mut atlas = world.resource_mut::<Assets<TextureAtlasLayout>>();

        // Each frame is 48x64, arranged in 8 columns and 6 rows
        let size = UVec2::new(48, 64);
        let col = 8;
        let row = 6;

        // Load textures
        AnimationAssets {
            idle_texture: world.resource::<AssetServer>().load(IDLE_SPRITESHEET_PATH),
            walk_texture: world.resource::<AssetServer>().load(WALK_SPRITESHEET_PATH),
            dash_texture: world.resource::<AssetServer>().load(DASH_SPRITESHEET_PATH),
            dash_dust_texture: world
                .resource::<AssetServer>()
                .load(DASH_DUST_SPRITESHEET_PATH),
            death_texture: world.resource::<AssetServer>().load(DEATH_SPRITESHEET_PATH),
            death_shadow_texture: world
                .resource::<AssetServer>()
                .load(DEATH_SHADOW_SPRITESHEET_PATH),
            shadow_texture: world.resource::<AssetServer>().load(SHADOW_SPRITE_PATH),

            idle_layout: world
                .resource_mut::<Assets<TextureAtlasLayout>>()
                .add(TextureAtlasLayout::from_grid(size, col, row, None, None)),
            walk_layout: world
                .resource_mut::<Assets<TextureAtlasLayout>>()
                .add(TextureAtlasLayout::from_grid(size, col, row, None, None)),
            dash_layout: world
                .resource_mut::<Assets<TextureAtlasLayout>>()
                .add(TextureAtlasLayout::from_grid(size, col, row, None, None)),
            dash_dust_layout: world
                .resource_mut::<Assets<TextureAtlasLayout>>()
                .add(TextureAtlasLayout::from_grid(size, col, row, None, None)),
            death_layout: world
                .resource_mut::<Assets<TextureAtlasLayout>>()
                .add(TextureAtlasLayout::from_grid(size, col, row, None, None)),
            death_shdw_layout: world
                .resource_mut::<Assets<TextureAtlasLayout>>()
                .add(TextureAtlasLayout::from_grid(size, col, row, None, None)),
        }
    }
}

impl AnimationAssets {
    pub fn get_sprite(&self, state: &PlayerAction) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
        match state {
            PlayerAction::IDLE => (self.idle_texture.clone(), self.idle_layout.clone()),
            PlayerAction::WALKING => (self.walk_texture.clone(), self.walk_layout.clone()),
            PlayerAction::DASHING => (self.dash_texture.clone(), self.dash_layout.clone()),
            PlayerAction::DYING => (self.death_texture.clone(), self.death_layout.clone()),
        }
    }
    pub fn get_animation(&self, state: &PlayerAction, direction: &Direction) -> Animation {
        // Find the sprite row
        let row = match direction {
            Direction::SOUTH => 0,
            Direction::SOUTHWEST => 1,
            Direction::WEST => 1,
            Direction::NORTHWEST => 2,
            Direction::NORTH => 3,
            Direction::NORTHEAST => 4,
            Direction::SOUTHEAST => 5,
            Direction::EAST => 5,
        };

        // Different states might have different frame counts and may be repeated
        let (frame_count, repeated) = match state {
            PlayerAction::IDLE => (8, true),
            PlayerAction::WALKING => (8, true),
            PlayerAction::DASHING => (8, false),
            PlayerAction::DYING => (8, false),
        };
        let timer = match state {
            PlayerAction::IDLE => Timer::from_seconds(IDLE_ANIMATION_IN_SEC, TimerMode::Repeating),
            PlayerAction::WALKING => {
                Timer::from_seconds(WALK_ANIMATION_IN_SEC, TimerMode::Repeating)
            }
            PlayerAction::DASHING => {
                Timer::from_seconds(DASH_ANIMATION_IN_SEC, TimerMode::Repeating)
            }
            PlayerAction::DYING => {
                Timer::from_seconds(DYING_ANIMATION_IN_SEC, TimerMode::Repeating)
            }
        };

        Animation {
            timer,
            first: row * frame_count,
            last: (row * frame_count) + frame_count - 1,
            repeated,
        }
    }
}
