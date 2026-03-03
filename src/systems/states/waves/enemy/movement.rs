use crate::systems::states::waves::enemy::components::{BossAttack, Enemy, RangedAttack};
use crate::systems::states::waves::player::components::Player;
use bevy::prelude::{GlobalTransform, Query, Res, Time, Transform, With, Without};

pub fn move_to_player(
    mut enemy_query: Query<(&mut Transform, &Enemy), (Without<RangedAttack>, Without<BossAttack>)>,
    mut ranged_enemy_query: Query<(&mut Transform, &Enemy, &RangedAttack), Without<BossAttack>>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation();

    // Basic
    for (mut transform, enemy) in &mut enemy_query {
        let direction = (player_pos - transform.translation).normalize_or_zero();
        transform.translation += direction * enemy.speed * time.delta_secs();
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
