use crate::systems::game::GameState;
use crate::systems::states::waves::components::Health;
use crate::systems::states::waves::player::components::{Player, PlayerAction};
use crate::systems::states::waves::resources::WaveManager;
use bevy::prelude::{NextState, Query, Res, ResMut, Time, With};

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

pub fn dying(mut player_query: Query<(&Health, &mut PlayerAction), With<Player>>) {
    for (health, mut action) in player_query.iter_mut() {
        if health.value <= 0. && *action != PlayerAction::DYING {
            *action = PlayerAction::DYING;
        }
    }
}
