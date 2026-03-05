use crate::systems::states::waves::weapons::components::WeaponKind::{MachineGun, Pistol, Shotgun};
use crate::systems::states::waves::weapons::components::{Bullet, Weapon};
use crate::systems::states::waves::weapons::resources::{ColorMeshes, GeometricMeshes};
use bevy::color::palettes::basic::{AQUA, RED};
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::mesh::Mesh2d;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub fn draw_bullet(mut world: DeferredWorld, context: HookContext) {
    // Create mesh and material
    let Some(bullet) = world.get::<Bullet>(context.entity) else {
        return;
    };
    let geo_meshes = world.resource::<GeometricMeshes>();

    let (mesh, material2d) = {
        match bullet.kind {
            Shotgun => (
                geo_meshes.square_large.clone(),
                ColorMeshes::get_color(world.resource_mut::<Assets<ColorMaterial>>().as_mut(), RED),
            ),
            Pistol => (
                geo_meshes.circle_medium.clone(),
                ColorMeshes::get_color(world.resource_mut::<Assets<ColorMaterial>>().as_mut(), RED),
            ),
            MachineGun => (
                geo_meshes.circle_small.clone(),
                ColorMeshes::get_color(
                    world.resource_mut::<Assets<ColorMaterial>>().as_mut(),
                    AQUA,
                ),
            ),
        }
    };

    world
        .commands()
        .entity(context.entity)
        .insert((Mesh2d(mesh), MeshMaterial2d(material2d)));
}

enum WeaponVisual {
    Mesh(Handle<Mesh>, Handle<ColorMaterial>),
    Custom(Box<dyn FnOnce(&mut EntityCommands)>),
}
pub fn draw_weapon(mut world: DeferredWorld, context: HookContext) {
    let visual = {
        let weapon_kind = world.get::<Weapon>(context.entity).unwrap().kind;
        let image = world.resource::<AssetServer>().load("sprites/wand.png");

        match weapon_kind {
            MachineGun => WeaponVisual::Custom(Box::new(|cmd| {
                let child = cmd
                    .commands()
                    .spawn((
                        Sprite::from_image(image),
                        Transform::from_rotation(Quat::from_rotation_z(3.0 * FRAC_PI_2))
                            .with_scale(Vec3::splat(0.75)),
                    ))
                    .id();
                cmd.add_child(child);
            })),
            Shotgun => {
                let geos = world.resource::<GeometricMeshes>();
                let colors = world.resource::<ColorMeshes>();
                WeaponVisual::Mesh(geos.rectangle_large.clone(), colors.red.clone())
            }
            Pistol => {
                let geos = world.resource::<GeometricMeshes>();
                let colors = world.resource::<ColorMeshes>();
                WeaponVisual::Mesh(geos.rectangle_small.clone(), colors.black.clone())
            }
        }
    };

    match visual {
        WeaponVisual::Mesh(mesh, material) => {
            world
                .commands()
                .entity(context.entity)
                .insert((Mesh2d(mesh), MeshMaterial2d(material)));
        }
        WeaponVisual::Custom(f) => {
            f(&mut world.commands().entity(context.entity));
        }
    }
}
