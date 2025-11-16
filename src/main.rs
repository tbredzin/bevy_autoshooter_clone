mod components;
mod messages;
mod resources;
mod systems;

use crate::resources::{WINDOW_HEIGHT, WINDOW_WIDTH, WaveManager};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::{UpdateMode, WinitSettings};
use std::time::Duration;
use systems::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Auto Shooter".to_string(),
                        resolution: WindowResolution::new(
                            WINDOW_WIDTH as u32,
                            WINDOW_HEIGHT as u32,
                        ),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // Remove texture bleeding/seam
        )
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
        .init_resource::<WaveManager>()
        .add_systems(Startup, (setup::setup, setup::setup_background).chain())
        // Logic
        .add_systems(
            Update,
            (
                player::movement::update_position,
                player::experience::handle_enemy_death,
                wave::update_wave_timer,
                wave::handle_wave_input,
                game::out_of_bounds_system,
                enemy::engine::update_spawning,
                enemy::engine::update_spawned,
                enemy::engine::update_move,
                enemy::engine::check_if_dead,
                combat::auto_shoot,
                combat::move_bullets,
                collision::check_bullet_enemy_collision,
                collision::check_player_enemy_collision,
            ),
        )
        // Rendering
        .add_systems(
            PostUpdate,
            (
                enemy::renderer::render_spawning,
                ui::update_ui,
                camera::camera_follow_player,
            ),
        )
        .add_message::<messages::EnemyDeathMessage>()
        .run();
}
