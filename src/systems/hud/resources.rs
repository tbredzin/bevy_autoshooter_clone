use bevy::asset::Handle;
use bevy::image::{Image, TextureAtlasLayout};
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct HUDTextureAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
