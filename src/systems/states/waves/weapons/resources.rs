use crate::systems::constants::tiles_to_pixels;
use crate::systems::states::waves::weapons::components::Weapon;
use crate::systems::states::waves::weapons::components::WeaponKind::{MachineGun, Pistol, Shotgun};
use bevy::math::Vec2;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct WeaponsLibrary {
    pub weapons: Vec<Weapon>,
}

impl WeaponsLibrary {}

impl Default for WeaponsLibrary {
    fn default() -> Self {
        WeaponsLibrary {
            weapons: vec![
                Weapon {
                    kind: MachineGun,
                    base_cooldown: 0.30,
                    base_damage: 5.0,
                    base_range: tiles_to_pixels(10.0),
                    damage_multiplier: 1.0,
                    fire_rate_multiplier: 1.0,
                    range_multiplier: 1.0,
                    bullet_size: Vec2::splat(2.),
                    weapon_size: Vec2::new(1., 1.),
                },
                Weapon {
                    base_cooldown: 0.75,
                    base_damage: 7.0,
                    base_range: tiles_to_pixels(12.0),
                    kind: Pistol,
                    damage_multiplier: 1.0,
                    fire_rate_multiplier: 1.0,
                    range_multiplier: 1.0,
                    bullet_size: Vec2::splat(3.),
                    weapon_size: Vec2::new(2., 5.),
                },
                Weapon {
                    base_cooldown: 1.20,
                    base_damage: 18.0,
                    base_range: tiles_to_pixels(8.0),
                    kind: Shotgun,
                    damage_multiplier: 1.0,
                    fire_rate_multiplier: 1.0,
                    range_multiplier: 1.0,
                    bullet_size: Vec2::splat(4.),
                    weapon_size: Vec2::new(2., 5.),
                },
            ],
        }
    }
}
