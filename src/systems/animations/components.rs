use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerAnimationBundle {
    pub sprite: Sprite,
    pub animation: Animation,
    pub player: PlayerSprite,
}

impl PlayerAnimationBundle {
    pub fn new(
        image: Handle<Image>,
        atlas: Handle<TextureAtlasLayout>,
        first: usize,
        last: usize,
    ) -> Self {
        Self {
            player: PlayerSprite::default(),
            sprite: Sprite::from_atlas_image(image, TextureAtlas::from(atlas)),
            animation: Animation {
                timer: Timer::from_seconds(0.12, TimerMode::Repeating),
                first,
                last,
                repeated: true,
            },
        }
    }
}

#[derive(Component, Debug)]
pub struct Animation {
    pub timer: Timer,
    pub first: usize,
    pub last: usize,
    pub repeated: bool,
}

#[derive(Component, Default, Debug)]
pub struct PlayerSprite {}

#[derive(Component)]
pub struct ShadowSprite;
