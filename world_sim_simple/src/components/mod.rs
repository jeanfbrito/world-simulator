use bevy::prelude::*;
use colored::*;

// Existing modules
pub mod position;
pub mod health;
pub mod name;
pub mod energy;
pub mod worker;
pub mod peasant;
pub mod goap_states;
pub mod resource;

// New consolidated modules
pub mod unit_state;
pub mod unit_needs_v2;
pub mod grid_position;

// Re-export existing components
pub use position::PositionComponent;
pub use health::HealthComponent;
pub use name::NameComponent;
pub use energy::EnergyComponent;
pub use worker::{WorkerTag, WorkerStats};
pub use peasant::{PeasantTag, PeasantConfig};
pub use goap_states::*;
pub use resource::ResourceNode;

// Re-export new consolidated components
pub use unit_state::{
    UnitNeeds, UnitInventory, UnitLocation, LocationType,
    UnitWorkState, UnitOwnership
};
pub use unit_needs_v2::{UnitNeedsV2, migrate_needs_system};
pub use grid_position::{GridPosition, VisualPosition, GridMovement, migrate_positions_system};

/// Plugin to register all components
pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        println!("{}", "[COMPONENTS] Registering component systems...".green());
        
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