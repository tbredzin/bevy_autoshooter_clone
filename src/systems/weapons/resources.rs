use crate::components::Weapon;
use crate::components::WeaponKind::{MachineGun, Pistol, Shotgun};
use crate::resources::tiles_to_pixels;
use bevy::asset::{Assets, Handle};
use bevy::color::palettes::basic::{BLACK, RED};
use bevy::color::palettes::css::PINK;
use bevy::color::Color;
use bevy::mesh::Mesh;
use bevy::prelude::TimerMode::Repeating;
use bevy::prelude::{Circle, ColorMaterial, Commands, Rectangle, ResMut, Resource, Timer};
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
                cooldown: Timer::from_seconds(0.1, Repeating),
                damage: 0.1,
                range: tiles_to_pixels(10.0),
            },
            Weapon {
                cooldown: Timer::from_seconds(1.0, Repeating),
                damage: 5.0,
                range: tiles_to_pixels(12.0),
                kind: Pistol,
            },
            Weapon {
                cooldown: Timer::from_seconds(1.0, Repeating),
                damage: 100.0,
                range: tiles_to_pixels(8.0),
                kind: Shotgun,
            },
        ],
    });
}
