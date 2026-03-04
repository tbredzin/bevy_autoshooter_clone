use crate::systems::states::waves::components::Direction;
use crate::systems::states::waves::enemy::components::{BossAttack, Enemy, RangedAttack};
use crate::systems::states::waves::player::components::Player;
use bevy::math::Vec2;
use bevy::prelude::{GlobalTransform, Query, Res, Time, Transform, With, Without};

pub fn move_to_player(
    mut enemy_query: Query<
        (&mut Transform, &mut Direction, &Enemy),
        (Without<RangedAttack>, Without<BossAttack>),
    >,
    mut ranged_enemy_query: Query<(&mut Transform, &Enemy, &RangedAttack), Without<BossAttack>>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation();

    // Basic
    for (mut transform, mut direction, enemy) in &mut enemy_query {
        let new_direction = (player_pos - transform.translation).normalize_or_zero();
        *direction = get_direction(new_direction.truncate());
        transform.translation += new_direction * enemy.speed * time.delta_secs();
    }

    // Ranged
    for (mut transform, enemy, ranged) in &mut ranged_enemy_query {
        let to_player = player_pos - transform.translation;
        let distance = to_player.length();
        let direction = to_player.normalize_or_zero();

        let preferred = ranged.preferred_distance;
        if distance > preferred + 60.0 {
            transform.translation += direction * enemy.speed * time.delta_secs();
        } else if distance < preferred - 60.0 {
            transform.translation -= direction * enemy.speed * 0.7 * time.delta_secs();
        }
    }
}

pub fn get_direction(translation: Vec2) -> Direction {
    match (translation.x, translation.y) {
        (x, y) if y < -0.5 && x.abs() < 0.5 => Direction::SOUTH,
        (x, y) if y < -0.5 && x > 0.5 => Direction::SOUTHEAST,
        (x, y) if x > 0.5 && y.abs() < 0.5 => Direction::EAST,
        (x, y) if y > 0.5 && x > 0.5 => Direction::NORTHEAST,
        (x, y) if y > 0.5 && x.abs() < 0.5 => Direction::NORTH,
        (x, y) if y > 0.5 && x < -0.5 => Direction::NORTHWEST,
        (x, y) if x < -0.5 && y.abs() < 0.5 => Direction::WEST,
        (x, y) if y < -0.5 && x < -0.5 => Direction::SOUTHWEST,
        _ => Direction::EAST,
    }
}
