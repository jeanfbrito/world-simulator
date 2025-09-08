//! Recipe processing system for crafting

use bevy_ecs::prelude::*;
use world_sim_interface::{Recipe, RecipeId, EntityId};
use crate::components::*;
use crate::recipes::RecipeRegistry;

/// System for processing recipes in buildings
pub fn recipe_system(
    mut query: Query<(&mut ProductionComponent, &mut StorageComponent, Entity), With<BuildingComponent>>,
    recipes: Res<RecipeRegistry>,
    mut events: ResMut<super::EventQueue>,
) {
    for (mut production, mut storage, entity) in query.iter_mut() {
        // Check if there's a recipe in progress
        if let Some(recipe_id) = production.current_recipe.clone() {
            if let Some(recipe) = recipes.get(&recipe_id) {
                // Update production
                if production.update_production(0.1) {
                    // Recipe complete - add outputs
                    for (resource, amount) in &recipe.outputs {
                        production.add_output(*resource, *amount);
                    }
                    
                    // Emit completion event
                    let recipe_id_str = recipe_id.as_str().to_string();
                    events.push(world_sim_interface::EngineEvent::RecipeCompleted {
                        recipe_id: recipe_id_str,
                        building_id: entity.index() as EntityId,
                        outputs: recipe.outputs.iter().map(|(r, a)| (*r, *a)).collect(),
                    });
                    
                    production.complete_recipe();
                }
            }
        }
        
        // Try to start next recipe if idle
        if production.current_recipe.is_none() {
            if let Some(recipe_id) = production.start_next_recipe() {
                if let Some(recipe) = recipes.get(&recipe_id) {
                    // Check inputs
                    if production.consume_inputs(&recipe.inputs) {
                        // Emit start event
                        let recipe_id_str = recipe_id.as_str().to_string();
                        events.push(world_sim_interface::EngineEvent::RecipeStarted {
                            recipe_id: recipe_id_str,
                            building_id: entity.index() as EntityId,
                            inputs_consumed: recipe.inputs.iter().map(|(r, a)| (*r, *a)).collect(),
                            estimated_completion: 0, // TODO: Calculate
                        });
                    } else {
                        // Not enough inputs
                        production.current_recipe = None;
                    }
                }
            }
        }
    }
}

/// System for handling recipe start commands
pub fn handle_recipe_commands(
    mut buildings: Query<(&mut ProductionComponent, Entity), With<BuildingComponent>>,
    mut recipe_requests: ResMut<RecipeRequests>,
    recipes: Res<RecipeRegistry>,
) {
    for request in recipe_requests.drain() {
        // Find building
        if let Some((mut production, _entity)) = buildings.iter_mut()
            .find(|(_, e)| e.index() as EntityId == request.building_id)
        {
            // Validate recipe exists
            if recipes.get(&request.recipe_id).is_some() {
                production.queue_recipe(request.recipe_id);
            }
        }
    }
}

/// Request to start a recipe
#[derive(Debug, Clone)]
pub struct RecipeRequest {
    pub building_id: EntityId,
    pub recipe_id: RecipeId,
}

/// Resource for recipe requests
#[derive(Resource, Default)]
pub struct RecipeRequests {
    requests: Vec<RecipeRequest>,
}

impl RecipeRequests {
    pub fn add(&mut self, building_id: EntityId, recipe_id: RecipeId) {
        self.requests.push(RecipeRequest { building_id, recipe_id });
    }
    
    pub fn drain(&mut self) -> std::vec::Drain<RecipeRequest> {
        self.requests.drain(..)
    }
}