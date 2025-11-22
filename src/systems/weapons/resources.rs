use crate::components::Weapon;
use crate::components::WeaponKind::{MachineGun, Pistol, Shotgun};
use crate::resources::tiles_to_pixels;
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::color::palettes::basic::{BLACK, RED};
use bevy::color::palettes::css::PINK;
use bevy::mesh::Mesh;
use bevy::prelude::{Circle, ColorMaterial, Commands, Rectangle, ResMut, Resource};
#[derive(Resource)]
pub struct GeometricMeshes {
    pub circle_small: Handle<Mesh>,
    pub circle_medium: Handle<Mesh>,
    pub circle_large: Handle<Mesh>,
    pub square_large: Handle<Mesh>,
    pub rectangle_small: Handle<Mesh>,
    pub rectangle_medium: Handle<Mesh>,
    pub rectangle_large: Handle<Mesh>,
}

#[derive(Resource)]
pub struct ColorMeshes {
    pub red: Handle<ColorMaterial>,
    pub black: Handle<ColorMaterial>,
    pub pink: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct WeaponsLibrary {
    pub weapons: Vec<Weapon>,
}

pub fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Load bullet assets
    commands.insert_resource(GeometricMeshes {
        circle_small: meshes.add(Circle::new(3.0)),
        circle_medium: meshes.add(Circle::new(10.0)),
        circle_large: meshes.add(Circle::new(25.0)),
        square_large: meshes.add(Rectangle::new(25.0, 25.0)),
        rectangle_small: meshes.add(Rectangle::new(25.0, 10.0)),
        rectangle_medium: meshes.add(Rectangle::new(35.0, 10.0)),
        rectangle_large: meshes.add(Rectangle::new(35.0, 20.0)),
    });
    commands.insert_resource(ColorMeshes {
        red: materials.add(Color::from(RED)),
        black: materials.add(Color::from(BLACK)),
        pink: materials.add(Color::from(PINK)),
    });

    // Load weapons
    commands.insert_resource(WeaponsLibrary {
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
    });
}
