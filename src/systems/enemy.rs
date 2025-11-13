use crate::components::{Enemy, Player};
use crate::resources::{ENEMY_SPEED, GAME_AREA, GameState, SPAWN_RATE};
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
) -> Result {
    if !game_state.in_wave {
        return Ok(());
    }

    game_state.enemy_spawn_timer -= time.delta_secs();

    if game_state.enemy_spawn_timer <= 0.0 {
        let mut rng = rand::rng();

        let player_pos = player_query.single()?.translation.xy();
        let min_distance = 200.0;
        let max_distance = Vec2::new(GAME_AREA.min.x, GAME_AREA.max.y).length();

        // Generate a random x,y using angle and distance from player_pos
        let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);
        let distance = rng.random_range(min_distance..max_distance);
        let x = (player_pos.x + angle.cos() * distance).clamp(GAME_AREA.min.x, GAME_AREA.max.x);
        let y = (player_pos.y + angle.sin() * distance).clamp(GAME_AREA.min.y, GAME_AREA.max.y);

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(15.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.3, 0.3))),
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                health: 1.0 * game_state.wave as f32,
            },
        ));

        game_state.enemy_spawn_timer = SPAWN_RATE / game_state.wave as f32;
    }
    Ok(())
}

pub fn move_enemies(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        for mut enemy_transform in &mut enemy_query {
            let direction =
                (player_transform.translation - enemy_transform.translation).normalize(); // move toward player
            enemy_transform.translation += direction * ENEMY_SPEED * time.delta_secs(); // at enemy speed
        }
    }
}
