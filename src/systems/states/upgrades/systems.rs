use crate::systems::game::GameState;
use crate::systems::input::resources::ActionState;
use crate::systems::states::upgrades::components::UpgradeCardState::{
    Applied, Holding, Selected, ToApply, Unselected,
};
use crate::systems::states::upgrades::components::*;
use crate::systems::states::waves::player::components::{Player, PlayerStats, StatKind};
use crate::systems::states::waves::player::experience::PlayerExperience;
use crate::systems::states::waves::weapons::components::{Weapon, WeaponCooldown};
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
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((mut stats, mut exp)) = player_query.single_mut() else {
        return;
    };

    for mut card in cards {
        if card.state == ToApply {
            exp.new_levels -= 1;
            match card.kind {
                StatKind::Damage => stats.damage_multiplier += card.value,
                StatKind::FireRate => stats.fire_rate_multiplier += card.value,
                StatKind::Range => stats.range_multiplier += card.value,
                StatKind::MaxHealth => stats.max_health += card.value,
                StatKind::Speed => stats.speed_multiplier += card.value,
            }

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

            if exp.new_levels == 0 {
                next_state.set(GameState::Shopping);
            }
            break;
        }
    }
}
