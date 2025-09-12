use crate::resources::ResourceType;
/// Resource node component for harvestable resources
///
/// Uses tick-based regeneration with integer counters
/// for deterministic resource respawning.
use bevy::prelude::*;

/// Resource node that can be harvested and regenerates over time
#[derive(Component, Clone, Debug, Reflect)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub amount: u32,
    pub max_amount: u32,
    pub yield_amount: u32, // Amount gained per gathering action

    // Tick-based regeneration
    pub regeneration_rate: u32,      // Amount regenerated per interval
    pub regeneration_interval: u32,  // Ticks between regeneration
    pub ticks_since_last_regen: u32, // Counter for regeneration timing

    // Full respawn after depletion
    pub respawn_time_ticks: u32, // Ticks to fully respawn when depleted
    pub ticks_since_depletion: u32, // Counter since resource was depleted

    // Resource quality/variety
    pub quality_modifier: f32,  // Affects yield quality
    pub seasonal_modifier: f32, // Changes with seasons
}

impl ResourceNode {
    pub fn new(resource_type: ResourceType, amount: u32) -> Self {
        Self {
            resource_type,
            amount,
            max_amount: amount,
            yield_amount: 5, // Default yield per gathering

            // Default: regenerate 1 unit every 5 seconds (50 ticks)
            regeneration_rate: 1,
            regeneration_interval: 50,
            ticks_since_last_regen: 0,

            // Default: fully respawn after 30 seconds (300 ticks)
            respawn_time_ticks: 300,
            ticks_since_depletion: 0,

            quality_modifier: 1.0,
            seasonal_modifier: 1.0,
        }
    }

    /// Create a fruit bush that regenerates berries
    pub fn fruit_bush(max_berries: u32) -> Self {
        Self {
            resource_type: ResourceType::Berries,
            amount: max_berries,
            max_amount: max_berries,
            yield_amount: 3,

            // Berries regenerate slowly: 1 berry every 60 seconds
            regeneration_rate: 1,
            regeneration_interval: 600,
            ticks_since_last_regen: 0,

            // Respawn after full depletion: 2 minutes
            respawn_time_ticks: 1200,
            ticks_since_depletion: 0,

            quality_modifier: 1.0,
            seasonal_modifier: 1.0,
        }
    }

    /// Create a tree that provides wood
    pub fn tree(wood_amount: u32) -> Self {
        Self {
            resource_type: ResourceType::Wood,
            amount: wood_amount,
            max_amount: wood_amount,
            yield_amount: 10,

            // Trees regenerate slowly: 5 wood every minute
            regeneration_rate: 5,
            regeneration_interval: 600,
            ticks_since_last_regen: 0,

            // Trees take long to respawn: 5 minutes
            respawn_time_ticks: 3000,
            ticks_since_depletion: 0,

            quality_modifier: 1.0,
            seasonal_modifier: 1.0,
        }
    }

    /// Harvest resources from the node
    pub fn harvest(&mut self, requested_amount: u32) -> u32 {
        let harvested = requested_amount.min(self.amount).min(self.yield_amount);
        self.amount -= harvested;

        if self.amount == 0 {
            self.ticks_since_depletion = 0;
        }

        harvested
    }

    /// Update regeneration on tick
    pub fn tick_update(&mut self) {
        // ResourceNode no longer handles regeneration - that's GrowingResource's job
        // This method is kept for compatibility but does nothing
        // All regeneration logic should be in GrowingResource component
    }

    /// Check if resource can be harvested
    pub fn can_harvest(&self) -> bool {
        self.amount > 0
    }

    /// Get current fill percentage
    pub fn fill_percentage(&self) -> f32 {
        self.amount as f32 / self.max_amount as f32
    }

    /// Apply seasonal effects
    pub fn apply_seasonal_modifier(&mut self, modifier: f32) {
        self.seasonal_modifier = modifier;
        // Could affect regeneration rate or yield
    }
}

/// Different types of resource nodes from Lua scripts
#[derive(Component, Clone, Debug, Reflect)]
pub struct ScriptedResourceNode {
    pub node_id: String,
    pub display_name: String,
    pub base_config: ResourceNodeConfig,
}

/// Configuration for resource nodes loaded from scripts
#[derive(Clone, Debug, Reflect)]
pub struct ResourceNodeConfig {
    pub resource_types: Vec<ResourceType>, // Can yield multiple types
    pub amounts: Vec<u32>,
    pub regeneration_rates: Vec<u32>,
    pub regeneration_intervals: Vec<u32>,
    pub respawn_time_ticks: u32,
    pub harvest_tool: Option<String>, // Required tool for better yield
    pub seasons: Vec<String>,         // Best seasons for harvesting
}

/// Tag for resource nodes that need regeneration updates
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct ResourceRegenerationTag;

/// Event when a resource regenerates
#[derive(Event, Clone, Debug)]
pub struct ResourceRegeneratedEvent {
    pub entity: Entity,
    pub resource_type: ResourceType,
    pub old_amount: u32,
    pub new_amount: u32,
    pub is_full_respawn: bool,
}

/// Event when a resource is depleted
#[derive(Event, Clone, Debug)]
pub struct ResourceDepletedEvent {
    pub entity: Entity,
    pub resource_type: ResourceType,
    pub harvester: Option<Entity>,
}
