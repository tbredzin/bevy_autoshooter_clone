use crate::components::{Enemy, Player, PreSpawn};
use crate::resources::{ENEMY_SPEED, GAME_AREA, GameState, SPAWN_RATE, ENEMY_SPAWN_TIME_IN_S, tiles_to_pixels};
use bevy::prelude::*;
use rand::Rng;

pub fn enemy_prespawn(
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
        let min_distance = tiles_to_pixels(2.0);
        let max_distance = tiles_to_pixels(50.0);

        // Generate a random spawn position
        let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);
        let distance = rng.random_range(min_distance..max_distance);
        let x = (player_pos.x + angle.cos() * distance).clamp(GAME_AREA.min.x, GAME_AREA.max.x);
        let y = (player_pos.y + angle.sin() * distance).clamp(GAME_AREA.min.y, GAME_AREA.max.y);
        let spawn_pos = Vec3::new(x, y, 0.0);

        // Spawn warning indicator
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(30.0))),
            MeshMaterial2d(materials.add(Color::srgba(1.0, 0.0, 0.0, 0.3))),
            Transform::from_translation(spawn_pos),
            PreSpawn {
                timer: ENEMY_SPAWN_TIME_IN_S,
                spawn_position: spawn_pos,
            },
        ));

        game_state.enemy_spawn_timer = SPAWN_RATE / game_state.wave as f32;
    }
    Ok(())
}

pub fn enemy_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut warning_query: Query<(Entity, &mut PreSpawn, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut warning, mut transform) in &mut warning_query {
        warning.timer -= time.delta_secs();

        // Pulsing effect
        let scale = 1.0 + (warning.timer * 5.0).sin() * 0.2;
        transform.scale = Vec3::splat(scale);

        // When timer expires, spawn the actual enemy
        if warning.timer <= 0.0 {
            commands.entity(entity).remove::<PreSpawn>();
            commands.entity(entity).insert((
                Enemy { health: 5.0 },
                Mesh2d(meshes.add(Circle::new(15.0))),
                MeshMaterial2d(materials.add(Color::srgb(1.0, 0.3, 0.3))),
            ));
        }
    }
}

pub fn move_enemies(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        for mut enemy_transform in &mut enemy_query {
            let direction =
                (player_transform.translation - enemy_transform.translation).normalize();
            enemy_transform.translation += direction * ENEMY_SPEED * time.delta_secs();
        }
    }
}
