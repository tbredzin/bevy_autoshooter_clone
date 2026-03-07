use bevy::prelude::*;

#[derive(Resource, Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActiveInputDevice {
    #[default]
    Keyboard,
    Gamepad,
}
#[derive(Resource, Default, Debug)]
pub struct ActionState {
    pub movement: Vec2,

    pub toggle_show_stats: bool,
    pub toggle_show_debug: bool,

    pub card_select: [bool; 4],

    pub start_next_wave: bool,

    pub add_weapon: bool,
}

impl ActionState {
    pub fn clear(&mut self) {
        self.movement = Vec2::ZERO;
        self.card_select = [false; 4];
        self.start_next_wave = false;
    }
}

#[derive(Resource)]
pub struct GamepadAsset {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

const SPRITESHEET_WIDTH: u32 = 35;
const SPRITESHEET_BEGIN: u32 = 71;

impl GamepadAsset {
    pub fn get_button_index(&self, button: &GamepadButton) -> usize {
        return match button {
            GamepadButton::West => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 0),
            GamepadButton::South => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 1),
            GamepadButton::North => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 2),
            GamepadButton::East => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 3),
            GamepadButton::LeftTrigger => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 3) + 20,
            GamepadButton::LeftTrigger2 => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 1) + 20,
            GamepadButton::RightTrigger => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 4) + 20,
            GamepadButton::RightTrigger2 => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 2) + 20,
            GamepadButton::Select => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 4),
            GamepadButton::Start => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 5),
            GamepadButton::Mode => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 5) + 20,
            GamepadButton::LeftThumb => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 0) + 12,
            GamepadButton::RightThumb => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 0) + 16,
            GamepadButton::DPadUp => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 3) + 7,
            GamepadButton::DPadDown => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 3) + 8,
            GamepadButton::DPadLeft => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 4) + 7,
            GamepadButton::DPadRight => SPRITESHEET_BEGIN + (SPRITESHEET_WIDTH * 4) + 8,
            _ => 0,
        } as usize;
    }
}
impl FromWorld for GamepadAsset {
    fn from_world(world: &mut World) -> Self {
        let texture = { world.resource::<AssetServer>() }.load("spritesheet/gdb-xbox-2.png");
        let mut atlas = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let layout = atlas.add(TextureAtlasLayout::from_grid(
            UVec2::splat(16u32), // tile size (width, height)
            35,                  // columns
            40,                  // rows
            None,                // no padding
            None,                // no offset
        ));
        GamepadAsset { texture, layout }
    }
}

#[derive(Resource, Default)]
pub struct KeyboardAsset {}

impl KeyboardAsset {
    pub fn keycode_label(&self, key: &KeyCode) -> &'static str {
        match key {
            // Movement
            KeyCode::KeyW | KeyCode::ArrowUp => "UP",
            KeyCode::KeyS | KeyCode::ArrowDown => "DOWN",
            KeyCode::KeyA | KeyCode::ArrowLeft => "LEFT",
            KeyCode::KeyD | KeyCode::ArrowRight => "RIGHT",
            // Common actions
            KeyCode::Space => "SPC",
            KeyCode::Enter => "ENTER",
            KeyCode::Tab => "TAB",
            KeyCode::Escape => "ESC",
            KeyCode::Backspace => "Backspace",
            KeyCode::ShiftLeft | KeyCode::ShiftRight => "SHIFT",
            KeyCode::ControlLeft | KeyCode::ControlRight => "CTRL",
            KeyCode::AltLeft | KeyCode::AltRight => "ALT",
            // Digits
            KeyCode::Digit0 => "0",
            KeyCode::Digit1 => "1",
            KeyCode::Digit2 => "2",
            KeyCode::Digit3 => "3",
            KeyCode::Digit4 => "4",
            KeyCode::Digit5 => "5",
            KeyCode::Digit6 => "6",
            KeyCode::Digit7 => "7",
            KeyCode::Digit8 => "8",
            KeyCode::Digit9 => "9",
            // Function keys
            KeyCode::F1 => "F1",
            KeyCode::F2 => "F2",
            KeyCode::F3 => "F3",
            KeyCode::F4 => "F4",
            KeyCode::F5 => "F5",
            KeyCode::F6 => "F6",
            KeyCode::F7 => "F7",
            KeyCode::F8 => "F8",
            KeyCode::F9 => "F9",
            KeyCode::F10 => "F10",
            KeyCode::F11 => "F11",
            KeyCode::F12 => "F12",
            _ => "?",
        }
    }
}
