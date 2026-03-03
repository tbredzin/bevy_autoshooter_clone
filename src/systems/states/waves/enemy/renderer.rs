use crate::systems::states::waves::enemy::components::{Enemy, Spawning};
use bevy::asset::Assets;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;

pub fn render_spawning(mut warning_query: Query<(&mut Spawning, &mut Transform)>, time: Res<Time>) {
    for (mut spawning, mut transform) in &mut warning_query {
        spawning.timer.tick(time.delta());

        // Pulsing effect
        let scale = 1.0 + (spawning.timer.elapsed_secs() * 5.0).sin() * 0.2;
        transform.scale = Vec3::splat(scale);
    }
}

pub fn on_enemy_spawning(mut world: DeferredWorld, context: HookContext) {
    let enemy = world.get::<Spawning>(context.entity).unwrap();
    let visual = enemy.kind.visual();
    let mesh_handle = {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        meshes.add(Circle::new(visual.radius * 2.0))
    };
    let material_handle = {
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        materials.add(visual.color.with_alpha(0.35))
    };

    world
        .commands()
        .entity(context.entity)
        .insert((Mesh2d(mesh_handle), MeshMaterial2d(material_handle)));
}

pub fn on_enemy_spawned(mut world: DeferredWorld, context: HookContext) {
    let enemy = world.get::<Enemy>(context.entity).unwrap();
    let visual = enemy.kind.visual();
    let mesh_handle = {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        meshes.add(Circle::new(visual.radius))
    };
    let material_handle = {
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        materials.add(visual.color)
    };

    world
        .commands()
        .entity(context.entity)
        .insert((Mesh2d(mesh_handle), MeshMaterial2d(material_handle)));
}
