use crate::systems::animations::components::{Animation, PlayerSprite};
use crate::systems::game::{GameOverStats, GameState};
use crate::systems::states::waves::components::Health;
use crate::systems::states::waves::player::components::{Player, PlayerAction};
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
    sprite_query: Query<(&Animation, &Sprite), With<PlayerSprite>>,
    mut player_query: Query<(&Health, &mut PlayerAction, &PlayerExperience), With<Player>>,
    wave_manager: Res<WaveManager>,
    mut game_over_stats: ResMut<GameOverStats>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((health, mut action, xp)) = player_query.single_mut() else {
        return;
    };

    if health.value > 0.0 {
        return;
    }

    if *action != PlayerAction::DYING {
        *action = PlayerAction::DYING;
    }

    let Ok((indices, sprite)) = sprite_query.single() else {
        return;
    };
    let Some(atlas) = &sprite.texture_atlas else {
        return;
    };

    // Dying animation is non-repeating; once we hit the last frame, go to GameOver
    if atlas.index >= indices.last {
        game_over_stats.wave_reached = wave_manager.wave;
        game_over_stats.level_reached = xp.level;
        game_over_stats.experience_total = xp.value;
        next_state.set(GameState::GameOver);
    }
}
