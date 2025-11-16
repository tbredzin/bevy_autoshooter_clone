use crate::components::WeaponKind::{MachineGun, Pistol, Shotgun};
use crate::components::{Bullet, Weapon};
use crate::resources::{ColorMeshes, GeometricMeshes};
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::mesh::Mesh2d;
use bevy::prelude::MeshMaterial2d;

pub fn draw_bullet(mut world: DeferredWorld, context: HookContext) {
    // Create mesh and material
    let Some(bullet) = world.get::<Bullet>(context.entity) else {
        return;
    };
    let geo_meshes = world.resource::<GeometricMeshes>();
    let color_meshes = world.resource::<ColorMeshes>();
    let (mesh, material2d) = {
        match bullet.kind {
            Shotgun => (geo_meshes.square_large.clone(), color_meshes.red.clone()),
            Pistol => (geo_meshes.circle_medium.clone(), color_meshes.black.clone()),
            MachineGun => (geo_meshes.circle_small.clone(), color_meshes.pink.clone()),
        }
    };

    world
        .commands()
        .entity(context.entity)
        .insert((Mesh2d(mesh), MeshMaterial2d(material2d)));
}

pub fn draw_weapon(mut world: DeferredWorld, context: HookContext) {
    // Create mesh and material
    let Some(weapon) = world.get::<Weapon>(context.entity) else {
        return;
    };
    let geo_meshes = world.resource::<GeometricMeshes>();
    let color_meshes = world.resource::<ColorMeshes>();
    let (mesh, material2d) = {
        match weapon.kind {
            Shotgun => (geo_meshes.rectangle_large.clone(), color_meshes.red.clone()),
            Pistol => (
                geo_meshes.rectangle_small.clone(),
                color_meshes.black.clone(),
            ),
            MachineGun => (
                geo_meshes.rectangle_medium.clone(),
                color_meshes.pink.clone(),
            ),
        }
    };

    world
        .commands()
        .entity(context.entity)
        .insert((Mesh2d(mesh), MeshMaterial2d(material2d)));
}
