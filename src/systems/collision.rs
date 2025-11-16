use crate::components::{Bullet, Enemy, Health, MarkedForDespawn, Player, Weapon};
use bevy::prelude::*;

pub fn check_bullet_enemy_collision(
    mut commands: Commands,
    weapons_query: Query<(&Weapon, &Children)>,
    bullet_query: Query<&GlobalTransform, With<Bullet>>,
    mut enemy_query: Query<(&GlobalTransform, &mut Health), With<Enemy>>,
) {
    // Collision radius squared (avoid sqrt in distance check)
    const COLLISION_RADIUS_SQ: f32 = 20.0 * 20.0;

    for (weapon, bullets) in weapons_query {
        for bullet in bullets {
            if let Ok(bullet_transform) = bullet_query.get(*bullet) {
                let bullet_pos = bullet_transform.translation();

                // Find first enemy within range
                for (enemy_transform, mut enemy_health) in &mut enemy_query {
                    let delta = bullet_pos - enemy_transform.translation();
                    let distance_sq = delta.length_squared();

                    if distance_sq < COLLISION_RADIUS_SQ {
                        commands.entity(*bullet).insert(MarkedForDespawn);
                        enemy_health.value = (enemy_health.value - weapon.damage).max(0.0);
                        break; // Bullet consumed, check next bullet
                    }
                }
            }
        }
    }
}

pub fn check_player_enemy_collision(
    enemy_query: Query<(&GlobalTransform, &Enemy)>,
    mut player_query: Query<(&GlobalTransform, &mut Health), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player_transform, player_health)) = &mut player_query.single_mut() else {
        return;
    };

    let player_pos = player_transform.translation();

    for (enemy_transform, enemy) in &enemy_query {
        let distance = player_pos.distance(enemy_transform.translation());
        if distance < 35.0 {
            player_health.value -= (enemy.damage * time.delta_secs()).max(0.0);
        }
    }
}
