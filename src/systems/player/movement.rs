use crate::resources::GAME_AREA;
use crate::systems::player::components::{Direction, Player, PlayerAction};
use crate::systems::player::resources::PLAYER_SPEED;
use crate::systems::player_upgrades::components::PlayerStats;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Gamepad, KeyCode, Query, Res, Single, Time, Transform, With};

pub fn update_position(
    keyboard: Res<ButtonInput<KeyCode>>,
    active_gamepad: Option<Single<&Gamepad>>,
    gamepads: Query<&Gamepad>,
    mut player_query: Query<
        (
            &mut Transform,
            &PlayerStats,
            &mut Direction,
            &mut PlayerAction,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let Ok((mut transform, stats, mut current_direction, mut action)) = player_query.single_mut()
    else {
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

    // Gamepad input (if connected)
    if let Some(gamepad) = active_gamepad.as_ref() {
        let gamepad_input = get_gamepad_movement(gamepad);

        // Add gamepad input to keyboard input (allows both to work simultaneously)
        direction += gamepad_input;
    }

    // Update direction enum for animation system
    if let Some(new_direction) = get_direction(&mut direction) {
        if *current_direction != new_direction {
            *current_direction = new_direction;
        }
    }

    // Apply movement
    if direction != Vec2::ZERO {
        direction = direction.normalize_or_zero();
        transform.translation +=
            direction.extend(0.0) * PLAYER_SPEED * stats.speed_multiplier * time.delta_secs();
        if *action != PlayerAction::WALKING {
            *action = PlayerAction::WALKING;
        }
    } else if *action != PlayerAction::IDLE {
        *action = PlayerAction::IDLE;
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

pub fn get_direction(translation: &mut Vec2) -> Option<Direction> {
    match (translation.x, translation.y) {
        (x, y) if x == 0.0 && y == 0.0 => None,
        (x, y) if y < -0.5 && x.abs() < 0.5 => Some(Direction::SOUTH),
        (x, y) if y < -0.5 && x > 0.5 => Some(Direction::SOUTHEAST),
        (x, y) if x > 0.5 && y.abs() < 0.5 => Some(Direction::EAST),
        (x, y) if y > 0.5 && x > 0.5 => Some(Direction::NORTHEAST),
        (x, y) if y > 0.5 && x.abs() < 0.5 => Some(Direction::NORTH),
        (x, y) if y > 0.5 && x < -0.5 => Some(Direction::NORTHWEST),
        (x, y) if x < -0.5 && y.abs() < 0.5 => Some(Direction::WEST),
        (x, y) if y < -0.5 && x < -0.5 => Some(Direction::SOUTHWEST),
        _ => None,
    }
}

pub fn get_gamepad_movement(gamepad: &Gamepad) -> Vec2 {
    let left_stick = gamepad.left_stick();
    const DEAD_ZONE: f32 = 0.15;
    if left_stick.length() > DEAD_ZONE {
        return left_stick;
    }

    let dpad = gamepad.dpad();
    if dpad.length() > 0.0 {
        return dpad.normalize_or_zero();
    }
    Vec2::ZERO
}
