use crate::systems::hud::components::{DisplayStatKind, StatsPopup, ICON_STATISTICS};
use crate::systems::hud::resources::HUDTextureAtlas;
use crate::systems::input::resources::ActionState;
use crate::systems::states::waves::components::Health;
use crate::systems::states::waves::player::components::{Player, PlayerStats, StatKind};
use crate::systems::states::waves::player::experience::PlayerExperience;
use crate::systems::states::waves::resources::WaveManager;
use bevy::color::Color;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::image::TextureAtlas;
use bevy::prelude;
use bevy::prelude::{
    default, AlignItems, BackgroundColor, BorderColor, BorderRadius, ChildOf, Commands,
    Entity, FlexDirection, ImageNode, JustifyContent, Node, PositionType, Query, Res, Text,
    TextColor, TextFont, UiRect, Val, With,
};

pub fn toggle_stats_popup(
    mut commands: Commands,
    stats_query: Query<Entity, With<StatsPopup>>,
    actions: Res<ActionState>,
    sprites: Res<HUDTextureAtlas>,
) -> prelude::Result {
    let ui_exists = !stats_query.is_empty();

    // Remove UI when show stats is toggled off
    if !actions.toggle_show_stats && ui_exists {
        for entity in &stats_query {
            commands.get_entity(entity)?.despawn();
        }
        return Ok(());
    }

    if ui_exists || !actions.toggle_show_stats {
        return Ok(());
    }

    commands
        .spawn((
            StatsPopup,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(80.0),
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                border: UiRect::all(Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)), // ← now a Node field
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
            BorderColor::all(Color::srgb(0.3, 0.5, 0.8)),
        ))
        .with_children(|parent| {
            spawn_stat_title(&sprites, parent);
            spawn_stat_row(parent, DisplayStatKind::Level, &sprites);
            spawn_stat_row(parent, DisplayStatKind::Experience, &sprites);
            spawn_separator(parent);
            for stat_kind in [
                StatKind::Damage,
                StatKind::FireRate,
                StatKind::Range,
                StatKind::Speed,
            ] {
                spawn_stat_row(parent, DisplayStatKind::PlayerStat(stat_kind), &sprites);
            }
            spawn_separator(parent);
            spawn_stat_row(parent, DisplayStatKind::Health, &sprites);
            spawn_separator(parent);
            spawn_stat_row(parent, DisplayStatKind::Wave, &sprites);
        });
    Ok(())
}

pub fn update_stats_popup(
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
