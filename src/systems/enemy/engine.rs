use crate::components::{Enemy, Player, Spawning};
use crate::resources::{
    ENEMY_HEALTH, ENEMY_SPAWN_TIME_IN_S, ENEMY_SPEED, GAME_AREA, GameState, SPAWN_RATE, WaveState,
    tiles_to_pixels,
};
use bevy::prelude::*;
use rand::Rng;

pub fn update_spawning(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) -> Result {
    if game_state.wave_state == WaveState::Ended {
        return Ok(());
    }

    game_state.enemy_spawn_timer -= time.delta_secs();

    if game_state.enemy_spawn_timer <= 0.0 {
        let mut rng = rand::rng();

        let player_pos = player_query.single()?.translation.xy();
        let min_distance = tiles_to_pixels(2.0);
        let max_distance = tiles_to_pixels(50.0);

        // Generate a random spawn position
        let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);
        let distance = rng.random_range(min_distance..max_distance);
        let x = (player_pos.x + angle.cos() * distance).clamp(GAME_AREA.min.x, GAME_AREA.max.x);
        let y = (player_pos.y + angle.sin() * distance).clamp(GAME_AREA.min.y, GAME_AREA.max.y);
        let spawn_pos = Vec3::new(x, y, 0.0);

        // Spawn warning indicator
        commands.spawn((
            Transform::from_translation(spawn_pos),
            Spawning {
                timer: ENEMY_SPAWN_TIME_IN_S,
            },
        ));

        game_state.enemy_spawn_timer = SPAWN_RATE / game_state.wave as f32;
    }
    Ok(())
}

pub fn update_spawned(
    mut commands: Commands,
    mut pre_spawn: Query<(Entity, &mut Spawning)>,
    time: Res<Time>,
) {
    for (entity, mut pre_spawn) in &mut pre_spawn {
        pre_spawn.timer -= time.delta_secs();
        // When timer expires, spawn the actual enemy
        if pre_spawn.timer <= 0.0 {
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
