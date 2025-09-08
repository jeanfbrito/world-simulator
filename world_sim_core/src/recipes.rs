//! Recipe registry for managing crafting recipes

use world_sim_interface::{Recipe, RecipeId, BuildingType};
use std::collections::HashMap;

/// Registry for managing all available recipes
#[derive(Debug, Clone, Default)]
pub struct RecipeRegistry {
    recipes: HashMap<RecipeId, Recipe>,
    by_building: HashMap<BuildingType, Vec<RecipeId>>,
}

impl RecipeRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn register(&mut self, recipe: Recipe) {
        if let Some(building) = &recipe.required_building {
            self.by_building
                .entry(building.clone())
                .or_insert_with(Vec::new)
                .push(recipe.id.clone());
        }
        
        self.recipes.insert(recipe.id.clone(), recipe);
    }
    
    pub fn get(&self, id: &RecipeId) -> Option<&Recipe> {
        self.recipes.get(id)
    }
    
    pub fn get_for_building(&self, building: &BuildingType) -> Vec<&Recipe> {
        self.by_building
            .get(building)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.recipes.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    pub fn all(&self) -> Vec<&Recipe> {
        self.recipes.values().collect()
    }
    
    pub fn contains(&self, recipe: &Recipe) -> bool {
        self.recipes.get(&recipe.id).map(|r| r == recipe).unwrap_or(false)
    }
}