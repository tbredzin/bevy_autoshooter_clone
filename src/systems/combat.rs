use crate::components::WeaponKind::*;
use crate::components::{Bullet, Enemy, Player, Weapon};
use crate::resources::{WaveManager, WaveState};
use crate::systems::weapons::utils;
use bevy::prelude::*;

pub fn auto_shoot(
    mut commands: Commands,
    weapons_query: Query<(&GlobalTransform, &mut Weapon)>,
    enemy_query: Query<&GlobalTransform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
    wave_manager: Res<WaveManager>,
) {
    if wave_manager.wave_state == WaveState::Ended {
        return;
    }

    for (weapon_transform, mut weapon) in weapons_query {
        weapon.cooldown.tick(time.delta());

        if !weapon.cooldown.is_finished() {
            continue; // still on cooldown -> continue
        }
        let weapon_pos = weapon_transform.translation();

        let Some(nearest_enemy) =
            utils::get_nearest_enemy(weapon_transform, enemy_query.iter().collect(), weapon.range)
        else {
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
                damage: weapon.damage,
                kind: weapon.kind,
            },
        ));

        // reset cooldown
        weapon.cooldown.reset();
    }
}
