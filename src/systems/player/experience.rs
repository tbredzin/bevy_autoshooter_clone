use crate::components::{Health, PlayerExperience};
use crate::messages::EnemyDeathMessage;
use crate::systems::player::components::Player;
use bevy::prelude::*;

pub fn handle_enemy_death(
    mut msg_reader: MessageReader<EnemyDeathMessage>,
    mut player_query: Query<(&mut PlayerExperience, &mut Health), With<Player>>,
) {
    for event in msg_reader.read() {
        // Handle the enemy death, e.g., update score or play sound
        println!("Enemy {:?} has died!", event.0);
        let Ok((experience, health)) = &mut player_query.single_mut() else {
            return;
        };
        experience.value += 1;
        // Level up check
        if experience.value >= experience.level * 10 {
            experience.level += 1;
            experience.levels_gained_this_wave += 1;
            health.max += 10.0;
        }
    }
}
