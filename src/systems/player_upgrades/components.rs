use bevy::color::Color;
use bevy::prelude::Component;

#[derive(Component)]
pub struct UpgradeUI;

#[derive(Component)]
pub struct UpgradeCardButton;

#[derive(Component)]
pub struct NextWaveButton;

#[derive(Component)]
pub struct UpgradeCard {
    pub upgrade: StatUpgrade,
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
        match upgrade {
            StatUpgrade::IncreaseDamage(amount) => self.damage_multiplier += amount,
            StatUpgrade::IncreaseFireRate(amount) => self.fire_rate_multiplier += amount,
            StatUpgrade::IncreaseRange(amount) => self.range_multiplier += amount,
            StatUpgrade::IncreaseMaxHealth(amount) => self.max_health += amount,
            StatUpgrade::IncreaseSpeed(amount) => self.speed_multiplier += amount,
        }
    }
}

#[derive(Clone, Debug)]
pub enum StatUpgrade {
    IncreaseDamage(f32),
    IncreaseFireRate(f32),
    IncreaseRange(f32),
    IncreaseMaxHealth(f32),
    IncreaseSpeed(f32),
}

impl StatUpgrade {
    pub fn get_display_info(&self) -> (&str, String, Color) {
        match self {
            StatUpgrade::IncreaseDamage(amount) => (
                "Damage Up",
                format!("+{:.0}% damage", amount * 100.0),
                Color::srgb(1.0, 0.4, 0.4),
            ),
            StatUpgrade::IncreaseFireRate(amount) => (
                "Fire Rate Up",
                format!("+{:.0}% fire rate", amount * 100.0),
                Color::srgb(1.0, 0.8, 0.2),
            ),
            StatUpgrade::IncreaseRange(amount) => (
                "Range Up",
                format!("+{:.0}% range", amount * 100.0),
                Color::srgb(0.4, 0.8, 1.0),
            ),
            StatUpgrade::IncreaseMaxHealth(amount) => (
                "Max Health Up",
                format!("+{:.0} max HP", amount),
                Color::srgb(0.2, 1.0, 0.2),
            ),
            StatUpgrade::IncreaseSpeed(amount) => (
                "Speed Up",
                format!("+{:.0}% movement speed", amount * 100.0),
                Color::srgb(0.4, 1.0, 0.8),
            ),
        }
    }

    pub fn get_rarity(&self) -> UpgradeRarity {
        match self {
            StatUpgrade::IncreaseDamage(_)
            | StatUpgrade::IncreaseFireRate(_)
            | StatUpgrade::IncreaseRange(_)
            | StatUpgrade::IncreaseMaxHealth(_)
            | StatUpgrade::IncreaseSpeed(_) => UpgradeRarity::Common,
        }
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
