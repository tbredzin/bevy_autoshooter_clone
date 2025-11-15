use crate::components::{Enemy, Health, Player, Spawning};
use crate::messages::EnemyDeathMessage;
use crate::resources::{
    ENEMY_HEALTH, ENEMY_SPAWN_TIME_IN_S, ENEMY_SPEED, GAME_AREA, SPAWN_RATE, WaveManager,
    WaveState, tiles_to_pixels,
};
use bevy::prelude::*;
use rand::Rng;

pub fn update_spawning(
    mut commands: Commands,
    mut wave_manager: ResMut<WaveManager>,
    player_query: Query<&GlobalTransform, With<Player>>,
    time: Res<Time>,
) {
    if wave_manager.wave_state == WaveState::Ended {
        return;
    }

    wave_manager.enemy_spawn_timer.tick(time.delta());

    if wave_manager.enemy_spawn_timer.is_finished() {
        let mut rng = rand::rng();

        let Ok(player_transform) = player_query.single() else {
            return;
        };

        let player_pos = player_transform.translation().truncate();
        let min_distance = tiles_to_pixels(2.0);
        let max_distance = tiles_to_pixels(50.0);

        // Generate random spawn position
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

        // Adjust spawn rate based on wave (faster spawning each wave)
        let base_rate = SPAWN_RATE / wave_manager.wave as f32;
        wave_manager
            .enemy_spawn_timer
            .set_duration(std::time::Duration::from_secs_f32(base_rate.max(0.01)));
    }
}

pub fn update_spawned(
    mut commands: Commands,
    mut pre_spawn_query: Query<(Entity, &mut Spawning)>,
    time: Res<Time>,
) {
    for (entity, mut spawning) in &mut pre_spawn_query {
        spawning.timer.tick(time.delta());

        if spawning.timer.is_finished() {
            commands.entity(entity).remove::<Spawning>();
            commands.entity(entity).insert((
                Enemy { damage: 10.0 },
                Health {
                    value: ENEMY_HEALTH,
                    max: ENEMY_HEALTH,
                },
            ));
        }
    }
}

pub fn update_move(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation();

    for mut enemy_transform in &mut enemy_query {
        let direction = (player_pos - enemy_transform.translation).normalize();
        enemy_transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}

pub fn check_if_dead(
    mut commands: Commands,
    mut message_writer: MessageWriter<EnemyDeathMessage>,
    query: Query<(Entity, &Health), With<Enemy>>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            message_writer.write(EnemyDeathMessage(entity));
            commands.entity(entity).despawn(); // Optionally despawn the enemy
        }
    }
}
