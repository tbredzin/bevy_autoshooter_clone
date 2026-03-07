use crate::systems::states::waves::enemy::kinds::EnemyKind;
use bevy::prelude::*;

#[derive(Message, Debug)]
pub struct EnemyDeathMessage {
    pub position: Vec3,
    pub xp_reward: u32,
    pub split_count: u32, // 0 for non-splitters
}

#[derive(Message, Debug)]
pub struct EnemySpawningMessage {
    pub entity: Entity,
    pub kind: EnemyKind,
}

#[derive(Message, Debug)]
pub struct EnemySpawnedMessage {
    pub entity: Entity,
    pub kind: EnemyKind,
    pub transform: Transform,
}
