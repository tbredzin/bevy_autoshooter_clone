use crate::components::{Bullet, Enemy, Weapon, WeaponCooldown};
use crate::systems::player::components::Player;
use crate::systems::weapons::utils;
use bevy::prelude::*;

pub fn auto_shoot(
    mut commands: Commands,
    weapons_query: Query<(&GlobalTransform, &mut Weapon, &mut WeaponCooldown)>,
    enemy_query: Query<&GlobalTransform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
) {
    for (weapon_transform, mut weapon, mut cooldown) in weapons_query {
        cooldown.timer.tick(time.delta());

        if !cooldown.timer.is_finished() {
            continue; // still on cooldown -> continue
        }
        let weapon_pos = weapon_transform.translation();

        let Some(nearest_enemy) = utils::get_nearest_enemy(
            weapon_transform,
            enemy_query.iter().collect(),
            weapon.base_range * weapon.range_multiplier,
        ) else {
            continue;
        };

        // Compute direction to enemy
        let direction = (nearest_enemy - weapon_pos).truncate().normalize();

        // Spawn a new bullet toward that direction
        commands.spawn((
            Transform::from_translation(weapon_pos).with_translation(Vec3::new(
                weapon_pos.x,
                weapon_pos.y,
                1.0,
            )),
            Bullet {
                direction,
                damage: weapon.base_damage * weapon.damage_multiplier,
                kind: weapon.kind,
            },
        ));

        // reset cooldown
        cooldown.timer.reset();
    }
}
