use crate::components::{Enemy, WaveEndedText};
use crate::resources::{GAME_AREA, GameState, WaveState};
use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;

pub fn update_game_state(
    mut game_state: ResMut<GameState>,
    mut enemy_query: Query<Entity, With<Enemy>>,
    pause_text: Query<Entity, With<WaveEndedText>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    match game_state.wave_state {
        WaveState::Running => {
            if let Ok(entity) = pause_text.single() {
                commands.entity(entity).despawn();
            }

            game_state.wave_timer -= time.delta_secs();
            if game_state.wave_timer <= 0.0 {
                game_state.wave_state = WaveState::Ended;
            }
        }
        WaveState::Ended => {
            // Clear enemies
            for entity in &mut enemy_query {
                commands.entity(entity).despawn();
            }
            if pause_text.is_empty() {
                commands
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center, // horizontal center
                            align_items: AlignItems::Center,         // vertical center
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
                    )); //TODO: is there a better way to align text in the middle ?
            }
        }
    }
}

pub fn out_of_bounds_system(mut commands: Commands, query: Query<(Entity, &Transform)>) -> Result {
    for (entity, transform) in query.iter() {
        let entity_pos = transform.translation.xy();
        if entity_pos.x < GAME_AREA.min.x
            || entity_pos.x > GAME_AREA.max.x
            || entity_pos.y < GAME_AREA.min.y
            || entity_pos.y > GAME_AREA.max.y
        {
            commands.entity(entity).despawn()
        }
    }
    Ok(())
}
