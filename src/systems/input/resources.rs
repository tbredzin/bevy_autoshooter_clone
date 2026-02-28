use bevy::prelude::*;

/// Tracks which physical input device was most recently used.
/// The last device to produce **any** input takes precedence for the rest
/// of that frame and all subsequent frames until the other device is used.
#[derive(Resource, Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActiveInputDevice {
    #[default]
    Keyboard,
    Gamepad,
}

/// A frame-level snapshot of all abstract game actions, populated by
/// [`super::systems::collect_actions`] in `PreUpdate`.
#[derive(Resource, Default, Debug)]
pub struct ActionState {
    // ── Movement ─────────────────────────────────────────────────────────────
    pub movement: Vec2,

    // ── HUD / meta ───────────────────────────────────────────────────────────
    pub toggle_show_stats: bool,
    pub toggle_show_debug: bool,

    // ── Upgrade selection ────────────────────────────────────────────────────
    pub card_select: [bool; 4],

    // ── Wave control ─────────────────────────────────────────────────────────
    pub start_next_wave: bool,
}

impl ActionState {
    pub fn clear(&mut self) {
        self.movement = Vec2::ZERO;
        self.card_select = [false; 4];
        self.start_next_wave = false;
    }
}
