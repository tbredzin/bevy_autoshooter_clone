use bevy::prelude::*;

/// Resource to track the first connected gamepad for single-player
#[derive(Resource)]
pub struct ActiveGamepad(pub Entity);

#[derive(Resource)]
pub struct GamepadAsset {
    pub layout: Handle<TextureAtlasLayout>,
    pub asset: Handle<Image>,
}

const GDB_SPRITESHEET_PATH: &'static str = "spritesheet/gdb-xbox-2.png";
pub fn load_animation_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.insert_resource(GamepadAsset {
        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(16, 16),
            35,
            40,
            None,
            None,
        )),
        asset: asset_server.load(GDB_SPRITESHEET_PATH),
    });
}

/// System to track gamepad connections and store the first connected one
pub fn handle_gamepad_connection(
    mut commands: Commands,
    active_gamepad: Option<Res<ActiveGamepad>>,
    gamepads: Query<Entity, Added<Gamepad>>,
) {
    // Only track the first gamepad if none is active
    if active_gamepad.is_none() {
        for gamepad_entity in &gamepads {
            info!("Gamepad connected: {:?}", gamepad_entity);
            commands.insert_resource(ActiveGamepad(gamepad_entity));
            break; // Only use the first one
        }
    }
}

/// Get movement input from gamepad (left stick or D-pad)
pub fn get_gamepad_movement(gamepad: &Gamepad) -> Vec2 {
    // Try left stick first (analog)
    let left_stick = gamepad.left_stick();

    // Apply deadzone
    const DEADZONE: f32 = 0.15;
    if left_stick.length() > DEADZONE {
        return left_stick;
    }

    // Fallback to D-pad (digital)
    let dpad = gamepad.dpad();
    if dpad.length() > 0.0 {
        return dpad.normalize_or_zero();
    }

    Vec2::ZERO
}
