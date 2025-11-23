use crate::components::{HUDLevelUp, HUDLevelUps, HUDText, Health};
use crate::resources::{HUDTextureAtlas, WaveManager};
use crate::systems::input::gamepad::ActiveGamepad;
use crate::systems::player::components::Player;
use crate::systems::player::experience::PlayerExperience;
use crate::systems::player_upgrades::components::{PlayerStats, StatKind};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

#[derive(Component)]
pub struct StatsDisplayUI;

const ICON_STATISTICS: usize = 13;
pub(crate) const ICON_LEVEL: usize = 3;
const ICON_EXPERIENCE: usize = 14;
const ICON_DAMAGE: usize = 12;
const ICON_FIRE_RATE: usize = 30;
const ICON_RANGE: usize = 18;
const ICON_HEALTH: usize = 49;
const ICON_SPEED: usize = 29;
const ICON_WAVE: usize = 59;
#[derive(Component)]
pub enum DisplayStatKind {
    Level,
    Experience,
    Wave,
    PlayerStat(StatKind),
    Health, // Special case: current/max health display
}

impl From<StatKind> for DisplayStatKind {
    fn from(kind: StatKind) -> DisplayStatKind {
        DisplayStatKind::PlayerStat(kind)
    }
}

impl DisplayStatKind {
    pub fn get_display_info(&self) -> (usize, &'static str, Color) {
        match self {
            DisplayStatKind::Level => (ICON_LEVEL, "Level", Color::srgb(1.0, 0.8, 0.2)),
            DisplayStatKind::Experience => {
                (ICON_EXPERIENCE, "Experience", Color::srgb(0.8, 0.6, 1.0))
            }
            DisplayStatKind::Wave => (ICON_WAVE, "Wave", Color::srgb(1.0, 1.0, 1.0)),
            DisplayStatKind::Health => (ICON_HEALTH, "Health", Color::srgb(0.2, 1.0, 0.3)),
            DisplayStatKind::PlayerStat(stat) => match stat {
                StatKind::Damage => (ICON_DAMAGE, "Damage", Color::srgb(1.0, 0.4, 0.4)),
                StatKind::FireRate => (ICON_FIRE_RATE, "Fire Rate", Color::srgb(1.0, 0.6, 0.2)),
                StatKind::Range => (ICON_RANGE, "Range", Color::srgb(0.4, 0.8, 1.0)),
                StatKind::MaxHealth => (ICON_HEALTH, "Health", Color::srgb(0.2, 1.0, 0.3)),
                StatKind::Speed => (ICON_SPEED, "Speed", Color::srgb(0.4, 1.0, 0.8)),
            },
        }
    }
}

pub fn update_ui(
    mut ui_query: Query<&mut Text, With<HUDText>>,
    wave_manager: Res<WaveManager>,
    player_query: Query<(&PlayerStats, &Health), With<Player>>,
) {
    let Ok((stats, player_health)) = player_query.single() else {
        return;
    };

    for mut text in &mut ui_query {
        **text = format!(
            "Wave: {} | HP: {:.0}/{:.0} | {}",
            wave_manager.wave,
            player_health.value,
            stats.max_health,
            format!("Time: {:.1}s", wave_manager.wave_timer.remaining_secs())
        );
    }
}

const GOLD: Color = Color::srgb(218.0, 145.0, 0.0);

pub fn show_level_ups(
    mut commands: Commands,
    xp_query: Query<&PlayerExperience, With<Player>>,
    level_ups_query: Query<(Entity, Option<&Children>), With<HUDLevelUps>>,
    sprites: Res<HUDTextureAtlas>,
) {
    let Ok(player_xp) = xp_query.single() else {
        return;
    };
    let Ok((parent, level_ups)) = level_ups_query.single() else {
        return;
    };

    let level_ups_count = level_ups.map(|ups| ups.len()).unwrap_or(0) as u32;
    if level_ups_count < player_xp.new_levels {
        commands.entity(parent).with_child((
            HUDLevelUp {},
            ImageNode::from_atlas_image(
                sprites.texture.clone(),
                TextureAtlas {
                    layout: sprites.layout.clone(),
                    index: ICON_LEVEL,
                },
            )
            .with_color(GOLD),
        ));
    }
}
pub fn clear_level_ups(mut commands: Commands, level_ups_query: Query<Entity, With<HUDLevelUp>>) {
    for entity in level_ups_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Shows detailed character statistics when Tab is held
pub fn show_stats_display(
    mut commands: Commands,
    stats_query: Query<Entity, With<StatsDisplayUI>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    sprites: Res<HUDTextureAtlas>,
    active_gamepad: Option<Res<ActiveGamepad>>,
    gamepads: Query<&Gamepad>,
) -> Result {
    let tab_pressed = keyboard.pressed(KeyCode::Tab);
    let ui_exists = !stats_query.is_empty();

    let select_pressed = if let Some(gamepad) = active_gamepad.as_ref() {
        if let Ok(gamepad) = gamepads.get(gamepad.0) {
            gamepad.pressed(GamepadButton::Select)
        } else {
            false
        }
    } else {
        false
    };

    // Remove UI when Tab is released
    if !tab_pressed && ui_exists && !select_pressed {
        for entity in &stats_query {
            commands.get_entity(entity)?.despawn();
        }
        return Ok(());
    }

    // Don't spawn if already exists or Tab not pressed
    if ui_exists || (!tab_pressed && !select_pressed) {
        return Ok(());
    }

    commands
        .spawn((
            StatsDisplayUI,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(80.0),
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
            BorderColor::all(Color::srgb(0.3, 0.5, 0.8)),
            BorderRadius::all(Val::Px(10.0)),
        ))
        .with_children(|parent| {
            spawn_stat_title(&sprites, parent);
            spawn_stat_row(parent, DisplayStatKind::Level, &sprites);
            spawn_stat_row(parent, DisplayStatKind::Experience, &sprites);
            spawn_separator(parent);
            for stat_kind in [StatKind::Damage, StatKind::FireRate, StatKind::Range] {
                spawn_stat_row(parent, DisplayStatKind::PlayerStat(stat_kind), &sprites);
            }
            spawn_separator(parent);
            spawn_stat_row(parent, DisplayStatKind::Health, &sprites);
            spawn_separator(parent);
            spawn_stat_row(parent, DisplayStatKind::Wave, &sprites);
        });
    Ok(())
}

/// Updates the stat values in the display
pub fn update_stats_display(
    mut stat_query: Query<(&mut Text, &DisplayStatKind)>,
    player_query: Query<(&PlayerStats, &Health, &PlayerExperience), With<Player>>,
    wave_manager: Res<WaveManager>,
) {
    let Ok((stats, health, xp)) = player_query.single() else {
        return;
    };

    for (mut text, display_kind) in &mut stat_query {
        **text = match display_kind {
            DisplayStatKind::Level => format!("{}", xp.level),
            DisplayStatKind::Experience => format!("{} XP", xp.value),
            DisplayStatKind::Wave => format!("{}", wave_manager.wave),
            DisplayStatKind::Health => format!("{:.0}/{:.0}", health.value, stats.max_health),
            DisplayStatKind::PlayerStat(stat_kind) => stats.format_value(*stat_kind),
        };
    }
}

// Helper function to create a separator
fn spawn_separator(parent: &mut RelatedSpawnerCommands<ChildOf>) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(2.0),
            margin: UiRect::vertical(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.5, 0.5, 0.6, 0.3)),
    ));
}

// Helper function to create a stat row

fn spawn_stat_title(sprites: &Res<HUDTextureAtlas>, parent: &mut RelatedSpawnerCommands<ChildOf>) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn(Node {
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|label_container| {
                label_container.spawn((
                    ImageNode::from_atlas_image(
                        sprites.texture.clone(),
                        TextureAtlas {
                            layout: sprites.layout.clone(),
                            index: ICON_STATISTICS,
                        },
                    ),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        ..default()
                    },
                ));
                label_container.spawn((
                    Text::new("CHARACTER STATS"),
                    TextFont {
                        font_size: 26.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 1.0)),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
            });
        });
}
fn spawn_stat_row(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    display_kind: DisplayStatKind,
    sprites: &Res<HUDTextureAtlas>,
) {
    let (texture_index, label, color) = display_kind.get_display_info();
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn(Node {
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|label_container| {
                label_container.spawn((
                    ImageNode::from_atlas_image(
                        sprites.texture.clone(),
                        TextureAtlas {
                            layout: sprites.layout.clone(),
                            index: texture_index,
                        },
                    ),
                    Node {
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        ..default()
                    },
                ));
                label_container.spawn((
                    Text::new(label),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.85, 0.9)),
                ));
                label_container.spawn((Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    ..default()
                },));
            });

            // Value - will be updated by update_stats_display
            row.spawn((
                Text::new(""),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(color),
                display_kind,
            ));
        });
}
