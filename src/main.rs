mod systems;

use crate::systems::game::{GameOverStats, GameState};
use crate::systems::hud::resources::HUDTextureAtlas;
use crate::systems::input::plugin::InputPlugin;
use crate::systems::input::resources::{GamepadAsset, KeyboardAsset};
use crate::systems::states::upgrades::resources::{RedrawCardsPool, UpgradeCardsPool};
use crate::systems::states::waves::resources::TilesTextureAtlas;
use crate::systems::states::waves::weapons::resources::{
    ColorMeshes, GeometricMeshes, WeaponsLibrary,
};
use crate::systems::states::{gameover, shopping, waves};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::{UpdateMode, WinitSettings};
use states::upgrades;
use std::time::Duration;
use systems::animations::plugins::PlayerAnimationPlugin;
use systems::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use systems::input::debug;
use systems::states::waves::enemy::messages;
use systems::states::waves::resources::WaveManager;
use systems::states::waves::{camera, collision, enemy, player, weapons};
use systems::*;
use systems::{game, hud};

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
        .init_resource::<UpgradeCardsPool>()
        .init_resource::<RedrawCardsPool>()
        .init_resource::<WaveManager>()
        .init_resource::<TilesTextureAtlas>()
        .init_resource::<HUDTextureAtlas>()
        .init_resource::<GamepadAsset>()
        .init_resource::<KeyboardAsset>()
        .init_resource::<GeometricMeshes>()
        .init_resource::<ColorMeshes>()
        .init_resource::<WeaponsLibrary>()
        .init_resource::<GameOverStats>()
        // ------------------------------------------------------------------------- //
        .add_systems(
            PreUpdate,
            (
                game::out_of_bounds_system,
                game::despawn_marked_entities,
                debug::debug_button_pressed,
                hud::top::update_level_up_indicator,
            ),
        )
        // ------------------------  In Wave state -------------------------------- //
        .add_systems(
            OnEnter(GameState::InWave),
            (
                hud::top::spawn_hud,
                waves::renderer::spawn_background,
                waves::renderer::spawn_entities,
                waves::systems::reset_wave_timers,
            ),
        )
        .add_systems(
            OnExit(GameState::InWave),
            (
                waves::renderer::despawn_background,
                waves::renderer::despawn_entities,
                hud::top::despawn_hud,
            ),
        )
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
                waves::systems::check_game_is_over,
                animations::systems::animate_game_over,
            )
                .run_if(in_state(GameState::InWave)),
        )
        .add_systems(
            PostUpdate,
            (
                hud::top::update,
                hud::stats::toggle_stats_popup,
                hud::stats::update_stats_popup,
                camera::camera_follow_player,
            )
                .run_if(in_state(GameState::InWave)),
        )
        // ------------------------  UpgradeSelection state -------------------------------- //
        .add_systems(
            OnEnter(GameState::UpgradeSelection),
            upgrades::renderer::spawn_upgrades_selection_ui,
        )
        .add_systems(
            OnExit(GameState::UpgradeSelection),
            upgrades::renderer::despawn_upgrades_selection_ui,
        )
        .add_systems(
            Update,
            (
                upgrades::systems::update_active_upgrade_card,
                upgrades::systems::apply_active_upgrade_card,
                upgrades::animations::animate_upgrade_cards,
                upgrades::animations::animate_holding_bars,
                upgrades::renderer::update_card_buttons,
                upgrades::renderer::redraw_upgrades_selection,
                upgrades::renderer::update_card_interaction,
            )
                .run_if(in_state(GameState::UpgradeSelection)),
        )
        // ------------------------  Shopping state -------------------------------- //
        .add_systems(
            OnEnter(GameState::Shopping),
            shopping::renderer::spawn_shopping,
        )
        .add_systems(
            OnExit(GameState::Shopping),
            shopping::renderer::despawn_shopping,
        )
        .add_systems(
            Update,
            (
                shopping::systems::start_next_wave,
                shopping::renderer::update_start_button_interaction,
            )
                .run_if(in_state(GameState::Shopping)),
        )
        // ------------------------  GameOver state --------------------------------- //
        .add_systems(
            OnEnter(GameState::GameOver),
            gameover::renderer::spawn_game_over_ui,
        )
        .add_systems(
            OnExit(GameState::GameOver),
            gameover::renderer::despawn_game_over_ui,
        )
        .add_systems(
            Update,
            (
                gameover::systems::handle_restart,
                gameover::renderer::update_restart_button_interaction,
            )
                .run_if(in_state(GameState::GameOver)),
        )
        .add_message::<messages::EnemyDeathMessage>()
        .run();
}
