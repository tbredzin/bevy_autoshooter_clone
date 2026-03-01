use crate::systems::animations::resources::AnimationAssets;
use crate::systems::animations::systems::*;
use bevy::prelude::*;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnimationAssets>().add_systems(
            Update,
            (animate_player_sprite, update_player_sprite).chain(),
        );
    }
}
