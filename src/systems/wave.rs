use crate::components::{Enemy, MarkedForDespawn, WaveEndedText};
use crate::resources::{WaveManager, WaveState};
use bevy::prelude::*;

/// Manages wave timer and state transitions
pub fn update_wave_timer(
    mut wave_manager: ResMut<WaveManager>,
    enemy_query: Query<Entity, With<Enemy>>,
    pause_text_query: Query<Entity, With<WaveEndedText>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    match wave_manager.wave_state {
        WaveState::Running => {
            // Remove pause text if it exists
            if let Ok(entity) = pause_text_query.single() {
                commands.entity(entity).insert(MarkedForDespawn);
            }

            wave_manager.wave_timer.tick(time.delta());

            if wave_manager.wave_timer.is_finished() {
                wave_manager.wave_state = WaveState::Ended;
            }
        }
        WaveState::Ended => {
            // Clear all enemies
            for entity in &enemy_query {
                commands.entity(entity).insert(MarkedForDespawn);
            }
        }
    }
}
