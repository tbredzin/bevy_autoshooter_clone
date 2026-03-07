use crate::systems::animations::animation::SpriteAnimation;
use crate::systems::animations::animator::SpriteAnimator;
use crate::systems::states::waves::components::Direction::EAST;
use crate::systems::states::waves::enemy::components::Spawning;
use crate::systems::states::waves::enemy::messages::{EnemySpawnedMessage, EnemySpawningMessage};
use crate::systems::states::waves::enemy::resources::EnemyAnimations;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;

pub fn update_spawning(mut warning_query: Query<(&mut Spawning, &mut Transform)>, time: Res<Time>) {
    for (mut spawning, mut transform) in &mut warning_query {
        spawning.timer.tick(time.delta());

        // Pulsing effect
        let scale = 1.0 + (spawning.timer.elapsed_secs() * 5.0).sin() * 0.2;
        transform.scale = Vec3::splat(scale);
    }
}

pub fn handle_enemy_spawning(
    mut commands: Commands,
    mut events: MessageReader<EnemySpawningMessage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for spawning in events.read() {
        let visual = spawning.kind.visual();
        let mesh_handle = meshes.add(Circle::new(visual.radius * 2.0));
        let material_handle = materials.add(visual.color.with_alpha(0.35));
        commands
            .entity(spawning.entity)
            .insert((Mesh2d(mesh_handle), MeshMaterial2d(material_handle)));
    }
}

pub fn handle_enemy_spawned(
    mut commands: Commands,
    anims: Res<EnemyAnimations>,
    sprites: Res<Assets<SpriteAnimation>>,
    mut events: MessageReader<EnemySpawnedMessage>,
) {
    for enemy in events.read() {
        let kind = enemy.kind;
        let shadow = anims.shadow_texture.clone();
        let handle = anims
            .get(kind, EAST)
            .expect(format!("{:?}/EAST must be registered", kind).as_str());
        let animation = sprites.get(handle.id()).cloned().unwrap();

        commands.entity(enemy.entity).remove::<Mesh2d>();
        commands
            .entity(enemy.entity)
            .remove::<MeshMaterial2d<ColorMaterial>>();
        let transform = enemy.transform.with_scale(Vec3::splat(2.0));
        let mut shadow_image = Sprite::from_image(shadow);
        shadow_image.custom_size = animation.to_sprite().custom_size;

        commands.entity(enemy.entity).insert((
            transform,
            animation.to_sprite(),
            SpriteAnimator::new(handle),
            children![(shadow_image, Transform::from_xyz(-1.0, -6.0, 0.0),)],
        ));
    }
}
