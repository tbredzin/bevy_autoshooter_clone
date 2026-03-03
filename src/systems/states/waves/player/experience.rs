use crate::systems::constants::NEXT_LEVEL_RATIO_PERCENT;
use crate::systems::states::waves::enemy::messages::EnemyDeathMessage;
use crate::systems::states::waves::player::components::Player;
use crate::systems::states::waves::player::components::PlayerStats;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct PlayerExperience {
    pub value: u32,
    pub level: u32,
    pub new_levels: u32,
}

impl Default for PlayerExperience {
    fn default() -> Self {
        Self {
            value: 0,
            level: 1,
            new_levels: 0,
        }
    }
}
pub fn handle_enemy_death(
    mut msg_reader: MessageReader<EnemyDeathMessage>,
    mut player_query: Query<(&mut PlayerExperience, &mut PlayerStats), With<Player>>,
) {
    for event in msg_reader.read() {
        let Ok((experience, stats)) = &mut player_query.single_mut() else {
            return;
        };

        // Level up check
        experience.value += event.xp_reward;
        if experience.value >= experience.level * NEXT_LEVEL_RATIO_PERCENT {
            experience.level += 1;
            experience.new_levels += 1;
            stats.max_health += 10.0;
        }
    }
}
