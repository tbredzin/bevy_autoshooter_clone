use crate::systems::player_upgrades::components::{StatKind, StatUpgrade, UpgradeRarity};
use bevy::prelude::Resource;
use rand::Rng;

#[derive(Resource)]
pub struct UpgradePool {
    pub upgrades: Vec<StatUpgrade>,
}

impl Default for UpgradePool {
    fn default() -> Self {
        Self {
            upgrades: vec![
                StatUpgrade::new(StatKind::Damage, 0.15, UpgradeRarity::Common),
                StatUpgrade::new(StatKind::FireRate, 0.20, UpgradeRarity::Rare),
                StatUpgrade::new(StatKind::Range, 0.25, UpgradeRarity::Common),
                StatUpgrade::new(StatKind::MaxHealth, 20.0, UpgradeRarity::Legendary),
                StatUpgrade::new(StatKind::Speed, 0.15, UpgradeRarity::Uncommon),
            ],
        }
    }
}

impl UpgradePool {
    pub fn generate_upgrades(&self, count: usize) -> Vec<StatUpgrade> {
        let weights: Vec<f32> = self.upgrades.iter().map(|u| u.rarity.get_odds()).collect();

        let mut rng = rand::rng();
        let mut selected = Vec::new();

        for _ in 0..count {
            let total_weight: f32 = weights.iter().sum();
            let mut roll = rng.random_range(0.0..total_weight);

            for (i, &weight) in weights.iter().enumerate() {
                roll -= weight;
                if roll <= 0.0 {
                    selected.push(self.upgrades[i].clone());
                    break;
                }
            }
        }

        selected
    }
}
