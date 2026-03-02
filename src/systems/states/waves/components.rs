use bevy::prelude::Component;
#[derive(Component)]
pub struct LevelBackground {}

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
pub struct Dying {}
