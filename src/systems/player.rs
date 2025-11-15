use crate::components::{Health, Player, PlayerExperience};
use crate::messages::EnemyDeathMessage;
use crate::resources::{GAME_AREA, PLAYER_SPEED, WaveManager, WaveState};
use bevy::prelude::*;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    wave_manager: Res<WaveManager>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if wave_manager.wave_state == WaveState::Ended {
        return;
    }

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

pub fn handle_enemy_death(
    mut msg_reader: MessageReader<EnemyDeathMessage>,
    mut player_query: Query<(&mut PlayerExperience, &mut Health), With<Player>>,
) {
    for event in msg_reader.read() {
        // Handle the enemy death, e.g., update score or play sound
        println!("Enemy {:?} has died!", event.0);
        let Ok((experience, health)) = &mut player_query.single_mut() else {
            return;
        };
        experience.value += 1;
        // Level up check
        if experience.value >= experience.level * 10 {
            experience.level += 1;
            health.max += 10.0;
        }
    }
}
