use bevy::prelude::*;

pub const TILE_SIZE: f32 = 64.0;
pub const TILES_X: u32 = 40;
pub const TILES_Y: u32 = 30;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 960.0;

pub const fn tiles_to_pixels(tiles: f32) -> f32 {
    tiles * TILE_SIZE
}

pub const ENEMY_SPEED: f32 = 80.0;
pub const ENEMY_HEALTH: f32 = 4.0;
pub const BULLET_SPEED: f32 = 500.0;
pub const WAVE_DURATION: f32 = 10.;
pub const SPAWN_RATE: f32 = 0.2;
pub const ENEMY_SPAWN_TIME_IN_S: f32 = 2.0;
pub const ENEMY_BASE_XP: u32 = 2;
pub const NEXT_LEVEL_RATIO_PERCENT: u32 = 10;
pub const NB_UPDATES_PER_LEVEL: usize = 4;

pub const GAME_AREA: Rect = Rect {
    min: Vec2 {
        x: -(TILES_X as f32 * TILE_SIZE) / 2.0,
        y: -(TILES_Y as f32 * TILE_SIZE) / 2.0,
    },
    max: Vec2 {
        x: (TILES_X as f32 * TILE_SIZE) / 2.0,
        y: (TILES_Y as f32 * TILE_SIZE) / 2.0,
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WaveState {
    Running,
    Ended,
}

#[derive(Resource)]
pub struct WaveManager {
    pub wave: u32,
    pub wave_timer: Timer,
    pub enemy_spawn_timer: Timer,
    pub wave_state: WaveState,
}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            wave: 1,
            wave_timer: Timer::from_seconds(WAVE_DURATION, TimerMode::Once),
            enemy_spawn_timer: Timer::from_seconds(SPAWN_RATE, TimerMode::Repeating),
            wave_state: WaveState::Running,
        }
    }
}

#[derive(Resource)]
pub struct TilesTextureAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct HUDTextureAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
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

#[derive(Resource)]
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
