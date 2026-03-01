use crate::systems::constants::tiles_to_pixels;
use crate::systems::states::waves::weapons::components::Weapon;
use crate::systems::states::waves::weapons::components::WeaponKind::{MachineGun, Pistol, Shotgun};
use bevy::asset::{Assets, Handle};
use bevy::color::palettes::basic::{BLACK, RED};
use bevy::color::palettes::css::PINK;
use bevy::color::Color;
use bevy::mesh::Mesh;
use bevy::prelude::{Circle, ColorMaterial, FromWorld, Rectangle, Resource, World};

#[derive(Resource)]
pub struct GeometricMeshes {
    pub circle_small: Handle<Mesh>,
    pub circle_medium: Handle<Mesh>,
    pub square_large: Handle<Mesh>,
    pub rectangle_small: Handle<Mesh>,
    pub rectangle_medium: Handle<Mesh>,
    pub rectangle_large: Handle<Mesh>,
}
impl FromWorld for GeometricMeshes {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        GeometricMeshes {
            circle_small: meshes.add(Circle::new(2.0)),
            circle_medium: meshes.add(Circle::new(8.0)),
            square_large: meshes.add(Rectangle::new(18.0, 18.0)),
            rectangle_small: meshes.add(Rectangle::new(18.0, 8.0)),
            rectangle_medium: meshes.add(Rectangle::new(18.0, 8.0)),
            rectangle_large: meshes.add(Rectangle::new(18.0, 18.0)),
        }
    }
}

#[derive(Resource)]
pub struct ColorMeshes {
    pub red: Handle<ColorMaterial>,
    pub black: Handle<ColorMaterial>,
    pub pink: Handle<ColorMaterial>,
}
impl FromWorld for ColorMeshes {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        ColorMeshes {
            red: materials.add(Color::from(RED)),
            black: materials.add(Color::from(BLACK)),
            pink: materials.add(Color::from(PINK)),
        }
    }
}

#[derive(Resource)]
pub struct WeaponsLibrary {
    pub weapons: Vec<Weapon>,
}

impl Default for WeaponsLibrary {
    fn default() -> Self {
        WeaponsLibrary {
            weapons: vec![
                Weapon {
                    kind: MachineGun,
                    base_cooldown: 0.1,
                    base_damage: 0.1,
                    base_range: tiles_to_pixels(10.0),
                    damage_multiplier: 1.0,
                    fire_rate_multiplier: 1.0,
                    range_multiplier: 1.0,
                },
                Weapon {
                    base_cooldown: 1.0,
                    base_damage: 5.0,
                    base_range: tiles_to_pixels(12.0),
                    kind: Pistol,
                    damage_multiplier: 1.0,
                    fire_rate_multiplier: 1.0,
                    range_multiplier: 1.0,
                },
                Weapon {
                    base_cooldown: 1.0,
                    base_damage: 100.0,
                    base_range: tiles_to_pixels(8.0),
                    kind: Shotgun,
                    damage_multiplier: 1.0,
                    fire_rate_multiplier: 1.0,
                    range_multiplier: 1.0,
                },
            ],
        }
    }
}
