use crate::components::{Player, UIText};
use crate::resources::{TILE_SIZE, TILES_X, TILES_Y, TilesTextureAtlas, tiles_to_pixels};
use bevy::prelude::*;
use rand::Rng;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((Camera2d, Msaa::Sample4));

    // Player
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.6, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player { fire_timer: 0.0 },
    ));

    // Game UI - Centered at top
    commands.spawn((
        Text::new("Wave: 1 | XP: 0 | Level: 1 | HP: 100/100"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            top: Val::Px(10.0),
            justify_self: JustifySelf::Center,
            ..default()
        },
        UIText,
    ));

    let texture = asset_server.load("spritesheet/spritesheet_tiles.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE as u32), // tile size (width, height)
        27,                             // columns
        20,                             // rows
        Some(UVec2::splat(10)),         // no padding
        None,                           // no offset
    );

    let layout_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(TilesTextureAtlas {
        texture,
        layout: layout_handle,
    });
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
