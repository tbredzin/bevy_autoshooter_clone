use crate::systems::constants::WAVE_DURATION;
use crate::systems::game::GameState;
use crate::systems::input::resources::ActionState;
use crate::systems::states::shopping::components::NextWaveButton;
use crate::systems::states::waves::components::Health;
use crate::systems::states::waves::player::components::{Player, PlayerStats};
use crate::systems::states::waves::player::experience::PlayerExperience;
use crate::systems::states::waves::resources::WaveManager;
use bevy::prelude::{Changed, NextState, Query, Res, ResMut, With};
use bevy::ui::Interaction;

pub fn start_next_wave(
    actions: Res<ActionState>,
    mut wave_manager: ResMut<WaveManager>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut PlayerStats, &mut PlayerExperience, &mut Health), With<Player>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NextWaveButton>)>,
) {
    let button_clicked = interaction_query.iter().any(|i| *i == Interaction::Pressed);
    let key_pressed = actions.start_next_wave;

    if !button_clicked && !key_pressed {
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
