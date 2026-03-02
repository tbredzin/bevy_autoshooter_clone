use crate::systems::game::{GameOverStats, GameState};
use crate::systems::input::resources::ActionState;
use crate::systems::states::gameover::components::RestartButton;
use crate::systems::states::waves::player::components::Player;
use crate::systems::states::waves::resources::WaveManager;
use bevy::prelude::*;

pub fn handle_restart(
    actions: Res<ActionState>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut wave_manager: ResMut<WaveManager>,
    mut game_over_stats: ResMut<GameOverStats>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    let button_clicked = interaction_query.iter().any(|i| *i == Interaction::Pressed);
    let key_pressed = actions.start_next_wave;

    if !button_clicked && !key_pressed {
        return;
    }

    *wave_manager = WaveManager::default();
    *game_over_stats = GameOverStats::default();

    for entity in &player_query {
        commands.entity(entity).despawn();
    }

    next_state.set(GameState::InWave);
}
