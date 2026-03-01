use crate::systems::game::GameState;
use crate::systems::input::resources::ActionState;
use crate::systems::states::upgrades::animations::UpgradeCardAnimation;
use crate::systems::states::upgrades::components::UpgradeCardState::*;
use crate::systems::states::upgrades::components::*;
use crate::systems::states::upgrades::resources::RedrawCardsPool;
use crate::systems::states::waves::player::components::{Player, PlayerStats, StatKind};
use crate::systems::states::waves::player::experience::PlayerExperience;
use crate::systems::states::waves::weapons::components::{Weapon, WeaponCooldown};
use bevy::prelude::TimerMode::Once;
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;

pub fn update_active_upgrade_card(
    actions: Res<ActionState>,
    mut card_query: Query<(&mut UpgradeCard, &mut UpgradeCardAnimation, &CardIndex)>,
    time: Res<Time>,
) {
    let mut active_card_index: Option<usize> = None;

    for (mut card, mut animation, index) in card_query.iter_mut() {
        let is_pressed = actions.card_select.get(index.0).copied().unwrap_or(false);

        match card.state {
            Unselected => {
                if is_pressed {
                    card.state = Holding;
                    animation.timer = Timer::from_seconds(HOLD_DURATION, Once);
                    active_card_index = Some(index.0);
                }
            }

            Holding => {
                if !is_pressed {
                    card.state = Unselected;
                } else {
                    animation.timer.tick(time.delta());
                    if animation.timer.just_finished() {
                        card.state = Selected;
                        animation.timer = Timer::from_seconds(SELECTION_ANIM_DURATION, Once);
                    }
                    active_card_index = Some(index.0);
                }
            }

            Selected => {
                animation.timer.tick(time.delta());
                if animation.timer.just_finished() {
                    card.state = ToApply;
                }
                active_card_index = Some(index.0);
            }

            ToApply | Applied => {
                active_card_index = Some(index.0);
            }
        }
    }

    // Reset focus on other cards
    if let Some(active) = active_card_index {
        for (mut card, _, index) in card_query.iter_mut() {
            if index.0 != active && !matches!(card.state, ToApply | Applied | Selected) {
                card.state = Unselected;
            }
        }
    }
}

pub fn apply_active_upgrade_card(
    mut cards: Query<&mut UpgradeCard>,
    mut player_query: Query<(&mut PlayerStats, &mut PlayerExperience), With<Player>>,
    mut weapon_query: Query<(&mut Weapon, &mut WeaponCooldown)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut redraw: ResMut<RedrawCardsPool>,
) {
    let Ok((mut stats, mut exp)) = player_query.single_mut() else {
        return;
    };
    for mut card in cards.iter_mut() {
        if card.state != ToApply {
            continue;
        }

        // Apply stat bonus
        match card.kind {
            StatKind::Damage => stats.damage_multiplier += card.value,
            StatKind::FireRate => stats.fire_rate_multiplier += card.value,
            StatKind::Range => stats.range_multiplier += card.value,
            StatKind::MaxHealth => stats.max_health += card.value,
            StatKind::Speed => stats.speed_multiplier += card.value,
        }

        // Propagate to all weapons immediately
        for (mut weapon, mut cooldown) in weapon_query.iter_mut() {
            weapon.damage_multiplier = stats.damage_multiplier;
            weapon.fire_rate_multiplier = stats.fire_rate_multiplier;
            weapon.range_multiplier = stats.range_multiplier;
            cooldown.timer = Timer::from_seconds(
                weapon.base_cooldown / weapon.fire_rate_multiplier,
                Repeating,
            );
        }

        card.state = Applied;
        exp.new_levels = exp.new_levels.saturating_sub(1);

        if exp.new_levels == 0 {
            next_state.set(GameState::Shopping);
        }

        redraw.0 = true;

        break; // one card consumed per frame
    }
}
