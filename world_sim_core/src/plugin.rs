//! Bevy plugin for the simulation engine

use bevy_app::{App, Plugin, Update, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_dogoap::prelude::*;
use crate::systems;
use crate::ai;
use crate::components;
use crate::recipes::RecipeRegistry;
use crate::resources::{WorldState, GameConfig};

/// Main simulation plugin
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        // Add GOAP plugin
        app.add_plugins(DogoapPlugin);
        
        // Register GOAP components
        register_components!(app, vec![
            components::IsHungry,
            components::HasEnergy,
            components::AtResource,
            components::AtStorage,
            components::AtHome,
            components::IsWorking,
            components::IsIdle,
            components::HasWood,
            components::HasFood,
            components::HasStone,
            components::InventoryFull,
            components::InventoryEmpty,
            components::HouseAvailable,
            components::StorageAvailable,
            components::FarmAvailable,
            components::PopulationCount,
            components::SettlementFood,
            components::SettlementWood,
            components::SettlementStone,
            components::HarvestComplete,
            components::BuildingComplete,
            components::DeliveryComplete,
            components::AtLocation
        ]);
        
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
        
        // Add GOAP systems
        app.add_systems(FixedUpdate, (
            // Sync world state with GOAP state
            ai::sync_goap_state_system,
            
            // GOAP action handlers
            systems::handle_eat_action,
            systems::handle_rest_action,
            systems::handle_sleep_action,
            systems::handle_harvest_wood_action,
            systems::handle_gather_food_action,
            systems::handle_go_to_resource_action,
            systems::handle_go_to_storage_action,
            systems::handle_store_resources_action,
            systems::handle_idle_action,
            
            // Update needs over time
            systems::update_worker_needs,
        ).chain());
    }
}