use crate::components::{Bullet, Enemy, Player};
use crate::resources::{GameState, WaveState};
use bevy::prelude::*;

pub fn check_bullet_enemy_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut enemy_query: Query<(Entity, &GlobalTransform, &mut Enemy)>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.wave_state == WaveState::Ended {
        return;
    }

    // Collision radius squared (avoid sqrt in distance check)
    const COLLISION_RADIUS_SQ: f32 = 20.0 * 20.0;

    for (bullet_entity, bullet_transform) in &bullet_query {
        let bullet_pos = bullet_transform.translation();

        for (enemy_entity, enemy_transform, mut enemy) in &mut enemy_query {
            let delta = bullet_pos - enemy_transform.translation();
            let distance_sq = delta.length_squared();

            if distance_sq < COLLISION_RADIUS_SQ {
                commands.entity(bullet_entity).despawn();
                enemy.health -= 1.0;

                if enemy.health <= 0.0 {
                    commands.entity(enemy_entity).despawn();
                    game_state.xp += 1;
                    if game_state.xp >= game_state.level * 10 {
                        game_state.level += 1;
                        game_state.max_health += 10.0;
                        game_state.health = game_state.max_health;
                    }
                }
                break;
            }
        }
    }
}

pub fn check_player_enemy_collision(
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) -> Result {
    if game_state.wave_state == WaveState::Ended {
        return Ok(());
    }
    let player_transform = player_query.single()?;
    for enemy_transform in &enemy_query {
        let distance = player_transform
            .translation
            .distance(enemy_transform.translation);
        if distance < 35.0 {
            game_state.health -= 10.0 * time.delta_secs();
            game_state.health = game_state.health.max(0.0);
        }
    }
    Ok(())
}
