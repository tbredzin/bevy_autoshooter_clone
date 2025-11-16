use crate::components::{Enemy, HUDText, Health, Player, PlayerExperience};
use crate::resources::{WaveManager, WaveState};
use bevy::prelude::*;

pub fn update_ui(
    mut ui_query: Query<&mut Text, With<HUDText>>,
    wave_manager: Res<WaveManager>,
    player_query: Query<(&PlayerExperience, &Health), With<Player>>,
    enemy_query: Query<&Enemy>,
) {
    let enemy_count = enemy_query.iter().count();

    let Ok((player_xp, player_health)) = player_query.single() else {
        return;
    };

    for mut text in &mut ui_query {
        **text = format!(
            "Wave: {} | XP: {} | Level: {} | HP: {:.0}/{:.0} | Enemies: {} | {}",
            wave_manager.wave,
            player_xp.value,
            player_xp.level,
            player_health.value,
            player_health.max,
            enemy_count,
            match wave_manager.wave_state {
                WaveState::Running =>
                    format!("Time: {:.1}s", wave_manager.wave_timer.remaining_secs()),
                WaveState::Ended => "Press SPACE or ENTER to start next wave".to_string(),
            }
        );
    }
}
