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
    pub upgrade: UpgradeType,
}

#[derive(Clone, Debug)]
pub enum UpgradeType {
    // Weapon upgrades
    IncreaseDamage(f32),
    IncreaseFireRate(f32),
    IncreaseRange(f32),

    // Player upgrades
    IncreaseMaxHealth(f32),
    IncreaseSpeed(f32),
    HealPlayer(f32),

    // Special upgrades
    AddPiercing,
    AddMultishot(u32),
    AddExplosive,
}

impl UpgradeType {
    pub fn get_display_info(&self) -> (&str, String, Color) {
        match self {
            UpgradeType::IncreaseDamage(amount) => (
                "Damage Up",
                format!("+{:.0}% damage to all weapons", amount * 100.0),
                Color::srgb(1.0, 0.4, 0.4),
            ),
            UpgradeType::IncreaseFireRate(amount) => (
                "Fire Rate Up",
                format!("+{:.0}% fire rate to all weapons", amount * 100.0),
                Color::srgb(1.0, 0.8, 0.2),
            ),
            UpgradeType::IncreaseRange(amount) => (
                "Range Up",
                format!("+{:.0}% range to all weapons", amount * 100.0),
                Color::srgb(0.4, 0.8, 1.0),
            ),
            UpgradeType::IncreaseMaxHealth(amount) => (
                "Max Health Up",
                format!("+{:.0} max health", amount),
                Color::srgb(0.2, 1.0, 0.2),
            ),
            UpgradeType::IncreaseSpeed(amount) => (
                "Speed Up",
                format!("+{:.0}% movement speed", amount * 100.0),
                Color::srgb(0.4, 1.0, 0.8),
            ),
            UpgradeType::HealPlayer(amount) => (
                "Heal",
                format!("Restore {:.0} health", amount),
                Color::srgb(1.0, 0.2, 0.6),
            ),
            UpgradeType::AddPiercing => (
                "Piercing Shots",
                "Bullets pierce through enemies".to_string(),
                Color::srgb(0.8, 0.2, 1.0),
            ),
            UpgradeType::AddMultishot(count) => (
                "Multishot",
                format!("Fire {} additional projectiles", count),
                Color::srgb(1.0, 0.6, 0.2),
            ),
            UpgradeType::AddExplosive => (
                "Explosive Rounds",
                "Bullets explode on impact".to_string(),
                Color::srgb(1.0, 0.5, 0.0),
            ),
        }
    }

    pub fn get_rarity(&self) -> UpgradeRarity {
        match self {
            UpgradeType::IncreaseDamage(_)
            | UpgradeType::IncreaseFireRate(_)
            | UpgradeType::IncreaseRange(_)
            | UpgradeType::IncreaseMaxHealth(_)
            | UpgradeType::HealPlayer(_) => UpgradeRarity::Common,

            UpgradeType::IncreaseSpeed(_) => UpgradeRarity::Uncommon,

            UpgradeType::AddPiercing | UpgradeType::AddMultishot(_) => UpgradeRarity::Rare,

            UpgradeType::AddExplosive => UpgradeRarity::Legendary,
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
