use crate::systems::input::debug;
use crate::systems::input::resources::{ActionState, ActiveInputDevice};
use crate::systems::input::systems::{collect_actions, detect_input_device};
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveInputDevice>()
            .init_resource::<ActionState>()
            .add_systems(Startup, debug::setup_input_hud)
            .add_systems(
                PreUpdate,
                (
                    detect_input_device,
                    collect_actions,
                    debug::update_active_device_indicator,
                ),
            );
    }
}
