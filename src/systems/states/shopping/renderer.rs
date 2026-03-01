use crate::systems::game::MarkedForDespawn;
use crate::systems::states::shopping::components::{NextWaveButton, ShoppingUI};
use bevy::prelude::*;

pub fn spawn_shopping(mut commands: Commands) {
    commands
        .spawn((
            ShoppingUI,
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
            parent.spawn((
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
            parent
                .spawn((
                    NextWaveButton,
                    Button,
                    Node {
                        width: Val::Px(400.0),
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
                    let button_text = "Start next waves...";
                    parent.spawn((
                        Text::new(button_text),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn despawn_shopping(mut commands: Commands, ui: Query<Entity, With<ShoppingUI>>) {
    for e in ui {
        commands.entity(e).insert(MarkedForDespawn);
    }
}
