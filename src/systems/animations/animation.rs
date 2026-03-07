use bevy::prelude::*;
use std::time::Duration;

#[derive(Asset, TypePath, Clone)]
pub struct SpriteAnimation {
    pub spritesheet: Spritesheet,
    pub frame_interval: Duration,
    pub repeat: bool,
}

#[derive(Clone)]
pub struct Spritesheet {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub first: usize,
    pub last: usize,
    pub flip_x: bool,
    pub custom_size: Option<Vec2>,
}

impl SpriteAnimation {
    pub fn from_row(
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        row: usize,
        frames: usize,
    ) -> Self {
        let first = row * frames;
        Self {
            frame_interval: Duration::from_millis(120),
            spritesheet: Spritesheet {
                first,
                image,
                layout,
                last: first + frames - 1,
                flip_x: false,
                custom_size: None,
            },
            repeat: false,
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.frame_interval = duration;
        self
    }

    pub fn looping(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }
    pub(crate) fn to_sprite(&self) -> Sprite {
        Sprite {
            image: self.spritesheet.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: self.spritesheet.layout.clone(),
                index: self.spritesheet.first,
            }),
            custom_size: self.spritesheet.custom_size,
            ..default()
        }
    }
}
