use crate::components::Health;
use crate::systems::player::experience::PlayerExperience;
use crate::systems::player_upgrades::components::PlayerStats;
use bevy::prelude::{Bundle, Component};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    // pub transform: Transform,
    pub player: Player,
    pub health: Health,
    pub xp: PlayerExperience,
    pub stats: PlayerStats,
    pub action: PlayerAction,
    pub direction: Direction,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            health: Default::default(),
            xp: Default::default(),
            stats: Default::default(),
            action: PlayerAction::IDLE,
            direction: Direction::EAST,
        }
    }
}

#[derive(Component, PartialEq, Clone, Copy, Debug, Default)]
pub enum PlayerAction {
    #[default]
    IDLE,
    WALKING,
    DASHING,
    DYING,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Default)]
pub enum Direction {
    #[default]
    EAST,
    NORTH,
    NORTHEAST,
    NORTHWEST,
    SOUTH,
    SOUTHEAST,
    SOUTHWEST,
    WEST,
}
