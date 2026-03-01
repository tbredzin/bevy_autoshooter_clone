use crate::systems::hud::resources::HUDTextureAtlas;
use crate::systems::states::waves::player::components::StatKind;
use bevy::prelude::*;

pub const ICON_STATISTICS: usize = 13;
pub const ICON_LEVEL: usize = 3;
pub const ICON_EXPERIENCE: usize = 14;
pub const ICON_DAMAGE: usize = 12;
pub const ICON_FIRE_RATE: usize = 30;
pub const ICON_RANGE: usize = 18;
pub const ICON_HEALTH: usize = 49;
pub const ICON_SPEED: usize = 29;
pub const ICON_WAVE: usize = 59;

#[derive(Component)]
pub struct StatsDisplayUI;

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

#[derive(Component)]
pub struct HUDText;

#[derive(Bundle)]
pub struct HUDTop {
    text: Text,
    font: TextFont,
    color: TextColor,
    location: Node,
    ui: HUDText,
}

impl HUDTop {
    pub fn new(text: String) -> Self {
        Self {
            text: Text::new(text),
            font: TextFont {
                font_size: 24.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
            location: Node {
                top: Val::Px(10.0),
                justify_self: JustifySelf::Center,
                ..default()
            },
            ui: HUDText,
        }
    }
}

#[derive(Component)]
pub struct HUDLevelUps {}

#[derive(Component)]
pub struct HUDLevelUp {}

const GOLD: Color = Color::srgb(218.0, 145.0, 0.0);
impl HUDLevelUp {
    pub fn render(sprites: Res<HUDTextureAtlas>) -> (HUDLevelUp, ImageNode) {
        (
            HUDLevelUp {},
            ImageNode::from_atlas_image(
                sprites.texture.clone(),
                TextureAtlas {
                    layout: sprites.layout.clone(),
                    index: ICON_LEVEL,
                },
            )
            .with_color(GOLD),
        )
    }
}
