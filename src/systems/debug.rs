use crate::resources::{GamepadAsset, KeyboardAsset};
use crate::systems::input::resources::{ActionState, ActiveInputDevice};
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;

#[derive(Component)]
pub struct InputIconsRow;
#[derive(Component)]
pub struct ActiveDeviceIndicator;
#[derive(Component)]
pub struct ButtonIcon {}
#[derive(Component)]
pub struct KeyIcon {}

const COLOR_KEYBOARD: Color = Color::srgb(0.4, 0.8, 1.0); // cyan-ish blue
const COLOR_GAMEPAD: Color = Color::srgb(1., 0., 0.); // green
const COLOR_INACTIVE: Color = Color::srgba(0.5, 0.5, 0.5, 0.4);

pub fn setup_input_hud(mut commands: Commands) {
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            bottom: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            align_items: AlignItems::FlexStart,
            ..default()
        },))
        .with_children(|parent| {
            // Active-device label — updated every frame by update_active_device_indicator
            parent.spawn((
                ActiveDeviceIndicator,
                Text::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(COLOR_INACTIVE), // starts dimmed (overlay is off by default)
            ));

            // Row that holds the individual input icons / keycap badges
            parent.spawn((
                InputIconsRow,
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(6.0),
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::Wrap,
                    max_width: Val::Px(400.0),
                    ..default()
                },
            ));
        });
}
pub fn update_active_device_indicator(
    active_device: Res<ActiveInputDevice>,
    actions: ResMut<ActionState>,
    mut label_query: Query<(&mut Text, &mut TextColor), With<ActiveDeviceIndicator>>,
) {
    let Ok((mut text, mut color)) = label_query.single_mut() else {
        return;
    };
    if !actions.toggle_show_debug {
        **text = "".to_string();
        return;
    }

    // Only re-write when the resource actually changed (avoids change detection churn)
    if !active_device.is_changed() {
        return;
    }

    match *active_device {
        ActiveInputDevice::Keyboard => {
            **text = "KEYBOARD".to_string();
            *color = TextColor(COLOR_KEYBOARD);
        }
        ActiveInputDevice::Gamepad => {
            **text = "GAMEPAD".to_string();
            *color = TextColor(COLOR_GAMEPAD);
        }
    }
}
pub fn display_button_pressed(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    keyboard_asset: Res<KeyboardAsset>,
    gamepad: Option<Single<&Gamepad>>,
    gamepad_asset: Res<GamepadAsset>,
    row_query: Query<Entity, With<InputIconsRow>>,
    actions: Res<ActionState>,
    active: Res<ActiveInputDevice>,
) {
    let Ok(row) = row_query.single() else {
        return;
    };
    if !actions.toggle_show_debug {
        commands.entity(row).despawn_children();
        return;
    }
    if !actions.is_changed() {
        return;
    }
    commands.entity(row).despawn_children();
    match *active {
        ActiveInputDevice::Keyboard => {
            for key in keyboard.get_pressed() {
                commands
                    .entity(row)
                    .with_child(show_key(key, &keyboard_asset));
            }
        }
        ActiveInputDevice::Gamepad => {
            // Spawn icons for newly pressed buttons
            if let Some(ref gp) = gamepad {
                for button in gp.get_pressed() {
                    commands
                        .entity(row)
                        .with_child(show_button(button, &gamepad_asset));
                }
            }
        }
    }
}

fn show_button(button: &GamepadButton, asset: &GamepadAsset) -> (ButtonIcon, ImageNode, Node) {
    (
        ButtonIcon {},
        ImageNode::from_atlas_image(
            asset.texture.clone(),
            TextureAtlas::from(asset.layout.clone()).with_index(asset.get_button_index(button)),
        ),
        Node {
            width: Val::Px(40.0),
            height: Val::Px(40.0),
            ..default()
        },
    )
}

fn show_key(
    key: &KeyCode,
    asset: &KeyboardAsset,
) -> (
    KeyIcon,
    Node,
    BackgroundColor,
    BorderColor,
    BorderRadius,
    SpawnRelatedBundle<ChildOf, Spawn<(Text, TextFont, TextColor)>>,
) {
    let label = asset.keycode_label(key);
    (
        KeyIcon {},
        Node {
            padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
            border: UiRect::all(Val::Px(2.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            min_width: Val::Px(32.0),
            height: Val::Px(32.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.12, 0.18, 0.25, 0.92)),
        BorderColor::all(COLOR_KEYBOARD),
        BorderRadius::all(Val::Px(5.0)),
        children![(
            Text::new(label),
            TextFont {
                font_size: 13.0,
                ..default()
            },
            TextColor(COLOR_KEYBOARD),
        )],
    )
}
