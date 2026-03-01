mod messages;
mod resources;
mod systems;

use crate::resources::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::systems::game::GameState;
use crate::systems::input::plugin::InputPlugin;
use crate::systems::states::upgrades::resources::UpgradePool;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::{UpdateMode, WinitSettings};
use states::upgrades;
use std::time::Duration;
use systems::animations::plugins::PlayerAnimationPlugin;
use systems::game;
use systems::input::debug;
use systems::states::waves::resources::WaveManager;
use systems::states::waves::{camera, collision, enemy, player, weapons};
use systems::states::{shopping, waves};
use systems::*;

fn main() {
    App::new()
        // ----------------------------- Plugins ---------------------------------- //
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
        .add_plugins((PlayerAnimationPlugin, InputPlugin))
        // ----------------------------- Resources ---------------------------------- //
        .init_state::<GameState>()
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::reactive(Duration::from_secs_f32(1.0 / 60.0)),
            unfocused_mode: UpdateMode::reactive(Duration::from_secs_f32(1.0 / 60.0)),
        })
        .init_resource::<UpgradePool>()
        .init_resource::<WaveManager>()
        // ------------------------------------------------------------------------- //
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
        .add_systems(
            PreUpdate,
            (
                game::out_of_bounds_system,
                game::despawn_marked_entities,
                debug::display_button_pressed,
                hud::systems::update_level_up_indicator,
            ),
        )
        // ------------------------  In Wave state -------------------------------- //
        .add_systems(OnEnter(GameState::InWave), (waves::systems::on_enter_wave,))
        .add_systems(OnExit(GameState::InWave), waves::systems::on_exit_wave)
        .add_systems(
            Update,
            (
                waves::systems::update_wave_timer,
                player::movement::update_position,
                player::experience::handle_enemy_death,
                enemy::engine::update_spawning,
                enemy::engine::update_spawned,
                enemy::engine::update_move,
                enemy::engine::check_if_dead,
                weapons::systems::update_weapon_positioning,
                weapons::systems::auto_shoot,
                collision::check_bullet_enemy_collision,
                collision::check_player_enemy_collision,
                weapons::systems::move_bullets,
                enemy::renderer::render_spawning,
            )
                .run_if(in_state(GameState::InWave)),
        )
        .add_systems(
            PostUpdate,
            (
                hud::systems::update,
                hud::systems::show_stats_display,
                hud::systems::update_stats_display,
                camera::camera_follow_player,
            )
                .run_if(in_state(GameState::InWave)),
        )
        // ------------------------  UpgradeSelection state -------------------------------- //
        .add_systems(
            OnEnter(GameState::UpgradeSelection),
            upgrades::renderer::on_enter_upgrade_mode,
        )
        .add_systems(
            OnExit(GameState::UpgradeSelection),
            upgrades::renderer::on_exit_upgrade_mode,
        )
        .add_systems(
            Update,
            (
                upgrades::systems::handle_update_selection,
                hud::systems::update_stats_display,
                upgrades::systems::apply_upgrade,
                upgrades::renderer::animate_card_selection,
            )
                .run_if(in_state(GameState::UpgradeSelection)),
        )
        // ------------------------  Shopping state -------------------------------- //
        .add_systems(
            OnEnter(GameState::Shopping),
            shopping::renderer::on_enter_shopping_mode,
        )
        .add_systems(
            OnExit(GameState::Shopping),
            shopping::renderer::on_exit_shopping_mode,
        )
        .add_systems(
            Update,
            shopping::systems::start_next_wave.run_if(in_state(GameState::Shopping)),
        )
        // ------------------------------------------------------------------------- //
        .add_message::<messages::EnemyDeathMessage>()
        .run();
}
