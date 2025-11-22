use crate::components::Health;
use crate::systems::player::experience::PlayerExperience;
use crate::systems::player_upgrades::components::PlayerStats;
use bevy::prelude::{Bundle, Component, Transform};

#[derive(Component)]
pub struct Player {}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub transform: Transform,
    pub player: Player,
    pub health: Health,
    pub xp: PlayerExperience,
    pub stats: PlayerStats,
}
