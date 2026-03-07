use crate::systems::game::{GameState, TextBundle};
use crate::systems::hud::components::{DisplayStatKind, StatsPopup, ICON_STATISTICS};
use crate::systems::hud::resources::HUDTextureAtlas;
use crate::systems::input::resources::ActionState;
use crate::systems::states::waves::components::Health;
use crate::systems::states::waves::player::components::{Player, PlayerStats, StatKind};
use crate::systems::states::waves::player::experience::PlayerExperience;
use crate::systems::states::waves::resources::WaveManager;
use bevy::color::Color;
use bevy::ecs::children;
use bevy::image::TextureAtlas;
use bevy::prelude;
use bevy::prelude::{
    default, AlignItems, BackgroundColor, BorderColor, BorderRadius, Bundle, Commands,
    DespawnOnExit, Entity, FlexDirection, ImageNode, JustifyContent, Node, PositionType, Query, Res
    , Text, UiRect, Val, With,
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

    commands.spawn((
        StatsPopup,
        DespawnOnExit(GameState::InWave),
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
        children![
            stat_title(&sprites),
            stat_row(DisplayStatKind::Level, &sprites),
            stat_row(DisplayStatKind::Experience, &sprites),
            separator(),
            stat_row(DisplayStatKind::PlayerStat(StatKind::Damage), &sprites),
            stat_row(DisplayStatKind::PlayerStat(StatKind::FireRate), &sprites),
            stat_row(DisplayStatKind::PlayerStat(StatKind::Range), &sprites),
            stat_row(DisplayStatKind::PlayerStat(StatKind::Speed), &sprites),
            separator(),
            stat_row(DisplayStatKind::Health, &sprites),
            separator(),
            stat_row(DisplayStatKind::Wave, &sprites)
        ],
    ));
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
fn separator() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(2.0),
            margin: UiRect::vertical(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.5, 0.5, 0.6, 0.3)),
    )
}

fn stat_title(sprites: &Res<HUDTextureAtlas>) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Node {
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
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
                ),
                (
                    TextBundle::new("CHARACTER STATS", 26.0, Color::srgb(0.9, 0.9, 1.0)),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                )
            ]
        )],
    )
}

fn stat_row(display_kind: DisplayStatKind, sprites: &Res<HUDTextureAtlas>) -> impl Bundle {
    let (texture_index, label, color) = display_kind.get_display_info();
    (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Node {
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
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
                ),
                (TextBundle::new(label, 18., Color::srgb(0.85, 0.85, 0.9)),),
                (Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    ..default()
                }),
                (TextBundle::new("", 20., color), display_kind,)
            ],
        )],
    )
}
