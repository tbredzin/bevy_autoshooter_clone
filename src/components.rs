use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub fire_timer: f32,
}

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct UIText;

#[derive(Component)]
pub struct PreSpawn {
    pub timer: f32,
    pub spawn_position: Vec3,
}

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}
