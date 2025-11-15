use crate::components::Player;
use crate::resources::{GAME_AREA, GameState, PLAYER_SPEED, WAVE_DURATION, WaveState};
use bevy::prelude::*;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) -> Result {
    if game_state.wave_state == WaveState::Ended {
        if keyboard.pressed(KeyCode::Space) || keyboard.pressed(KeyCode::Enter) {
            game_state.wave += 1;
            game_state.wave_state = WaveState::Running;
            game_state.wave_timer = WAVE_DURATION;
        }
        return Ok(());
    }
    if let Ok(mut transform) = player_query.single_mut() {
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

        // clamp to game area
        transform.translation.x = transform
            .translation
            .x
            .clamp(GAME_AREA.min.x, GAME_AREA.max.x);
        transform.translation.y = transform
            .translation
            .y
            .clamp(GAME_AREA.min.y, GAME_AREA.max.y);
    }
    Ok(())
}
