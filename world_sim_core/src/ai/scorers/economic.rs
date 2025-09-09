//! Economic and resource-related scorers for Utility AI

use bevy_ecs::prelude::*;
use big_brain::prelude::*;
use crate::components::*;

/// Scores based on potential profit from trading
#[derive(Component, ScorerBuilder, Clone, Debug)]
pub struct ProfitScorer;

/// Scores based on inventory fullness
#[derive(Component, ScorerBuilder, Clone, Debug)]
pub struct InventoryFullScorer;

/// Scores based on resource scarcity
#[derive(Component, ScorerBuilder, Clone, Debug)]
pub struct ResourceScarcityScorer {
    pub resource_type: world_sim_interface::ResourceType,
}

/// System that scores profit opportunities
pub fn profit_scorer_system(
    mut query: Query<(&Actor, &mut Score, &ProfitScorer)>,
    workers: Query<&InventoryComponent>,
) {
    for (Actor(actor), mut score, _) in query.iter_mut() {
        if let Ok(inventory) = workers.get(*actor) {
            // Calculate potential profit based on inventory value
            let wood_value = inventory.get_resource_amount(world_sim_interface::ResourceType::Wood) * 2;
            let food_value = inventory.get_resource_amount(world_sim_interface::ResourceType::Food) * 3;
            let stone_value = inventory.get_resource_amount(world_sim_interface::ResourceType::Stone) * 4;
            
            let total_value = (wood_value + food_value + stone_value) as f32;
            
            // Score based on value (cap at 0.7 to not override survival)
            score.set((total_value / 100.0).min(0.7));
        }
    }
}

/// System that scores based on inventory being full
pub fn inventory_full_scorer_system(
    mut query: Query<(&Actor, &mut Score, &InventoryFullScorer)>,
    workers: Query<&InventoryComponent>,
) {
    for (Actor(actor), mut score, _) in query.iter_mut() {
        if let Ok(inventory) = workers.get(*actor) {
            let fullness = inventory.current_weight as f32 / inventory.max_weight as f32;
            
            // High score when inventory is nearly full
            if fullness > 0.9 {
                score.set(0.8);
            } else if fullness > 0.75 {
                score.set(0.5);
            } else {
                score.set(0.0);
            }
        }
    }
}

/// System that scores based on resource scarcity in settlement
pub fn resource_scarcity_scorer_system(
    mut query: Query<(&Actor, &mut Score, &ResourceScarcityScorer)>,
    settlement_storage: Query<&StorageComponent>,
) {
    for (Actor(_actor), mut score, scorer) in query.iter_mut() {
        let mut total_amount = 0u32;
        
        // Check all storage buildings
        for storage in settlement_storage.iter() {
            total_amount += storage.get_resource_amount(scorer.resource_type);
        }
        
        // Score higher when resource is scarce
        if total_amount < 10 {
            score.set(0.9);
        } else if total_amount < 25 {
            score.set(0.6);
        } else if total_amount < 50 {
            score.set(0.3);
        } else {
            score.set(0.0);
        }
    }
}