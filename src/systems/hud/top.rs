use crate::systems::constants::{NEXT_LEVEL_RATIO_PERCENT, WAVE_DURATION};
use crate::systems::hud::components::{
    HUDBottomBorder, HUDHealthFill, HUDHealthText, HUDLevelText, HUDLevelUp, HUDLevelUps,
    HUDTimeFill, HUDTimeText, HUDTopBar, HUDWaveText, HUDXPFill, FILL_HEALTH, FILL_HEALTH_DANGER,
    FILL_TIMER, FILL_TIMER_URGENT, FILL_XP, ICON_HEALTH, ICON_LEVEL, ICON_TIMER, ICON_WAVE, TINT_LEVEL,
    TINT_TIMER, TINT_WAVE,
};
use crate::systems::hud::resources::HUDTextureAtlas;
use crate::systems::states::menu::renderer::palette_color;
use crate::systems::states::waves::components::Health;
use crate::systems::states::waves::player::components::{Player, PlayerStats};
use crate::systems::states::waves::player::experience::PlayerExperience;
use crate::systems::states::waves::resources::WaveManager;
use bevy::color::palettes::css::*;
use bevy::prelude::*;

// ── Layout constants ──────────────────────────────────────────────────────────
const BAR_HEIGHT: f32 = 44.0;
const BAR_PADDING_H: f32 = 16.0;
const BAR_PADDING_V: f32 = 6.0;
const ICON_SIZE: f32 = 20.0;
const PILL_GAP: f32 = 10.0;

const DIVIDER_COLOR: Color = Color::srgba(0.5, 0.5, 0.6, 0.35);
const BG_TRACK: Color = Color::srgba(0.0, 0.0, 0.0, 0.50);
const COL_DIMTEXT: Color = Color::srgba(0.75, 0.65, 0.90, 0.60);

const NB_LEVEL_UPS_PER_ROW: f32 = 6.0;
const BORDER_CYCLE_SPEED: f32 = 1.4;

// ── Spawn ─────────────────────────────────────────────────────────────────────
pub fn spawn_hud(mut commands: Commands, sprites: Res<HUDTextureAtlas>) {
    // ── Root bar ─────────────────────────────────────────────────────────────
    commands
        .spawn((
            HUDTopBar,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                height: Val::Px(BAR_HEIGHT),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::Srgba(MIDNIGHT_BLUE)),
            ZIndex(200),
        ))
        .with_children(|bar| {
            // ── Pills row ─────────────────────────────────────────────────────
            bar.spawn(Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(BAR_PADDING_H), Val::Px(BAR_PADDING_V)),
                column_gap: Val::Px(PILL_GAP),
                ..default()
            })
            .with_children(|row| {
                spawn_health_pill(row, &sprites);
                spawn_divider(row);
                spawn_wave_badge(row, &sprites);
                spawn_divider(row);
                spawn_timer_pill(row, &sprites);
                spawn_divider(row);
                spawn_level_pill(row, &sprites);
            });

            // ── Animated bottom border ────────────────────────────────────────
            bar.spawn((
                HUDBottomBorder,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(2.0),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ));
        });

    // ── Level-up indicators (top-right, unchanged behaviour) ─────────────────
    commands.spawn((
        HUDLevelUps {},
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            top: Val::Px(BAR_HEIGHT + 8.0),
            flex_direction: FlexDirection::RowReverse,
            column_gap: Val::Px(8.0),
            row_gap: Val::Px(8.0),
            width: Val::Px(NB_LEVEL_UPS_PER_ROW * (24.0 + 8.0)),
            height: Val::Px(24.0),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
    ));
}

// ── Despawn ───────────────────────────────────────────────────────────────────
pub fn despawn_hud(
    mut commands: Commands,
    bar: Single<Entity, With<HUDTopBar>>,
    levelups: Single<Entity, With<HUDLevelUps>>,
) {
    commands.entity(bar.entity()).despawn();
    commands.entity(levelups.entity()).despawn();
}

// ── Per-frame update ──────────────────────────────────────────────────────────
pub fn update(
    // health
    mut hp_fill: Query<
        (&mut Node, &mut BackgroundColor),
        (With<HUDHealthFill>, Without<HUDTimeFill>),
    >,
    mut hp_text: Query<
        &mut Text,
        (
            With<HUDHealthText>,
            Without<HUDTimeText>,
            Without<HUDLevelText>,
        ),
    >,
    // wave
    mut wave_text: Query<
        &mut Text,
        (
            With<HUDWaveText>,
            Without<HUDHealthText>,
            Without<HUDTimeText>,
            Without<HUDLevelText>,
        ),
    >,
    // timer
    mut t_fill: Query<
        (&mut Node, &mut BackgroundColor),
        (With<HUDTimeFill>, Without<HUDHealthFill>),
    >,
    mut t_text: Query<
        &mut Text,
        (
            With<HUDTimeText>,
            Without<HUDHealthText>,
            Without<HUDWaveText>,
            Without<HUDLevelText>,
        ),
    >,
    // level / xp
    mut lv_text: Query<
        &mut Text,
        (
            With<HUDLevelText>,
            Without<HUDHealthText>,
            Without<HUDWaveText>,
            Without<HUDTimeText>,
        ),
    >,
    mut xp_fill: Query<
        &mut Node,
        (
            With<HUDXPFill>,
            Without<HUDHealthFill>,
            Without<HUDTimeFill>,
        ),
    >,
    // data
    wave_manager: Res<WaveManager>,
    player_query: Query<(&PlayerStats, &PlayerExperience, &Health), With<Player>>,
) {
    let Ok((stats, xp, health)) = player_query.single() else {
        return;
    };

    // ── Health ────────────────────────────────────────────────────────────────
    let hp_ratio = (health.value / stats.max_health).clamp(0.0, 1.0);
    if let Ok((mut node, mut color)) = hp_fill.single_mut() {
        node.width = Val::Percent(hp_ratio * 100.0);
        *color = BackgroundColor(if hp_ratio < 0.25 {
            FILL_HEALTH_DANGER
        } else {
            FILL_HEALTH
        });
    }
    if let Ok(mut text) = hp_text.single_mut() {
        **text = format!("{:.0} / {:.0}", health.value, stats.max_health);
    }

    // ── Wave ──────────────────────────────────────────────────────────────────
    if let Ok(mut text) = wave_text.single_mut() {
        **text = format!("{}", wave_manager.wave);
    }

    // ── Timer ─────────────────────────────────────────────────────────────────
    let remaining = wave_manager.wave_timer.remaining_secs();
    let time_ratio = (remaining / WAVE_DURATION).clamp(0.0, 1.0);
    if let Ok((mut node, mut color)) = t_fill.single_mut() {
        node.width = Val::Percent(time_ratio * 100.0);
        *color = BackgroundColor(if remaining < 5.0 {
            FILL_TIMER_URGENT
        } else {
            FILL_TIMER
        });
    }
    if let Ok(mut text) = t_text.single_mut() {
        **text = format!("{:.0}s", remaining.ceil());
    }

    // ── Level / XP ────────────────────────────────────────────────────────────
    if let Ok(mut text) = lv_text.single_mut() {
        **text = format!("Lv {}", xp.level);
    }
    // xp_to_next mirrors the threshold in experience.rs
    let xp_to_next = xp.level * NEXT_LEVEL_RATIO_PERCENT;
    let xp_ratio = (xp.value as f32 / xp_to_next as f32).clamp(0.0, 1.0);
    if let Ok(mut node) = xp_fill.single_mut() {
        node.width = Val::Percent(xp_ratio * 100.0);
    }
}

// ── Animated bottom border ────────────────────────────────────────────────────
pub fn animate_hud_border(
    time: Res<Time>,
    mut border_query: Query<&mut BackgroundColor, With<HUDBottomBorder>>,
) {
    let color = palette_color(time.elapsed_secs() * BORDER_CYCLE_SPEED);
    for mut bg in &mut border_query {
        *bg = BackgroundColor(color);
    }
}

// ── Level-up indicator (unchanged logic) ─────────────────────────────────────
pub fn update_level_up_indicator(
    mut commands: Commands,
    xp_query: Query<&PlayerExperience, With<Player>>,
    level_ups_query: Query<(Entity, Option<&Children>), With<HUDLevelUps>>,
    sprites: Res<HUDTextureAtlas>,
) {
    let Ok(player_xp) = xp_query.single() else {
        return;
    };
    let Ok((hud_level_ups, children)) = level_ups_query.single() else {
        return;
    };
    match children {
        None => {
            if player_xp.new_levels >= 1 {
                commands
                    .entity(hud_level_ups)
                    .with_child(HUDLevelUp::render(sprites));
            }
        }
        Some(icons) => {
            if icons.len() > player_xp.new_levels as usize {
                commands.entity(*icons.last().unwrap()).despawn();
            }
            if icons.len() < player_xp.new_levels as usize {
                commands
                    .entity(hud_level_ups)
                    .with_child(HUDLevelUp::render(sprites));
            }
        }
    };
}

// ── Pill helpers ──────────────────────────────────────────────────────────────

fn spawn_icon(
    parent: &mut ChildSpawnerCommands,
    sprites: &HUDTextureAtlas,
    index: usize,
    tint: Color,
) {
    parent.spawn((
        ImageNode::from_atlas_image(
            sprites.texture.clone(),
            TextureAtlas {
                layout: sprites.layout.clone(),
                index,
            },
        )
        .with_color(tint),
        Node {
            width: Val::Px(ICON_SIZE),
            height: Val::Px(ICON_SIZE),
            flex_shrink: 0.0,
            ..default()
        },
    ));
}

fn spawn_divider(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Px(1.0),
            height: Val::Px(28.0),
            flex_shrink: 0.0,
            ..default()
        },
        BackgroundColor(DIVIDER_COLOR),
    ));
}

/// ❤  HP pill — left side, grows to fill remaining space.
fn spawn_health_pill(parent: &mut ChildSpawnerCommands, sprites: &HUDTextureAtlas) {
    parent
        .spawn(Node {
            flex_grow: 2.0,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|pill| {
            spawn_icon(pill, sprites, ICON_HEALTH, Color::from(RED));

            // Sub-label
            pill.spawn((
                Text::new("HP"),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
                TextColor(COL_DIMTEXT),
            ));

            // Progress track
            pill.spawn((
                Node {
                    flex_grow: 1.0,
                    height: Val::Px(10.0),
                    border_radius: BorderRadius::all(Val::Px(5.0)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(BG_TRACK),
            ))
            .with_children(|track| {
                track.spawn((
                    HUDHealthFill,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(FILL_HEALTH),
                ));
            });

            // Numeric readout
            pill.spawn((
                HUDHealthText,
                Text::new("100 / 100"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// Wave badge — fixed width, centred number.
fn spawn_wave_badge(parent: &mut ChildSpawnerCommands, sprites: &HUDTextureAtlas) {
    parent
        .spawn(Node {
            width: Val::Px(90.0),
            flex_shrink: 0.0,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|badge| {
            spawn_icon(badge, sprites, ICON_WAVE, TINT_WAVE);

            badge.spawn((
                Text::new("WAVE"),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(COL_DIMTEXT),
            ));

            badge.spawn((
                HUDWaveText,
                Text::new("1"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::Srgba(INDIAN_RED)),
            ));
        });
}

/// ⏱ Timer pill — fixed width, drains left-to-right.
fn spawn_timer_pill(parent: &mut ChildSpawnerCommands, sprites: &HUDTextureAtlas) {
    parent
        .spawn(Node {
            width: Val::Px(150.0),
            flex_shrink: 0.0,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|pill| {
            spawn_icon(pill, sprites, ICON_TIMER, TINT_TIMER);

            // Track
            pill.spawn((
                Node {
                    flex_grow: 1.0,
                    height: Val::Px(10.0),
                    border_radius: BorderRadius::all(Val::Px(5.0)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(BG_TRACK),
            ))
            .with_children(|track| {
                track.spawn((
                    HUDTimeFill,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(FILL_TIMER),
                ));
            });

            // Seconds readout
            pill.spawn((
                HUDTimeText,
                Text::new("60s"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(TINT_TIMER),
            ));
        });
}

/// ★ Level / XP pill — fixed width, XP mini-bar below level text.
fn spawn_level_pill(parent: &mut ChildSpawnerCommands, sprites: &HUDTextureAtlas) {
    parent
        .spawn(Node {
            width: Val::Px(140.0),
            flex_shrink: 0.0,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|pill| {
            spawn_icon(pill, sprites, ICON_LEVEL, Color::Srgba(GOLDENROD));

            // Level text + XP mini-bar stacked vertically
            pill.spawn(Node {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(3.0),
                ..default()
            })
            .with_children(|stack| {
                // "Lv N"
                stack.spawn((
                    HUDLevelText,
                    Text::new("Lv 1"),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(TINT_LEVEL),
                ));

                // XP mini-bar
                stack
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(4.0),
                            border_radius: BorderRadius::all(Val::Px(2.0)),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        BackgroundColor(BG_TRACK),
                    ))
                    .with_children(|track| {
                        track.spawn((
                            HUDXPFill,
                            Node {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                border_radius: BorderRadius::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(FILL_XP),
                        ));
                    });
            });
        });
}
