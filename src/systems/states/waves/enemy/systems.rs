use crate::systems::game::MarkedForDespawn;
use crate::systems::states::waves::components::Health;
use crate::systems::states::waves::enemy::components::{Enemy, Spawning, Splitter};
use crate::systems::states::waves::enemy::kinds::EnemyKind;
use crate::systems::states::waves::enemy::messages::EnemyDeathMessage;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Entity, GlobalTransform, MessageReader, MessageWriter, Query, Timer, TimerMode,
    Transform,
};

pub fn check_if_dead(
    mut commands: Commands,
    mut message_writer: MessageWriter<EnemyDeathMessage>,
    query: Query<(Entity, &Health, &Enemy, &GlobalTransform, Option<&Splitter>)>,
) {
    for (entity, health, enemy, transform, splitter) in query.iter() {
        if health.value <= 0.0 {
            message_writer.write(EnemyDeathMessage {
                entity,
                kind: enemy.kind,
                position: transform.translation(),
                xp_reward: enemy.xp_reward,
                split_count: splitter.map(|s| s.split_count).unwrap_or(0),
            });
            commands.entity(entity).insert(MarkedForDespawn);
        }
    }
}

pub fn handle_splitter_death(
    mut commands: Commands,
    mut msg_reader: MessageReader<EnemyDeathMessage>,
) {
    for msg in msg_reader.read() {
        if msg.split_count == 0 {
            continue;
        }

        for i in 0..msg.split_count {
            let angle = (i as f32 / msg.split_count as f32) * std::f32::consts::TAU;
            let offset = Vec2::new(angle.cos(), angle.sin()) * 30.0;
            let spawn_pos = msg.position.truncate() + offset;

            commands.spawn((
                Transform::from_translation(spawn_pos.extend(0.0)),
                Spawning {
                    timer: Timer::from_seconds(0.3, TimerMode::Once),
                    kind: EnemyKind::SmallSplitter,
                },
            ));
        }
    }
}
