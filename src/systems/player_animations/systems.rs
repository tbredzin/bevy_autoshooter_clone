use crate::systems::player::components::{Direction, PlayerAction};
use crate::systems::player_animations::components::*;
use crate::systems::player_animations::resources::AnimationAssets;
use bevy::prelude::*;

/// System to update the animation direction based on movement direction
pub fn update_player_sprite(
    mut commands: Commands,
    animation_assets: Res<AnimationAssets>,
    mut query: Query<
        (Entity, &Direction, &PlayerAction, &mut Sprite),
        (
            With<PlayerSprite>,
            Or<(Changed<Direction>, Changed<PlayerAction>)>,
        ),
    >,
) {
    for (entity, direction, action, mut sprite) in query.iter_mut() {
        let new_indices = get_animation_indices(action, direction);
        let (new_animation, new_layout) = animation_assets.get_animation_sprite(action);
        sprite.image = new_animation;
        sprite.texture_atlas = Some(TextureAtlas::from(new_layout).with_index(new_indices.first));
        commands.entity(entity).insert(new_indices);
    }
}

/// System to animate sprites by cycling through texture atlas indices
pub fn animate_player_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index += 1;
                if atlas.index >= indices.last {
                    atlas.index = indices.first
                }
            }
        }
    }
}

/// Helper function to get animation indices for a given state and direction
pub fn get_animation_indices(state: &PlayerAction, direction: &Direction) -> AnimationIndices {
    // Find the sprite row
    let row = match direction {
        Direction::SOUTH => 0,
        Direction::SOUTHWEST => 1,
        Direction::WEST => 1,
        Direction::NORTHWEST => 2,
        Direction::NORTH => 3,
        Direction::NORTHEAST => 4,
        Direction::SOUTHEAST => 5,
        Direction::EAST => 5,
    };
    // Different states might have different frame counts
    let frame_count = match state {
        PlayerAction::IDLE => 8,
        PlayerAction::WALKING => 8,
        PlayerAction::DASHING => 8,
        PlayerAction::DYING => 8,
    };
    AnimationIndices {
        first: row * frame_count,
        last: (row * frame_count) + frame_count - 1,
    }
}
