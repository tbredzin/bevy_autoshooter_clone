// src/systems/upgrades/renderer
use crate::systems::constants::NB_UPDATES_PER_LEVEL;
use crate::systems::game::MarkedForDespawn;
use crate::systems::hud::resources::HUDTextureAtlas;
use crate::systems::input::resources::GamepadAsset;
use crate::systems::states::upgrades::components::UpgradeCardState::{
    Applied, Holding, Selected, ToApply, Unselected,
};
use crate::systems::states::upgrades::components::*;
use crate::systems::states::upgrades::resources::UpgradeCardsPool;
use crate::systems::states::waves::player;
use crate::systems::states::waves::player::components::Player;
use bevy::color::palettes::css::GOLD;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use std::f32::consts::TAU;

pub fn spawn_upgrades_selection_ui(
    mut commands: Commands,
    upgrade_pool: Res<UpgradeCardsPool>,
    player_query: Query<&player::experience::PlayerExperience, With<Player>>,
    sprites: Res<HUDTextureAtlas>,
    gamepad_asset: Res<GamepadAsset>,
    active_gamepad: Option<Single<&Gamepad>>,
) {
    let Ok(player_xp) = player_query.single() else {
        return;
    };
    let has_gamepad = active_gamepad.is_some();
    commands
        .spawn((
            UpgradeUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|parent| {
            println!("{:?}", player_xp);
            if player_xp.new_levels > 0 {
                display_upgrade_menu(
                    upgrade_pool.generate_upgrades(NB_UPDATES_PER_LEVEL),
                    parent,
                    &sprites,
                    &gamepad_asset,
                    has_gamepad,
                )
            };
        });
}
pub fn despawn_upgrades_selection_ui(mut commands: Commands, ui: Query<Entity, With<UpgradeUI>>) {
    for e in ui {
        commands.entity(e).insert(MarkedForDespawn);
    }
}

// Utility functions
fn display_upgrade_menu(
    upgrades: Vec<UpgradeCard>,
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    sprites: &Res<HUDTextureAtlas>,
    gamepad_asset: &Res<GamepadAsset>,
    has_gamepad: bool,
) {
    // Title
    println!("Upgrades: {:?}", upgrades);
    parent.spawn((
        Text::new("Choose an Upgrade"),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        },
    ));

    // Gamepad hint
    if has_gamepad {
        parent.spawn((
            Text::new("🎮 A/B/X/Y to select • START to skip"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
    }

    // Cards container
    parent
        .spawn(Node {
            width: Val::Percent(90.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|parent| {
            for (index, upgrade) in upgrades.into_iter().enumerate() {
                spawn_upgrade_card(parent, upgrade, sprites, gamepad_asset, index);
            }
        });
}

fn spawn_upgrade_card(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    upgrade: UpgradeCard,
    sprites: &Res<HUDTextureAtlas>,
    gamepad_asset: &Res<GamepadAsset>,
    index: usize,
) {
    let (texture_index, description, icon_color) = upgrade.get_display_info();
    let border_color = upgrade.rarity.get_color();
    let gamepad_button_index = 71 + (35 * index);

    parent.spawn((
        upgrade,
        UpgradeCardAnimation::default(),
        CardIndex(index),
        ZIndex(1),
        Node {
            width: Val::Px(280.0),
            height: Val::Px(380.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(4.0)),
            overflow: Overflow::clip(), // Clip the progress fill
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        BorderColor::all(border_color),
        children![
            (
                CardProgressFill,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(0.0), // Starts at 0, fills up
                    bottom: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
                ZIndex(1), // Behind content
            ),
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ZIndex(2), // Above progress fill
                children![
                    (
                        Text::new(format!("{:?}", upgrade.rarity)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(border_color),
                    ),
                    (
                        Node {
                            width: Val::Px(80.0),
                            height: Val::Px(80.0),
                            margin: UiRect::vertical(Val::Px(20.0)),
                            border: UiRect::all(Val::Px(3.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(40.0)),
                            ..default()
                        },
                        BackgroundColor(icon_color),
                        BorderColor::all(Color::WHITE),
                        children![(
                            ImageNode::from_atlas_image(
                                sprites.texture.clone(),
                                TextureAtlas {
                                    layout: sprites.layout.clone(),
                                    index: texture_index,
                                },
                            )
                            .with_color(Color::BLACK),
                            Node {
                                width: px(32),
                                height: px(32),
                                ..default()
                            },
                        )]
                    ),
                    (
                        Text::new(upgrade.get_full_title()),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                    ),
                    (
                        Text::new(description),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        TextLayout::new_with_justify(Justify::Center),
                        Node {
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                    ),
                    (
                        UpgradeCardButton,
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.6, 0.3)),
                        BorderColor::all(Color::srgb(0.4, 0.8, 0.4)),
                        children![(
                            ImageNode::from_atlas_image(
                                gamepad_asset.texture.clone(),
                                TextureAtlas::from(gamepad_asset.layout.clone())
                                    .with_index(gamepad_button_index),
                            ),
                            Node {
                                width: Val::Px(32.0),
                                ..default()
                            }
                        )]
                    )
                ]
            )
        ],
    ));
}

pub fn animate_card_selection(
    mut card_query: Query<(
        &Children,
        &mut UpgradeCard,
        &mut UpgradeCardAnimation,
        &mut UiTransform,
        &mut ZIndex,
    )>,
    mut fill_query: Query<(&mut Node, &mut BackgroundColor), With<CardProgressFill>>,
    time: Res<Time>,
) {
    for (children, mut card, mut animation, mut transform, mut z) in &mut card_query {
        match card.state {
            Unselected => {
                for child in children.iter() {
                    if let Ok((mut node, _)) = fill_query.get_mut(child) {
                        if node.height != Val::Percent(0.0) {
                            node.height = Val::Percent(0.0);
                        }
                    }
                }
            }
            Holding => {
                for child in children.iter() {
                    if let Ok((mut node, mut bg_color)) = fill_query.get_mut(child) {
                        animation.timer.tick(time.delta());
                        let progress = animation.timer.fraction();

                        // Update fill height
                        node.height = Val::Percent(progress * 100.0);

                        // Color transitions from transparent to rarity color
                        let alpha = (progress * 0.6).min(0.6); // Max 60% opacity
                        *bg_color = BackgroundColor(Color::from(GOLD).with_alpha(alpha));
                    }
                }
            }
            Selected => {
                animation.timer.tick(time.delta());
                let progress = animation.timer.fraction();
                let angle = (progress * SELECTION_FREQUENCY * TAU).sin() * (1.0 - progress) * 0.15; // Dampens over time
                let scale = 1.2 + (1.0 - progress).powi(2) * 0.2;
                transform.rotation = Rot2::from(angle);
                transform.scale = Vec2::splat(scale);
                if animation.timer.just_finished() {
                    card.state = ToApply;
                }
                z.0 = 10;
            }
            ToApply | Applied => {
                // Keep final transform state or reset
                transform.rotation = Rot2::IDENTITY;
                transform.scale = Vec2::ONE;
                z.0 = 1;
            }
        }
    }
}
