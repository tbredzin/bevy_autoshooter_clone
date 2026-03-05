use crate::systems::animations::animation::SpriteAnimation;
use crate::systems::animations::animator::SpriteAnimator;
use crate::systems::states::waves::components::Direction::EAST;
use crate::systems::states::waves::enemy::components::{Enemy, Spawning};
use crate::systems::states::waves::enemy::resources::EnemyAnimations;
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
    let kind = { world.get::<Enemy>(context.entity).map(|e| e.kind).unwrap() };
    let shadow = { world.resource::<EnemyAnimations>().shadow_texture.clone() };
    let handle = {
        world
            .resource::<EnemyAnimations>()
            .get(kind, EAST)
            .expect(format!("{:?}/EAST must be registered", kind).as_str())
    };
    let animation = world
        .resource::<Assets<SpriteAnimation>>()
        .get(handle.id())
        .cloned()
        .unwrap();

    world.commands().entity(context.entity).remove::<Mesh2d>();
    world
        .commands()
        .entity(context.entity)
        .remove::<MeshMaterial2d<ColorMaterial>>();

    let mut transform = { world.get_mut::<Transform>(context.entity).unwrap() };
    *transform = transform.with_scale(Vec3::splat(2.0));

    let mut shadow_image = Sprite::from_image(shadow);
    shadow_image.custom_size = animation.to_sprite().custom_size;
    world.commands().entity(context.entity).insert((
        animation.to_sprite(),
        SpriteAnimator::new(handle),
        children![(shadow_image, Transform::from_xyz(-1.0, -6.0, 0.0),)],
    ));
}
