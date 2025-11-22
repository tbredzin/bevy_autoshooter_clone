use crate::components::*;
use crate::resources::{tiles_to_pixels, TilesTextureAtlas, TILES_X, TILES_Y, TILE_SIZE};
use crate::systems::player::components::{Player, PlayerBundle};
use crate::systems::player::experience::PlayerExperience;
use crate::systems::player_upgrades::components::PlayerStats;
use crate::systems::weapons::resources::WeaponsLibrary;
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;
use rand::Rng;
use std::f32::consts;

pub fn init_resources(
    mut commands: Commands,
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
}

pub fn spawn_background(mut commands: Commands, atlas: Res<TilesTextureAtlas>) {
    let mut rng = rand::rng();
    for x in 0..TILES_X + 1 {
        for y in 0..TILES_Y + 1 {
            let pos_x = tiles_to_pixels(x as f32 - TILES_X as f32 / 2.0);
            let pos_y = tiles_to_pixels(y as f32 - TILES_Y as f32 / 2.0);

            // Use different tile indices for variety
            let tile_index = rng.random_range(0..4);

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
    let mut player_entity = commands.spawn((
        PlayerBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            player: Player {},
            health: Health::default(),
            xp: PlayerExperience::default(),
            stats: PlayerStats::default(),
        },
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.6, 1.0))),
    ));

    // Give all weapons available to player
    let total_weapons = weapons_library.weapons.len();
    let orbit_radius = 12.0 * weapons_library.weapons.len() as f32; // Distance from player center
    let sector_arc = consts::TAU / (total_weapons as f32) * 0.8; // 80% of full sector

    for (index, weapon) in weapons_library.weapons.iter().enumerate() {
        let angle = consts::TAU * (index as f32) / (total_weapons as f32);
        player_entity.with_child((
            weapon.clone(),
            WeaponCooldown {
                timer: Timer::from_seconds(weapon.base_cooldown, Repeating),
            },
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
