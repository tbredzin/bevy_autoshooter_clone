use crate::systems::constants::{SPAWN_RATE, TILE_SIZE, WAVE_DURATION};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::UVec2;
use bevy::prelude::{FromWorld, Resource, Timer, TimerMode, World};

#[derive(Resource)]
pub struct WaveManager {
    pub wave: u32,
    pub wave_timer: Timer,
    pub enemy_spawn_timer: Timer,
}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            wave: 3,
            wave_timer: Timer::from_seconds(WAVE_DURATION, TimerMode::Once),
            enemy_spawn_timer: Timer::from_seconds(SPAWN_RATE, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct TilesTextureAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for TilesTextureAtlas {
    fn from_world(world: &mut World) -> Self {
        let texture = { world.resource::<AssetServer>() }.load("spritesheet/spritesheet_tiles.png");
        let mut atlas = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let layout = atlas.add(TextureAtlasLayout::from_grid(
            UVec2::splat(TILE_SIZE as u32), // tile size (width, height)
            27,                             // columns
            20,                             // rows
            Some(UVec2::splat(10)),         // padding
            None,                           // no offset
        ));
        TilesTextureAtlas { texture, layout }
    }
}
