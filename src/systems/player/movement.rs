use crate::resources::GAME_AREA;
use crate::systems::player::components::{Direction, Player, PlayerAction};
use crate::systems::player::resources::PLAYER_SPEED;
use crate::systems::player_upgrades::components::PlayerStats;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Query, Res, Time, Transform, With};

pub fn update_position(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<
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
    for (mut transform, stats, mut current_direction, mut action) in player_query {
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

        if let Some(new_direction) = match (direction.x, direction.y) {
            (0.0, -1.0) => Some(Direction::SOUTH),
            (1.0, -1.0) => Some(Direction::SOUTHEAST),
            (1.0, 0.0) => Some(Direction::EAST),
            (1.0, 1.0) => Some(Direction::NORTHEAST),
            (0.0, 1.0) => Some(Direction::NORTH),
            (-1.0, 1.0) => Some(Direction::NORTHWEST),
            (-1.0, 0.0) => Some(Direction::WEST),
            (-1.0, -1.0) => Some(Direction::SOUTHWEST),
            _ => None,
        } {
            if *current_direction != new_direction {
                *current_direction = new_direction;
            }
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            transform.translation +=
                direction.extend(0.0) * PLAYER_SPEED * stats.speed_multiplier * time.delta_secs();
            if *action != PlayerAction::WALKING {
                *action = PlayerAction::WALKING
            }
        } else {
            if *action != PlayerAction::IDLE {
                *action = PlayerAction::IDLE
            }
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
}
