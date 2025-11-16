use crate::components::{Enemy, MarkedForDespawn, WaveEndedText};
use crate::resources::{SPAWN_RATE, WAVE_DURATION, WaveManager, WaveState};
use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;

/// Manages wave timer and state transitions
pub fn update_wave_timer(
    mut wave_manager: ResMut<WaveManager>,
    enemy_query: Query<Entity, With<Enemy>>,
    pause_text_query: Query<Entity, With<WaveEndedText>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    match wave_manager.wave_state {
        WaveState::Running => {
            // Remove pause text if it exists
            if let Ok(entity) = pause_text_query.single() {
                commands.entity(entity).insert(MarkedForDespawn);
            }

            wave_manager.wave_timer.tick(time.delta());

            if wave_manager.wave_timer.is_finished() {
                wave_manager.wave_state = WaveState::Ended;
            }
        }
        WaveState::Ended => {
            // Clear all enemies
            for entity in &enemy_query {
                commands.entity(entity).insert(MarkedForDespawn);
            }

            // Spawn pause text if it doesn't exist
            if pause_text_query.is_empty() {
                commands
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_child((
                        WaveEndedText,
                        Text::new("Press ENTER or SPACE \nto continue to next Wave"),
                        TextShadow::default(),
                        TextColor(YELLOW.into()),
                        TextFont::default().with_font_size(48.),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
            }
        }
    }
}

/// Handles input to start next wave
pub fn handle_wave_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut wave_manager: ResMut<WaveManager>,
) {
    if wave_manager.wave_state == WaveState::Ended
        && (keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter))
    {
        wave_manager.wave += 1;
        wave_manager.wave_state = WaveState::Running;

        // Reset wave timer
        wave_manager
            .wave_timer
            .set_duration(std::time::Duration::from_secs_f32(WAVE_DURATION));
        wave_manager.wave_timer.reset();

        // Reset enemy spawn timer to base rate
        wave_manager
            .enemy_spawn_timer
            .set_duration(std::time::Duration::from_secs_f32(SPAWN_RATE));
        wave_manager.enemy_spawn_timer.reset();
    }
}
