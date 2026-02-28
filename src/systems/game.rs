use crate::components::{Enemy, Health, MarkedForDespawn, WaveEndedText};
use crate::resources::{tiles_to_pixels, WaveManager, WaveState, GAME_AREA};
use crate::systems::input::resources::ActionState;
use crate::systems::player::components::Player;
use crate::systems::player::experience::PlayerExperience;
use crate::systems::player_upgrades::components::{NextWaveButton, PlayerStats};
use bevy::prelude::*;

pub fn out_of_bounds_system(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<MarkedForDespawn>>,
) {
    const MARGIN: f32 = tiles_to_pixels(2.0);

    for (entity, transform) in &query {
        let entity_pos = transform.translation().truncate();

        if entity_pos.x < GAME_AREA.min.x - MARGIN
            || entity_pos.x > GAME_AREA.max.x + MARGIN
            || entity_pos.y < GAME_AREA.min.y - MARGIN
            || entity_pos.y > GAME_AREA.max.y + MARGIN
        {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_marked_entities(
    mut commands: Commands,
    query: Query<Entity, With<MarkedForDespawn>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

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

pub fn start_next_wave(
    actions: Res<ActionState>,
    mut wave_manager: ResMut<WaveManager>,
    next_wave_menu_query: Query<Entity, With<NextWaveButton>>,
    mut player_query: Query<(&mut PlayerStats, &mut PlayerExperience, &mut Health), With<Player>>,
) {
    // Only if the player press start next wave
    if actions.start_next_wave && !next_wave_menu_query.is_empty() {
        println!("Starting next wave");
        // Reset wave timer
        wave_manager
            .wave_timer
            .set_duration(std::time::Duration::from_secs_f32(
                crate::resources::WAVE_DURATION,
            ));
        wave_manager.wave_timer.reset();

        // Reset enemy spawn timer to base rate
        wave_manager
            .enemy_spawn_timer
            .set_duration(std::time::Duration::from_secs_f32(
                crate::resources::SPAWN_RATE,
            ));
        wave_manager.enemy_spawn_timer.reset();

        // Reset player's counters and health
        for (stats, mut experience, mut health) in player_query.iter_mut() {
            println!("Resetting level counter from {}", experience.new_levels);
            experience.new_levels = 0;
            health.value = stats.max_health;
        }

        wave_manager.wave += 1;
        wave_manager.wave_state = WaveState::Running;
    }
}
