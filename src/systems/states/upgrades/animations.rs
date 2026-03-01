use crate::systems::states::upgrades::components::UpgradeCardState::{
    Applied, Holding, Selected, ToApply, Unselected,
};
use crate::systems::states::upgrades::components::{
    CardHoldBar, CardProgressFill, UpgradeCard, SELECTION_FREQUENCY,
};
use bevy::color::{Alpha, Color};
use bevy::math::{Rot2, Vec2};
use bevy::prelude::{
    BackgroundColor, BorderColor, Children, Component, Node, Query, RelationshipTarget, Timer,
    UiTransform, Val, With, ZIndex,
};
use std::f32::consts::TAU;

#[derive(Component, Debug)]
pub struct UpgradeCardAnimation {
    pub timer: Timer,
}

impl Default for UpgradeCardAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
        }
    }
}

pub fn animate_upgrade_cards(
    mut card_query: Query<(
        &Children,
        &UpgradeCard,
        &UpgradeCardAnimation,
        &mut UiTransform,
        &mut ZIndex,
        &mut BorderColor,
    )>,
    mut fill_query: Query<(&mut Node, &mut BackgroundColor), With<CardProgressFill>>,
) {
    for (children, card, animation, mut transform, mut z_index, mut border) in &mut card_query {
        let rarity_color = card.rarity.get_color();

        match card.state {
            Unselected => {
                transform.scale = Vec2::ONE;
                transform.rotation = Rot2::IDENTITY;
                z_index.0 = 1;
                *border = BorderColor::all(rarity_color.with_alpha(0.5));

                for child in children.iter() {
                    if let Ok((mut node, mut bg)) = fill_query.get_mut(child) {
                        node.height = Val::Percent(0.0);
                        *bg = BackgroundColor(Color::NONE);
                    }
                }
            }

            Holding => {
                let p = animation.timer.fraction();

                transform.scale = Vec2::splat(1.0 + p * 0.04);
                z_index.0 = 2;
                *border = BorderColor::all(rarity_color.with_alpha(0.5 + p * 0.5));

                for child in children.iter() {
                    if let Ok((mut node, mut bg)) = fill_query.get_mut(child) {
                        node.height = Val::Percent(p * 100.0);
                        *bg = BackgroundColor(rarity_color.with_alpha(p * 0.12));
                    }
                }
            }

            Selected => {
                let p = animation.timer.fraction();
                let angle = (p * SELECTION_FREQUENCY * TAU).sin() * (1.0 - p).powi(2) * 0.08;
                let scale = 1.04 + (1.0 - p).powi(2) * 0.12;

                transform.rotation = Rot2::from(angle);
                transform.scale = Vec2::splat(scale);
                z_index.0 = 10;

                for child in children.iter() {
                    if let Ok((mut node, mut bg)) = fill_query.get_mut(child) {
                        node.height = Val::Percent(100.0);
                        let flash = (1.0 - p * 2.0).max(0.0);
                        *bg = BackgroundColor(rarity_color.with_alpha(0.15 + flash * 0.25));
                    }
                }
                *border = BorderColor::all(rarity_color);
            }

            ToApply | Applied => {
                transform.rotation = Rot2::IDENTITY;
                transform.scale = Vec2::ONE;
                z_index.0 = 1;
            }
        }
    }
}

pub fn animate_holding_bars(
    card_query: Query<(&Children, &UpgradeCard, &UpgradeCardAnimation)>,
    child_query: Query<&Children>,
    mut bar_query: Query<&mut Node, With<CardHoldBar>>,
) {
    for (card_children, card, animation) in &card_query {
        let progress = match card.state {
            Holding => animation.timer.fraction(),
            Selected | ToApply | Applied => 1.0,
            Unselected => 0.0,
        };

        for &child in card_children {
            if let Ok(grandchildren) = child_query.get(child) {
                for &gc in grandchildren {
                    if let Ok(mut node) = bar_query.get_mut(gc) {
                        node.width = Val::Percent(progress * 100.0);
                    }
                }
            }
        }
    }
}
