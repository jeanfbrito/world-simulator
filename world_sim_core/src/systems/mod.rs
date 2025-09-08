//! Game systems for simulation logic

pub mod harvest_system;
pub mod movement_system;
pub mod recipe_system;
pub mod building_system;
pub mod goap_systems;

// Re-export systems
pub use harvest_system::{harvest_system, start_harvest_system, HarvestRequests};
pub use movement_system::{movement_system, pathfinding_system, handle_move_commands, MoveRequests};
pub use recipe_system::{recipe_system, handle_recipe_commands, RecipeRequests};
pub use building_system::{building_system, handle_build_commands, BuildRequests};

// Re-export GOAP systems
pub use goap_systems::*;

// Common resources
use bevy_ecs::prelude::*;
use world_sim_interface::EngineEvent;

/// Event queue for collecting events
#[derive(Resource, Default)]
pub struct EventQueue {
    events: Vec<EngineEvent>,
}

impl EventQueue {
    pub fn push(&mut self, event: EngineEvent) {
        self.events.push(event);
    }
    
    pub fn drain(&mut self) -> Vec<EngineEvent> {
        self.events.drain(..).collect()
    }
}