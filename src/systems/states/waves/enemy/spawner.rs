use crate::systems::constants::{tiles_to_pixels, ENEMY_SPAWN_TIME_IN_S, GAME_AREA, SPAWN_RATE};
use crate::systems::game::GameState;
use crate::systems::states::waves::components::{Direction, Dying, Health};
use crate::systems::states::waves::enemy::components::{
    BossAttack, Enemy, RangedAttack, Spawning, Splitter,
};
use crate::systems::states::waves::enemy::kinds::EnemyKind;
use crate::systems::states::waves::enemy::messages::{EnemySpawnedMessage, EnemySpawningMessage};
use crate::systems::states::waves::player::components::Player;
use crate::systems::states::waves::resources::WaveManager;
use bevy::prelude::*;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::RngExt;

pub fn create_enemy_spawning(
    mut commands: Commands,
    mut wave_manager: ResMut<WaveManager>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Dying>)>,
    mut events: MessageWriter<EnemySpawningMessage>,
    time: Res<Time>,
) {
    wave_manager.enemy_spawn_timer.tick(time.delta());
    if !wave_manager.enemy_spawn_timer.is_finished() {
        return;
    }
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let wave = wave_manager.wave;
    let kind = EnemyKind::random_for_wave(wave);
    let spawn_pos = generate_spawn_position(player_transform.translation().truncate());

    let entity = commands
        .spawn((
            Transform::from_translation(spawn_pos.extend(0.0)),
            Spawning {
                timer: Timer::from_seconds(ENEMY_SPAWN_TIME_IN_S, TimerMode::Once),
                kind,
            },
            DespawnOnExit(GameState::InWave),
        ))
        .id();
    events.write(EnemySpawningMessage { entity, kind });

    let base_rate = SPAWN_RATE / wave as f32;
    wave_manager
        .enemy_spawn_timer
        .set_duration(std::time::Duration::from_secs_f32(base_rate.max(0.01)));
}

pub fn spawn_enemies(
    mut commands: Commands,
    mut pre_spawn_query: Query<(Entity, &mut Spawning, &Transform)>,
    mut events: MessageWriter<EnemySpawnedMessage>,
    time: Res<Time>,
    wave_manager: Res<WaveManager>,
) {
    for (entity, mut spawning, transform) in &mut pre_spawn_query {
        spawning.timer.tick(time.delta());
        if !spawning.timer.is_finished() {
            continue;
        }
        let transform = *transform;
        let kind = spawning.kind;
        let wave = wave_manager.wave;
        let stats = kind.stats(wave);

        let mut entity_cmd = commands.entity(entity);
        entity_cmd.remove::<Spawning>();
        entity_cmd.insert((
            Enemy {
                damage: stats.contact_damage,
                speed: stats.speed,
                kind,
                xp_reward: stats.xp_reward,
            },
            Direction::EAST,
            Health {
                value: stats.health,
            },
        ));

        match kind {
            EnemyKind::Splitter => {
                entity_cmd.insert(Splitter { split_count: 4 });
            }
            EnemyKind::Ranged => {
                entity_cmd.insert(RangedAttack {
                    timer: Timer::from_seconds(2.5, TimerMode::Repeating),
                    preferred_distance: 300.0,
                    projectile_damage: stats.contact_damage * 2.5,
                });
            }
            _ => {}
        }
        events.write(EnemySpawnedMessage {
            entity,
            kind,
            transform,
        });
    }
}
pub fn spawn_boss(
    mut commands: Commands,
    wave_manager: Res<WaveManager>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Dying>)>,
    mut events: MessageWriter<EnemySpawnedMessage>,
) {
    let wave = wave_manager.wave;

    let kind = if wave % 5 == 0 {
        EnemyKind::Boss
    } else if wave % 3 == 0 {
        EnemyKind::MiniBoss
    } else {
        return;
    };

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let spawn_pos = generate_spawn_position(player_transform.translation().truncate());
    let stats = kind.stats(wave);

    let transform = Transform::from_translation(spawn_pos.extend(0.0));
    let mut entity_cmd = commands.spawn((
        transform,
        Direction::EAST,
        Enemy {
            damage: stats.contact_damage,
            speed: stats.speed,
            kind,
            xp_reward: stats.xp_reward,
        },
        Health {
            value: stats.health,
        },
    ));

    match kind {
        EnemyKind::Boss => {
            entity_cmd.insert(BossAttack::default());
        }
        EnemyKind::MiniBoss => {
            entity_cmd.insert(RangedAttack {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                preferred_distance: 350.0,
                projectile_damage: stats.contact_damage * 0.7,
            });
        }
        _ => {}
    }

    events.write(EnemySpawnedMessage {
        entity: entity_cmd.id(),
        kind,
        transform,
    });
}

// helper functions

fn generate_spawn_position(player_pos: Vec2) -> Vec2 {
    const MIN_SPAWN_DISTANCE: f32 = tiles_to_pixels(3.0);
    const EDGE_MARGIN: f32 = tiles_to_pixels(0.5);

    // Create the spawn area rectangle (game area with margin)
    let spawning_rect = Rect::from_corners(
        GAME_AREA.min + Vec2::splat(EDGE_MARGIN),
        GAME_AREA.max - Vec2::splat(EDGE_MARGIN),
    );
    // The area in which we don't want enemy to spawn as it is to close of the player position
    let safe_rect = Rect::from_corners(
        player_pos - Vec2::splat(MIN_SPAWN_DISTANCE),
        player_pos + Vec2::splat(MIN_SPAWN_DISTANCE),
    );

    // Compute for region around the intersection of the spawn spot and the safe spot
    let intersection = spawning_rect.intersect(safe_rect);
    let regions = vec![
        // LEFT side
        Rect::from_corners(
            spawning_rect.min,
            Vec2::new(intersection.min.x, spawning_rect.max.y),
        ),
        // RIGHT side
        Rect::from_corners(
            Vec2::new(intersection.max.x, spawning_rect.min.y),
            spawning_rect.max,
        ),
        // TOP side
        Rect::from_corners(
            Vec2::new(intersection.min.x, intersection.max.y),
            Vec2::new(intersection.max.x, spawning_rect.max.y),
        ),
        // BOTTOM side
        Rect::from_corners(
            spawning_rect.min,
            Vec2::new(intersection.max.x, intersection.min.y),
        ),
    ];

    // Weighted random selection based on area, the bigger an area is the more changes the enemy spawns in it
    let regions_weight: Vec<f32> = regions
        .iter()
        .map(|region| region.width() * region.height())
        .collect();

    let mut rng = rand::rng();
    let chosen_region = regions[WeightedIndex::new(&regions_weight)
        .unwrap()
        .sample(&mut rng)];

    Vec2::new(
        rng.random_range(chosen_region.min.x..chosen_region.max.x),
        rng.random_range(chosen_region.min.y..chosen_region.max.y),
    )
}
