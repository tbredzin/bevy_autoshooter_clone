// src/systems/player_upgrades/renderer
use crate::resources::{HUDTextureAtlas, NB_UPDATES_PER_LEVEL};
use crate::systems::player;
use crate::systems::player::components::Player;
use crate::systems::player_upgrades::components::NextWaveButton;
use crate::systems::player_upgrades::components::UpgradeCardSelectionState::{
    Applied, Holding, Selected, ToApply, Unselected,
};
use crate::systems::player_upgrades::components::*;
use crate::systems::player_upgrades::resources::UpgradePool;
use bevy::color::palettes::css::GOLD;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use std::f32::consts::TAU;

/// Spawns the upgrade UI when wave ends
pub fn show_upgrade_ui(
    mut commands: Commands,
    upgrade_pool: Res<UpgradePool>,
    ui_query: Query<Entity, With<UpgradeUI>>,
    player_query: Query<&player::experience::PlayerExperience, With<Player>>,
    sprites: Res<HUDTextureAtlas>,
    active_gamepad: Option<Single<&Gamepad>>,
) {
    // Only spawn UI once when wave ends
    if ui_query.is_empty() {
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
                if player_xp.new_levels == 0 {
                    show_no_upgrade(parent, has_gamepad)
                } else {
                    show_upgrades(
                        upgrade_pool.generate_upgrades(NB_UPDATES_PER_LEVEL),
                        parent,
                        &sprites,
                        has_gamepad,
                    )
                };
            });
    }
}

// Utility functions

fn show_upgrades(
    upgrades: Vec<StatUpgrade>,
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    sprites: &Res<HUDTextureAtlas>,
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
                spawn_upgrade_card(parent, upgrade, sprites, index, has_gamepad);
            }
        });
}

fn show_no_upgrade(parent: &mut RelatedSpawnerCommands<ChildOf>, has_gamepad: bool) {
    // No upgrades available, show message and button together
    parent.spawn((
        Text::new("No upgrades available this wave"),
        TextFont {
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        },
    ));

    // Spawn button directly inside the UI
    parent
        .spawn((
            NextWaveButton,
            Button,
            Node {
                width: Val::Px(300.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.7, 0.3)),
            BorderColor::all(Color::srgb(0.3, 0.9, 0.4)),
        ))
        .with_children(|parent| {
            let button_text = if has_gamepad {
                "PRESS START"
            } else {
                "START NEXT WAVE"
            };
            parent.spawn((
                Text::new(button_text),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}
pub fn hide_upgrade_ui(
    mut commands: Commands,
    cards: Query<&UpgradeCard>,
    ui_query: Query<Entity, With<UpgradeUI>>,
) {
    for selection in cards {
        if selection.state == Applied {
            for ui in ui_query {
                commands.entity(ui).despawn();
            }
        }
    }
}

fn spawn_upgrade_card(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    upgrade: StatUpgrade,
    sprites: &Res<HUDTextureAtlas>,
    index: usize,
    has_gamepad: bool,
) {
    let (texture_index, description, icon_color) = upgrade.get_display_info();
    let border_color = upgrade.rarity.get_color();
    let gamepad_label = match index {
        0 => "A",
        1 => "B",
        2 => "X",
        3 => "Y",
        _ => "",
    };

    parent.spawn((
        UpgradeCard::from(upgrade.clone()),
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
                            ..default()
                        },
                        BackgroundColor(icon_color),
                        BorderColor::all(Color::WHITE),
                        BorderRadius::all(Val::Px(40.0)),
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
                            if has_gamepad {
                                Text::new(format!("Press {}", gamepad_label))
                            } else {
                                Text::new("SELECT")
                            },
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
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
