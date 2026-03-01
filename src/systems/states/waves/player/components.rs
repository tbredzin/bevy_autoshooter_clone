use crate::systems::states::waves::player::experience::PlayerExperience;
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

/// Core player statistics that affect gameplay
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatKind {
    Damage,
    FireRate,
    Range,
    MaxHealth,
    Speed,
}

#[derive(Component, Clone, Debug)]
pub struct PlayerStats {
    pub damage_multiplier: f32,
    pub fire_rate_multiplier: f32,
    pub range_multiplier: f32,
    pub max_health: f32,
    pub speed_multiplier: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            damage_multiplier: 1.0,
            fire_rate_multiplier: 1.0,
            range_multiplier: 1.0,
            max_health: 100.0,
            speed_multiplier: 1.0,
        }
    }
}

impl PlayerStats {
    /// Get the current value for a specific stat kind
    pub fn get_value(&self, kind: StatKind) -> f32 {
        match kind {
            StatKind::Damage => self.damage_multiplier,
            StatKind::FireRate => self.fire_rate_multiplier,
            StatKind::Range => self.range_multiplier,
            StatKind::MaxHealth => self.max_health,
            StatKind::Speed => self.speed_multiplier,
        }
    }

    /// Format a stat value for display
    pub fn format_value(&self, kind: StatKind) -> String {
        match kind {
            StatKind::MaxHealth => format!("{:.0}", self.max_health),
            _ => format!("x{:.2}", self.get_value(kind)),
        }
    }
}

#[derive(Component)]
pub struct Health {
    pub value: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self { value: 100.0 }
    }
}
