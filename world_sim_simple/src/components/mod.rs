use bevy::prelude::*;
use colored::*;
use std::collections::HashMap;

// Existing modules
pub mod energy;
pub mod goap_states;
pub mod growth;
pub mod health;
pub mod name;
pub mod peasant;
pub mod position;
pub mod resource;
pub mod unit; // New unified unit system

// New consolidated modules
pub mod claimed_resource;
pub mod grid_position;
pub mod movement_config;
pub mod movement_tracker;
pub mod occupation;
pub mod storage;
pub mod unit_mind;
pub mod unit_state;
pub mod work_progress;

// DEPRECATED: GlobalResourceClaims is no longer used
// Resources now track their own claims with timeout in ResourceNode.claimed_by
// This provides better performance and automatic cleanup of expired claims
// Keeping stub for backwards compatibility during migration
#[derive(Resource, Default)]
pub struct GlobalResourceClaims {
    /// Map from resource entity to claimant entity
    claims: HashMap<Entity, Entity>,
    /// Map from position to claimant (for position-based resources)
    position_claims: HashMap<(u32, u32), Entity>,
}

impl GlobalResourceClaims {
    /// Try to claim a resource atomically - returns true if successful
    pub fn try_claim_resource(&mut self, resource: Entity, claimant: Entity) -> bool {
        if self.claims.contains_key(&resource) {
            false // Already claimed
        } else {
            self.claims.insert(resource, claimant);
            true // Successfully claimed
        }
    }

    /// Try to claim a position atomically - returns true if successful
    pub fn try_claim_position(&mut self, pos: (u32, u32), claimant: Entity) -> bool {
        if self.position_claims.contains_key(&pos) {
            false // Already claimed
        } else {
            self.position_claims.insert(pos, claimant);
            true // Successfully claimed
        }
    }

    /// Try to claim both resource and position atomically - returns true if both succeed
    pub fn try_claim_both(&mut self, resource: Entity, pos: (u32, u32), claimant: Entity) -> bool {
        // Check if either is already claimed
        if self.claims.contains_key(&resource) || self.position_claims.contains_key(&pos) {
            false // Either resource or position already claimed
        } else {
            // Claim both atomically
            self.claims.insert(resource, claimant);
            self.position_claims.insert(pos, claimant);
            true // Successfully claimed both
        }
    }

    /// Release a resource claim
    pub fn release_claim(&mut self, resource: Entity) {
        self.claims.remove(&resource);
    }

    /// Release a position claim
    pub fn release_position_claim(&mut self, pos: (u32, u32)) {
        self.position_claims.remove(&pos);
    }

    /// Release both resource and position claims for a claimant
    pub fn release_claimant_claims(&mut self, claimant: Entity) {
        // Remove all claims by this claimant
        self.claims.retain(|_, &mut v| v != claimant);
        self.position_claims.retain(|_, &mut v| v != claimant);
    }

    /// Check if a resource is claimed
    pub fn is_claimed(&self, resource: Entity) -> bool {
        self.claims.contains_key(&resource)
    }

    /// Check if a position is claimed
    pub fn is_position_claimed(&self, pos: (u32, u32)) -> bool {
        self.position_claims.contains_key(&pos)
    }

    /// Get the claimant of a resource
    pub fn get_claimant(&self, resource: Entity) -> Option<Entity> {
        self.claims.get(&resource).copied()
    }

    /// Get the claimant of a position
    pub fn get_position_claimant(&self, pos: (u32, u32)) -> Option<Entity> {
        self.position_claims.get(&pos).copied()
    }

    /// Debug: Get total number of claims
    pub fn total_claims(&self) -> usize {
        self.claims.len() + self.position_claims.len()
    }
}

// Re-export existing components
pub use energy::EnergyComponent;
pub use goap_states::*;
pub use health::HealthComponent;
pub use name::NameComponent;
pub use peasant::{PeasantConfig, PeasantTag};
pub use position::PositionComponent;
pub use growth::{
    DepletionBehavior, GrowingResource, GrowthPattern, GrowthUpdate, ResourceGrowthEvent,
    TreeStage, CropStage, GrowthEnabledTag,
};
pub use resource::{
    ResourceDepletedEvent, ResourceNode, ResourceRegeneratedEvent, ResourceRegenerationTag,
};
pub use unit::{UnitStats, UnitTag, UnitType};

// Re-export new consolidated components
pub use claimed_resource::ClaimedResource;
pub use grid_position::{migrate_positions_system, GridMovement, GridPosition, VisualPosition};
pub use movement_config::{MovementEffects, MovementSpeed};
pub use movement_tracker::TilesWalked;
pub use occupation::{GridOccupant, SolidObstacle, Walkable, OccupationSize, OccupationChangedEvent};
pub use storage::{
    Stockpile, StorageBuilding, StorageChangeType, StorageChangedEvent, StorageTask,
    StorageTaskState, StorageUpdateTag, Warehouse,
};
pub use unit_mind::UnitMind;
pub use unit_state::{
    LocationType, UnitInventory, UnitLocation, UnitNeeds, UnitOwnership, UnitWorkState,
};
pub use work_progress::*;

// Re-export the global resource claims
// pub use GlobalResourceClaims; // Already defined in this module, no need to re-export

/// Plugin to register all components
pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        println!(
            "{}",
            "[COMPONENTS] Registering component systems...".green()
        );

        app.register_type::<PositionComponent>()
            .register_type::<HealthComponent>()
            .register_type::<NameComponent>()
            .register_type::<EnergyComponent>()
            .register_type::<PeasantTag>()
            .register_type::<PeasantConfig>()
            .register_type::<UnitStats>()
            .register_type::<UnitTag>()
            .register_type::<UnitType>();

        // Register the global resource claims resource
        app.init_resource::<GlobalResourceClaims>();

        // Register GOAP states
        register_goap_states(app);

        println!("{}", "[COMPONENTS] ✓ All components registered".green());
    }
}
