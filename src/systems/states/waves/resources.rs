use crate::resources::{SPAWN_RATE, WAVE_DURATION};
use bevy::prelude::{Resource, Timer, TimerMode};

#[derive(Resource)]
pub struct WaveManager {
    pub wave: u32,
    pub wave_timer: Timer,
    pub enemy_spawn_timer: Timer,
}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            wave: 1,
            wave_timer: Timer::from_seconds(WAVE_DURATION, TimerMode::Once),
            enemy_spawn_timer: Timer::from_seconds(SPAWN_RATE, TimerMode::Repeating),
        }
    }
}
