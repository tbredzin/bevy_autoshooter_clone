use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub fire_timer: f32,
}

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct UIText;