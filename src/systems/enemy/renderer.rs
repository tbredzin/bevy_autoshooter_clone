use crate::components::Spawning;
use crate::resources::{GameState, WaveState};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;

pub fn render_spawning(
    mut warning_query: Query<(&mut Spawning, &mut Transform)>,
    game_state: ResMut<GameState>,
    time: Res<Time>,
) -> Result {
    if game_state.wave_state == WaveState::Ended {
        return Ok(());
    }
    for (mut pre_spawn, mut transform) in &mut warning_query {
        pre_spawn.timer -= time.delta_secs();

        // Pulsing effect
        let scale = 1.0 + (pre_spawn.timer * 5.0).sin() * 0.2;
        transform.scale = Vec3::splat(scale);
    }
    Ok(())
}

pub fn on_enemy_spawning(mut world: DeferredWorld, context: HookContext) {
    // Create mesh and material
    let mesh_handle = {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        meshes.add(Circle::new(30.0))
    };

    let material_handle = {
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        materials.add(Color::srgba(1.0, 0.0, 0.0, 0.3))
    };

    world
        .commands()
        .entity(context.entity)
        .insert((Mesh2d(mesh_handle), MeshMaterial2d(material_handle)));
}

pub fn on_enemy_spawned(mut world: DeferredWorld, context: HookContext) {
    // Create mesh and material
    let mesh_handle = {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        meshes.add(Circle::new(15.0))
    };

    let material_handle = {
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        materials.add(Color::srgb(1.0, 0.3, 0.3))
    };

    world
        .commands()
        .entity(context.entity)
        .insert((Mesh2d(mesh_handle), MeshMaterial2d(material_handle)));
}
