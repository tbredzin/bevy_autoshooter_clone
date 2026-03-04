use crate::systems::constants::{ENEMY_BASE_DAMAGE, ENEMY_BASE_XP, ENEMY_HEALTH, ENEMY_SPEED};
use bevy::color::Color;
use bevy::prelude::Srgba;
use rand::RngExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EnemyKind {
    #[default]
    Basic,
    Fast,
    Tank,
    Splitter,
    SmallSplitter,
    Ranged,
    MiniBoss,
    Boss,
}

pub struct EnemyVisual {
    pub radius: f32,
    pub color: Color,
}

pub struct EnemyStats {
    pub health: f32,
    pub speed: f32,
    pub contact_damage: f32,
    pub xp_reward: u32,
}

impl EnemyKind {
    pub fn visual(&self) -> EnemyVisual {
        let (radius, color) = match self {
            EnemyKind::Basic => (15.0, Color::srgb(1.0, 0.3, 0.3)),
            EnemyKind::Fast => (10.0, Color::srgb(1.0, 0.85, 0.1)),
            EnemyKind::Tank => (22.0, Color::srgb(0.5, 0.3, 0.85)),
            EnemyKind::Splitter => (18.0, Color::srgb(0.2, 0.85, 0.4)),
            EnemyKind::SmallSplitter => (8.0, Color::Srgba(Srgba::hex("FF00FF").unwrap())),
            EnemyKind::Ranged => (13.0, Color::srgb(0.1, 0.75, 0.95)),
            EnemyKind::MiniBoss => (35.0, Color::srgb(1.0, 0.45, 0.0)),
            EnemyKind::Boss => (55.0, Color::srgb(0.85, 0.1, 0.9)),
        };
        EnemyVisual { radius, color }
    }

    pub fn stats(&self, wave: u32) -> EnemyStats {
        let wave_scale = 1.0 + wave as f32 * 0.12;
        let (health_ratio, speed_ratio, damage_ratio, xp_ratio) = match self {
            EnemyKind::Basic => (1.0, 1.0, 1.0, 1),
            EnemyKind::Fast => (0.5, 1.5, 0.6, 1),
            EnemyKind::Tank => (2.0, 0.5, 2.0, 3),
            EnemyKind::Splitter => (1.2, 0.85, 0.8, 2),
            EnemyKind::SmallSplitter => (0.3, 1.3, 0.4, 1),
            EnemyKind::Ranged => (0.8, 0.6, 0.3, 2), // low contact dmg, shoots instead
            EnemyKind::MiniBoss => (8.0, 0.65, 3.0, 10),
            EnemyKind::Boss => (25.0, 0.4, 5.0, 50),
        };
        EnemyStats {
            health: ENEMY_HEALTH * health_ratio * wave_scale,
            speed: ENEMY_SPEED * speed_ratio,
            contact_damage: ENEMY_BASE_DAMAGE * damage_ratio,
            xp_reward: ENEMY_BASE_XP * xp_ratio,
        }
    }

    /// Weighted random pick for the regular spawn timer based on wave
    pub fn random_for_wave(wave: u32) -> Self {
        let mut pool: Vec<(EnemyKind, f32)> = vec![(EnemyKind::Basic, 60.0)];
        if wave >= 2 {
            pool.push((EnemyKind::Fast, 25.0));
        }
        if wave >= 3 {
            pool.push((EnemyKind::Tank, 15.0));
        }
        if wave >= 4 {
            pool.push((EnemyKind::Splitter, 15.0));
        }
        if wave >= 5 {
            pool.push((EnemyKind::Ranged, 20.0));
        }

        let total: f32 = pool.iter().map(|(_, w)| w).sum();
        let mut rng = rand::rng();
        let mut roll: f32 = rng.random_range(0.0..total);
        for (kind, weight) in &pool {
            roll -= weight;
            if roll <= 0.0 {
                return *kind;
            }
        }
        EnemyKind::Basic
    }
}
