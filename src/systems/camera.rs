use crate::components::Player;
use crate::resources::{GAME_AREA, tiles_to_pixels};
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

        camera_transform.translation = camera_transform.translation.lerp(
            Vec3::new(target_x, target_y, camera_transform.translation.z),
            time.delta_secs(),
        );
    }
}
