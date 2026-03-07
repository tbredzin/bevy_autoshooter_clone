use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, Name, Timer};
use bevy::time::TimerMode::Repeating;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum WeaponKind {
    MachineGun,
    Pistol,
    Shotgun,
}

#[derive(Component, Clone, Debug)]
pub struct Weapon {
    pub base_damage: f32,
    pub base_cooldown: f32,
    pub base_range: f32,
    pub kind: WeaponKind,
    pub bullet_size: Vec2,
    pub weapon_size: Vec2,
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

#[derive(Component, Clone, Debug)]
pub struct Bullet {
    pub direction: Vec2,
    pub damage: f32,
    pub kind: WeaponKind,
}

#[derive(Bundle, Clone)]
pub struct WeaponBundle {
    pub name: Name,
    pub weapon: Weapon,
    pub cooldown: WeaponCooldown,
}
impl WeaponBundle {
    pub fn new(name: String, weapon: Weapon, cooldown: f32) -> Self {
        Self {
            name: Name::new(name),
            weapon,
            cooldown: WeaponCooldown {
                timer: Timer::from_seconds(cooldown, Repeating),
            },
        }
    }
}
