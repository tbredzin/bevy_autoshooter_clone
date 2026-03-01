use crate::systems::hud::components::DisplayStatKind;
use crate::systems::states::upgrades::components::UpgradeCardState::Unselected;
use crate::systems::states::waves::player::components::StatKind;
use bevy::color::Color;
use bevy::prelude::{Component, Timer};
use std::fmt;
use std::fmt::Display;

// ─────────────────────────────────────────────────────────────────────────────
// UI markers
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct UpgradeSelectionUI;

#[derive(Component)]
pub struct UpgradeCardsRow;

/// Marks the top-right input badge on each card.
/// The badge contains both a KeyboardLabel and a GamepadLabel child,
/// exactly one of which is visible at any time.
#[derive(Component)]
pub struct CardKeyBadge(pub usize);

/// Keyboard "1/2/3/4" text node inside a CardKeyBadge.
#[derive(Component)]
pub struct KeyboardLabel;

/// Gamepad button image node inside a CardKeyBadge.
#[derive(Component)]
pub struct GamepadLabel;

/// Absolute background fill that rises from the card bottom while the player holds.
#[derive(Component)]
pub struct CardProgressFill;

/// Thin horizontal bar at the card bottom that fills left→right while holding.
#[derive(Component)]
pub struct CardHoldBar;

// ─────────────────────────────────────────────────────────────────────────────
// UpgradeCard
// ─────────────────────────────────────────────────────────────────────────────

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

// ─────────────────────────────────────────────────────────────────────────────
// Rarity
// ─────────────────────────────────────────────────────────────────────────────

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

// ─────────────────────────────────────────────────────────────────────────────
// Timing / animation constants
// ─────────────────────────────────────────────────────────────────────────────

/// How long (s) a button must be held to confirm the selection.
pub const HOLD_DURATION: f32 = 1.0;

/// How long (s) the burst animation plays after confirming.
pub const SELECTION_ANIM_DURATION: f32 = 0.5;

/// Wobble frequency multiplier during the burst animation.
pub const SELECTION_FREQUENCY: f32 = 4.0;

/// Gamepad button atlas sprite indices for card slots 0-3 (West / South / North / East).
/// Formula: SPRITESHEET_BEGIN(71) + SPRITESHEET_WIDTH(35) × row
pub const GAMEPAD_BUTTON_INDICES: [usize; 4] = [71, 106, 141, 176];

/// Which card slot maps to which button.
#[derive(Component)]
pub struct CardIndex(pub usize);

// ─────────────────────────────────────────────────────────────────────────────
// Animation state
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Component, Debug)]
pub struct UpgradeCardAnimation {
    /// Holding  → counts up to HOLD_DURATION         (ticked in systems.rs)
    /// Selected → counts up to SELECTION_ANIM_DURATION (ticked in systems.rs)
    pub timer: Timer,
}

impl Default for UpgradeCardAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
        }
    }
}
