use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerAnimationBundle {
    pub animation: Sprite,
    pub timer: AnimationTimer,
    pub indices: AnimationIndices,
    pub sprite: PlayerSprite,
}

impl PlayerAnimationBundle {
    pub fn new(
        image: Handle<Image>,
        atlas: Handle<TextureAtlasLayout>,
        first: usize,
        last: usize,
    ) -> Self {
        Self {
            animation: Sprite::from_atlas_image(image, TextureAtlas::from(atlas)),
            timer: AnimationTimer::default(),
            indices: AnimationIndices { first, last },
            sprite: PlayerSprite::default(),
        }
    }
}

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
}

impl Default for AnimationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Default, Debug)]
pub struct PlayerSprite {}

#[derive(Component)]
pub struct ShadowSprite;
