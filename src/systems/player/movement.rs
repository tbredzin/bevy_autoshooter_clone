use crate::resources::GAME_AREA;
use crate::systems::input::resources::ActionState;
use crate::systems::player::components::{Direction, Player, PlayerAction};
use crate::systems::player::resources::PLAYER_SPEED;
use crate::systems::player_upgrades::components::PlayerStats;
use bevy::math::Vec2;
use bevy::prelude::{Query, Res, Time, Transform, With};

pub fn update_position(
    actions: Res<ActionState>,
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

    let direction = actions.movement;

    // Update direction enum for animation system
    if let Some(new_direction) = get_direction(direction) {
        if *current_direction != new_direction {
            *current_direction = new_direction;
        }
    }

    // Apply movement
    if direction != Vec2::ZERO {
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

pub fn get_direction(translation: Vec2) -> Option<Direction> {
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
