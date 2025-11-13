use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 200.0;
pub const ENEMY_SPEED: f32 = 80.0;
pub const BULLET_SPEED: f32 = 500.0;
pub const FIRE_RATE: f32 = 0.5;
pub const WAVE_DURATION: f32 = 20.0;
pub const SPAWN_RATE: f32 = 0.5;

pub const GAME_AREA: Rect = Rect {
    min: Vec2 {
        x: -600.0,
        y: -300.0,
    },
    max: Vec2 { x: 600.0, y: 300.0 },
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
