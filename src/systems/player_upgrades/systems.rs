use crate::components::{Health, Weapon, WeaponCooldown};
use crate::resources::{WaveManager, WaveState};
use crate::systems::input::gamepad::ActiveGamepad;
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
    mut player_query: Query<(&mut PlayerStats, &mut PlayerExperience), With<Player>>,
    mut weapon_query: Query<(&mut Weapon, &mut WeaponCooldown)>,
) {
    for (interaction, child_of, _) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!("handle_upgrade_selection: {:?}", interaction);
            if let Ok(card) = card_query.get(child_of.parent()) {
                apply_upgrade(&card.upgrade, &mut player_query, &mut weapon_query);
                // Remove UI
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
            start_next_wave(
                &mut commands,
                &button_query,
                &ui_query,
                &mut wave_manager,
                &mut player_query,
            );
        }
    }
}

/// Handle gamepad input for upgrade selection
pub fn handle_gamepad_upgrade_selection(
    mut commands: Commands,
    active_gamepad: Option<Res<ActiveGamepad>>,
    gamepads: Query<&Gamepad>,
    card_query: Query<(Entity, &UpgradeCard)>,
    ui_query: Query<Entity, With<UpgradeUI>>,
    mut player_query: Query<(&mut PlayerStats, &mut PlayerExperience, &mut Health), With<Player>>,
    mut weapon_query: Query<(&mut Weapon, &mut WeaponCooldown)>,
    button_query: Query<Entity, With<NextWaveButton>>,
    mut wave_manager: ResMut<WaveManager>,
) {
    let Some(active_gamepad) = active_gamepad.as_ref() else {
        return;
    };
    let Ok(gamepad) = gamepads.get(active_gamepad.0) else {
        return;
    };

    // Check for Next Wave button press (Start/Menu button)
    if gamepad.just_pressed(GamepadButton::Start) && !button_query.is_empty() {
        start_next_wave_gamepad(
            &mut commands,
            &button_query,
            &ui_query,
            &mut wave_manager,
            &mut player_query,
        );
        return;
    }

    // Handle upgrade card selection with face buttons
    if !card_query.is_empty() {
        let cards: Vec<(Entity, &UpgradeCard)> = card_query.iter().collect();
        println!("cards: {:?}", cards);
        let selected_index = if gamepad.just_pressed(GamepadButton::West) {
            Some(0) // A/Cross - First card
        } else if gamepad.just_pressed(GamepadButton::South) {
            Some(1) // B/Circle - Second card
        } else if gamepad.just_pressed(GamepadButton::North) {
            Some(2) // Y/Triangle - Fourth card
        } else if gamepad.just_pressed(GamepadButton::East) {
            Some(3) // X/Square - Third card
        } else {
            None
        };

        if let Some(index) = selected_index {
            if let Some((_, card)) = cards.get(index) {
                apply_upgrade_gamepad(&card.upgrade, &mut player_query, &mut weapon_query);
                // Remove UI
                for entity in &ui_query {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn start_next_wave_gamepad(
    commands: &mut Commands,
    button_query: &Query<Entity, With<NextWaveButton>>,
    ui_query: &Query<Entity, With<UpgradeUI>>,
    wave_manager: &mut ResMut<WaveManager>,
    player_query: &mut Query<(&mut PlayerStats, &mut PlayerExperience, &mut Health), With<Player>>,
) {
    println!("Next Wave (gamepad) clicked!");

    // Remove button and UI
    for entity in button_query {
        commands.entity(entity).despawn();
    }
    for entity in ui_query {
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
    for (stats, mut experience, mut health) in player_query.iter_mut() {
        println!("Resetting level counter from {}", experience.new_levels);
        experience.new_levels = 0;
        health.value = stats.max_health;
    }
}

fn apply_upgrade_gamepad(
    upgrade: &StatUpgrade,
    player_query: &mut Query<(&mut PlayerStats, &mut PlayerExperience, &mut Health), With<Player>>,
    weapon_query: &mut Query<(&mut Weapon, &mut WeaponCooldown)>,
) {
    let Ok((mut stats, mut exp, _)) = player_query.single_mut() else {
        return;
    };
    println!(
        "Applying upgrade (gamepad) from {:?} to {:?}, {:?}",
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

fn start_next_wave(
    commands: &mut Commands,
    button_query: &Query<Entity, With<NextWaveButton>>,
    ui_query: &Query<Entity, With<UpgradeUI>>,
    wave_manager: &mut ResMut<WaveManager>,
    player_query: &mut Query<(&mut PlayerExperience, &PlayerStats, &mut Health), With<Player>>,
) {
    println!("Next Wave button clicked!");

    // Remove button and UI
    for entity in button_query {
        commands.entity(entity).despawn();
    }
    for entity in ui_query {
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

fn apply_upgrade(
    upgrade: &StatUpgrade,
    player_query: &mut Query<(&mut PlayerStats, &mut PlayerExperience), With<Player>>,
    weapon_query: &mut Query<(&mut Weapon, &mut WeaponCooldown)>,
) {
    let Ok((mut stats, mut exp)) = player_query.single_mut() else {
        return;
    };
    println!("Applying upgrade from {:?} to {:?}", upgrade.kind, stats);
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
