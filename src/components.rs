use crate::systems::enemy::renderer::*;
use crate::systems::weapons::renderer::{draw_bullet, draw_weapon};
use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub value: f32,
}
impl Default for Health {
    fn default() -> Self {
        Self { value: 100.0 }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum WeaponKind {
    MachineGun,
    Pistol,
    Shotgun,
}

#[derive(Component, Clone)]
#[component(on_add = draw_weapon)]
pub struct Weapon {
    pub base_damage: f32,
    pub base_cooldown: f32,
    pub base_range: f32,
    pub kind: WeaponKind,
    // Calculated from PlayerStats:
    pub damage_multiplier: f32,
    pub fire_rate_multiplier: f32,
    pub range_multiplier: f32,
}
#[derive(Component, Clone)]
pub struct WeaponCooldown {
    pub timer: Timer,
}

/// Defines the circular sector area where a weapon can move
#[derive(Component, Clone)]
pub struct WeaponArea {
    /// Radius from player center where weapon orbits
    pub orbit_radius: f32,
    /// How far the weapon can move within its sector (in radians)
    pub sector_arc: f32,
    pub center_arc: f32,
}

impl WeaponArea {
    pub fn angle_range(&self) -> (f32, f32) {
        let center = self.center_arc;
        let half_arc = self.sector_arc / 2.0;
        (center - half_arc, center + half_arc)
    }
}

#[derive(Component, Clone)]
#[component(on_add = draw_bullet)]
pub struct Bullet {
    pub direction: Vec2,
    pub damage: f32,
    pub kind: WeaponKind,
}

#[derive(Component)]
pub struct WaveEndedText;

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

#[derive(Component)]
#[component(on_add = on_enemy_spawning)]
pub struct Spawning {
    pub timer: Timer,
}

#[derive(Component)]
#[component(on_add = on_enemy_spawned)]
pub struct Enemy {
    pub damage: f32,
}

#[derive(Component)]
pub struct MarkedForDespawn;
