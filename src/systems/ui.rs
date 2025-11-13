use crate::components::UIText;
use crate::resources::GameState;
use bevy::prelude::*;

pub fn update_ui(mut ui_query: Query<&mut Text, With<UIText>>, game_state: Res<GameState>) {
    for mut text in &mut ui_query {
        let status = if game_state.in_wave {
            format!("Time: {:.1}s", game_state.wave_timer)
        } else {
            format!("Next wave in: {:.1}s", 3.0 - game_state.wave_timer)
        };
        **text = format!(
            "Wave: {} | XP: {} | Level: {} | HP: {:.0}/{:.0} | {}",
            game_state.wave,
            game_state.xp,
            game_state.level,
            game_state.health,
            game_state.max_health,
            status
        );
    }
}
