mod components;
mod messages;
mod resources;
mod systems;

use crate::resources::{WaveManager, WaveState, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::systems::player_animations::plugins::PlayerAnimationPlugin;
use crate::systems::player_upgrades::resources::UpgradePool;
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
        .add_plugins(PlayerAnimationPlugin)
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::reactive(Duration::from_secs_f32(1.0 / 60.0)),
            unfocused_mode: UpdateMode::reactive(Duration::from_secs_f32(1.0 / 60.0)),
        })
        .init_resource::<WaveManager>()
        .init_resource::<UpgradePool>()
        .add_systems(
            Startup,
            (
                setup::init_resources,
                weapons::resources::init,
                setup::spawn_entities,
                setup::spawn_background,
            )
                .chain(),
        )
        // Logic
        .add_systems(
            PreUpdate,
            (game::out_of_bounds_system, game::despawn_marked_entities),
        )
        .add_systems(
            Update,
            (
                wave::update_wave_timer,
                (
                    player::movement::update_position,
                    player::experience::handle_enemy_death,
                    enemy::engine::update_spawning,
                    enemy::engine::update_spawned,
                    enemy::engine::update_move,
                    enemy::engine::check_if_dead,
                    weapons::systems::update_weapon_positioning,
                    combat::auto_shoot,
                    collision::check_bullet_enemy_collision,
                    collision::check_player_enemy_collision,
                    weapons::systems::move_bullets,
                )
                    .run_if(|wave: Res<WaveManager>| wave.wave_state == WaveState::Running),
                (
                    player_upgrades::renderer::show_upgrade_ui,
                    player_upgrades::systems::handle_upgrade_selection,
                    player_upgrades::systems::handle_next_wave_button,
                )
                    .run_if(|wave: Res<WaveManager>| wave.wave_state == WaveState::Ended),
            ),
        )
        // Rendering
        .add_systems(
            PostUpdate,
            (
                enemy::renderer::render_spawning,
                hud::update_ui,
                hud::show_stats_display,
                hud::update_stats_display,
                hud::show_level_ups
                    .run_if(|wave: Res<WaveManager>| wave.wave_state == WaveState::Running),
                hud::clear_level_ups
                    .run_if(|wave: Res<WaveManager>| wave.wave_state == WaveState::Ended),
                camera::camera_follow_player,
            ),
        )
        .add_message::<messages::EnemyDeathMessage>()
        .run();
}
