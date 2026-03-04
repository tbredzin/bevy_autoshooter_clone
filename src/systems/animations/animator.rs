use crate::systems::animations::animation::SpriteAnimation;
use bevy::prelude::*;

#[derive(Component)]
pub struct SpriteAnimator {
    pub current: Handle<SpriteAnimation>,
    pub(crate) pending: Option<Handle<SpriteAnimation>>,
    pub frame_timer: Timer,
    pub finished: bool,
}

impl SpriteAnimator {
    /// `pending` is pre-set so the tick system initialises the sprite on the very first frame.
    pub fn new(handle: Handle<SpriteAnimation>) -> Self {
        Self {
            current: handle.clone(),
            pending: Some(handle),
            frame_timer: Timer::from_seconds(1.0 / 8.0, TimerMode::Repeating),
            finished: false,
        }
    }

    /// Stages a clip change. No-op if the handle is already current.
    /// The actual atlas/image update happens inside the tick system.
    pub fn switch(&mut self, handle: Handle<SpriteAnimation>) {
        if handle != self.current {
            self.pending = Some(handle);
        }
    }
}

pub fn count_frames(
    resource: &Assets<TextureAtlasLayout>,
    layout: &Handle<TextureAtlasLayout>,
) -> usize {
    let atlas = resource.get(layout).unwrap();
    (atlas.size.x / atlas.textures[0].width()) as usize
}
