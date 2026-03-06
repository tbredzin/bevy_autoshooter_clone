use crate::systems::animations::messages::AnimationEnded;
use crate::systems::game::{GameOverStats, GameState, MarkedForDespawn};
use crate::systems::states::waves::components::Action::DYING;
use crate::systems::states::waves::components::{Action, BackgroundMusic, Health};
use crate::systems::states::waves::enemy::components::Enemy;
use crate::systems::states::waves::player::components::Player;
use crate::systems::states::waves::player::experience::PlayerExperience;
use crate::systems::states::waves::resources::WaveManager;
use bevy::audio::Volume;
use bevy::prelude::*;

const MUSIC_FADEOUT_START_SECS: f32 = 5.0;

pub fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    let num: u8 = rand::random_range(1..3);
    let audio1 = asset_server.load(format!("musics/music{num}.ogg"));
    commands.spawn((
        BackgroundMusic,
        AudioPlayer::new(audio1),
        PlaybackSettings::LOOP,
    ));
}

pub fn stop_background_audio(
    mut commands: Commands,
    audio_query: Query<Entity, With<BackgroundMusic>>,
) {
    if let Ok(audio) = audio_query.single() {
        commands.entity(audio).insert(MarkedForDespawn);
    }
}

pub fn update_background_audio(
    wave_manager: Res<WaveManager>,
    mut audio_query: Query<&mut AudioSink, With<BackgroundMusic>>,
) {
    let remaining = wave_manager.wave_timer.remaining_secs();
    if remaining >= MUSIC_FADEOUT_START_SECS {
        return;
    }

    let Ok(mut sink) = audio_query.single_mut() else {
        return;
    };

    // t goes 1.0 → 0.0 as remaining goes from MUSIC_FADEOUT_START_SECS → 0
    let t = (remaining / MUSIC_FADEOUT_START_SECS).clamp(0.0, 1.0);
    sink.set_volume(Volume::Linear(t));
}

pub fn reset_wave_timers(mut wave_manager: ResMut<WaveManager>) {
    wave_manager.wave_timer.reset();
    wave_manager.enemy_spawn_timer.reset();
}

pub fn update_wave_timer(
    mut wave_manager: ResMut<WaveManager>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_query: Query<&Action, With<Player>>,
    time: Res<Time>,
) {
    let Ok((action)) = player_query.single_mut() else {
        return;
    };
    if *action == DYING {
        return;
    }

    wave_manager.wave_timer.tick(time.delta());
    if wave_manager.wave_timer.just_finished() {
        next_state.set(GameState::UpgradeSelection);
    }
}
const Y_SORT_BASE: f32 = 25.0;
const Y_SORT_SCALE: f32 = 0.01;

pub fn y_sort_enemies(mut query: Query<&mut Transform, With<Enemy>>) {
    for mut transform in &mut query {
        transform.translation.z = Y_SORT_BASE - transform.translation.y * Y_SORT_SCALE;
    }
}

pub fn y_sort_player(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut query {
        transform.translation.z = Y_SORT_BASE - transform.translation.y * Y_SORT_SCALE;
    }
}

pub fn check_game_is_over(
    mut anim_ended_reader: MessageReader<AnimationEnded>,
    mut player_query: Query<(Entity, &Health, &mut Action, &PlayerExperience), With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    wave_manager: Res<WaveManager>,
    mut game_over_stats: ResMut<GameOverStats>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    let Ok((player_entity, health, mut action, xp)) = player_query.single_mut() else {
        return;
    };

    if health.value > 0.0 {
        return;
    }

    // First frame health hits zero: trigger the dying clip and wait.
    if *action != DYING {
        *action = DYING;
        return;
    }
    for e in enemy_query {
        commands.entity(e).insert(MarkedForDespawn);
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
