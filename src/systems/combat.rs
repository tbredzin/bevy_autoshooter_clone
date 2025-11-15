use crate::components::{Bullet, Enemy, Player, Weapon};
use crate::resources::{BULLET_SPEED, WaveManager, WaveState};
use bevy::prelude::*;

pub fn auto_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<(&GlobalTransform, &Children), With<Player>>,
    mut weapons_query: Query<&mut Weapon>,
    enemy_query: Query<&GlobalTransform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
    wave_manager: Res<WaveManager>,
) {
    if wave_manager.wave_state == WaveState::Ended {
        return;
    }

    let Ok((player_transform, children)) = player_query.single() else {
        return;
    };
    for child in children.iter() {
        let Ok(player_weapon) = &mut weapons_query.get_mut(child) else {
            continue;
        };

        player_weapon.fire_rate.tick(time.delta());

        if player_weapon.fire_rate.is_finished() {
            // Find nearest enemy
            let player_pos = player_transform.translation();

            if let Some(nearest_enemy) = enemy_query.iter().min_by_key(|enemy_transform| {
                player_pos.distance(enemy_transform.translation()) as i32
            }) {
                if player_pos.distance(nearest_enemy.translation()) > player_weapon.range {
                    continue; // No enemy at range for this weapon
                }

                let direction = (nearest_enemy.translation() - player_pos)
                    .truncate()
                    .normalize();
                let damage = player_weapon.damage;

                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(player_weapon.bullet_size))),
                    MeshMaterial2d(materials.add(player_weapon.bullet_color)),
                    Transform::from_translation(player_pos),
                    Bullet { direction, damage },
                ));

                player_weapon.fire_rate.reset();
            }
        }
    }
}

pub fn move_bullets(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullet_query {
        transform.translation += bullet.direction.extend(0.0) * BULLET_SPEED * time.delta_secs();
    }
}
