use crate::systems::game::GameState;
use crate::systems::states::waves::resources::WaveManager;
use bevy::prelude::{NextState, Res, ResMut, Time};

pub fn reset_wave_timers(mut wave_manager: ResMut<WaveManager>) {
    wave_manager.wave_timer.reset();
    wave_manager.enemy_spawn_timer.reset();
}

pub fn update_wave_timer(
    mut wave_manager: ResMut<WaveManager>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    wave_manager.wave_timer.tick(time.delta());
    if wave_manager.wave_timer.just_finished() {
        next_state.set(GameState::UpgradeSelection);
    }
}
