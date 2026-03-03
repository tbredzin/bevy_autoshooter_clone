use crate::systems::states::waves::enemy::components::{
    BossAttack, BossPhase, Enemy, Hostile, RangedAttack,
};
use crate::systems::states::waves::player::components::Player;
use crate::systems::states::waves::weapons::components::{Bullet, WeaponKind};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{
    Circle, ColorMaterial, Commands, GlobalTransform, MeshMaterial2d, Query, Res, ResMut, Time,
    Timer, TimerMode, Transform, With, Without,
};

pub fn update_enemy_shoot(
    mut commands: Commands,
    mut attacker_query: Query<(&GlobalTransform, &Enemy, &mut RangedAttack)>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation();

    for (transform, enemy, mut ranged) in &mut attacker_query {
        let enemy_pos = transform.translation();
        let to_player = player_pos - enemy_pos;

        if to_player.length() > ranged.preferred_distance * 1.5 {
            continue;
        }

        ranged.timer.tick(time.delta());
        if !ranged.timer.just_finished() {
            continue;
        }

        let direction = to_player.truncate().normalize_or_zero();
        let proj_color = enemy.kind.visual().color;

        commands.spawn((
            Transform::from_translation(enemy_pos),
            Bullet {
                kind: WeaponKind::Pistol,
                direction,
                speed: ranged.projectile_speed,
                damage: ranged.projectile_damage,
            },
            Hostile,
            Mesh2d(meshes.add(Circle::new(5.0))),
            MeshMaterial2d(materials.add(proj_color)),
        ));
    }
}

pub fn update_boss_shoot(
    mut commands: Commands,
    mut boss_query: Query<(&mut Transform, &Enemy, &mut BossAttack)>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation();

    for (mut transform, enemy, mut boss) in &mut boss_query {
        let to_player = (player_pos - transform.translation).truncate();
        boss.phase_timer.tick(time.delta());

        match boss.phase {
            // ── Chasing: slow approach; after timer, lock a charge direction
            BossPhase::Chasing => {
                let dir = to_player.normalize_or_zero();
                transform.translation += dir.extend(0.0) * enemy.speed * time.delta_secs();

                if boss.phase_timer.just_finished() {
                    boss.charge_direction = to_player.normalize_or_zero();
                    boss.phase = BossPhase::Charging;
                    boss.phase_timer = Timer::from_seconds(1.2, TimerMode::Once);
                }
            }

            // ── Charging: rush forward; fire 8-way spread on completion
            BossPhase::Charging => {
                transform.translation +=
                    boss.charge_direction.extend(0.0) * enemy.speed * 4.5 * time.delta_secs();

                if boss.phase_timer.just_finished() {
                    // Fire a radial spread
                    let boss_pos = transform.translation;
                    let proj_color = Color::srgb(0.95, 0.2, 0.95);
                    for i in 0..8u32 {
                        let angle = (i as f32 / 8.0) * std::f32::consts::TAU;
                        let dir = Vec2::new(angle.cos(), angle.sin());
                        commands.spawn((
                            Transform::from_translation(boss_pos),
                            Bullet {
                                kind: WeaponKind::Shotgun,
                                direction: dir,
                                speed: 260.0,
                                damage: enemy.damage * 0.6,
                            },
                            Hostile,
                            Mesh2d(meshes.add(Circle::new(7.0))),
                            MeshMaterial2d(materials.add(proj_color)),
                        ));
                    }

                    boss.phase = BossPhase::Cooldown;
                    boss.phase_timer = Timer::from_seconds(3.0, TimerMode::Once);
                }
            }

            // ── Cooldown: slow drift toward player; then resume chasing
            BossPhase::Cooldown => {
                let dir = to_player.normalize_or_zero();
                transform.translation += dir.extend(0.0) * enemy.speed * 0.4 * time.delta_secs();

                if boss.phase_timer.just_finished() {
                    boss.phase = BossPhase::Chasing;
                    boss.phase_timer = Timer::from_seconds(4.0, TimerMode::Once);
                }
            }
        }
    }
}
