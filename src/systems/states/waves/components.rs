use bevy::prelude::Component;
#[derive(Component)]
pub struct LevelBackground {}

#[derive(Component)]
pub struct LevelOverlay;

#[derive(Component)]
pub struct Health {
    pub value: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self { value: 100.0 }
    }
}

#[derive(Component)]
pub struct Dying;

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Component, PartialEq, Clone, Copy, Debug, Default, Hash, Eq)]
pub enum Action {
    #[default]
    IDLE,
    WALKING,
    DASHING,
    DYING,
}

#[derive(Component, Clone, Copy, Debug, Eq, Default, PartialEq, Hash)]
pub enum Direction {
    #[default]
    EAST,
    NORTH,
    NORTHEAST,
    NORTHWEST,
    SOUTH,
    SOUTHEAST,
    SOUTHWEST,
    WEST,
}
