use crate::systems::game::GameState;
use crate::systems::states::menu::renderer::{despawn_main_menu, spawn_main_menu};
use crate::systems::states::menu::systems::{
    animate_button_borders, animate_divider, animate_title_colors, handle_menu_input,
    play_background_audio, stop_background_audio,
};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (spawn_main_menu, play_background_audio),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (despawn_main_menu, stop_background_audio),
        )
        .add_systems(
            Update,
            (
                handle_menu_input,
                animate_title_colors,
                animate_divider,
                animate_button_borders,
            )
                .run_if(in_state(GameState::MainMenu)),
        );
    }
}
