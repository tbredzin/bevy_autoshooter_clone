use crate::systems::animations::animation::SpriteAnimation;
use crate::systems::animations::animator;
use crate::systems::animations::messages::AnimationEnded;
use bevy::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteAnimation>()
            .add_message::<AnimationEnded>()
            .add_systems(Update, animator::tick_sprite_animators);
    }
}
