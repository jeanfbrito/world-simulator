//! Building components for structures

use bevy_ecs::prelude::*;
use world_sim_interface::{BuildingType, EntityId, RecipeId, ResourceType};
use std::collections::{HashMap, VecDeque};

/// Component for building entities
#[derive(Component, Debug, Clone)]
pub struct BuildingComponent {
    pub building_type: BuildingType,
    pub health: u32,
    pub max_health: u32,
    pub construction_progress: f32,
    pub is_complete: bool,
    pub assigned_workers: Vec<EntityId>,
    pub max_workers: usize,
    pub owner: Option<EntityId>,
}

impl BuildingComponent {
    pub fn new(building_type: BuildingType) -> Self {
        let (max_health, max_workers) = match building_type {
            BuildingType::House => (100, 0),
            BuildingType::Stockpile => (50, 2),
            BuildingType::Granary => (80, 2),
            BuildingType::Farm => (75, 3),
            BuildingType::Sawmill => (150, 2),
            BuildingType::Quarry => (180, 3),
            BuildingType::Mine => (200, 4),
            BuildingType::Smithy => (130, 2),
            BuildingType::Barracks => (250, 1),
            BuildingType::Workshop => (100, 2),
            BuildingType::Market => (120, 3),
            BuildingType::Well => (80, 1),
            BuildingType::Tavern => (140, 2),
            BuildingType::Temple => (180, 1),
            BuildingType::Wall => (300, 0),
            BuildingType::Tower => (250, 2),
            BuildingType::Gate => (200, 1),
            BuildingType::Warehouse => (160, 3),
            BuildingType::Bakery => (90, 2),
            BuildingType::Brewery => (110, 2),
            BuildingType::Butcher => (95, 2),
            BuildingType::Fishery => (85, 2),
        };
        
        Self {
            building_type,
            health: 0,
            max_health,
            construction_progress: 0.0,
            is_complete: false,
            assigned_workers: Vec::new(),
            max_workers,
            owner: None,
        }
    }
    
    pub fn with_owner(mut self, owner: EntityId) -> Self {
        self.owner = Some(owner);
        self
    }
    
    pub fn add_construction_progress(&mut self, progress: f32) {
        self.construction_progress = (self.construction_progress + progress).min(1.0);
        if self.construction_progress >= 1.0 {
            self.is_complete = true;
            self.health = self.max_health;
        }
    }
    
    pub fn assign_worker(&mut self, worker: EntityId) -> bool {
        if self.assigned_workers.len() < self.max_workers 
            && !self.assigned_workers.contains(&worker) {
            self.assigned_workers.push(worker);
            true
        } else {
            false
        }
    }
    
    pub fn remove_worker(&mut self, worker: EntityId) -> bool {
        if let Some(index) = self.assigned_workers.iter().position(|&w| w == worker) {
            self.assigned_workers.remove(index);
            true
        } else {
            false
        }
    }
    
    pub fn damage(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);
    }
    
    pub fn repair(&mut self, amount: u32) {
        if self.is_complete {
            self.health = (self.health + amount).min(self.max_health);
        }
    }
    
    pub fn is_destroyed(&self) -> bool {
        self.is_complete && self.health == 0
    }
    
    pub fn has_capacity(&self) -> bool {
        self.assigned_workers.len() < self.max_workers
    }
}

/// Component for buildings that store resources
#[derive(Component, Debug, Clone)]
pub struct StorageComponent {
    pub capacity: u32,
    pub resources: HashMap<ResourceType, u32>,
}

impl StorageComponent {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            resources: HashMap::new(),
        }
    }
    
    pub fn add_resource(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        let current_total: u32 = self.resources.values().sum();
        let available_space = self.capacity.saturating_sub(current_total);
        let amount_to_add = amount.min(available_space);
        
        *self.resources.entry(resource_type).or_insert(0) += amount_to_add;
        amount_to_add
    }
    
    pub fn remove_resource(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        if let Some(stored) = self.resources.get_mut(&resource_type) {
            let amount_to_remove = amount.min(*stored);
            *stored -= amount_to_remove;
            if *stored == 0 {
                self.resources.remove(&resource_type);
            }
            amount_to_remove
        } else {
            0
        }
    }
    
    pub fn get_amount(&self, resource_type: ResourceType) -> u32 {
        self.resources.get(&resource_type).copied().unwrap_or(0)
    }
    
    pub fn has_resources(&self, requirements: &HashMap<ResourceType, u32>) -> bool {
        requirements.iter().all(|(resource, amount)| {
            self.get_amount(*resource) >= *amount
        })
    }
    
    pub fn is_full(&self) -> bool {
        let current_total: u32 = self.resources.values().sum();
        current_total >= self.capacity
    }
    
    pub fn space_available(&self) -> u32 {
        let current_total: u32 = self.resources.values().sum();
        self.capacity.saturating_sub(current_total)
    }
}

/// Component for buildings that process recipes
#[derive(Component, Debug, Clone)]
pub struct ProductionComponent {
    pub current_recipe: Option<RecipeId>,
    pub production_queue: VecDeque<RecipeId>,
    pub progress: f32,
    pub production_speed: f32,
    pub input_storage: HashMap<ResourceType, u32>,
    pub output_storage: HashMap<ResourceType, u32>,
}

impl ProductionComponent {
    pub fn new(production_speed: f32) -> Self {
        Self {
            current_recipe: None,
            production_queue: VecDeque::new(),
            progress: 0.0,
            production_speed,
            input_storage: HashMap::new(),
            output_storage: HashMap::new(),
        }
    }
    
    pub fn queue_recipe(&mut self, recipe: RecipeId) {
        self.production_queue.push_back(recipe);
    }
    
    pub fn start_next_recipe(&mut self) -> Option<RecipeId> {
        if self.current_recipe.is_none() {
            self.current_recipe = self.production_queue.pop_front();
            self.progress = 0.0;
        }
        self.current_recipe.clone()
    }
    
    pub fn update_production(&mut self, delta_time: f32) -> bool {
        if self.current_recipe.is_some() {
            self.progress += self.production_speed * delta_time;
            if self.progress >= 1.0 {
                self.progress = 0.0;
                return true;
            }
        }
        false
    }
    
    pub fn complete_recipe(&mut self) {
        self.current_recipe = None;
        self.progress = 0.0;
    }
    
    pub fn add_input(&mut self, resource: ResourceType, amount: u32) {
        *self.input_storage.entry(resource).or_insert(0) += amount;
    }
    
    pub fn consume_inputs(&mut self, requirements: &HashMap<ResourceType, u32>) -> bool {
        // Check if we have all inputs
        if !requirements.iter().all(|(r, a)| self.input_storage.get(r).unwrap_or(&0) >= a) {
            return false;
        }
        
        // Consume the inputs
        for (resource, amount) in requirements {
            if let Some(stored) = self.input_storage.get_mut(resource) {
                *stored -= amount;
                if *stored == 0 {
                    self.input_storage.remove(resource);
                }
            }
        }
        true
    }
    
    pub fn add_output(&mut self, resource: ResourceType, amount: u32) {
        *self.output_storage.entry(resource).or_insert(0) += amount;
    }
    
    pub fn collect_output(&mut self, resource: ResourceType, max_amount: u32) -> u32 {
        if let Some(stored) = self.output_storage.get_mut(&resource) {
            let collected = (*stored).min(max_amount);
            *stored -= collected;
            if *stored == 0 {
                self.output_storage.remove(&resource);
            }
            collected
        } else {
            0
        }
    }
}