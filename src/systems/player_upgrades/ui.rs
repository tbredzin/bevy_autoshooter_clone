use crate::components;
use crate::components::{MarkedForDespawn, Player};
use crate::resources::{WaveManager, WaveState};
use crate::systems::player_upgrades::components::*;
use crate::systems::player_upgrades::resources::AvailableUpgradesResource;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

/// Spawns the upgrade UI when wave ends
pub fn show_upgrade_ui(
    mut commands: Commands,
    wave_manager: Res<WaveManager>,
    available_upgrades: Res<AvailableUpgradesResource>,
    ui_query: Query<Entity, With<UpgradeUI>>,
    player_query: Query<&components::PlayerExperience, With<Player>>,
) {
    // Only spawn UI once when wave ends
    if wave_manager.wave_state == WaveState::Ended && ui_query.is_empty() {
        let Ok(player_xp) = player_query.single() else {
            return;
        };
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
                if player_xp.levels_gained_this_wave == 0 {
                    show_no_upgrade(parent)
                } else {
                    show_upgrades(available_upgrades.generate(&mut rand::rng(), 4), parent)
                };
            });
    }
}

/// Removes upgrade UI when wave starts
pub fn hide_upgrade_ui(
    mut commands: Commands,
    wave_manager: Res<WaveManager>,
    ui_query: Query<Entity, With<UpgradeUI>>,
    button_query: Query<Entity, With<NextWaveButton>>,
) {
    if wave_manager.wave_state == WaveState::Running {
        for entity in &ui_query {
            commands.entity(entity).insert(MarkedForDespawn);
        }
        for entity in &button_query {
            commands.entity(entity).insert(MarkedForDespawn);
        }
    }
}

// Utility functions

fn show_upgrades(upgrades: Vec<UpgradeType>, parent: &mut RelatedSpawnerCommands<ChildOf>) {
    // Title
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
            for upgrade in upgrades {
                spawn_upgrade_card(parent, upgrade);
            }
        });
}

fn show_no_upgrade(parent: &mut RelatedSpawnerCommands<ChildOf>) {
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
            parent.spawn((
                Text::new("START NEXT WAVE"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_upgrade_card(parent: &mut RelatedSpawnerCommands<ChildOf>, upgrade: UpgradeType) {
    let (title, description, icon_color) = upgrade.get_display_info();
    let rarity = upgrade.get_rarity();
    let border_color = rarity.get_color();

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
                Text::new(format!("{:?}", rarity)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(border_color),
            ));

            // Icon (colored circle)
            parent.spawn((
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(icon_color),
                BorderColor::all(Color::WHITE),
                BorderRadius::all(Val::Px(40.0)),
            ));

            // Title
            parent.spawn((
                Text::new(title),
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

            // Select button
            parent
                .spawn((
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
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("SELECT"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}
