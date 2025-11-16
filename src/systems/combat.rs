use crate::components::WeaponKind::*;
use crate::components::{Bullet, Enemy, Player, Weapon};
use crate::resources::{BulletMeshes, WaveManager, WaveState, BULLET_SPEED};
use bevy::prelude::*;

pub fn auto_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    weapons_query: Query<(Entity, &GlobalTransform, &mut Weapon)>,
    enemy_query: Query<&GlobalTransform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
    wave_manager: Res<WaveManager>,
    bullet_meshes: Res<BulletMeshes>,
) {
    if wave_manager.wave_state == WaveState::Ended {
        return;
    }

    for (weapon_entity, weapon_global_transform, mut weapon) in weapons_query {
        weapon.fire_rate.tick(time.delta());
        if weapon.fire_rate.is_finished() {
            // Get weapon's world position
            let weapon_pos = weapon_global_transform.translation();

            if let Some(nearest_enemy) = enemy_query.iter().min_by_key(|enemy_transform| {
                weapon_pos.distance(enemy_transform.translation()) as i32
            }) {
                if weapon_pos.distance(nearest_enemy.translation()) > weapon.range {
                    continue; // No enemy at range for this weapon
                }

                let direction = (nearest_enemy.translation() - weapon_pos)
                    .truncate()
                    .normalize();

                commands.spawn((
                    match weapon.kind {
                        Shotgun => Mesh2d(bullet_meshes.square_large.clone()),
                        Pistol => Mesh2d(bullet_meshes.circle_medium.clone()),
                        MachineGun => Mesh2d(bullet_meshes.circle_small.clone()),
                    },
                    MeshMaterial2d(materials.add(weapon.bullet_color)),
                    Transform::from_translation(weapon_pos).with_translation(Vec3::new(
                        weapon_pos.x,
                        weapon_pos.y,
                        1.0,
                    )),
                    Bullet {
                        direction,
                        damage: weapon.damage,
                    },
                ));
                weapon.fire_rate.reset();
            }
        }
    }
}

pub fn move_bullets(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullet_query {
        transform.translation += bullet.direction.extend(0.0) * BULLET_SPEED * time.delta_secs();
    }
}
