use bevy::asset::{AssetServer, Assets, Handle};
use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::UVec2;
use bevy::prelude::{FromWorld, Resource, World};

#[derive(Resource)]
pub struct HUDTextureAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for HUDTextureAtlas {
    fn from_world(world: &mut World) -> Self {
        let texture =
            { world.resource::<AssetServer>() }.load("spritesheet/IconGodotNode/spritesheet.png");
        let mut atlas = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let layout = atlas.add(TextureAtlasLayout::from_grid(
            UVec2::splat(16u32), // tile size (width, height)
            16,                  // columns
            6,                   // rows
            None,                // no padding
            None,                // no offset
        ));
        HUDTextureAtlas { texture, layout }
    }
}
