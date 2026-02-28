use crate::components::{Weapon, WeaponCooldown};
use crate::systems::input::resources::ActionState;
use crate::systems::player::components::Player;
use crate::systems::player::experience::PlayerExperience;
use crate::systems::player_upgrades::components::UpgradeCardSelectionState::{
    Applied, Holding, Selected, ToApply, Unselected,
};
use crate::systems::player_upgrades::components::*;
use bevy::prelude::TimerMode::Once;
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;

pub fn handle_update_selection(
    actions: Res<ActionState>,
    mut card_query: Query<(&mut UpgradeCard, &mut UpgradeCardAnimation, &CardIndex)>,
    time: Res<Time>,
) {
    let card = card_query
        .iter_mut()
        .find(|(card, _, index)| actions.card_select[index.0] || card.state == Selected);

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
                    animation.timer = Timer::from_seconds(HOLD_DURATION, Once);
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
            animation.timer = Timer::from_seconds(HOLD_DURATION, Once);
        }
    }
}

pub fn apply_upgrade(
    cards: Query<&mut UpgradeCard, Changed<UpgradeCard>>,
    mut player_query: Query<(&mut PlayerStats, &mut PlayerExperience), With<Player>>,
    weapon_query: Query<(&mut Weapon, &mut WeaponCooldown)>,
) {
    let Ok((mut stats, mut exp)) = player_query.single_mut() else {
        return;
    };

    for mut card in cards {
        if card.state == ToApply {
            println!(
                "Applying upgrade from {:?} to {:?}, {:?}",
                card.upgrade, stats, exp
            );
            exp.new_levels -= 1;

            stats.apply_upgrade(&card.upgrade);

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
