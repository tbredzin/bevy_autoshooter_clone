use bevy::prelude::*;

pub const TILE_SIZE: f32 = 64.0;
pub const TILES_X: u32 = 20;
pub const TILES_Y: u32 = 15;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 960.0;

pub const fn tiles_to_pixels(tiles: f32) -> f32 {
    tiles * TILE_SIZE
}

pub const ENEMY_SPEED: f32 = 80.0;
pub const ENEMY_HEALTH: f32 = 4.0;
pub const BULLET_SPEED: f32 = 500.0;
pub const WAVE_DURATION: f32 = 30.0;
pub const SPAWN_RATE: f32 = 0.5;
pub const ENEMY_SPAWN_TIME_IN_S: f32 = 2.0;
pub const ENEMY_BASE_XP: u32 = 1;
pub const NEXT_LEVEL_RATIO_PERCENT: u32 = 10;

pub const GAME_AREA: Rect = Rect {
    min: Vec2 {
        x: -(TILES_X as f32 * TILE_SIZE) / 2.0,
        y: -(TILES_Y as f32 * TILE_SIZE) / 2.0,
    },
    max: Vec2 {
        x: (TILES_X as f32 * TILE_SIZE) / 2.0,
        y: (TILES_Y as f32 * TILE_SIZE) / 2.0,
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WaveState {
    Running,
    Ended,
}

#[derive(Resource)]
pub struct WaveManager {
    pub wave: u32,
    pub wave_timer: Timer,
    pub enemy_spawn_timer: Timer,
    pub wave_state: WaveState,
}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            wave: 1,
            wave_timer: Timer::from_seconds(WAVE_DURATION, TimerMode::Once),
            enemy_spawn_timer: Timer::from_seconds(SPAWN_RATE, TimerMode::Repeating),
            wave_state: WaveState::Running,
        }
    }
}

#[derive(Resource)]
pub struct TilesTextureAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct HUDTextureAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
