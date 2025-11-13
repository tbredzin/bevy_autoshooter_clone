use crate::components::Enemy;
use crate::resources::{GAME_AREA, GameState, WAVE_DURATION};
use bevy::prelude::*;

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
pub fn out_of_bounds_system(mut commands: Commands, query: Query<(Entity, &Transform)>) -> Result {
    for (entity, transform) in query.iter() {
        let entity_pos = transform.translation.xy();
        if entity_pos.x < GAME_AREA.min.x
            || entity_pos.x > GAME_AREA.max.x
            || entity_pos.y < GAME_AREA.min.y
            || entity_pos.y > GAME_AREA.max.y
        {
            commands.entity(entity).despawn()
        }
    }
    Ok(())
}
