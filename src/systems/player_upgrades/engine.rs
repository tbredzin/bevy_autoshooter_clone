use crate::components::{Health, Weapon, WeaponCooldown};
use crate::resources::{WaveManager, WaveState};
use crate::systems::player::components::Player;
use crate::systems::player::experience::PlayerExperience;
use crate::systems::player_upgrades::components::*;
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;

pub fn handle_upgrade_selection(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ChildOf, &UpgradeCardButton),
        Changed<Interaction>,
    >,
    card_query: Query<&UpgradeCard>,
    ui_query: Query<Entity, With<UpgradeUI>>,
    mut player_query: Query<(&mut PlayerStats, &mut PlayerExperience, &mut Health), With<Player>>,
    mut weapon_query: Query<(&mut Weapon, &mut WeaponCooldown)>,
) {
    for (interaction, child_of, _) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!("handle_upgrade_selection: {:?}", interaction);
            if let Ok(card) = card_query.get(child_of.parent()) {
                apply_upgrade(&card.upgrade, &mut player_query, &mut weapon_query);
                // Remove UI and show next wave button
                for entity in &ui_query {
                    commands.entity(entity).despawn();
                }
            } else {
                println!("handle_upgrade_selection: card not found");
            }
        }
    }
}

pub fn handle_next_wave_button(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<NextWaveButton>)>,
    button_query: Query<Entity, With<NextWaveButton>>,
    ui_query: Query<Entity, With<UpgradeUI>>,
    mut wave_manager: ResMut<WaveManager>,
    mut player_query: Query<(&mut PlayerExperience, &PlayerStats, &mut Health), With<Player>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!("Next Wave button clicked!");

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

            // Reset player's counters and health
            for (mut experience, stats, mut health) in player_query.iter_mut() {
                println!("Resetting level counter from {}", experience.new_levels);
                experience.new_levels = 0;
                health.value = stats.max_health;
            }
        }
    }
}

fn apply_upgrade(
    upgrade: &StatUpgrade,
    player_query: &mut Query<(&mut PlayerStats, &mut PlayerExperience, &mut Health), With<Player>>,
    weapon_query: &mut Query<(&mut Weapon, &mut WeaponCooldown)>,
) {
    let Ok((mut stats, mut exp, mut health)) = player_query.single_mut() else {
        return;
    };
    println!(
        "Applying upgrade from {:?} to {:?}, {:?}",
        upgrade, stats, exp
    );
    exp.new_levels -= 1;

    // Apply stat upgrade
    stats.apply_upgrade(upgrade);

    // Apply stat changes to existing weapons
    for (mut weapon, mut cooldown) in weapon_query {
        weapon.damage_multiplier = stats.damage_multiplier;
        weapon.fire_rate_multiplier = stats.fire_rate_multiplier;
        weapon.range_multiplier = stats.range_multiplier;
        cooldown.timer = Timer::from_seconds(
            weapon.base_cooldown / weapon.fire_rate_multiplier,
            Repeating,
        )
    }
}
