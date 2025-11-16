use crate::components::{Bullet, Enemy, Player, Weapon, WeaponArea};
use crate::resources::{WaveManager, WaveState, BULLET_SPEED};
use crate::systems::weapons::utils;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{GlobalTransform, Query, Res, Time, Transform, With};

/// Smoothly moves and rotates weapons within their designated sectors to aim at nearest enemy
pub fn update_weapon_positioning(
    mut weapon_query: Query<(&mut Transform, &Weapon, &WeaponArea)>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    time: Res<Time>,
    wave_manager: Res<WaveManager>,
) {
    if wave_manager.wave_state == WaveState::Ended {
        return;
    }

    // Get player location
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation();

    for (mut weapon_transform, weapon, weapon_area) in &mut weapon_query {
        // Find nearest enemy within weapon range
        let nearest_enemy =
            utils::get_nearest_enemy(player_transform, enemy_query.iter().collect(), weapon.range);

        let Some(enemy_pos) = nearest_enemy else {
            continue;
        };

        // Calculate direction to enemy relative to player
        let player_to_enemy = (enemy_pos - player_pos).truncate();
        let enemy_angle = player_to_enemy.y.atan2(player_to_enemy.x);

        // Clamp angle to weapon's allowed sector
        let (min_angle, max_angle) = weapon_area.angle_range();
        let target_angle = utils::clamp_angle_to_range(enemy_angle, min_angle, max_angle);

        // Calculate target position within the sector
        let target_x = target_angle.cos() * weapon_area.orbit_radius;
        let target_y = target_angle.sin() * weapon_area.orbit_radius;
        let new_weapon_translation = Vec3::new(target_x, target_y, weapon_transform.translation.z);

        // Smoothly interpolate to target position
        let smoothing = 8.0;
        weapon_transform.translation = weapon_transform
            .translation
            .lerp(new_weapon_translation, time.delta_secs() * smoothing);

        // Rotate weapon to face enemy
        let weapon_to_enemy = player_to_enemy - new_weapon_translation.truncate();
        let target_rotation = Quat::from_rotation_z(weapon_to_enemy.y.atan2(weapon_to_enemy.x));

        weapon_transform.rotation = weapon_transform
            .rotation
            .slerp(target_rotation, time.delta_secs() * smoothing);
    }
}

pub fn move_bullets(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullet_query {
        transform.translation += bullet.direction.extend(0.0) * BULLET_SPEED * time.delta_secs();
    }
}
