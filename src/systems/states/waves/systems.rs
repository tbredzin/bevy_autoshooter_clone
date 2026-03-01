use crate::systems::game::GameState;
use crate::systems::game::MarkedForDespawn;
use crate::systems::states::waves::enemy::components::Enemy;
use crate::systems::states::waves::resources::WaveManager;
use bevy::prelude::{Commands, Entity, NextState, Query, Res, ResMut, Time, With};

pub fn on_enter_wave(mut wave_manager: ResMut<WaveManager>) {
    wave_manager.wave_timer.reset();
    wave_manager.enemy_spawn_timer.reset();
}

pub fn on_exit_wave(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in &enemy_query {
        commands.entity(entity).insert(MarkedForDespawn);
    }
}

pub fn update_wave_timer(
    mut wave_manager: ResMut<WaveManager>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    wave_manager.wave_timer.tick(time.delta());
    if wave_manager.wave_timer.just_finished() {
        next_state.set(GameState::UpgradeSelection);
    }
}
