use crate::systems::player_animations::resources::load_animation_assets;
use crate::systems::player_animations::systems::*;
use bevy::prelude::*;

/// Plugin that handles all player animation logic
pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_animation_assets).add_systems(
            Update,
            (
                // update_player_animation_state,
                // update_sprite_on_state_change,
                animate_player_sprite,
                update_player_sprite,
            )
                .chain(),
        );
    }
}
