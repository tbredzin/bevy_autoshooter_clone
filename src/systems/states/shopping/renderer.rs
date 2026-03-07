use crate::systems::game::GameState;
use crate::systems::states::shopping::components::{NextWaveButton, ShoppingUI};
use bevy::prelude::*;

pub fn spawn_shopping(mut commands: Commands) {
    commands
        .spawn((
            ShoppingUI,
            DespawnOnExit(GameState::Shopping),
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
            parent.spawn((
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
                children![(
                    Text::new("Start next waves..."),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )],
            ));
        });
}
pub fn update_start_button_interaction(
    mut query: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor), With<NextWaveButton>>,
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
