use crate::systems::hud::components::DisplayStatKind;
use crate::systems::states::upgrades::components::UpgradeCardState::Unselected;
use crate::systems::states::waves::player::components::StatKind;
use bevy::color::Color;
use bevy::prelude::Component;
use std::fmt;
use std::fmt::Display;

/// How long (s) a button must be held to confirm the selection.
pub const HOLD_DURATION: f32 = 1.0;

/// How long (s) the burst animation plays after confirming.
pub const SELECTION_ANIM_DURATION: f32 = 0.5;

/// Wobble frequency multiplier during the burst animation.
pub const SELECTION_FREQUENCY: f32 = 4.0;

/// Gamepad button atlas sprite indices for card slots 0-3 (West / South / North / East).
/// Formula: SPRITESHEET_BEGIN(71) + SPRITESHEET_WIDTH(35) × row
pub const GAMEPAD_BUTTON_INDICES: [usize; 4] = [71, 106, 141, 176];

#[derive(Component)]
pub struct UpgradeSelectionUI;

#[derive(Component)]
pub struct CardDeckBundle;

#[derive(Component)]
pub struct CardButton;

#[derive(Component)]
pub struct KeyboardLabel;

#[derive(Component)]
pub struct GamepadLabel;

#[derive(Component)]
pub struct CardProgressFill;

#[derive(Component)]
pub struct CardHoldBar;

#[derive(Component)]
pub struct CardIndex(pub usize);

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
impl Display for UpgradeRarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl UpgradeRarity {
    pub fn get_color(&self) -> Color {
        match self {
            UpgradeRarity::Common => Color::srgb(0.75, 0.75, 0.75),
            UpgradeRarity::Uncommon => Color::srgb(0.3, 0.9, 0.45),
            UpgradeRarity::Rare => Color::srgb(0.35, 0.55, 1.0),
            UpgradeRarity::Legendary => Color::srgb(1.0, 0.75, 0.0),
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
