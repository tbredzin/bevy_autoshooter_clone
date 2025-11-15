use crate::components::{HUDBundle, Player, PlayerBundle};
use crate::resources::{
    tiles_to_pixels, TilesTextureAtlas, FIRE_RATE, TILES_X, TILES_Y, TILE_SIZE,
};
use bevy::prelude::*;
use rand::Rng;

pub fn setup(
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

    // Camera
    commands.spawn((Camera2d, Msaa::Sample4));

    // Player
    commands.spawn(PlayerBundle {
        mesh: Mesh2d(meshes.add(Circle::new(20.0))),
        mesh_material2d: MeshMaterial2d(materials.add(Color::srgb(0.2, 0.6, 1.0))),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        player: Player {
            fire_timer: Timer::from_seconds(FIRE_RATE, TimerMode::Repeating),
        },
    });

    // HUD
    commands.spawn(HUDBundle::new(
        "Wave: 1 | XP: 0 | Level: 1 | HP: 100/100".to_string(),
    ));
}

pub fn setup_background(mut commands: Commands, atlas: Res<TilesTextureAtlas>) {
    let mut rng = rand::rng();

    // Using tile constants - guaranteed to be whole numbers
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
