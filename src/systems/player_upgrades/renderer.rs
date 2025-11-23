// src/systems/player_upgrades/renderer
use crate::resources::HUDTextureAtlas;
use crate::systems::input::gamepad::{ActiveGamepad, GamepadAsset};
use crate::systems::player::components::Player;
use crate::systems::player::experience::PlayerExperience;
use crate::systems::player_upgrades::components::NextWaveButton;
use crate::systems::player_upgrades::components::*;
use crate::systems::player_upgrades::resources::UpgradePool;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

/// Spawns the upgrade UI when wave ends
pub fn show_upgrade_ui(
    mut commands: Commands,
    upgrade_pool: Res<UpgradePool>,
    ui_query: Query<Entity, With<UpgradeUI>>,
    player_query: Query<&PlayerExperience, With<Player>>,
    sprites: Res<HUDTextureAtlas>,
    gamepad_asset: Res<GamepadAsset>,
    active_gamepad: Option<Res<ActiveGamepad>>,
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
                        upgrade_pool.generate_upgrades(4),
                        parent,
                        &sprites,
                        &gamepad_asset,
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
                spawn_upgrade_card(parent, upgrade, sprites, gamepad_asset, index, has_gamepad);
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

fn spawn_upgrade_card(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    upgrade: StatUpgrade,
    sprites: &Res<HUDTextureAtlas>,
    gamepad_asset: &Res<GamepadAsset>,
    index: usize,
    has_gamepad: bool,
) {
    let (texture_index, description, icon_color) = upgrade.get_display_info();
    let border_color = upgrade.rarity.get_color();

    // Gamepad button labels
    let gamepad_button_index = 71 + (35 * index);

    parent
        .spawn((
            UpgradeCard {
                upgrade: upgrade.clone(),
            },
            Node {
                width: Val::Px(280.0),
                height: Val::Px(380.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            BorderColor::all(border_color),
        ))
        .with_children(|parent| {
            // Rarity badge
            parent.spawn((
                Text::new(format!("{:?}", upgrade.rarity)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(border_color),
            ));

            // Icon (symbol with colored background)
            parent
                .spawn((
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
                ))
                .with_children(|parent| {
                    parent.spawn((
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
                    ));
                });

            // Title
            parent.spawn((
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
            ));

            // Description
            parent.spawn((
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
            ));

            parent.spawn((
                UpgradeCardButton,
                Button,
                ImageNode::from_atlas_image(
                    gamepad_asset.asset.clone(),
                    TextureAtlas::from(gamepad_asset.layout.clone())
                        .with_index(gamepad_button_index),
                ),
                Node {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    ..default()
                },
            ));
        });
}
