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

/// Hover feedback on the restart button
pub fn update_restart_button(
    mut query: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor), With<RestartButton>>,
) {
    for (interaction, mut bg, mut border) in &mut query {
        match interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0.2, 0.6, 0.2));
                *border = BorderColor::all(Color::srgb(0.4, 1.0, 0.4));
            }
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb(0.1, 0.35, 0.1));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0.15, 0.45, 0.15));
                *border = BorderColor::all(Color::srgb(0.3, 0.8, 0.3));
            }
        }
    }
}
