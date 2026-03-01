use crate::systems::states::upgrades::components::{UpgradeCard, UpgradeRarity};
use crate::systems::states::waves::player::components::StatKind;
use bevy::prelude::Resource;
use rand::RngExt;

#[derive(Resource)]
pub struct UpgradePool {
    pub upgrades: Vec<UpgradeCard>,
}

impl Default for UpgradePool {
    fn default() -> Self {
        Self {
            upgrades: vec![
                UpgradeCard::new(StatKind::Damage, 0.15, UpgradeRarity::Common),
                UpgradeCard::new(StatKind::FireRate, 0.20, UpgradeRarity::Rare),
                UpgradeCard::new(StatKind::Range, 0.25, UpgradeRarity::Common),
                UpgradeCard::new(StatKind::MaxHealth, 20.0, UpgradeRarity::Legendary),
                UpgradeCard::new(StatKind::Speed, 0.15, UpgradeRarity::Uncommon),
            ],
        }
    }
}

impl UpgradePool {
    pub fn generate_upgrades(&self, count: usize) -> Vec<UpgradeCard> {
        let weights: Vec<f32> = self.upgrades.iter().map(|u| u.rarity.get_odds()).collect();

        let mut rng = rand::rng();
        let mut selected = Vec::new();

        for _ in 0..count {
            let total_weight: f32 = weights.iter().sum();
            let mut roll = rng.random_range(0.0..total_weight);

            for (i, &weight) in weights.iter().enumerate() {
                roll -= weight;
                if roll <= 0.0 {
                    selected.push(self.upgrades[i]);
                    break;
                }
            }
        }

        selected
    }
}
