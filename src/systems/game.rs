use crate::components::MarkedForDespawn;
use crate::resources::{GAME_AREA, tiles_to_pixels};
use bevy::prelude::*;

pub fn out_of_bounds_system(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<MarkedForDespawn>>,
) {
    const MARGIN: f32 = tiles_to_pixels(0.5);

    for (entity, transform) in &query {
        let entity_pos = transform.translation().truncate();

        if entity_pos.x < GAME_AREA.min.x - MARGIN
            || entity_pos.x > GAME_AREA.max.x + MARGIN
            || entity_pos.y < GAME_AREA.min.y - MARGIN
            || entity_pos.y > GAME_AREA.max.y + MARGIN
        {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_marked_entities(
    mut commands: Commands,
    query: Query<Entity, With<MarkedForDespawn>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
