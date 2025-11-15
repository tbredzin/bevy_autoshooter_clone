use crate::components::{Enemy, HUDText};
use crate::resources::{GameState, WaveState};
use bevy::prelude::*;

pub fn update_ui(
    mut ui_query: Query<&mut Text, With<HUDText>>,
    enemy_query: Query<&Enemy>,
    game_state: Res<GameState>,
) {
    // Update every frame to show real-time enemy count
    let enemy_count = enemy_query.iter().count();

    for mut text in &mut ui_query {
        **text = format!(
            "Wave: {} | XP: {} | Level: {} | HP: {:.0}/{:.0} | Enemies: {} | {}",
            game_state.wave,
            game_state.xp,
            game_state.level,
            game_state.health,
            game_state.max_health,
            enemy_count,
            match game_state.wave_state {
                WaveState::Running =>
                    format!("Time: {:.1}s", game_state.wave_timer.remaining_secs()),
                WaveState::Ended => "Press SPACE or ENTER to start next wave".to_string(),
            }
        );
    }
}
