use bevy::prelude::*;
use crate::components::{Enemy, Bullet};
use crate::resources::{GameState, WAVE_DURATION};

pub fn update_wave_timer(
    mut game_state: ResMut<GameState>,
    mut enemy_query: Query<Entity, With<Enemy>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if game_state.in_wave {
        game_state.wave_timer -= time.delta_secs();
        if game_state.wave_timer <= 0.0 {
            game_state.in_wave = false;
            // Clear enemies
            for entity in &mut enemy_query {
                commands.entity(entity).despawn();
            }
        }
    } else {
        // Wait 3 seconds between waves
        game_state.wave_timer += time.delta_secs();
        if game_state.wave_timer >= 3.0 {
            game_state.wave += 1;
            game_state.in_wave = true;
            game_state.wave_timer = WAVE_DURATION;
        }
    }
}

pub fn cleanup_offscreen(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in &bullet_query {
        if transform.translation.x.abs() > 1000.0 || transform.translation.y.abs() > 1000.0 {
            commands.entity(entity).despawn();
        }
    }
}