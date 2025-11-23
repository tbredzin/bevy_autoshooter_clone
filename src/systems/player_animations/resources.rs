use crate::systems::player::components::PlayerAction;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationAssets {
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub idle_texture: Handle<Image>,

    pub walk_layout: Handle<TextureAtlasLayout>,
    pub walk_texture: Handle<Image>,

    pub dash_layout: Handle<TextureAtlasLayout>,
    pub dash_texture: Handle<Image>,
    pub dash_dust_layout: Handle<TextureAtlasLayout>,
    pub dash_dust_texture: Handle<Image>,

    pub death_layout: Handle<TextureAtlasLayout>,
    pub death_texture: Handle<Image>,
    pub death_shadow_layout: Handle<TextureAtlasLayout>,
    pub death_shadow_texture: Handle<Image>,

    pub shadow_texture: Handle<Image>,
}

impl AnimationAssets {
    pub fn get_animation_sprite(
        &self,
        state: &PlayerAction,
    ) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
        match state {
            PlayerAction::IDLE => (self.idle_texture.clone(), self.idle_layout.clone()),
            PlayerAction::WALKING => (self.walk_texture.clone(), self.walk_layout.clone()),
            PlayerAction::DASHING => (self.dash_texture.clone(), self.dash_layout.clone()),
            PlayerAction::DYING => (self.death_texture.clone(), self.death_layout.clone()),
        }
    }
}

const IDLE_SPRITESHEET_PATH: &'static str = "spritesheet/player/Idle_spritesheet_8x6.png";
const WALK_SPRITESHEET_PATH: &'static str = "spritesheet/player/walk_spritesheet_8x6.png";
const DASH_SPRITESHEET_PATH: &'static str = "spritesheet/player/dash_spritesheet_8x6.png";
const DASH_DUST_SPRITESHEET_PATH: &'static str = "spritesheet/player/dash_dust_spritesheet_8x6.png";
const DEATH_SPRITESHEET_PATH: &'static str = "spritesheet/player/death_spritesheet_8x6.png";
const SHADOW_SPRITE_PATH: &'static str = "spritesheet/player/shadow_sprite.png";
const DEATH_SHADOW_SPRITESHEET_PATH: &'static str =
    "spritesheet/player/death_shadow_spritesheet_8x6.png";

/// System to load all animation assets at startup
pub(crate) fn load_animation_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Each frame is 48x64, arranged in 8 columns and 6 rows
    let frame_size = UVec2::new(48, 64);
    let columns = 8;
    let rows = 6;

    // Create layouts for all spritesheets
    let idle_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        frame_size, columns, rows, None, None,
    ));
    let walk_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        frame_size, columns, rows, None, None,
    ));
    let dash_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        frame_size, columns, rows, None, None,
    ));
    let dash_dust_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        frame_size, columns, rows, None, None,
    ));
    let death_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        frame_size, columns, rows, None, None,
    ));
    let death_shadow_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        frame_size, columns, rows, None, None,
    ));

    // Load textures
    let assets = AnimationAssets {
        idle_layout,
        idle_texture: asset_server.load(IDLE_SPRITESHEET_PATH),

        walk_layout,
        walk_texture: asset_server.load(WALK_SPRITESHEET_PATH),

        dash_layout,
        dash_texture: asset_server.load(DASH_SPRITESHEET_PATH),
        dash_dust_layout,
        dash_dust_texture: asset_server.load(DASH_DUST_SPRITESHEET_PATH),

        death_layout,
        death_texture: asset_server.load(DEATH_SPRITESHEET_PATH),
        death_shadow_layout,
        death_shadow_texture: asset_server.load(DEATH_SHADOW_SPRITESHEET_PATH),

        shadow_texture: asset_server.load(SHADOW_SPRITE_PATH),
    };

    commands.insert_resource(assets);
}
