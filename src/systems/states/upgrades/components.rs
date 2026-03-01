use crate::systems::hud::components::DisplayStatKind;
use crate::systems::states::upgrades::components::UpgradeCardState::Unselected;
use crate::systems::states::waves::player::components::StatKind;
use bevy::color::Color;
use bevy::prelude::{Component, Timer};

#[derive(Component)]
pub struct UpgradeUI;

#[derive(Component)]
pub struct UpgradeCardButton;

#[derive(Component, Copy, Clone, Debug)]
pub struct UpgradeCard {
    pub state: UpgradeCardState,
    pub kind: StatKind,
    pub value: f32,
    pub rarity: UpgradeRarity,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UpgradeCardState {
    Unselected,
    Holding,
    Selected,
    ToApply,
    Applied,
}

impl UpgradeCard {
    pub fn new(kind: StatKind, value: f32, rarity: UpgradeRarity) -> Self {
        Self {
            state: Unselected,
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

/// How long the button must be held to confirm selection (in seconds)
pub const HOLD_DURATION: f32 = 1.0;
pub const SELECTION_FREQUENCY: f32 = 4.0;

/// Component to mark which card corresponds to which gamepad button
#[derive(Component)]
pub struct CardIndex(pub usize);

/// Component for the progress bar fill overlay
#[derive(Component)]
pub struct CardProgressFill;

#[derive(Component, Debug)]
pub struct UpgradeCardAnimation {
    pub timer: Timer,
}

impl Default for UpgradeCardAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
        }
    }
}
