use crate::components::{Enemy, Player, Spawning};
use crate::resources::{
    tiles_to_pixels, GameState, WaveState, ENEMY_HEALTH, ENEMY_SPAWN_TIME_IN_S, ENEMY_SPEED, GAME_AREA,
    SPAWN_RATE,
};
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

pub fn update_spawning(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if game_state.wave_state == WaveState::Ended {
        return;
    }

    game_state.enemy_spawn_timer.tick(time.delta());

    if game_state.enemy_spawn_timer.is_finished() {
        let mut rng = rand::rng();

        let Ok(player_transform) = player_query.single() else {
            return;
        };

        let player_pos = player_transform.translation.truncate();
        let min_distance = tiles_to_pixels(2.0);
        let max_distance = tiles_to_pixels(50.0);

        // Generate a random spawn position
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let distance = rng.random_range(min_distance..max_distance);
        let x = (player_pos.x + angle.cos() * distance).clamp(GAME_AREA.min.x, GAME_AREA.max.x);
        let y = (player_pos.y + angle.sin() * distance).clamp(GAME_AREA.min.y, GAME_AREA.max.y);
        let spawn_pos = Vec3::new(x, y, 0.0);

        // Spawn warning indicator
        commands.spawn((
            Transform::from_translation(spawn_pos),
            Spawning {
                timer: Timer::from_seconds(ENEMY_SPAWN_TIME_IN_S, TimerMode::Once),
            },
        ));

        // Adjust spawn rate based on wave
        let new_duration = (SPAWN_RATE / game_state.wave as f32).max(0.01);
        game_state
            .enemy_spawn_timer
            .set_duration(Duration::from_secs_f32(new_duration));
    }
}

pub fn update_spawned(
    mut commands: Commands,
    mut spawning_query: Query<(Entity, &mut Spawning)>,
    time: Res<Time>,
) {
    for (entity, mut spawning) in &mut spawning_query {
        spawning.timer.tick(time.delta());
        if spawning.timer.is_finished() {
            commands.entity(entity).remove::<Spawning>();
            commands.entity(entity).insert((Enemy {
                health: ENEMY_HEALTH,
            },));
        }
    }
}

pub fn update_move(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        for mut enemy_transform in &mut enemy_query {
            let direction =
                (player_transform.translation - enemy_transform.translation).normalize();
            enemy_transform.translation += direction * ENEMY_SPEED * time.delta_secs();
        }
    }
}
