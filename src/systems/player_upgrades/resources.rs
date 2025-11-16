use crate::systems::player_upgrades::components::UpgradeType;
use bevy::prelude::Resource;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::Rng;

#[derive(Resource, Default)]
pub struct AppliedUpgrades {
    pub damage_multiplier: f32,
    pub fire_rate_multiplier: f32,
    pub range_multiplier: f32,
    pub speed_multiplier: f32,
    pub has_piercing: bool,
    pub multishot_count: u32,
    pub has_explosive: bool,
}
#[derive(Resource)]
pub struct AvailableUpgradesResource {
    pub upgrades: Vec<UpgradeType>,
    pub weights: Vec<f32>,
}

impl Default for AvailableUpgradesResource {
    fn default() -> Self {
        let all_upgrades = vec![
            UpgradeType::IncreaseDamage(0.15),
            UpgradeType::IncreaseFireRate(0.2),
            UpgradeType::IncreaseRange(0.25),
            UpgradeType::IncreaseMaxHealth(20.0),
            UpgradeType::IncreaseSpeed(0.15),
            UpgradeType::HealPlayer(30.0),
            UpgradeType::AddPiercing,
            UpgradeType::AddMultishot(2),
            UpgradeType::AddExplosive,
        ];
        Self {
            upgrades: all_upgrades.clone(),
            weights: all_upgrades
                .clone()
                .iter()
                .map(|a| a.get_rarity().get_odds())
                .collect(),
        }
    }
}
impl AvailableUpgradesResource {
    pub(crate) fn generate(&self, rng: &mut impl Rng, count: u32) -> Vec<UpgradeType> {
        let selector = WeightedIndex::new(self.weights.clone()).unwrap();
        let mut upgrades: Vec<UpgradeType> = vec![];
        for _ in 0..count {
            upgrades.push(self.upgrades[selector.sample(rng)].clone());
        }
        upgrades
    }
}
