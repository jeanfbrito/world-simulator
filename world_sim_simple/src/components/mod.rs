use bevy::prelude::*;
use colored::*;

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
pub mod worker;

// New consolidated modules
pub mod grid_position;
pub mod movement_config;
pub mod movement_tracker;
pub mod storage;
pub mod unit_mind;
pub mod unit_needs_v2;
pub mod unit_state;
pub mod work_progress;

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
pub use worker::{WorkerStats, WorkerTag};

// Re-export new consolidated components
pub use grid_position::{migrate_positions_system, GridMovement, GridPosition, VisualPosition};
pub use movement_config::{MovementEffects, MovementSpeed};
pub use movement_tracker::TilesWalked;
pub use storage::{
    Stockpile, StorageBuilding, StorageChangeType, StorageChangedEvent, StorageTask,
    StorageTaskState, StorageUpdateTag, Warehouse,
};
pub use unit_mind::UnitMind;
pub use unit_needs_v2::{migrate_needs_system, UnitNeedsV2};
pub use unit_state::{
    LocationType, UnitInventory, UnitLocation, UnitNeeds, UnitOwnership, UnitWorkState,
};
pub use work_progress::*;

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
            .register_type::<WorkerTag>()
            .register_type::<WorkerStats>()
            .register_type::<PeasantTag>()
            .register_type::<PeasantConfig>();

        // Register GOAP states
        register_goap_states(app);

        println!("{}", "[COMPONENTS] ✓ All components registered".green());
    }
}
