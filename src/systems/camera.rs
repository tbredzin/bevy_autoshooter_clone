use crate::resources::{GAME_AREA, tiles_to_pixels};
use crate::systems::player::components::Player;
use bevy::prelude::*;

/// Smoothly follows the player with the camera, clamped to game boundaries
pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single()
        && let Ok(mut camera_transform) = camera_query.single_mut()
    {
        let player_pos = player_transform.translation;

        // Clamp camera position to keep it within game area bounds
        let target_x = player_pos.x.clamp(
            GAME_AREA.min.x + tiles_to_pixels(1.0),
            GAME_AREA.max.x - tiles_to_pixels(1.0),
        );
        let target_y = player_pos.y.clamp(
            GAME_AREA.min.y + tiles_to_pixels(1.0),
            GAME_AREA.max.y - tiles_to_pixels(1.0),
        );

        let smoothing = 2.0; // Higher = snappier, lower = smoother
        let target_pos = Vec3::new(target_x, target_y, camera_transform.translation.z);

        let new_pos = camera_transform
            .translation
            .lerp(target_pos, time.delta_secs() * smoothing);

        // Round to nearest pixel to be pixel perfect
        camera_transform.translation.x = new_pos.x.round();
        camera_transform.translation.y = new_pos.y.round();
        camera_transform.translation.z = new_pos.z;
    }
}
