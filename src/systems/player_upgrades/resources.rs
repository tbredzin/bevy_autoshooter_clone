use crate::systems::player_upgrades::components::StatUpgrade;
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
                StatUpgrade::IncreaseDamage(0.15),
                StatUpgrade::IncreaseFireRate(0.20),
                StatUpgrade::IncreaseRange(0.25),
                StatUpgrade::IncreaseMaxHealth(20.0),
                StatUpgrade::IncreaseSpeed(0.15),
            ],
        }
    }
}

impl UpgradePool {
    pub fn generate_upgrades(&self, count: usize) -> Vec<StatUpgrade> {
        let weights: Vec<f32> = self
            .upgrades
            .iter()
            .map(|u| u.get_rarity().get_odds())
            .collect();

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
