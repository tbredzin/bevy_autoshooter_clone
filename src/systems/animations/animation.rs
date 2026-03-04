use crate::systems::animations::components::AnimationDuration;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Asset, TypePath, Clone)]
pub struct SpriteAnimation {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub first: usize,
    pub last: usize,
    pub frame_interval: Duration,
    pub repeat: bool,
    pub flip_x: bool,
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
            image,
            layout,
            first,
            last: first + frames - 1,
            frame_interval: Duration::from_millis(120),
            repeat: false,
            flip_x: false,
        }
    }

    pub fn with_duration(mut self, duration: AnimationDuration) -> Self {
        let frame_count = self.last - self.first + 1;
        self.frame_interval = duration.frame_interval(frame_count);
        self
    }

    pub fn looping(mut self) -> Self {
        self.repeat = true;
        self
    }

    pub fn reversed(mut self, reversed: bool) -> Self {
        self.flip_x = reversed;
        self
    }
}
