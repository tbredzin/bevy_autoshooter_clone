use crate::components::WeaponKind::{MachineGun, Pistol, Shotgun};
use crate::components::*;
use crate::resources::{
    tiles_to_pixels, ColorMeshes, GeometricMeshes, TilesTextureAtlas, WeaponsLibrary, TILES_X, TILES_Y,
    TILE_SIZE,
};
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;
use rand::Rng;
use std::f32::consts;

pub fn init_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load spritesheet as tile texture atlas resource
    let texture = asset_server.load("spritesheet/spritesheet_tiles.png");
    commands.insert_resource(TilesTextureAtlas {
        texture,
        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(TILE_SIZE as u32), // tile size (width, height)
            27,                             // columns
            20,                             // rows
            Some(UVec2::splat(10)),         // no padding
            None,                           // no offset
        )),
    });

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

pub fn spawn_background(mut commands: Commands, atlas: Res<TilesTextureAtlas>) {
    let mut rng = rand::rng();
    for x in 0..TILES_X + 1 {
        for y in 0..TILES_Y + 1 {
            let pos_x = tiles_to_pixels(x as f32 - TILES_X as f32 / 2.0);
            let pos_y = tiles_to_pixels(y as f32 - TILES_Y as f32 / 2.0);

            // Use different tile indices for variety
            let tile_index = rng.random_range(12..16);

            commands.spawn((
                Sprite::from_atlas_image(
                    atlas.texture.clone(),
                    TextureAtlas {
                        layout: atlas.layout.clone(),
                        index: tile_index,
                    },
                ),
                Transform::from_xyz(pos_x, pos_y, -10.0),
            ));
        }
    }
}

pub fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    weapons_library: Res<WeaponsLibrary>,
) -> Result {
    // Camera
    commands.spawn((Camera2d, Msaa::Sample4));

    // HUD
    commands.spawn(HUDBundle::new(
        "Wave: 1 | XP: 0 | Level: 1 | HP: 100/100".to_string(),
    ));

    // Player
    let mut player_entity = commands.spawn(PlayerBundle {
        mesh: Mesh2d(meshes.add(Circle::new(20.0))),
        mesh_material2d: MeshMaterial2d(materials.add(Color::srgb(0.2, 0.6, 1.0))),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        player: Player {},
        health: Health {
            value: 100.0,
            max: 100.0,
        },
        xp: PlayerExperience { value: 0, level: 1 },
    });

    // Give all weapons available to player
    let total_weapons = weapons_library.weapons.len();
    let orbit_radius = 12.0 * weapons_library.weapons.len() as f32; // Distance from player center
    let sector_arc = consts::TAU / (total_weapons as f32) * 0.8; // 80% of full sector

    for (index, weapon) in weapons_library.weapons.iter().enumerate() {
        let angle = consts::TAU * (index as f32) / (total_weapons as f32);
        player_entity.with_child((
            weapon.clone(),
            Transform::from_xyz(angle.cos() * orbit_radius, angle.sin() * orbit_radius, 0.0),
            WeaponArea {
                orbit_radius: orbit_radius.max(10.0),
                center_arc: angle,
                sector_arc,
            },
        ));
    }
    Ok(())
}
