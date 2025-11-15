use crate::components::{Bullet, Enemy, Player};
use crate::resources::{GameState, WaveState, BULLET_SPEED};
use bevy::prelude::*;

pub fn auto_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&GlobalTransform, &mut Player)>,
    enemy_query: Query<&GlobalTransform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if game_state.wave_state == WaveState::Ended {
        return;
    }

    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return;
    };

    player.fire_timer.tick(time.delta());

    if player.fire_timer.is_finished() {
        // Find nearest enemy
        let player_pos = player_transform.translation();

        if let Some(nearest_enemy) = enemy_query
            .iter()
            .min_by_key(|enemy_transform| player_pos.distance(enemy_transform.translation()) as i32)
        {
            let direction = (nearest_enemy.translation() - player_pos)
                .truncate()
                .normalize();

            commands.spawn((
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.2))),
                Transform::from_translation(player_pos),
                Bullet { direction },
            ));

            player.fire_timer.reset();
        }
    }
}

pub fn move_bullets(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullet_query {
        transform.translation += bullet.direction.extend(0.0) * BULLET_SPEED * time.delta_secs();
    }
}
