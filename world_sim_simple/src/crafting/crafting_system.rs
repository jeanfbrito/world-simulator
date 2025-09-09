use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::debug::{DebugSystem, DebugLevel};
use super::{Recipe, CraftingStationType};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraftingResult {
    Success(Vec<(String, u32)>), // Item names and quantities
    InProgress(f32),              // Progress percentage
    Failed(String),               // Reason for failure
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingTask {
    pub recipe_id: String,
    pub recipe_name: String,
    pub progress: f32,
    pub total_time: f32,
    pub station: CraftingStationType,
    pub crafter: Option<Entity>,
}

impl CraftingTask {
    pub fn new(recipe: &Recipe, station: CraftingStationType, crafter: Option<Entity>) -> Self {
        let speed_mult = station.speed_multiplier();
        Self {
            recipe_id: recipe.id.clone(),
            recipe_name: recipe.name.clone(),
            progress: 0.0,
            total_time: recipe.crafting_time / speed_mult,
            station,
            crafter,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) -> bool {
        self.progress += delta_time;
        self.progress >= self.total_time
    }
    
    pub fn progress_percentage(&self) -> f32 {
        (self.progress / self.total_time * 100.0).min(100.0)
    }
}

#[derive(Resource, Default)]
pub struct CraftingSystem {
    active_tasks: Vec<CraftingTask>,
    queued_tasks: VecDeque<CraftingTask>,
    completed_items: Vec<(String, u32)>,
    max_active: usize,
}

impl CraftingSystem {
    pub fn new() -> Self {
        Self {
            active_tasks: Vec::new(),
            queued_tasks: VecDeque::new(),
            completed_items: Vec::new(),
            max_active: 5,
        }
    }
    
    pub fn start_crafting(
        &mut self,
        recipe: &Recipe,
        station: CraftingStationType,
        crafter: Option<Entity>,
    ) -> CraftingResult {
        // Check if station matches requirements
        if let Some(required) = recipe.station_required {
            if station != required && station != CraftingStationType::None {
                return CraftingResult::Failed(
                    format!("Recipe requires {}", required.name())
                );
            }
        }
        
        let task = CraftingTask::new(recipe, station, crafter);
        
        info!("[CRAFTING] Started crafting {} at {}", 
            task.recipe_name, station.name());
        
        if self.active_tasks.len() < self.max_active {
            self.active_tasks.push(task);
        } else {
            self.queued_tasks.push_back(task);
        }
        
        CraftingResult::InProgress(0.0)
    }
    
    pub fn update(&mut self, delta_time: f32, debug: &DebugSystem) {
        // Update active tasks
        let mut completed_indices = Vec::new();
        
        for (index, task) in self.active_tasks.iter_mut().enumerate() {
            if task.update(delta_time) {
                completed_indices.push(index);
                
                debug.log(
                    DebugLevel::Info,
                    "CRAFTING",
                    &format!("Completed crafting {}", task.recipe_name)
                );
                
                // Add to completed items (simplified - should look up actual recipe outputs)
                self.completed_items.push((task.recipe_name.clone(), 1));
            }
        }
        
        // Remove completed tasks
        for index in completed_indices.iter().rev() {
            self.active_tasks.remove(*index);
        }
        
        // Move queued tasks to active
        while self.active_tasks.len() < self.max_active && !self.queued_tasks.is_empty() {
            if let Some(task) = self.queued_tasks.pop_front() {
                debug.log(
                    DebugLevel::Debug,
                    "CRAFTING",
                    &format!("Moving {} from queue to active", task.recipe_name)
                );
                self.active_tasks.push(task);
            }
        }
    }
    
    pub fn cancel_task(&mut self, recipe_id: &str) -> bool {
        // Check active tasks
        if let Some(pos) = self.active_tasks.iter().position(|t| t.recipe_id == recipe_id) {
            let task = self.active_tasks.remove(pos);
            info!("[CRAFTING] Cancelled crafting {}", task.recipe_name);
            return true;
        }
        
        // Check queued tasks
        if let Some(pos) = self.queued_tasks.iter().position(|t| t.recipe_id == recipe_id) {
            let task = self.queued_tasks.remove(pos).unwrap();
            info!("[CRAFTING] Cancelled queued {}", task.recipe_name);
            return true;
        }
        
        false
    }
    
    pub fn get_active_tasks(&self) -> &[CraftingTask] {
        &self.active_tasks
    }
    
    pub fn get_queued_count(&self) -> usize {
        self.queued_tasks.len()
    }
    
    pub fn collect_completed(&mut self) -> Vec<(String, u32)> {
        std::mem::take(&mut self.completed_items)
    }
}