use crate::systems::states::waves::weapons::components::{Bullet, Weapon};
use bevy::prelude::{Entity, Message, Name, Transform};

#[derive(Message, Debug)]
pub struct WeaponSpawnedMessage {
    pub name: Name,
    pub entity: Entity,
    pub player: Entity,
    pub weapon: Weapon,
}

#[derive(Message, Debug)]
pub struct BulletSpawnedMessage {
    pub entity: Entity,
    pub bullet: Bullet,
    pub transform: Transform,
}
