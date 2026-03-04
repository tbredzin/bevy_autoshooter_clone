use crate::systems::animations::messages::AnimationEnded;
use crate::systems::game::{GameOverStats, GameState};
use crate::systems::states::waves::components::{Action, Health};
use crate::systems::states::waves::player::components::Player;
use crate::systems::states::waves::player::experience::PlayerExperience;
use crate::systems::states::waves::resources::WaveManager;
use bevy::prelude::*;

pub fn reset_wave_timers(mut wave_manager: ResMut<WaveManager>) {
    wave_manager.wave_timer.reset();
    wave_manager.enemy_spawn_timer.reset();
}

pub fn update_wave_timer(
    mut wave_manager: ResMut<WaveManager>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    wave_manager.wave_timer.tick(time.delta());
    if wave_manager.wave_timer.just_finished() {
        next_state.set(GameState::UpgradeSelection);
    }
}

pub fn check_game_is_over(
    mut anim_ended_reader: MessageReader<AnimationEnded>,
    mut player_query: Query<(Entity, &Health, &mut Action, &PlayerExperience), With<Player>>,
    wave_manager: Res<WaveManager>,
    mut game_over_stats: ResMut<GameOverStats>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((player_entity, health, mut action, xp)) = player_query.single_mut() else {
        return;
    };

    if health.value > 0.0 {
        return;
    }

    // First frame health hits zero: trigger the dying clip and wait.
    if *action != Action::DYING {
        *action = Action::DYING;
        return;
    }

    // Once the non-repeating death clip finishes, AnimationEnded fires.
    for ev in anim_ended_reader.read() {
        if ev.entity == player_entity {
            game_over_stats.wave_reached = wave_manager.wave;
            game_over_stats.level_reached = xp.level;
            game_over_stats.experience_total = xp.value;
            next_state.set(GameState::GameOver);
        }
    }
}
