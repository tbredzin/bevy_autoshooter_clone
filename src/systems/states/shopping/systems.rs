use crate::resources::WAVE_DURATION;
use crate::systems::game::GameState;
use crate::systems::input::resources::ActionState;
use crate::systems::states::waves::player::components::Health;
use crate::systems::states::waves::player::components::{Player, PlayerStats};
use crate::systems::states::waves::player::experience::PlayerExperience;
use crate::systems::states::waves::resources::WaveManager;
use bevy::prelude::{NextState, Query, Res, ResMut, With};

pub fn start_next_wave(
    actions: Res<ActionState>,
    mut wave_manager: ResMut<WaveManager>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut PlayerStats, &mut PlayerExperience, &mut Health), With<Player>>,
) {
    if !actions.start_next_wave {
        return;
    }

    wave_manager.wave += 1;
    wave_manager
        .wave_timer
        .set_duration(std::time::Duration::from_secs_f32(WAVE_DURATION));
    for (stats, mut xp, mut health) in &mut player_query {
        xp.new_levels = 0;
        health.value = stats.max_health;
    }
    next_state.set(GameState::InWave);
}
