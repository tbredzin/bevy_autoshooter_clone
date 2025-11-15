use crate::resources::GAME_AREA;
use bevy::prelude::*;

pub fn out_of_bounds_system(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in &query {
        let entity_pos = transform.translation().truncate();

        if entity_pos.x < GAME_AREA.min.x
            || entity_pos.x > GAME_AREA.max.x
            || entity_pos.y < GAME_AREA.min.y
            || entity_pos.y > GAME_AREA.max.y
        {
            commands.entity(entity).despawn();
        }
    }
}
