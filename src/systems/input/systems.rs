use crate::systems::input::resources::{ActionState, ActiveInputDevice};
use bevy::input::gamepad::GamepadEvent;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
const GAMEPAD_DEAD_ZONE: f32 = 0.15;

pub fn detect_input_device(
    mut gamepad_events: MessageReader<GamepadEvent>,
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut active_device: ResMut<ActiveInputDevice>,
) {
    // Detect keyboard key stroke
    if keyboard_events.read().next().is_some() {
        *active_device = ActiveInputDevice::Keyboard;
        return;
    }

    // Detect gamepad button event
    for event in gamepad_events.read() {
        match event {
            GamepadEvent::Connection(e) => info!("Gamepad connection: {:?}", e),
            GamepadEvent::Button(_) => {
                *active_device = ActiveInputDevice::Gamepad;
                return;
            }
            GamepadEvent::Axis(_) => {
                *active_device = ActiveInputDevice::Gamepad;
                return;
            }
        }
    }
}

pub fn collect_actions(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepad: Option<Single<&Gamepad>>,
    active_device: ResMut<ActiveInputDevice>,
    mut actions: ResMut<ActionState>,
) {
    actions.clear();
    match *active_device {
        ActiveInputDevice::Gamepad => {
            if let Some(gamepad) = gamepad {
                collect_gamepad_actions(&gamepad, &mut actions);
            }
        }
        ActiveInputDevice::Keyboard => {
            collect_keyboard_actions(keyboard, &mut actions);
        }
    }
}

fn collect_keyboard_actions(
    keyboard: Res<ButtonInput<KeyCode>>,
    actions: &mut ResMut<ActionState>,
) {
    for key in keyboard.get_just_pressed() {
        match key {
            KeyCode::Tab => actions.toggle_show_stats ^= true,
            KeyCode::F1 => actions.toggle_show_debug ^= true,
            KeyCode::Enter | KeyCode::Space => actions.start_next_wave |= true,
            KeyCode::Digit1 => actions.card_select[0] |= true,
            KeyCode::Digit2 => actions.card_select[1] |= true,
            KeyCode::Digit3 => actions.card_select[2] |= true,
            KeyCode::Digit4 => actions.card_select[3] |= true,
            KeyCode::Unidentified(_) => {}
            _ => {}
        }
    }
    for key in keyboard.get_pressed() {
        match key {
            KeyCode::KeyW | KeyCode::ArrowUp => actions.movement.y += 1.0,
            KeyCode::KeyS | KeyCode::ArrowDown => actions.movement.y -= 1.0,
            KeyCode::KeyA | KeyCode::ArrowLeft => actions.movement.x -= 1.0,
            KeyCode::KeyD | KeyCode::ArrowRight => actions.movement.x += 1.0,
            _ => {}
        }
    }
    actions.movement = actions.movement.normalize_or_zero();
}

fn collect_gamepad_actions(gamepad: &Gamepad, actions: &mut ResMut<ActionState>) {
    for button in gamepad.get_just_pressed() {
        match button {
            GamepadButton::West => actions.card_select[0] |= true,
            GamepadButton::South => actions.card_select[1] |= true,
            GamepadButton::North => actions.card_select[2] |= true,
            GamepadButton::East => actions.card_select[3] |= true,
            GamepadButton::Start => actions.start_next_wave |= true,
            GamepadButton::Select => actions.toggle_show_stats ^= true,
            GamepadButton::LeftThumb => actions.toggle_show_debug ^= true,
            _ => {}
        }
    }
    if gamepad.left_stick().length() > GAMEPAD_DEAD_ZONE {
        actions.movement = gamepad.left_stick().normalize_or_zero();
    } else if gamepad.dpad().length() > 0.0 {
        actions.movement = gamepad.dpad().normalize_or_zero();
    }
}
