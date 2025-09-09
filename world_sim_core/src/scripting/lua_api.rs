//! Lua API bindings for exposing game functionality to scripts

use bevy_ecs::prelude::*;
use bevy::prelude::AssetServer;
use super::types::{LuaScript, ScriptCollection};
use bevy_reflect::Reflect;
use crate::recipes::RecipeRegistry;
use crate::components::*;
use world_sim_interface::{Recipe, RecipeId, ResourceType, BuildingType};
use std::collections::HashMap;

/// Register the Lua API for accessing game components and systems
pub fn register_lua_api(mut commands: Commands) {
    // This will be called at startup to register our API
    // The actual registration happens through the LuaScriptingPlugin
    
    // Register type mappings for Lua access
    register_recipe_api();
    register_worker_api();
    register_world_api();
}

/// Register recipe-related Lua functions
fn register_recipe_api() {
    // Recipe creation and manipulation functions will be exposed here
    // These are registered with bevy_mod_scripting's type system
}

/// Register worker/AI-related Lua functions
fn register_worker_api() {
    // Worker component access and AI behavior functions
}

/// Register world state Lua functions
fn register_world_api() {
    // World state queries and modifications
}

/// Lua-accessible recipe builder
#[derive(Clone, Debug, Reflect)]
pub struct LuaRecipeBuilder {
    pub id: String,
    pub name: String,
    pub inputs: HashMap<String, u32>,
    pub outputs: HashMap<String, u32>,
    pub duration_ticks: u64,
    pub required_building: Option<String>,
}

impl LuaRecipeBuilder {
    /// Create a new recipe builder
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            duration_ticks: 10,
            required_building: None,
        }
    }
    
    /// Set the recipe name
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    
    /// Add an input resource requirement
    pub fn add_input(&mut self, resource: String, amount: u32) -> &mut Self {
        self.inputs.insert(resource, amount);
        self
    }
    
    /// Add an output resource
    pub fn add_output(&mut self, resource: String, amount: u32) -> &mut Self {
        self.outputs.insert(resource, amount);
        self
    }
    
    /// Set the duration in ticks
    pub fn set_duration(&mut self, ticks: u64) -> &mut Self {
        self.duration_ticks = ticks;
        self
    }
    
    /// Set the required building
    pub fn set_building(&mut self, building: String) -> &mut Self {
        self.required_building = Some(building);
        self
    }
    
    /// Build the final recipe
    pub fn build(&self) -> Result<Recipe, String> {
        // Convert string resource types to enum
        let mut recipe_inputs = HashMap::new();
        for (resource_str, amount) in &self.inputs {
            let resource_type = parse_resource_type(resource_str)?;
            recipe_inputs.insert(resource_type, *amount);
        }
        
        let mut recipe_outputs = HashMap::new();
        for (resource_str, amount) in &self.outputs {
            let resource_type = parse_resource_type(resource_str)?;
            recipe_outputs.insert(resource_type, *amount);
        }
        
        let building = self.required_building.as_ref()
            .map(|b| parse_building_type(b))
            .transpose()?;
        
        Ok(Recipe {
            id: RecipeId::new(self.id.clone()),
            name: self.name.clone(),
            inputs: recipe_inputs,
            outputs: recipe_outputs,
            duration_ticks: self.duration_ticks,
            required_building: building,
        })
    }
}

/// Parse string to ResourceType enum
fn parse_resource_type(s: &str) -> Result<ResourceType, String> {
    match s {
        "Wood" => Ok(ResourceType::Wood),
        "Food" => Ok(ResourceType::Food),
        "Stone" => Ok(ResourceType::Stone),
        "Planks" => Ok(ResourceType::Planks),
        "Tools" => Ok(ResourceType::Tools),
        "Wheat" => Ok(ResourceType::Wheat),
        _ => Err(format!("Unknown resource type: {}", s)),
    }
}

/// Parse string to BuildingType enum
fn parse_building_type(s: &str) -> Result<BuildingType, String> {
    match s {
        "Sawmill" => Ok(BuildingType::Sawmill),
        "Bakery" => Ok(BuildingType::Bakery),
        "Workshop" => Ok(BuildingType::Workshop),
        "House" => Ok(BuildingType::House),
        "Farm" => Ok(BuildingType::Farm),
        "Stockpile" => Ok(BuildingType::Stockpile),
        _ => Err(format!("Unknown building type: {}", s)),
    }
}

/// Lua-accessible worker modifier
#[derive(Clone, Debug, Reflect)]
pub struct LuaWorkerModifier {
    pub work_speed_multiplier: f32,
    pub fatigue_rate_multiplier: f32,
    pub hunger_rate_multiplier: f32,
    pub skill_bonus: f32,
}

impl Default for LuaWorkerModifier {
    fn default() -> Self {
        Self {
            work_speed_multiplier: 1.0,
            fatigue_rate_multiplier: 1.0,
            hunger_rate_multiplier: 1.0,
            skill_bonus: 0.0,
        }
    }
}

/// Lua-accessible AI behavior modifier
#[derive(Clone, Debug, Reflect)]
pub struct LuaAIModifier {
    pub goal_priorities: HashMap<String, f32>,
    pub personality_traits: Vec<String>,
    pub decision_threshold: f32,
}

impl Default for LuaAIModifier {
    fn default() -> Self {
        Self {
            goal_priorities: HashMap::new(),
            personality_traits: Vec::new(),
            decision_threshold: 0.5,
        }
    }
}

/// System to apply Lua recipe modifications
pub fn apply_lua_recipe_modifiers(
    mut recipe_registry: ResMut<RecipeRegistry>,
    lua_recipes: Query<&LuaRecipeBuilder>,
) {
    for recipe_builder in lua_recipes.iter() {
        match recipe_builder.build() {
            Ok(recipe) => {
                recipe_registry.register(recipe);
            }
            Err(e) => {
                tracing::error!("Failed to build Lua recipe: {}", e);
            }
        }
    }
}

/// System to apply Lua worker modifiers
pub fn apply_lua_worker_modifiers(
    mut workers: Query<(&mut WorkerComponent, &LuaWorkerModifier)>,
) {
    for (mut worker, modifier) in workers.iter_mut() {
        // Apply modifiers to worker stats
        worker.work_speed *= modifier.work_speed_multiplier;
        worker.fatigue_rate *= modifier.fatigue_rate_multiplier;
        worker.hunger_rate *= modifier.hunger_rate_multiplier;
        worker.skill_level += modifier.skill_bonus;
    }
}

/// System to apply Lua AI modifiers
pub fn apply_lua_ai_modifiers(
    mut ai_query: Query<(&mut crate::ai::AICoordinator, &LuaAIModifier)>,
) {
    for (mut coordinator, modifier) in ai_query.iter_mut() {
        // Apply AI behavior modifications
        // This would modify goal priorities and decision thresholds
        // Implementation depends on AICoordinator structure
    }
}