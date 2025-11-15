use crate::components::HUDText;
use crate::resources::{GameState, WaveState};
use bevy::prelude::*;

pub fn update_ui(mut ui_query: Query<&mut Text, With<HUDText>>, game_state: Res<GameState>) {
    for mut text in &mut ui_query {
        **text = format!(
            "Wave: {} | XP: {} | Level: {} | HP: {:.0}/{:.0} | {}",
            game_state.wave,
            game_state.xp,
            game_state.level,
            game_state.health,
            game_state.max_health,
            match game_state.wave_state {
                WaveState::Running => format!("Time: {:.1}s", game_state.wave_timer),
                WaveState::Ended => "Press SPACE or ENTER to start next wave".to_string(),
            }
        );
    }
}
