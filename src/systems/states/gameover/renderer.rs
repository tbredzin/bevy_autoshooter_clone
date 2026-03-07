use crate::systems::game::{GameOverStats, GameState};
use crate::systems::states::gameover::components::{GameOverUI, RestartButton};
use bevy::prelude::*;

const BG: Color = Color::srgba(0.0, 0.0, 0.0, 0.88);
const STATS_BG: Color = Color::srgb(0.07, 0.07, 0.12);

pub fn spawn_game_over_ui(mut commands: Commands, stats: Res<GameOverStats>) {
    let wave_text = stats.wave_reached.to_string();
    let level_text = stats.level_reached.to_string();
    let xp_text = format!("{} XP", stats.experience_total);

    commands.spawn((
        GameOverUI,
        DespawnOnExit(GameState::GameOver),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(24.0),
            ..default()
        },
        BackgroundColor(BG),
        ZIndex(200),
        children![
            // ── Title ────────────────────────────────────────────────────────
            (
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.2, 0.2)),
            ),
            // ── Subtitle ─────────────────────────────────────────────────────
            (
                Text::new("You have fallen..."),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.55, 0.55, 0.65)),
            ),
            // ── Stats section ─────────────────────────────────────────────────
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(32.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    row_gap: Val::Px(14.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(STATS_BG),
                BorderColor::all(Color::srgb(0.3, 0.3, 0.45)),
                children![
                    (
                        Text::new("RUN SUMMARY"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.75)),
                    ),
                    stat_row("Wave Reached", wave_text, Color::srgb(1.0, 0.78, 0.2)),
                    stat_row("Level Reached", level_text, Color::srgb(0.4, 0.8, 1.0)),
                    stat_row("Total XP", xp_text, Color::srgb(0.7, 0.5, 1.0)),
                ],
            ),
            // ── Restart button ───────────────────────────────────────────────
            (
                RestartButton,
                Button,
                Node {
                    width: Val::Px(320.0),
                    height: Val::Px(64.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.0)),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.45, 0.15)),
                BorderColor::all(Color::srgb(0.3, 0.8, 0.3)),
                children![(
                    Text::new("Play Again"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )],
            ),
            // ── Restart hint ─────────────────────────────────────────────────
            (
                Text::new("Press Enter / Start to restart"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.4, 0.4, 0.5)),
            ),
        ],
    ));
}

// ── Helpers ───────────────────────────────────────────────────────────────────
fn stat_row(label: &'static str, value: String, color: Color) -> impl Bundle {
    (
        Node {
            width: Val::Px(300.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new(label),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.85)),
            ),
            (
                Text::new(value),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(color),
            ),
        ],
    )
}

pub fn update_restart_button_interaction(
    mut query: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor), With<RestartButton>>,
) {
    for (interaction, mut bg, mut border) in &mut query {
        match interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0.2, 0.6, 0.2));
                *border = BorderColor::all(Color::srgb(0.4, 1.0, 0.4));
            }
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb(0.1, 0.35, 0.1));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0.15, 0.45, 0.15));
                *border = BorderColor::all(Color::srgb(0.3, 0.8, 0.3));
            }
        }
    }
}
