use crate::systems::states::waves::enemy;
use bevy::prelude::{Component, Timer};
use enemy::renderer::{on_enemy_spawned, on_enemy_spawning};

#[derive(Component)]
#[component(on_add = on_enemy_spawning)]
pub struct Spawning {
    pub timer: Timer,
}

#[derive(Component)]
#[component(on_add = on_enemy_spawned)]
pub struct Enemy {
    pub damage: f32,
}
