use crate::components::{Health, PlayerExperience};
use bevy::prelude::{Bundle, Component, Transform};

#[derive(Component)]
pub struct Player {}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub transform: Transform,
    pub player: Player,
    pub health: Health,
    pub xp: PlayerExperience,
}
