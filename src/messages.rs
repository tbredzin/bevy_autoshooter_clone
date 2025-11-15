use bevy::prelude::*;

#[derive(Message)]
pub struct EnemyDeathMessage(pub Entity);
