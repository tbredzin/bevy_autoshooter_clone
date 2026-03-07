use crate::systems::constants::BULLET_SPEED;
use crate::systems::game::GameState;
use crate::systems::input::resources::ActionState;
use crate::systems::states::waves::components::Dying;
use crate::systems::states::waves::enemy::components::Enemy;
use crate::systems::states::waves::player::components::Player;
use crate::systems::states::waves::weapons::components::{
    Bullet, Weapon, WeaponArea, WeaponBundle, WeaponCooldown,
};
use crate::systems::states::waves::weapons::messages::{
    BulletSpawnedMessage, WeaponSpawnedMessage,
};
use crate::systems::states::waves::weapons::resources::WeaponsLibrary;
use crate::systems::states::waves::weapons::utils;
use bevy::math::{Quat, Vec3};
use bevy::prelude::*;
use std::f32::consts;

pub fn add_weapon(
    mut commands: Commands,
    mut actions: ResMut<ActionState>,
    player: Single<(Entity, &Children), With<Player>>,
    weapons_query: Query<&Weapon>,
    weapons_resource: Res<WeaponsLibrary>,
    mut events: MessageWriter<WeaponSpawnedMessage>,
) {
    if !actions.add_weapon {
        return;
    }
    actions.add_weapon = false;

    let index = rand::random_range(0..weapons_resource.weapons.len());
    let weapon = weapons_resource.weapons.get(index).unwrap();
    let (entity, children) = player.into_inner();
    let weapons_count = weapons_query.iter_many(children).count();
    let weapon_bundle = WeaponBundle::new(
        format!("{:?}-{}", weapon.kind, weapons_count),
        weapon.clone(),
        weapon.base_cooldown,
    );
    println!("adding new weapon {:?}", weapon_bundle.name);

    let weapon_entity = commands.spawn(weapon_bundle.clone()).id();
    commands.entity(entity).add_child(weapon_entity);
    events.write(WeaponSpawnedMessage {
        name: weapon_bundle.name,
        weapon: weapon_bundle.weapon,
        entity: weapon_entity,
        player: entity,
    });
}

/// Smoothly moves and rotates weapons within their designated sectors to aim at nearest enemy
pub fn update_weapon_positioning(
    mut weapon_query: Query<(&mut Transform, &Weapon, &WeaponArea)>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Dying>)>,
    time: Res<Time>,
) {
    // Get player location
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation();

    for (mut weapon_transform, weapon, weapon_area) in &mut weapon_query {
        // Find nearest enemy within weapon range
        let nearest_enemy = utils::get_nearest_enemy(
            player_transform,
            enemy_query.iter().collect(),
            weapon.base_range * weapon.range_multiplier,
        );

        let Some(enemy_pos) = nearest_enemy else {
            continue;
        };

        // Calculate direction to enemy relative to player
        let player_to_enemy = (enemy_pos - player_pos).truncate();
        let enemy_angle = player_to_enemy.y.atan2(player_to_enemy.x);

        // Clamp angle to weapon's allowed sector
        let (min_angle, max_angle) = weapon_area.angle_range();
        let target_angle = utils::clamp_angle_to_range(enemy_angle, min_angle, max_angle);

        // Calculate target position within the sector
        let target_x = target_angle.cos() * weapon_area.orbit_radius;
        let target_y = target_angle.sin() * weapon_area.orbit_radius;
        let new_weapon_translation = Vec3::new(target_x, target_y, weapon_transform.translation.z);

        // Smoothly interpolate to target position
        let smoothing = 8.0;
        weapon_transform.translation = weapon_transform
            .translation
            .lerp(new_weapon_translation, time.delta_secs() * smoothing);

        // Rotate weapon to face enemy
        let weapon_to_enemy = player_to_enemy - new_weapon_translation.truncate();
        let target_rotation = Quat::from_rotation_z(weapon_to_enemy.y.atan2(weapon_to_enemy.x));

        weapon_transform.rotation = weapon_transform
            .rotation
            .slerp(target_rotation, time.delta_secs() * smoothing);
    }
}

pub fn move_bullets(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullet_query {
        transform.translation += bullet.direction.extend(0.0) * BULLET_SPEED * time.delta_secs();
    }
}

pub fn auto_shoot(
    mut commands: Commands,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Dying>)>,
    weapons_query: Query<(&GlobalTransform, &mut Weapon, &mut WeaponCooldown)>,
    enemy_query: Query<&GlobalTransform, (With<Enemy>, Without<Player>)>,
    mut events: MessageWriter<BulletSpawnedMessage>,
    time: Res<Time>,
) {
    if player_query.is_empty() {
        return;
    }
    for (weapon_transform, weapon, mut cooldown) in weapons_query {
        cooldown.timer.tick(time.delta());

        if !cooldown.timer.is_finished() {
            continue; // still on cooldown -> continue
        }
        let weapon_pos = weapon_transform.translation().truncate();

        let Some(nearest_enemy) = utils::get_nearest_enemy(
            weapon_transform,
            enemy_query.iter().collect(),
            weapon.base_range * weapon.range_multiplier,
        ) else {
            continue;
        };

        // Compute direction to enemy
        let direction = (nearest_enemy.truncate() - weapon_pos).normalize();
        let spawn_offset = direction * 10.0; // push bullet forward by 20px

        // Spawn a new bullet toward that direction
        let transform = Transform::from_translation(weapon_pos.extend(1.0))
            .with_translation(Vec3::new(
                weapon_pos.x + spawn_offset.x,
                weapon_pos.y + spawn_offset.y,
                1.0,
            ))
            .with_scale(weapon.bullet_size.extend(1.0));
        let bullet = Bullet {
            direction,
            damage: weapon.base_damage * weapon.damage_multiplier,
            kind: weapon.kind,
        };
        let entity = commands
            .spawn((transform, bullet.clone(), DespawnOnExit(GameState::InWave)))
            .id();

        events.write(BulletSpawnedMessage {
            entity,
            bullet,
            transform,
        });

        // reset cooldown
        cooldown.timer.reset();
    }
}

pub fn recalculate_weapon_area(
    mut commands: Commands,
    player_query: Query<&Children, With<Player>>,
    weapon_query: Query<(Entity, &Name), With<Weapon>>,
    mut events: MessageReader<WeaponSpawnedMessage>,
) {
    for event in events.read() {
        let children = player_query.get(event.player).unwrap();
        let total_weapons = weapon_query.iter_many(children).count();
        let sector_arc = consts::TAU / (total_weapons as f32) * 0.8; // 80% of full sector,

        // For each weapons of player
        for (index, (entity, name)) in weapon_query.iter_many(children).enumerate() {
            println!("Recalculating weapon area of weapon {:?}", name);
            let orbit_radius = (10.0 * total_weapons as f32).max(20.0); // Distance from player center
            let center_arc = consts::TAU * (index as f32) / (total_weapons as f32);
            let angle = consts::TAU * (index as f32) / (total_weapons as f32);

            commands.entity(entity).insert((
                Transform::from_xyz(angle.cos() * orbit_radius, angle.sin() * orbit_radius, 0.0),
                WeaponArea {
                    orbit_radius,
                    center_arc,
                    sector_arc,
                },
            ));
        }
    }
}
