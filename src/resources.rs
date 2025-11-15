use bevy::prelude::*;

pub const TILE_SIZE: f32 = 64.0; // pixels per tile
pub const TILES_X: u32 = 40; // horizontal tiles
pub const TILES_Y: u32 = 30; // vertical tiles

// Window/viewport size
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 960.0;

pub const fn tiles_to_pixels(tiles: f32) -> f32 {
    tiles * TILE_SIZE
}

// Game constants (in pixels for physics/collision)
pub const PLAYER_SPEED: f32 = 200.0;
pub const ENEMY_SPEED: f32 = 100.0;
pub const BULLET_SPEED: f32 = 500.0;
pub const FIRE_RATE: f32 = 0.5;
pub const WAVE_DURATION: f32 = 20.0;
pub const SPAWN_RATE: f32 = 0.5;

// Game area calculated from tile dimensions (exact whole tiles)
pub const GAME_AREA: Rect = Rect {
    min: Vec2 {
        x: -(TILES_X as f32 * TILE_SIZE) / 2.0, // -640.0
        y: -(TILES_Y as f32 * TILE_SIZE) / 2.0, // -480.0
    },
    max: Vec2 {
        x: (TILES_X as f32 * TILE_SIZE) / 2.0, // 640.0
        y: (TILES_Y as f32 * TILE_SIZE) / 2.0, // 480.0
    },
};

#[derive(Resource)]
pub struct GameState {
    pub wave: u32,
    pub xp: u32,
    pub level: u32,
    pub wave_timer: f32,
    pub enemy_spawn_timer: f32,
    pub in_wave: bool,
    pub health: f32,
    pub max_health: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            wave: 1,
            xp: 0,
            level: 1,
            wave_timer: WAVE_DURATION,
            enemy_spawn_timer: 0.0,
            in_wave: true,
            health: 100.0,
            max_health: 100.0,
        }
    }
}

#[derive(Resource)]
pub struct TilesTextureAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
