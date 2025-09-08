//! Bevy plugin for the simulation engine

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use crate::systems;
use crate::recipes::RecipeRegistry;
use crate::resources::{WorldState, GameConfig};

/// Main simulation plugin
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        // Add resources
        app.init_resource::<WorldState>()
           .init_resource::<GameConfig>()
           .init_resource::<systems::EventQueue>()
           .init_resource::<systems::HarvestRequests>()
           .init_resource::<systems::MoveRequests>()
           .init_resource::<systems::BuildRequests>()
           .init_resource::<systems::RecipeRequests>()
           .insert_resource(RecipeRegistry::new());
        
        // Add systems in proper order
        app.add_systems(Update, (
            // Input handling
            systems::handle_move_commands,
            systems::handle_build_commands,
            systems::handle_recipe_commands,
            
            // Core systems
            systems::pathfinding_system,
            systems::movement_system,
            systems::start_harvest_system,
            systems::harvest_system,
            systems::building_system,
            systems::recipe_system,
        ).chain());
    }
}