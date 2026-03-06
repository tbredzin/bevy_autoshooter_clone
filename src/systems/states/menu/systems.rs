use crate::systems::game::{GameState, MarkedForDespawn};
use crate::systems::input::resources::ActionState;
use crate::systems::states::menu::components::{
    AnimatedBorder, DividerSegment, QuitButton, StartButton, TitleWord,
};
use crate::systems::states::menu::renderer::{palette_color, DIVIDER_SEGMENTS};
use crate::systems::states::waves::components::BackgroundMusic;
use bevy::app::AppExit;
use bevy::prelude::*;

const TITLE_CYCLE_SPEED: f32 = 1.4;
const BORDER_CYCLE_SPEED: f32 = 1.8;
const DIVIDER_WAVE_SPEED: f32 = 2.2;
const DIVIDER_WAVE_WIDTH: f32 = 2.5;

pub fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    let audio1 = asset_server.load("musics/intro.ogg".to_string());
    commands.spawn((
        BackgroundMusic,
        AudioPlayer::new(audio1),
        PlaybackSettings::LOOP,
    ));
}

pub fn stop_background_audio(
    mut commands: Commands,
    audio_query: Query<Entity, With<BackgroundMusic>>,
) {
    if let Ok(audio) = audio_query.single() {
        commands.entity(audio).insert(MarkedForDespawn);
    }
}

pub fn handle_menu_input(
    actions: Res<ActionState>,
    start_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    quit_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let start_clicked = start_query.iter().any(|i| *i == Interaction::Pressed);
    let quit_clicked = quit_query.iter().any(|i| *i == Interaction::Pressed);

    if start_clicked || actions.start_next_wave {
        next_state.set(GameState::InWave);
        return;
    }
    if quit_clicked {
        exit.write(AppExit::Success);
    }
}
/// Each TitleWord cycles through the palette at the same speed but a
/// different phase, so the three words always show different hues.
pub fn animate_title_colors(time: Res<Time>, mut query: Query<(&TitleWord, &mut TextColor)>) {
    let t = time.elapsed_secs();
    for (word, mut color) in &mut query {
        let phase = t * TITLE_CYCLE_SPEED + word.phase;
        *color = TextColor(palette_color(phase));
    }
}

/// A bright pulse travels left→right across the divider segments.
/// Each segment's hue also slowly shifts with time.
pub fn animate_divider(time: Res<Time>, mut query: Query<(&DividerSegment, &mut BackgroundColor)>) {
    let t = time.elapsed_secs();
    for (seg, mut bg) in &mut query {
        let seg_phase = (seg.index as f32 / DIVIDER_SEGMENTS as f32) * std::f32::consts::TAU;

        // Traveling wave: position of the bright peak in [0, TAU)
        let wave_pos = (t * DIVIDER_WAVE_SPEED).rem_euclid(std::f32::consts::TAU);

        // Distance from this segment to the wave peak (circular)
        let mut dist = (seg_phase - wave_pos).abs();
        if dist > std::f32::consts::PI {
            dist = std::f32::consts::TAU - dist;
        }

        // Brightness falls off as a cosine from the peak
        let brightness = ((1.0 - (dist / DIVIDER_WAVE_WIDTH).min(1.0)) * std::f32::consts::PI
            / 2.0)
            .cos()
            .abs();
        let alpha = 0.30 + brightness * 0.70;

        // Hue: each segment cycles through its own slice of the palette
        let hue_phase = seg_phase + t * 0.6;
        let color = palette_color(hue_phase).with_alpha(alpha);
        *bg = BackgroundColor(color);
    }
}

/// Both button borders cycle through the palette continuously.
/// On hover the border snaps to full brightness white; returns on None.
pub fn animate_button_borders(
    time: Res<Time>,
    mut query: Query<(
        &AnimatedBorder,
        &Interaction,
        &mut BorderColor,
        &mut BackgroundColor,
    )>,
) {
    let t = time.elapsed_secs();
    for (anim, interaction, mut border, mut bg) in &mut query {
        match interaction {
            Interaction::Hovered => {
                // Snap to white border, brighter background
                *border = BorderColor::all(Color::WHITE);
                *bg = BackgroundColor(Color::srgba(0.22, 0.10, 0.40, 0.95));
            }
            Interaction::Pressed => {
                *border = BorderColor::all(Color::WHITE);
                *bg = BackgroundColor(Color::srgba(0.10, 0.04, 0.20, 0.98));
            }
            Interaction::None => {
                let phase = t * BORDER_CYCLE_SPEED + anim.phase;
                *border = BorderColor::all(palette_color(phase));
                *bg = BackgroundColor(Color::srgba(0.14, 0.06, 0.28, 0.92));
            }
        }
    }
}
