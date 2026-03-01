use crate::resources::{tiles_to_pixels, GAME_AREA};
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    InWave,
    UpgradeSelection,
    Shopping,
}

#[derive(Component)]
pub struct MarkedForDespawn;

pub fn out_of_bounds_system(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<MarkedForDespawn>>,
) {
    const MARGIN: f32 = tiles_to_pixels(2.0);

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
