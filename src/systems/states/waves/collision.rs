use crate::systems::game::MarkedForDespawn;
use crate::systems::states::waves::components::{Dying, Health};
use crate::systems::states::waves::enemy::components::{Enemy, Hostile};
use crate::systems::states::waves::player::components::Player;
use crate::systems::states::waves::weapons::components::Bullet;
use bevy::prelude::*;

const COLLISION_RADIUS_SQ: f32 = 20.0 * 20.0;

pub fn check_bullet_enemy_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &GlobalTransform, &Bullet), Without<Hostile>>,
    mut enemy_query: Query<(&GlobalTransform, &mut Health, &Enemy)>,
) {
    for (bullet_entity, bullet_transform, bullet) in &bullet_query {
        let bullet_pos = bullet_transform.translation();

        for (enemy_transform, mut enemy_health, enemy) in &mut enemy_query {
            let radius = enemy.kind.visual().radius;
            let delta = bullet_pos - enemy_transform.translation();

            if delta.length_squared() < radius * radius {
                commands.entity(bullet_entity).insert(MarkedForDespawn);
                enemy_health.value = (enemy_health.value - bullet.damage).max(0.0);
                break;
            }
        }
    }
}

pub fn check_player_enemy_collision(
    mut commands: Commands,
    enemy_query: Query<(&GlobalTransform, &Enemy)>,
    bullet_query: Query<(Entity, &GlobalTransform, &Bullet), With<Hostile>>,
    mut player_query: Query<
        (Entity, &GlobalTransform, &mut Health),
        (With<Player>, Without<Dying>),
    >,
    time: Res<Time>,
) {
    let Ok((player_entity, player_transform, mut player_health)) = player_query.single_mut() else {
        return;
    };
    let player_pos = player_transform.translation();

    // Check enemy bullets
    for (bullet_entity, bullet_transform, bullet) in &bullet_query {
        let delta = player_pos - bullet_transform.translation();
        if delta.length_squared() < COLLISION_RADIUS_SQ {
            commands.entity(bullet_entity).insert(MarkedForDespawn);
            player_health.value = (player_health.value - bullet.damage).max(0.0);
            if player_health.value <= 0.0 {
                commands.entity(player_entity).insert(Dying {});
            }
        }
    }

    // Check enemy body
    for (enemy_transform, enemy) in &enemy_query {
        let radius = enemy.kind.visual().radius + 12.0; // player half-width
        let distance_sq = player_pos.distance_squared(enemy_transform.translation());

        if distance_sq < radius * radius {
            player_health.value -= enemy.damage * time.delta_secs();
            if player_health.value <= 0.0 {
                player_health.value = 0.0;
                commands.entity(player_entity).insert(Dying {});
            }
        }
    }
}
