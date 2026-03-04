use bevy::prelude::*;

#[derive(Message)]
pub struct AnimationEnded {
    pub entity: Entity,
}
