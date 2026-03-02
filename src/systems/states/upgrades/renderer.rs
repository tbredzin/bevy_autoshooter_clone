// src/systems/upgrades/renderer
use crate::systems::constants::NB_UPDATES_PER_LEVEL;
use crate::systems::game::{GameState, MarkedForDespawn};
use crate::systems::hud::resources::HUDTextureAtlas;
use crate::systems::input::resources::{ActiveInputDevice, GamepadAsset};
use crate::systems::states::upgrades::animations::UpgradeCardAnimation;
use crate::systems::states::upgrades::components::*;
use crate::systems::states::upgrades::resources::{RedrawCardsPool, UpgradeCardsPool};
use crate::systems::states::waves::player;
use crate::systems::states::waves::player::components::Player;
use bevy::color::palettes::css::GOLD;
use bevy::prelude::*;

const GREY: Color = Color::srgb(0.55, 0.55, 0.65);
const BG_CARD: Color = Color::srgb(0.07, 0.07, 0.12);
const BG_CARD_HOVER: Color = Color::srgb(0.07, 0.18, 0.12);
const BG_OVERLAY: Color = Color::srgba(0.0, 0.0, 0.0, 0.88);
const CARD_W: f32 = 230.0;
const CARD_H: f32 = 340.0;
const ICON_SIZE: f32 = 72.0;

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
}

impl TextBundle {
    pub fn new(text: impl Into<String>, font_size: f32, color: Color) -> Self {
        Self {
            text: Text::new(text),
            font: TextFont {
                font_size,
                ..default()
            },
            color: TextColor::from(color),
        }
    }
}
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
    let upgrades = upgrade_pool.generate_upgrades(NB_UPDATES_PER_LEVEL);

    commands.spawn((
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
        children![
            TextBundle::new("LEVEL UP", 52.0, Color::WHITE),
            TextBundle::new("Choose an upgrade", 20.0, GREY),
            deck_bundle(upgrades, &sprites, &gamepad_asset),
        ],
    ));
}

pub fn despawn_upgrades_selection_ui(
    mut commands: Commands,
    ui: Single<Entity, With<UpgradeSelectionUI>>,
) {
    commands.entity(ui.entity()).insert(MarkedForDespawn);
}

pub fn update_card_buttons(
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

pub fn redraw_upgrades_selection(
    mut commands: Commands,
    ui: Single<Entity, With<UpgradeSelectionUI>>,
    upgrade_pool: Res<UpgradeCardsPool>,
    sprites: Res<HUDTextureAtlas>,
    gamepad_asset: Res<GamepadAsset>,
    mut redraw_cards_pool: ResMut<RedrawCardsPool>,
) {
    if !redraw_cards_pool.0 {
        return;
    }

    let upgrades = upgrade_pool.generate_upgrades(NB_UPDATES_PER_LEVEL);
    commands.entity(ui.entity()).despawn_children();
    commands.entity(ui.entity()).with_children(|parent| {
        parent.spawn(TextBundle::new("LEVEL UP", 52.0, Color::WHITE));
        parent.spawn(TextBundle::new("Choose an upgrade", 20.0, GREY));
        parent.spawn(deck_bundle(upgrades, &sprites, &gamepad_asset));
    });

    redraw_cards_pool.0 = false;
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────
fn deck_bundle(
    upgrades: Vec<UpgradeCard>,
    sprites: &Res<HUDTextureAtlas>,
    gamepad_asset: &Res<GamepadAsset>,
) -> impl Bundle {
    (
        CardDeckBundle {},
        (
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(18.0),
                ..default()
            },
            children![
                card_bundle(0, *upgrades.get(0).unwrap(), sprites, gamepad_asset),
                card_bundle(1, *upgrades.get(1).unwrap(), sprites, gamepad_asset),
                card_bundle(2, *upgrades.get(2).unwrap(), sprites, gamepad_asset),
                card_bundle(3, *upgrades.get(3).unwrap(), sprites, gamepad_asset),
            ],
        ),
    )
}

fn card_bundle(
    idx: usize,
    card: UpgradeCard,
    atlas: &Res<HUDTextureAtlas>,
    asset: &Res<GamepadAsset>,
) -> impl Bundle {
    let (texture_index, description, icon_bg_color) = card.get_display_info();
    let rarity_color = card.rarity.get_color();

    // Key labels for keyboard: 1/2/3/4
    let key_label = ["1", "2", "3", "4"].get(idx).copied().unwrap_or("?");
    // Gamepad atlas index for this slot
    let gp_index = GAMEPAD_BUTTON_INDICES
        .get(idx)
        .copied()
        .unwrap_or(GAMEPAD_BUTTON_INDICES[0]);

    (
        card,
        UpgradeCardAnimation::default(),
        CardIndex(idx),
        Button,
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
        children![
            // ── Background fill (rises from bottom during hold) ──────────────
            (
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
            ),
            // ── Top row: rarity chip  +  key/button badge ────────────────────
            (
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(12.0)),
                    ..default()
                },
                children![
                    (
                        Node {
                            padding: UiRect::axes(Val::Px(8.0), Val::Px(3.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        },
                        BackgroundColor(rarity_color.with_alpha(0.15)),
                        BorderColor::all(rarity_color.with_alpha(0.5)),
                        children![TextBundle::new(card.rarity.to_string(), 10.0, rarity_color)],
                    ),
                    (
                        CardButton,
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
                        children![
                            (
                                KeyboardLabel, // Keyboard label — shown when keyboard is active
                                TextBundle::new(key_label, 14.0, Color::srgb(0.75, 0.75, 0.9)),
                                Node {
                                    display: Display::None,
                                    ..default()
                                },
                            ),
                            (
                                GamepadLabel, // Gamepad icon — shown when gamepad is active
                                ImageNode::from_atlas_image(
                                    asset.texture.clone(),
                                    TextureAtlas::from(asset.layout.clone()).with_index(gp_index),
                                ),
                                Node {
                                    width: Val::Px(22.0),
                                    height: Val::Px(22.0),
                                    display: Display::Flex,
                                    ..default()
                                },
                            )
                        ],
                    )
                ],
            ),
            // ── Icon circle ─────────────────────────────────────────────────
            (
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
                children![(
                    ImageNode::from_atlas_image(
                        atlas.texture.clone(),
                        TextureAtlas {
                            layout: atlas.layout.clone(),
                            index: texture_index,
                        },
                    )
                    .with_color(icon_bg_color),
                    Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                )],
            ),
            // ── Stat title ──────────────────────────────────────────────────
            (
                Text::new(card.get_full_title()),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
            ),
            // ── Description ─────────────────────────────────────────────────
            (
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
            ),
            // ── Spacer ───────────────────────────────────────────────────────
            Node {
                flex_grow: 1.0,
                ..default()
            },
            // ── Hold progress bar track ──────────────────────────────────────
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(4.0),
                    border_radius: BorderRadius::all(Val::Px(2.0)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.22)),
                children![(
                    CardHoldBar,
                    Node {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::from(GOLD)),
                )],
            ),
            // ── Hold hint text ───────────────────────────────────────────────
            (
                TextBundle::new("Hold to select", 11.0, Color::srgb(0.35, 0.35, 0.45)),
                Node {
                    margin: UiRect::top(Val::Px(6.0)),
                    ..default()
                },
            )
        ],
    )
}

pub fn update_card_interaction(
    mut query: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor), With<UpgradeCard>>,
) {
    for (interaction, mut bg, mut border) in &mut query {
        match interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(BG_CARD_HOVER);
            }
            Interaction::Pressed => {
                *bg = BackgroundColor(BG_CARD_HOVER);
            }
            Interaction::None => {
                *bg = BackgroundColor(BG_CARD);
            }
        }
    }
}
