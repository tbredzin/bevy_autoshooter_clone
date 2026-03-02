use crate::systems::animations::components::*;
use crate::systems::animations::resources::AnimationAssets;
use crate::systems::states::waves::player::components::{Direction, PlayerAction};
use bevy::prelude::*;

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
        println!(
            "change animation -> direction: {:?} /  action: {:?}",
            direction, action
        );
        let new_indices = get_animation_indices(action, direction);
        let (new_animation, new_layout) = animation_assets.get_animation_sprite(action);
        sprite.image = new_animation;
        sprite.texture_atlas = Some(TextureAtlas::from(new_layout).with_index(new_indices.first));
        commands.entity(entity).insert(new_indices);
    }
}

pub fn animate_player_sprite(
    time: Res<Time>,
    query: Single<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    let (indices, mut animation, mut sprite) = query.into_inner();
    animation.timer.tick(time.delta());
    if animation.timer.just_finished() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index < indices.last {
                atlas.index += 1;
            }
            if indices.repeated && atlas.index == indices.last {
                atlas.index = indices.first
            }
        }
    }
}

/// Helper functions
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
    // Different states might have different frame counts and may be repeated
    let (frame_count, repeated) = match state {
        PlayerAction::IDLE => (8, true),
        PlayerAction::WALKING => (8, true),
        PlayerAction::DASHING => (8, false),
        PlayerAction::DYING => (8, false),
    };

    AnimationIndices {
        first: row * frame_count,
        last: (row * frame_count) + frame_count - 1,
        repeated,
    }
}
