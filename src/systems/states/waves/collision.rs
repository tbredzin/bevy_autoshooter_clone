use crate::systems::game::MarkedForDespawn;
use crate::systems::states::waves::components::{Dying, Health};
use crate::systems::states::waves::enemy::components::Enemy;
use crate::systems::states::waves::player::components::Player;
use crate::systems::states::waves::weapons::components::Bullet;
use bevy::prelude::*;

// Collision radius squared (avoid sqrt in distance check)
const COLLISION_RADIUS_SQ: f32 = 20.0 * 20.0;

pub fn check_bullet_enemy_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &GlobalTransform, &Bullet)>,
    mut enemy_query: Query<(&GlobalTransform, &mut Health), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform, bullet) in &bullet_query {
        let bullet_pos = bullet_transform.translation();

        // Find first enemy within range
        for (enemy_transform, mut enemy_health) in &mut enemy_query {
            let delta = bullet_pos - enemy_transform.translation();
            let distance_sq = delta.length_squared();

            if distance_sq < COLLISION_RADIUS_SQ {
                commands.entity(bullet_entity).insert(MarkedForDespawn);
                enemy_health.value = (enemy_health.value - bullet.damage).max(0.0);
                break; // Bullet consumed, check next bullet
            }
        }
    }
}

pub fn check_player_enemy_collision(
    mut commands: Commands,
    enemy_query: Query<(&GlobalTransform, &Enemy)>,
    mut player_query: Query<
        (Entity, &GlobalTransform, &mut Health),
        (With<Player>, Without<Dying>),
    >,
    time: Res<Time>,
) {
    let Ok((player_entity, player_transform, player_health)) = &mut player_query.single_mut()
    else {
        return;
    };

    let player_pos = player_transform.translation();

    for (enemy_transform, enemy) in &enemy_query {
        let distance_sq = player_pos.distance_squared(enemy_transform.translation());
        if distance_sq < COLLISION_RADIUS_SQ {
            player_health.value -= enemy.damage * time.delta_secs();
            if player_health.value < 0.0 {
                player_health.value = 0.0;
                commands.entity(*player_entity).insert(Dying {});
            }
        }
    }
}
