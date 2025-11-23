use crate::systems::hud::DisplayStatKind;
use bevy::color::Color;
use bevy::prelude::Component;

#[derive(Component)]
pub struct UpgradeUI;

#[derive(Component)]
pub struct UpgradeCardButton;

#[derive(Component)]
pub struct NextWaveButton;

#[derive(Component, Debug)]
pub struct UpgradeCard {
    pub upgrade: StatUpgrade,
}

/// Enum representing all player stats that can be upgraded
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatKind {
    Damage,
    FireRate,
    Range,
    MaxHealth,
    Speed,
}

/// Core player statistics that affect gameplay
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
    pub fn apply_upgrade(&mut self, upgrade: &StatUpgrade) {
        match upgrade.kind {
            StatKind::Damage => self.damage_multiplier += upgrade.value,
            StatKind::FireRate => self.fire_rate_multiplier += upgrade.value,
            StatKind::Range => self.range_multiplier += upgrade.value,
            StatKind::MaxHealth => self.max_health += upgrade.value,
            StatKind::Speed => self.speed_multiplier += upgrade.value,
        }
    }

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

/// Represents an upgrade that can be applied to a stat
#[derive(Clone, Debug)]
pub struct StatUpgrade {
    pub kind: StatKind,
    pub value: f32,
    pub rarity: UpgradeRarity,
}

impl StatUpgrade {
    pub fn new(kind: StatKind, value: f32, rarity: UpgradeRarity) -> Self {
        Self {
            kind,
            value,
            rarity,
        }
    }
    pub fn get_display_info(&self) -> (usize, String, Color) {
        let display = DisplayStatKind::from(self.kind);
        let (texture_index, name, color) = display.get_display_info();
        let description = match self.kind {
            StatKind::MaxHealth => format!("+{:.0} max HP", self.value),
            _ => format!("+{:.0}% {}", self.value * 100.0, name.to_lowercase()),
        };
        (texture_index, description, color)
    }

    pub fn get_full_title(&self) -> String {
        let display = DisplayStatKind::from(self.kind);
        let (_, name, _) = display.get_display_info();
        format!("{} Up", name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UpgradeRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl UpgradeRarity {
    pub fn get_color(&self) -> Color {
        match self {
            UpgradeRarity::Common => Color::srgb(0.7, 0.7, 0.7),
            UpgradeRarity::Uncommon => Color::srgb(0.3, 1.0, 0.3),
            UpgradeRarity::Rare => Color::srgb(0.3, 0.5, 1.0),
            UpgradeRarity::Legendary => Color::srgb(1.0, 0.8, 0.0),
        }
    }
    pub fn get_odds(&self) -> f32 {
        match self {
            UpgradeRarity::Common => 60.0,
            UpgradeRarity::Uncommon => 30.0,
            UpgradeRarity::Rare => 9.0,
            UpgradeRarity::Legendary => 1.0,
        }
    }
}
