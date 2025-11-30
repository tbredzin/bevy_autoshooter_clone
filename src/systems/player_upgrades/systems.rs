use crate::components::{Health, Weapon, WeaponCooldown};
use crate::resources::{WaveManager, WaveState};
use crate::systems::player::components::Player;
use crate::systems::player::experience::PlayerExperience;
use crate::systems::player_upgrades::components::UpgradeCardSelectionState::{
    Applied, Holding, Selected, ToApply, Unselected,
};
use crate::systems::player_upgrades::components::*;
use bevy::prelude::TimerMode::Once;
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;

pub fn apply_upgrade(
    cards: Query<(&mut UpgradeCard), Changed<UpgradeCard>>,
    mut player_query: Query<(&mut PlayerStats, &mut PlayerExperience), With<Player>>,
    weapon_query: Query<(&mut Weapon, &mut WeaponCooldown)>,
) {
    let Ok((mut stats, mut exp)) = player_query.single_mut() else {
        return;
    };

    for (mut card) in cards {
        if card.state == ToApply {
            println!(
                "Applying upgrade from {:?} to {:?}, {:?}",
                card.upgrade, stats, exp
            );
            exp.new_levels -= 1;

            // Apply stat upgrade
            stats.apply_upgrade(&card.upgrade);

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
            card.state = Applied;
            break;
        }
    }
}

//TODO: mutualize
pub fn handle_gamepad_update_selection(
    gamepad: Single<&Gamepad>, //TODO: support multiple
    mut card_query: Query<(&mut UpgradeCard, &mut UpgradeCardAnimation, &CardIndex)>,
    time: Res<Time>,
) {
    let gamepad = gamepad.into_inner();
    let card = card_query
        .iter_mut()
        .find(|(card, _, index)| card_button_is_pressed(gamepad, index) || card.state == Selected);

    let mut holding_card_index = 255;
    if let Some((mut card, mut animation, index)) = card {
        match card.state {
            Unselected => {
                card.state = Holding;
            }
            Holding => {
                animation.timer.tick(time.delta());
                if animation.timer.just_finished() {
                    card.state = Selected;
                    animation.timer = Timer::from_seconds(1.0, Once);
                }
            }
            Selected => {
                animation.timer.tick(time.delta());
                if animation.timer.is_finished() {
                    card.state = ToApply;
                }
            }
            _ => {}
        }
        holding_card_index = index.0;
    }

    for (mut upgrade, mut animation, index) in card_query {
        if index.0 != holding_card_index && (upgrade.state != Selected || upgrade.state != ToApply)
        {
            upgrade.state = Unselected;
            animation.timer = Timer::from_seconds(1.0, Once);
        }
    }
}

pub fn handle_next_wave_button(
    mut commands: Commands,
    optional_gamepad: Option<Single<&Gamepad>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<NextWaveButton>)>,
    button_query: Query<Entity, With<NextWaveButton>>,
    ui_query: Query<Entity, With<UpgradeUI>>,
    mut wave_manager: ResMut<WaveManager>,
    mut player_query: Query<(&mut PlayerStats, &mut PlayerExperience, &mut Health), With<Player>>,
) {
    // Check for Next Wave button press (Keyboard)
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            for entity in button_query {
                commands.entity(entity).despawn();
            }
            for entity in ui_query {
                commands.entity(entity).despawn();
            }
            start_next_wave(&mut wave_manager, &mut player_query);
            return;
        }
    }

    // Gamepad
    if let Some(gamepad) = optional_gamepad.as_ref() {
        if gamepad.just_pressed(GamepadButton::Start) && !button_query.is_empty() {
            for entity in ui_query {
                commands.entity(entity).despawn();
            }
            start_next_wave(&mut wave_manager, &mut player_query);
        }
    }
}

/** UTILS **/

fn start_next_wave(
    wave_manager: &mut ResMut<WaveManager>,
    player_query: &mut Query<(&mut PlayerStats, &mut PlayerExperience, &mut Health), With<Player>>,
) {
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

fn card_button_is_pressed(gamepad: &Gamepad, index: &CardIndex) -> bool {
    match index.0 {
        0 => gamepad.pressed(GamepadButton::West),
        1 => gamepad.pressed(GamepadButton::South),
        2 => gamepad.pressed(GamepadButton::North),
        3 => gamepad.pressed(GamepadButton::East),
        _ => false,
    }
}
