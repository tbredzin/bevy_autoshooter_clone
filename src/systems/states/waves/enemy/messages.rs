use crate::systems::states::waves::enemy::kinds::EnemyKind;
use bevy::prelude::*;

#[derive(Message)]
pub struct EnemyDeathMessage {
    pub entity: Entity,
    pub kind: EnemyKind,
    pub position: Vec3,
    pub xp_reward: u32,
    pub split_count: u32, // 0 for non-splitters
}
