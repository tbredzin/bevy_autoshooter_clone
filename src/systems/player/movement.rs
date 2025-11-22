use crate::resources::GAME_AREA;
use crate::systems::player::components::Player;
use crate::systems::player::resources::PLAYER_SPEED;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Query, Res, Time, Transform, With};

pub fn update_position(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
        transform.translation += direction.extend(0.0) * PLAYER_SPEED * time.delta_secs();
    }

    // Clamp to game area
    transform.translation.x = transform
        .translation
        .x
        .clamp(GAME_AREA.min.x, GAME_AREA.max.x);
    transform.translation.y = transform
        .translation
        .y
        .clamp(GAME_AREA.min.y, GAME_AREA.max.y);
}
