use crate::systems::states::menu::components::{
    AnimatedBorder, DividerSegment, MainMenuUI, QuitButton, StartButton, TitleWord,
};
use bevy::color::palettes::css::*;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use std::f32::consts::TAU;

pub const BG: Color = Color::Srgba(MIDNIGHT_BLUE);
pub const PALETTE: [Color; 6] = [
    Color::Srgba(INDIAN_RED),
    Color::Srgba(GOLDENROD),
    Color::Srgba(LIMEGREEN),
    Color::Srgba(LIGHT_SKY_BLUE),
    Color::Srgba(MEDIUM_VIOLET_RED),
    Color::Srgba(MAGENTA),
];
pub const DIVIDER_SEGMENTS: usize = 24;

pub fn palette_color(phase: f32) -> Color {
    let n = PALETTE.len() as f32;
    // Normalize phase to [0, n)
    let pos = (phase / TAU).rem_euclid(1.0) * n;
    let lo = pos.floor() as usize % PALETTE.len();
    let hi = (lo + 1) % PALETTE.len();
    let t = pos.fract();

    let a = PALETTE[lo].to_linear();
    let b = PALETTE[hi].to_linear();
    Color::linear_rgb(
        a.red + (b.red - a.red) * t,
        a.green + (b.green - a.green) * t,
        a.blue + (b.blue - a.blue) * t,
    )
}

pub fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenuUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(36.0),
                ..default()
            },
            BackgroundColor(BG),
            ZIndex(300),
            children![(
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(20.0),
                    ..default()
                },
                children![(
                    Text::new("*"),
                    TextFont {
                        font_size: 48.0,
                        ..default()
                    },
                    TextColor(Color::Srgba(LIGHT_SKY_BLUE)),
                    children![
                        (
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(4.0),
                                ..default()
                            },
                            children![
                                (
                                    Text::new("WAR"),
                                    TextFont {
                                        font_size: 96.0,
                                        ..default()
                                    },
                                    TextColor(Color::Srgba(INDIAN_RED)),
                                    TitleWord { phase: 0.0 },
                                ),
                                (
                                    Text::new("IN"),
                                    TextFont {
                                        font_size: 32.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                    TitleWord { phase: TAU / 3.0 },
                                ),
                                (
                                    Text::new("WONDERLAND"),
                                    TextFont {
                                        font_size: 54.0,
                                        ..default()
                                    },
                                    TextColor(Color::Srgba(VIOLET)),
                                    TitleWord {
                                        phase: 2.0 * std::f32::consts::TAU / 3.0,
                                    },
                                )
                            ],
                        ),
                        (
                            Text::new("*"),
                            TextFont {
                                font_size: 48.0,
                                ..default()
                            },
                            TextColor(Color::Srgba(MAGENTA)),
                        )
                    ],
                )],
            )],
        ))
        .with_children(|root| {
            spawn_divider(root);
            spawn_buttons(root);
            spawn_hint(root);
        });
}

pub fn despawn_main_menu(mut commands: Commands, ui: Single<Entity, With<MainMenuUI>>) {
    commands.entity(ui.entity()).despawn();
}

fn spawn_divider(root: &mut RelatedSpawnerCommands<ChildOf>) {
    root.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(3.0),
        ..default()
    })
    .with_children(|row| {
        for i in 0..DIVIDER_SEGMENTS {
            // Pre-assign a starting color so there's no flash on frame 0
            let initial_phase = (i as f32 / DIVIDER_SEGMENTS as f32) * TAU;
            let color = palette_color(initial_phase);

            row.spawn((
                DividerSegment { index: i },
                Node {
                    width: Val::Px(18.0),
                    height: Val::Px(5.0),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(color),
            ));
        }
    });
}

fn spawn_buttons(root: &mut RelatedSpawnerCommands<ChildOf>) {
    root.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(18.0),
            ..default()
        },
        children![
            (
                StartButton,
                AnimatedBorder { phase: 0.0 },
                Button,
                Node {
                    width: Val::Px(340.0),
                    height: Val::Px(68.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect {
                        left: Val::Px(4.0),
                        right: Val::Px(1.0),
                        top: Val::Px(1.0),
                        bottom: Val::Px(4.0),
                    },
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.14, 0.06, 0.28, 0.92)),
                BorderColor::all(Color::Srgba(LIMEGREEN)),
                children![(
                    Text::new("> FALL IN <"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )],
            ),
            (
                QuitButton,
                AnimatedBorder {
                    phase: std::f32::consts::PI, // starts at opposite side of palette
                },
                Button,
                Node {
                    width: Val::Px(340.0),
                    height: Val::Px(68.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect {
                        left: Val::Px(1.0),
                        right: Val::Px(4.0),
                        top: Val::Px(4.0),
                        bottom: Val::Px(1.0),
                    },
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.14, 0.06, 0.28, 0.92)),
                BorderColor::all(MAGENTA),
                children![(
                    Text::new("x FLEE x"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )],
            )
        ],
    ));
}

fn spawn_hint(root: &mut RelatedSpawnerCommands<ChildOf>) {
    root.spawn((
        Text::new("Enter / Space to start"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgba(0.75, 0.65, 0.90, 0.55)),
    ));
}
