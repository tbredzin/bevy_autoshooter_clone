use crate::systems::states::waves::enemy::kinds::EnemyKind;
use bevy::prelude::{Component, Timer, TimerMode, Vec2};

#[derive(Component)]
pub struct Spawning {
    pub timer: Timer,
    pub kind: EnemyKind,
}

#[derive(Component)]
pub struct Enemy {
    pub damage: f32,
    pub speed: f32,
    pub kind: EnemyKind,
    pub xp_reward: u32,
}

#[derive(Component)]
pub struct Splitter {
    pub split_count: u32,
}
#[derive(Component)]
pub struct RangedAttack {
    pub timer: Timer,
    pub preferred_distance: f32,
    pub projectile_damage: f32,
}
#[derive(Component)]
pub struct Hostile;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BossPhase {
    Chasing,
    Charging,
    Cooldown,
}
#[derive(Component)]
pub struct BossAttack {
    pub phase: BossPhase,
    pub phase_timer: Timer,
    pub charge_direction: Vec2,
}

impl Default for BossAttack {
    fn default() -> Self {
        Self {
            phase: BossPhase::Chasing,
            phase_timer: Timer::from_seconds(4.0, TimerMode::Once),
            charge_direction: Vec2::ZERO,
        }
    }
}
