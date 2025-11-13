use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::components::{Enemy, Player};
use crate::resources::{GameState, ENEMY_SPEED};

pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !game_state.in_wave {
        return;
    }

    game_state.enemy_spawn_timer -= time.delta_secs();

    if game_state.enemy_spawn_timer <= 0.0 {
        let Ok(window) = window_query.single() else { return };
        let mut rng = rand::rng();

        let spawn_margin = 50.0;
        let (x, y) = if rng.random_bool(0.5) {
            // Spawn on left/right
            let x = if rng.random_bool(0.5) {
                -window.width() / 2.0 - spawn_margin
            } else {
                window.width() / 2.0 + spawn_margin
            };
            let y = rng.random_range(-window.height() / 2.0..window.height() / 2.0);
            (x, y)
        } else {
            // Spawn on top/bottom
            let x = rng.random_range(-window.width() / 2.0..window.width() / 2.0);
            let y = if rng.random_bool(0.5) {
                -window.height() / 2.0 - spawn_margin
            } else {
                window.height() / 2.0 + spawn_margin
            };
            (x, y)
        };

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(15.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.3, 0.3))),
            Transform::from_xyz(x, y, 0.0),
            Enemy { health: 3.0 * game_state.wave as f32 },
        ));

        game_state.enemy_spawn_timer = 1.0 / game_state.wave as f32;
    }
}

pub fn move_enemies(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        for mut enemy_transform in &mut enemy_query {
            let direction = (player_transform.translation - enemy_transform.translation)
                .truncate()
                .normalize();
            enemy_transform.translation += direction.extend(0.0) * ENEMY_SPEED * time.delta_secs();
        }
    }
}