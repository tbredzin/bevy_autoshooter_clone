use bevy::prelude::*;
use crate::components::{Player, Enemy, Bullet};
use crate::resources::{GameState, FIRE_RATE, BULLET_SPEED};

pub fn auto_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&Transform, &mut Player)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if !game_state.in_wave {
        return;
    }

    if let Ok((player_transform, mut player)) = player_query.single_mut() {
        player.fire_timer -= time.delta_secs();

        if player.fire_timer <= 0.0 {
            // Find nearest enemy
            if let Some(nearest_enemy) = enemy_query
                .iter()
                .min_by_key(|enemy_transform| {
                    let dist = player_transform.translation.distance(enemy_transform.translation);
                    (dist * 100.0) as i32
                })
            {
                let direction = (nearest_enemy.translation - player_transform.translation)
                    .truncate()
                    .normalize();

                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(5.0))),
                    MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.2))),
                    Transform::from_translation(player_transform.translation),
                    Bullet { direction },
                ));

                player.fire_timer = FIRE_RATE;
            }
        }
    }
}

pub fn move_bullets(
    mut bullet_query: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut transform, bullet) in &mut bullet_query {
        transform.translation += bullet.direction.extend(0.0) * BULLET_SPEED * time.delta_secs();
    }
}