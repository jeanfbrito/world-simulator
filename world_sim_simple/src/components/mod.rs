use bevy::prelude::*;
use colored::*;

pub mod position;
pub mod health;
pub mod name;
pub mod energy;
pub mod worker;
pub mod goap_states;
pub mod resource;

pub use position::PositionComponent;
pub use health::HealthComponent;
pub use name::NameComponent;
pub use energy::EnergyComponent;
pub use worker::{WorkerTag, WorkerStats};
pub use goap_states::*;
pub use resource::ResourceNode;

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
           .register_type::<WorkerStats>();
        
        // Register GOAP states
        register_goap_states(app);
           
        println!("{}", "[COMPONENTS] ✓ All components registered".green());
    }
}