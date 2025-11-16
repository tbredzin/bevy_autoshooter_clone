// src/systems/upgrade_system.rs
// Add this new file to implement the upgrade selection UI

use crate::components;
use crate::components::{Player, PlayerExperience, Weapon};
use crate::resources::{WaveManager, WaveState};
use crate::systems::player_upgrades::components::*;
use crate::systems::player_upgrades::resources::*;
use bevy::prelude::*;
use components::Health;
// ============================================================================
// Systems
// ============================================================================

/// Handles upgrade selection button clicks
pub fn handle_upgrade_selection(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ChildOf, &UpgradeCardButton),
        Changed<Interaction>,
    >,
    card_query: Query<&UpgradeCard>,
    ui_query: Query<Entity, With<UpgradeUI>>,
    mut applied_upgrades: ResMut<AppliedUpgrades>,
    mut player_query: Query<(&mut Health, &mut PlayerExperience), With<Player>>,
    mut weapon_query: Query<&mut Weapon>,
) {
    for (interaction, child_of, _) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Get the upgrade from the parent card
            if let Ok(card) = card_query.get(child_of.parent()) {
                apply_upgrade(
                    &card.upgrade,
                    &mut applied_upgrades,
                    &mut player_query,
                    &mut weapon_query,
                );

                // Remove just the upgrade UI and show next wave button
                for entity in &ui_query {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

/// Handles next wave button clicks
pub fn handle_next_wave_button(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<NextWaveButton>)>,
    button_query: Query<Entity, With<NextWaveButton>>,
    ui_query: Query<Entity, With<UpgradeUI>>,
    mut wave_manager: ResMut<WaveManager>,
    mut player_query: Query<&mut components::PlayerExperience, With<Player>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!("Next Wave button clicked!"); // Debug

            // Remove button and UI
            for entity in &button_query {
                commands.entity(entity).despawn();
            }
            for entity in &ui_query {
                commands.entity(entity).despawn();
            }

            // Increment wave and start
            wave_manager.wave += 1;
            wave_manager.wave_state = WaveState::Running;

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

            // Reset player's level counter for the new wave
            if let Ok(mut experience) = player_query.single_mut() {
                println!(
                    "Resetting level counter from {}",
                    experience.levels_gained_this_wave
                );
                experience.levels_gained_this_wave = 0;
            }
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn apply_upgrade(
    upgrade: &UpgradeType,
    applied_upgrades: &mut AppliedUpgrades,
    player_query: &mut Query<(&mut Health, &mut PlayerExperience), With<Player>>,
    weapon_query: &mut Query<&mut Weapon>,
) {
    let Ok((mut health, mut exp)) = player_query.single_mut() else {
        return;
    };
    match upgrade {
        UpgradeType::IncreaseDamage(amount) => {
            applied_upgrades.damage_multiplier += amount;
            for mut weapon in weapon_query.iter_mut() {
                weapon.damage *= 1.0 + amount;
            }
        }
        UpgradeType::IncreaseFireRate(amount) => {
            applied_upgrades.fire_rate_multiplier += amount;
            for mut weapon in weapon_query.iter_mut() {
                let current_duration = weapon.cooldown.duration().as_secs_f32();
                weapon
                    .cooldown
                    .set_duration(std::time::Duration::from_secs_f32(
                        current_duration / (1.0 + amount),
                    ));
            }
        }
        UpgradeType::IncreaseRange(amount) => {
            applied_upgrades.range_multiplier += amount;
            for mut weapon in weapon_query.iter_mut() {
                weapon.range *= 1.0 + amount;
            }
        }
        UpgradeType::IncreaseMaxHealth(amount) => {
            health.max += amount;
            health.value += amount; // Also heal
        }
        UpgradeType::IncreaseSpeed(amount) => {
            applied_upgrades.speed_multiplier += amount;
            // Speed will be applied in movement system
        }
        UpgradeType::HealPlayer(amount) => {
            health.value = (health.value + amount).min(health.max);
        }
        UpgradeType::AddPiercing => {
            applied_upgrades.has_piercing = true;
        }
        UpgradeType::AddMultishot(count) => {
            applied_upgrades.multishot_count += count;
        }
        UpgradeType::AddExplosive => {
            applied_upgrades.has_explosive = true;
        }
    }
    exp.levels_gained_this_wave -= 1;
}
