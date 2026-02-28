use crate::systems::input::resources::{ActionState, ActiveInputDevice};
use crate::systems::input::systems::collect_actions;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveInputDevice>()
            .init_resource::<ActionState>()
            // Runs before Update so every system in Update sees a fresh snapshot.
            .add_systems(PreUpdate, collect_actions);
    }
}
