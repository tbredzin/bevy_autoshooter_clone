use crate::components::{Enemy, Health, MarkedForDespawn, Spawning};
use crate::messages::EnemyDeathMessage;
use crate::resources::{
    ENEMY_HEALTH, ENEMY_SPAWN_TIME_IN_S, ENEMY_SPEED, GAME_AREA, SPAWN_RATE, WaveManager,
    tiles_to_pixels,
};
use crate::systems::player::components::Player;
use bevy::prelude::*;
use rand::Rng;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

pub fn update_spawning(
    mut commands: Commands,
    mut wave_manager: ResMut<WaveManager>,
    player_query: Query<&GlobalTransform, With<Player>>,
    time: Res<Time>,
) -> Result {
    wave_manager.enemy_spawn_timer.tick(time.delta());
    if !wave_manager.enemy_spawn_timer.is_finished() {
        return Ok(());
    }

    let Ok(player_transform) = player_query.single() else {
        return Ok(());
    };

    let spawn_pos = generate_spawn_position(player_transform.translation().truncate())?;

    // Spawn warning indicator
    commands.spawn((
        Transform::from_translation(spawn_pos.extend(0.0)),
        Spawning {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME_IN_S, TimerMode::Once),
        },
    ));

    // Adjust spawn rate based on wave (faster spawning each wave)
    let base_rate = SPAWN_RATE / wave_manager.wave as f32;
    wave_manager
        .enemy_spawn_timer
        .set_duration(std::time::Duration::from_secs_f32(base_rate.max(0.01)));
    Ok(())
}

pub fn update_spawned(
    mut commands: Commands,
    mut pre_spawn_query: Query<(Entity, &mut Spawning)>,
    time: Res<Time>,
) {
    for (entity, mut spawning) in &mut pre_spawn_query {
        spawning.timer.tick(time.delta());

        if spawning.timer.is_finished() {
            commands.entity(entity).remove::<Spawning>();
            commands.entity(entity).insert((
                Enemy { damage: 10.0 },
                Health {
                    value: ENEMY_HEALTH,
                },
            ));
        }
    }
}

pub fn update_move(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation();

    for mut enemy_transform in &mut enemy_query {
        let direction = (player_pos - enemy_transform.translation).normalize();
        enemy_transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}

pub fn check_if_dead(
    mut commands: Commands,
    mut message_writer: MessageWriter<EnemyDeathMessage>,
    query: Query<(Entity, &Health), With<Enemy>>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            message_writer.write(EnemyDeathMessage(entity));
            commands.entity(entity).insert(MarkedForDespawn);
        }
    }
}

// helper functions

fn generate_spawn_position(player_pos: Vec2) -> Result<Vec2> {
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
    let chosen_region = regions[WeightedIndex::new(&regions_weight)?.sample(&mut rng)];

    Ok(Vec2::new(
        rng.random_range(chosen_region.min.x..chosen_region.max.x),
        rng.random_range(chosen_region.min.y..chosen_region.max.y),
    ))
}
