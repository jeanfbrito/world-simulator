use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Peasant unit configuration loaded from Lua
/// Based on assets/packs/stronghold/scripts/units/peasant.lua
#[derive(Component, Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct PeasantConfig {
    pub id: String,
    pub name: String,
    pub category: String,

    // Cost
    pub gold_cost: u32,

    // Stats
    pub health: f32,
    pub speed: f32,

    // Work capabilities
    pub can_work: bool,
    pub can_construct: bool,
    pub can_repair: bool,
    pub can_firefight: bool,
    pub can_haul: bool,

    // Combat (when drafted)
    pub attack_damage: u32,
    pub defense: u32,
    pub morale: u32,

    // Resources they can gather
    pub can_gather: Vec<String>,

    // Basic needs
    pub needs_food: bool,
    pub needs_shelter: bool,
}

impl Default for PeasantConfig {
    fn default() -> Self {
        Self {
            id: "peasant".to_string(),
            name: "Peasant".to_string(),
            category: "civilian".to_string(),

            // Cost
            gold_cost: 0, // Free, comes from population

            // Stats
            health: 10.0,
            speed: 5.0,

            // Work capabilities
            can_work: true,
            can_construct: true,
            can_repair: true,
            can_firefight: true,
            can_haul: true,

            // Combat (when drafted)
            attack_damage: 2,
            defense: 0,
            morale: 2,

            // Resources they can gather
            can_gather: vec![
                "wood".to_string(),
                "stone".to_string(),
                "iron".to_string(),
                "food".to_string(),
            ],

            // Basic needs
            needs_food: true,
            needs_shelter: true,
        }
    }
}

/// Component to mark an entity as a Peasant with specific configuration
#[derive(Component, Clone, Debug, Reflect)]
pub struct PeasantTag {
    pub config: PeasantConfig,
}

impl PeasantTag {
    pub fn new() -> Self {
        Self {
            config: PeasantConfig::default(),
        }
    }

    pub fn with_config(config: PeasantConfig) -> Self {
        Self { config }
    }
}

impl Default for PeasantTag {
    fn default() -> Self {
        Self::new()
    }
}
