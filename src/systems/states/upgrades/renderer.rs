// src/systems/upgrades/renderer
use crate::systems::constants::NB_UPDATES_PER_LEVEL;
use crate::systems::game::{GameState, MarkedForDespawn};
use crate::systems::hud::resources::HUDTextureAtlas;
use crate::systems::input::resources::{ActiveInputDevice, GamepadAsset};
use crate::systems::states::upgrades::components::UpgradeCardState::*;
use crate::systems::states::upgrades::components::*;
use crate::systems::states::upgrades::resources::{RedrawCardsPool, UpgradeCardsPool};
use crate::systems::states::waves::player;
use crate::systems::states::waves::player::components::Player;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use std::f32::consts::TAU;

const BG_CARD: Color = Color::srgb(0.07, 0.07, 0.12);
const BG_OVERLAY: Color = Color::srgba(0.0, 0.0, 0.0, 0.88);
const CARD_W: f32 = 230.0;
const CARD_H: f32 = 340.0;
const ICON_SIZE: f32 = 16.0;

pub fn spawn_upgrades_selection_ui(
    mut commands: Commands,
    upgrade_pool: Res<UpgradeCardsPool>,
    player_query: Query<&player::experience::PlayerExperience, With<Player>>,
    sprites: Res<HUDTextureAtlas>,
    gamepad_asset: Res<GamepadAsset>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(player_xp) = player_query.single() else {
        return;
    };
    if player_xp.new_levels == 0 {
        next_state.set(GameState::Shopping);
        return;
    }

    commands
        .spawn((
            UpgradeSelectionUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(32.0),
                ..default()
            },
            BackgroundColor(BG_OVERLAY),
            ZIndex(100),
        ))
        .with_children(|parent| {
            spawn_header(parent);
            let upgrades = upgrade_pool.generate_upgrades(NB_UPDATES_PER_LEVEL);
            spawn_card_row(parent, upgrades, &sprites, &gamepad_asset);
        });
}

pub fn despawn_upgrades_selection_ui(
    mut commands: Commands,
    ui: Single<Entity, With<UpgradeSelectionUI>>,
) {
    commands.entity(ui.entity()).insert(MarkedForDespawn);
}

/// Toggles the Display of keyboard labels vs gamepad icons in every CardKeyBadge.
pub fn update_card_key_badges(
    active_device: Res<ActiveInputDevice>,
    mut kb_labels: Query<&mut Node, (With<KeyboardLabel>, Without<GamepadLabel>)>,
    mut gp_labels: Query<&mut Node, (With<GamepadLabel>, Without<KeyboardLabel>)>,
) {
    if !active_device.is_changed() {
        return;
    }

    let is_gamepad = *active_device == ActiveInputDevice::Gamepad;

    for mut node in kb_labels.iter_mut() {
        node.display = if is_gamepad {
            Display::None
        } else {
            Display::Flex
        };
    }
    for mut node in gp_labels.iter_mut() {
        node.display = if is_gamepad {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn draw_new_cards(
    mut commands: Commands,
    ui: Single<Entity, (With<UpgradeSelectionUI>)>,
    upgrade_pool: Res<UpgradeCardsPool>,
    sprites: Res<HUDTextureAtlas>,
    gamepad_asset: Res<GamepadAsset>,
    mut redraw: ResMut<RedrawCardsPool>,
) {
    if !redraw.0 {
        return;
    }

    let upgrades = upgrade_pool.generate_upgrades(NB_UPDATES_PER_LEVEL);
    commands.entity(ui.entity()).despawn_children();
    commands.entity(ui.entity()).with_children(|parent| {
        spawn_header(parent);
        spawn_card_row(parent, upgrades, &sprites, &gamepad_asset);
    });

    redraw.0 = false;
}

// ─────────────────────────────────────────────────────────────────────────────
// Layout helpers
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_header(parent: &mut RelatedSpawnerCommands<ChildOf>) {
    parent.spawn((
        Text::new("LEVEL UP"),
        TextFont {
            font_size: 52.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
    parent.spawn((
        Text::new("Choose an upgrade"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.55, 0.55, 0.65)),
    ));
}

fn spawn_card_row(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    upgrades: Vec<UpgradeCard>,
    sprites: &Res<HUDTextureAtlas>,
    gamepad_asset: &Res<GamepadAsset>,
) {
    parent
        .spawn((
            UpgradeCardsRow {},
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(18.0),
                ..default()
            },
        ))
        .with_children(|row| {
            for (index, upgrade) in upgrades.into_iter().enumerate() {
                spawn_upgrade_card(row, upgrade, sprites, gamepad_asset, index);
            }
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// Card spawning
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_upgrade_card(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    upgrade: UpgradeCard,
    sprites: &Res<HUDTextureAtlas>,
    gamepad_asset: &Res<GamepadAsset>,
    card_index: usize,
) {
    let (texture_index, description, icon_bg_color) = upgrade.get_display_info();
    let rarity_color = upgrade.rarity.get_color();

    // Key labels for keyboard: 1/2/3/4
    let key_label = ["1", "2", "3", "4"].get(card_index).copied().unwrap_or("?");
    // Gamepad atlas index for this slot
    let gp_index = GAMEPAD_BUTTON_INDICES
        .get(card_index)
        .copied()
        .unwrap_or(GAMEPAD_BUTTON_INDICES[0]);

    parent
        .spawn((
            upgrade,
            UpgradeCardAnimation::default(),
            CardIndex(card_index),
            ZIndex(1),
            Node {
                width: Val::Px(CARD_W),
                height: Val::Px(CARD_H),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect {
                    top: Val::Px(14.0),
                    left: Val::Px(16.0),
                    right: Val::Px(16.0),
                    bottom: Val::Px(14.0),
                },
                border: UiRect::all(Val::Px(2.0)),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(BG_CARD),
            BorderColor::all(rarity_color.with_alpha(0.6)),
        ))
        .with_children(|card| {
            // ── Background fill (rises from bottom during hold) ──────────────
            card.spawn((
                CardProgressFill,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(0.0),
                    bottom: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                ZIndex(-1),
            ));

            // ── Top row: rarity chip  +  key/button badge ────────────────────
            card.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(12.0)),
                ..default()
            })
            .with_children(|row| {
                // Rarity badge (left)
                row.spawn((
                    Node {
                        padding: UiRect::axes(Val::Px(8.0), Val::Px(3.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(rarity_color.with_alpha(0.15)),
                    BorderColor::all(rarity_color.with_alpha(0.5)),
                ))
                .with_children(|badge| {
                    badge.spawn((
                        Text::new(upgrade.rarity.to_string()),
                        TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(rarity_color),
                    ));
                });

                // Input badge (right) — contains BOTH keyboard text AND gamepad icon,
                // one of which is hidden via Display::None depending on active device.
                row.spawn((
                    CardKeyBadge(card_index),
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.14, 0.14, 0.20)),
                    BorderColor::all(Color::srgb(0.35, 0.35, 0.5)),
                ))
                .with_children(|badge| {
                    // Keyboard label — shown when keyboard is active
                    badge.spawn((
                        KeyboardLabel,
                        Text::new(key_label),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.75, 0.75, 0.9)),
                        Node {
                            display: Display::None,
                            ..default()
                        },
                    ));

                    // Gamepad icon — shown when gamepad is active
                    badge.spawn((
                        GamepadLabel,
                        ImageNode::from_atlas_image(
                            gamepad_asset.texture.clone(),
                            TextureAtlas::from(gamepad_asset.layout.clone()).with_index(gp_index),
                        ),
                        Node {
                            width: Val::Px(22.0),
                            height: Val::Px(22.0),
                            display: Display::Flex,
                            ..default()
                        },
                    ));
                });
            });

            // ── Icon circle ─────────────────────────────────────────────────
            card.spawn((
                Node {
                    width: Val::Px(ICON_SIZE),
                    height: Val::Px(ICON_SIZE),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(ICON_SIZE / 2.0)),
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },
                BackgroundColor(icon_bg_color.with_alpha(0.2)),
                BorderColor::all(icon_bg_color.with_alpha(0.8)),
            ))
            .with_children(|circle| {
                circle.spawn((
                    ImageNode::from_atlas_image(
                        sprites.texture.clone(),
                        TextureAtlas {
                            layout: sprites.layout.clone(),
                            index: texture_index,
                        },
                    )
                    .with_color(icon_bg_color),
                    Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                ));
            });

            // ── Stat title ──────────────────────────────────────────────────
            card.spawn((
                Text::new(upgrade.get_full_title()),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
            ));

            // ── Description ─────────────────────────────────────────────────
            card.spawn((
                Text::new(description),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(Color::srgb(0.65, 0.65, 0.75)),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },
            ));

            // ── Spacer ───────────────────────────────────────────────────────
            card.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });

            // ── Hold progress bar track ──────────────────────────────────────
            card.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(4.0),
                    border_radius: BorderRadius::all(Val::Px(2.0)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.22)),
            ))
            .with_children(|track| {
                track.spawn((
                    CardHoldBar,
                    Node {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(rarity_color),
                ));
            });

            // ── Hold hint text ───────────────────────────────────────────────
            card.spawn((
                Text::new("Hold to select"),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
                TextColor(Color::srgb(0.35, 0.35, 0.45)),
                Node {
                    margin: UiRect::top(Val::Px(6.0)),
                    ..default()
                },
            ));
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// Animation system
//
// NOTE: timer ticking is done in handle_update_selection, NOT here.
//       This system is purely visual — it reads timer.fraction() as progress.
// ─────────────────────────────────────────────────────────────────────────────

pub fn animate_card_selection(
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

/// Updates the thin hold-bar width for each card.
/// The bar is a grandchild, so it needs its own traversal pass.
pub fn animate_hold_bars(
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
