use crate::resources::ResourceType;
/// Resource node component for harvestable resources
///
/// Uses tick-based regeneration with integer counters
/// for deterministic resource respawning.
use bevy::prelude::*;
use std::collections::HashMap;

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
    
    // Resource claiming system with timeout
    #[reflect(ignore)]
    pub claimed_by: HashMap<Entity, u32>, // Map entity to claim timestamp (tick)
    pub max_workers: usize,               // Maximum simultaneous workers (1 for berries, 4 for rocks, etc.)
    pub claim_timeout: u32,               // Ticks before a claim expires (default: 100 ticks = 10 seconds)
}

impl ResourceNode {
    pub fn new(resource_type: ResourceType, amount: u32) -> Self {
        // Default max_workers based on resource type
        let max_workers = match resource_type {
            ResourceType::Berries => 1,  // Only one person can harvest berries at a time
            ResourceType::Stone => 4,    // Multiple miners can work a rock deposit
            ResourceType::Wood => 2,     // Two lumberjacks can work the same tree
            _ => 1,                     // Default to single worker
        };
        
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

            claimed_by: HashMap::new(),
            max_workers,
            claim_timeout: 100,  // 10 seconds default timeout
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

            claimed_by: HashMap::new(),
            max_workers: 1,  // Only one person can pick berries at a time
            claim_timeout: 150,  // 15 seconds for berry picking
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

            claimed_by: HashMap::new(),
            max_workers: 2,  // Two lumberjacks
            claim_timeout: 200,  // 20 seconds for tree chopping can work on the same tree
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
    
    /// Try to claim this resource for a unit with timeout tracking
    /// Returns true if claim was successful
    pub fn try_claim_with_timeout(&mut self, claimer: Entity, current_tick: u32) -> bool {
        // First cleanup any expired claims
        self.cleanup_expired_claims(current_tick);

        // Check if already claimed by this entity - refresh the timestamp
        if self.claimed_by.contains_key(&claimer) {
            self.claimed_by.insert(claimer, current_tick);
            return true;  // Already claimed, timestamp refreshed
        }

        // Check if there's room for another worker
        if self.claimed_by.len() < self.max_workers {
            self.claimed_by.insert(claimer, current_tick);
            return true;
        }

        false  // No room for more workers
    }

    /// Legacy claim method (without timeout) - for backwards compatibility
    pub fn try_claim(&mut self, claimer: Entity) -> bool {
        // Use a very high tick value so it doesn't expire
        self.try_claim_with_timeout(claimer, u32::MAX)
    }
    
    /// Release a claim on this resource
    pub fn release_claim(&mut self, claimer: Entity) {
        self.claimed_by.remove(&claimer);
    }

    /// Cleanup expired claims based on timeout
    pub fn cleanup_expired_claims(&mut self, current_tick: u32) {
        self.claimed_by.retain(|_, timestamp| {
            // Keep claims that haven't expired yet
            // Handle wraparound for u32::MAX (legacy claims)
            *timestamp == u32::MAX || current_tick.saturating_sub(*timestamp) < self.claim_timeout
        });
    }

    /// Refresh a claim to prevent timeout
    pub fn refresh_claim(&mut self, claimer: Entity, current_tick: u32) {
        if self.claimed_by.contains_key(&claimer) {
            self.claimed_by.insert(claimer, current_tick);
        }
    }
    
    /// Check if a specific entity has claimed this resource
    pub fn is_claimed_by(&self, entity: Entity) -> bool {
        self.claimed_by.contains_key(&entity)
    }
    
    /// Check if resource is fully claimed (no room for more workers)
    /// Note: This doesn't clean expired claims - call cleanup_expired_claims first if needed
    pub fn is_fully_claimed(&self) -> bool {
        self.claimed_by.len() >= self.max_workers
    }

    /// Check if resource is fully claimed after cleaning expired claims
    pub fn is_fully_claimed_with_cleanup(&mut self, current_tick: u32) -> bool {
        self.cleanup_expired_claims(current_tick);
        self.claimed_by.len() >= self.max_workers
    }
    
    /// Get number of current claimers
    pub fn claim_count(&self) -> usize {
        self.claimed_by.len()
    }
    
    /// Clear all claims (useful when resource is depleted)
    pub fn clear_claims(&mut self) {
        self.claimed_by.clear();
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
