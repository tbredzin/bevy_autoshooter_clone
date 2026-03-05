use crate::systems::hud::resources::HUDTextureAtlas;
use crate::systems::states::waves::player::components::StatKind;
use bevy::color::palettes::css::DARK_VIOLET;
use bevy::prelude::*;

// ── Existing icon indices (IconGodotNode spritesheet, 16×16 tiles, 16 cols × 6 rows) ─────
pub const ICON_STATISTICS: usize = 13;
pub const ICON_LEVEL: usize = 3;
pub const ICON_EXPERIENCE: usize = 14;
pub const ICON_DAMAGE: usize = 12;
pub const ICON_FIRE_RATE: usize = 30;
pub const ICON_RANGE: usize = 18;
pub const ICON_HEALTH: usize = 49;
pub const ICON_SPEED: usize = 29;
pub const ICON_WAVE: usize = 59;
pub const ICON_TIMER: usize = 8;
pub const ICON_LEVEL_UP: usize = 54;

// vivid red  — heart
pub const TINT_WAVE: Color = Color::srgb(0.85, 0.35, 0.25); // indian-red — skull/wave
pub const TINT_TIMER: Color = Color::srgb(0.53, 0.81, 0.98); // sky-blue   — hourglass
pub const TINT_LEVEL: Color = Color::srgb(0.85, 0.65, 0.13); // goldenrod  — star

// ── Bar fill colors ───────────────────────────────────────────────────────────────────────
pub const FILL_HEALTH: Color = Color::srgb(0.20, 0.90, 0.30); // bright green (healthy)
pub const FILL_HEALTH_DANGER: Color = Color::srgb(0.85, 0.20, 0.20); // red        (< 25 % HP)
pub const FILL_TIMER: Color = Color::srgb(0.53, 0.81, 0.98); // sky-blue
pub const FILL_TIMER_URGENT: Color = Color::srgb(0.85, 0.65, 0.13); // goldenrod  (< 5 s)
pub const FILL_XP: Color = Color::srgb(0.80, 0.60, 1.00); // violet

// ── Stats-popup display kinds ─────────────────────────────────────────────────────────────
#[derive(Component)]
pub struct StatsPopup;

#[derive(Component)]
pub enum DisplayStatKind {
    Level,
    Experience,
    Wave,
    PlayerStat(StatKind),
    Health,
}

impl From<StatKind> for DisplayStatKind {
    fn from(kind: StatKind) -> DisplayStatKind {
        DisplayStatKind::PlayerStat(kind)
    }
}

impl DisplayStatKind {
    pub fn get_display_info(&self) -> (usize, &'static str, Color) {
        match self {
            DisplayStatKind::Level => (ICON_LEVEL, "Level", Color::srgb(1.0, 0.8, 0.2)),
            DisplayStatKind::Experience => {
                (ICON_EXPERIENCE, "Experience", Color::srgb(0.8, 0.6, 1.0))
            }
            DisplayStatKind::Wave => (ICON_WAVE, "Wave", Color::srgb(1.0, 1.0, 1.0)),
            DisplayStatKind::Health => (ICON_HEALTH, "Health", Color::srgb(0.2, 1.0, 0.3)),
            DisplayStatKind::PlayerStat(stat) => match stat {
                StatKind::Damage => (ICON_DAMAGE, "Damage", Color::srgb(1.0, 0.4, 0.4)),
                StatKind::FireRate => (ICON_FIRE_RATE, "Fire Rate", Color::srgb(1.0, 0.6, 0.2)),
                StatKind::Range => (ICON_RANGE, "Range", Color::srgb(0.4, 0.8, 1.0)),
                StatKind::MaxHealth => (ICON_HEALTH, "Health", Color::srgb(0.2, 1.0, 0.3)),
                StatKind::Speed => (ICON_SPEED, "Speed", Color::srgb(0.4, 1.0, 0.8)),
            },
        }
    }
}

// ── New top-bar marker components ─────────────────────────────────────────────────────────

/// Root node of the top HUD bar — despawned on OnExit(InWave).
#[derive(Component)]
pub struct HUDTopBar;

/// The fill node inside the health progress track.
#[derive(Component)]
pub struct HUDHealthFill;

/// The "XX / YY" text inside the health pill.
#[derive(Component)]
pub struct HUDHealthText;

/// The wave number text inside the wave badge.
#[derive(Component)]
pub struct HUDWaveText;

/// The fill node inside the time progress track (drains as time passes).
#[derive(Component)]
pub struct HUDTimeFill;

/// The remaining-seconds text inside the timer pill.
#[derive(Component)]
pub struct HUDTimeText;

/// The "Lv N" text inside the level pill.
#[derive(Component)]
pub struct HUDLevelText;

/// The fill node inside the XP mini-bar.
#[derive(Component)]
pub struct HUDXPFill;

/// The thin bottom border of the top bar — driven by the palette animation.
#[derive(Component)]
pub struct HUDBottomBorder;

// ── Level-up indicator (unchanged) ───────────────────────────────────────────────────────

#[derive(Component)]
pub struct HUDLevelUps {}

#[derive(Component)]
pub struct HUDLevelUp {}

impl HUDLevelUp {
    pub fn render(sprites: Res<HUDTextureAtlas>) -> (HUDLevelUp, ImageNode, Node) {
        (
            HUDLevelUp {},
            ImageNode::from_atlas_image(
                sprites.texture.clone(),
                TextureAtlas {
                    layout: sprites.layout.clone(),
                    index: ICON_LEVEL_UP,
                },
            )
            .with_color(Color::Srgba(DARK_VIOLET)),
            Node {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                ..default()
            },
        )
    }
}
