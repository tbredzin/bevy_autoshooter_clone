// src/systems/gamepad_debug
use crate::resources::GamepadAsset;
use bevy::prelude::*;

/// Component to mark the button display container
#[derive(Component)]
pub struct GamepadButtonDisplay;

/// Component to mark individual button icons
#[derive(Component)]
pub struct ButtonIcon {
    pub button: GamepadButton,
}

/// Resource to control whether the display is enabled
#[derive(Resource, Default)]
pub struct GamepadDisplayEnabled(pub bool);

/// Startup system to create the button display container
pub fn setup_gamepad_display(mut commands: Commands) {
    commands.insert_resource(GamepadDisplayEnabled(false)); // Disabled by default

    // Create the container in bottom left
    commands.spawn((
        GamepadButtonDisplay,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            bottom: Val::Px(20.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            ..default()
        },
    ));
}

/// System to detect button presses and spawn icons
pub fn display_button_presses(
    mut commands: Commands,
    gamepad_query: Query<&Gamepad>,
    display_query: Query<Entity, With<GamepadButtonDisplay>>,
    icon_query: Query<&ButtonIcon>,
    gamepad_asset: Res<GamepadAsset>,
    enabled: Res<GamepadDisplayEnabled>,
) {
    if !enabled.0 {
        return;
    }

    let Ok(container) = display_query.single() else {
        return;
    };

    // Check all connected gamepads
    for gamepad in &gamepad_query {
        for button in gamepad.get_pressed() {
            // If not already displayed
            if !icon_query.iter().any(|icon| icon.button == *button) {
                commands.entity(container).with_child((
                    ButtonIcon { button: *button },
                    ImageNode::from_atlas_image(
                        gamepad_asset.texture.clone(),
                        TextureAtlas::from(gamepad_asset.layout.clone())
                            .with_index(gamepad_asset.get_button_index(&button)),
                    ),
                    Node {
                        width: Val::Px(48.0),
                        height: Val::Px(48.0),
                        ..default()
                    },
                ));
            }
        }
    }
}
pub fn hide_button_released(
    mut commands: Commands,
    gamepad_query: Query<&Gamepad, Changed<Gamepad>>,
    icon_query: Query<(Entity, &ButtonIcon)>,
    enabled: Res<GamepadDisplayEnabled>,
) {
    if !enabled.0 {
        return;
    }
    for gamepad in &gamepad_query {
        for (entity, displayed_button) in icon_query {
            if !gamepad.pressed(displayed_button.button) {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// System to toggle the display on/off (example: press F1)
pub fn toggle_gamepad_display(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut enabled: ResMut<GamepadDisplayEnabled>,
    mut icon_query: Query<Entity, With<ButtonIcon>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        enabled.0 = !enabled.0;
        if !enabled.0 {
            for entity in &mut icon_query {
                commands.entity(entity).despawn();
            }
        }
        println!(
            "Gamepad button display: {}",
            if enabled.0 { "ON" } else { "OFF" }
        );
    }
}
