use crate::components::{Enemy, HUDText, Health, PlayerExperience};
use crate::resources::WaveManager;
use crate::systems::player::components::Player;
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
            "Wave: {} | XP: {} | Level: {} | HP: {:.0}/{:.0} | New Level: {} | Enemies: {} | {}",
            wave_manager.wave,
            player_xp.value,
            player_xp.level,
            player_health.value,
            player_health.max,
            player_xp.levels_gained_this_wave,
            enemy_count,
            format!("Time: {:.1}s", wave_manager.wave_timer.remaining_secs())
        );
    }
}
