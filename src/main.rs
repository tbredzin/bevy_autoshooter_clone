mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy::dev_tools::fps_overlay::{FpsOverlayPlugin, FpsOverlayConfig};
use bevy::winit::{WinitSettings, UpdateMode};
use std::time::Duration;
use resources::GameState;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 20.0,
                    ..default()
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                enabled: true,
                refresh_interval: Default::default(),
                frame_time_graph_config: Default::default(),
            },
        })
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::reactive(Duration::from_secs_f32(1.0 / 60.0)),
            unfocused_mode: UpdateMode::reactive(Duration::from_secs_f32(1.0 / 60.0)),
        })
        .init_resource::<GameState>()
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (
            player::player_movement,
            combat::auto_shoot,
            combat::move_bullets,
            enemy::spawn_enemies,
            enemy::move_enemies,
            collision::check_bullet_enemy_collision,
            collision::check_player_enemy_collision,
            game::update_wave_timer,
            game::cleanup_offscreen,
            ui::update_ui,
        ))
        .run();
}