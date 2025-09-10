use bevy::prelude::*;
use std::collections::HashMap;
use crate::resources::ResourceType;

/// Consolidated unit needs (replaces IsHungry, HasEnergy, etc.)
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct UnitNeeds {
    pub hunger: f32,      // 0.0 = full, 1.0 = starving
    pub energy: f32,      // 0.0 = exhausted, 1.0 = full energy
    pub morale: f32,      // 0.0 = demoralized, 1.0 = happy
    pub shelter: bool,    // Has a house/shelter
}

impl UnitNeeds {
    pub fn new() -> Self {
        Self {
            hunger: 0.3,  // Start slightly hungry
            energy: 1.0,  // Full energy
            morale: 0.7,  // Good morale
            shelter: false, // No initial shelter
        }
    }
    
    pub fn is_hungry(&self) -> bool {
        self.hunger > 0.5
    }
    
    pub fn is_tired(&self) -> bool {
        self.energy < 0.3
    }
    
    pub fn needs_shelter(&self) -> bool {
        !self.shelter
    }
    
    pub fn update(&mut self, delta_time: f32) {
        // Hunger increases over time
        self.hunger = (self.hunger + 0.05 * delta_time).min(1.0);
        
        // Energy decreases over time (slower if resting)
        self.energy = (self.energy - 0.03 * delta_time).max(0.0);
        
        // Morale affected by needs
        if self.hunger > 0.7 || self.energy < 0.2 {
            self.morale = (self.morale - 0.02 * delta_time).max(0.0);
        } else if self.hunger < 0.3 && self.energy > 0.7 && self.shelter {
            self.morale = (self.morale + 0.01 * delta_time).min(1.0);
        }
    }
}

/// Consolidated inventory (replaces HasWood, HasStone, HasFood, etc.)
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct UnitInventory {
    pub items: HashMap<ResourceType, u32>,
    pub max_weight: f32,
    pub current_weight: f32,
}

impl UnitInventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            max_weight: 100.0,
            current_weight: 0.0,
        }
    }
    
    pub fn with_starting_items() -> Self {
        let mut inventory = Self::new();
        // Start with some basic supplies
        inventory.add_item(ResourceType::Berries, 5);
        inventory.add_item(ResourceType::Wood, 2);
        inventory
    }
    
    pub fn add_item(&mut self, resource: ResourceType, amount: u32) -> bool {
        let weight = resource.weight() * amount as f32;
        if self.current_weight + weight <= self.max_weight {
            *self.items.entry(resource).or_insert(0) += amount;
            self.current_weight += weight;
            true
        } else {
            false
        }
    }
    
    pub fn remove_item(&mut self, resource: ResourceType, amount: u32) -> bool {
        if let Some(current) = self.items.get_mut(&resource) {
            if *current >= amount {
                *current -= amount;
                self.current_weight -= resource.weight() * amount as f32;
                if *current == 0 {
                    self.items.remove(&resource);
                }
                return true;
            }
        }
        false
    }
    
    pub fn has_item(&self, resource: ResourceType, amount: u32) -> bool {
        self.items.get(&resource).copied().unwrap_or(0) >= amount
    }
    
    pub fn get_amount(&self, resource: ResourceType) -> u32 {
        self.items.get(&resource).copied().unwrap_or(0)
    }
    
    pub fn is_full(&self) -> bool {
        self.current_weight >= self.max_weight * 0.9
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Consolidated location state (replaces AtResource, AtStorage, AtHome, etc.)
#[derive(Component, Clone, Debug, Reflect)]
pub struct UnitLocation {
    pub current_tile: (usize, usize),
    pub destination: Option<(usize, usize)>,
    pub at_building: Option<Entity>,
    pub location_type: LocationType,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum LocationType {
    Wilderness,
    Storage,
    Home,
    Workshop,
    Farm,
    Resource(ResourceType),
}

impl Default for UnitLocation {
    fn default() -> Self {
        Self {
            current_tile: (32, 32), // Center of map
            destination: None,
            at_building: None,
            location_type: LocationType::Wilderness,
        }
    }
}

impl UnitLocation {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            current_tile: (x, y),
            destination: None,
            at_building: None,
            location_type: LocationType::Wilderness,
        }
    }
    
    pub fn is_at_storage(&self) -> bool {
        matches!(self.location_type, LocationType::Storage)
    }
    
    pub fn is_at_home(&self) -> bool {
        matches!(self.location_type, LocationType::Home)
    }
    
    pub fn is_at_resource(&self) -> bool {
        matches!(self.location_type, LocationType::Resource(_))
    }
    
    pub fn set_destination(&mut self, x: usize, y: usize) {
        self.destination = Some((x, y));
    }
    
    pub fn clear_destination(&mut self) {
        self.destination = None;
    }
    
    pub fn has_arrived(&self) -> bool {
        if let Some((dest_x, dest_y)) = self.destination {
            self.current_tile == (dest_x, dest_y)
        } else {
            true
        }
    }
}

/// Unit work state for tracking current activity
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct UnitWorkState {
    pub is_working: bool,
    pub current_task: Option<String>,
    pub task_progress: f32,
    pub task_target: Option<Entity>,
}

impl UnitWorkState {
    pub fn start_task(&mut self, task: String, target: Option<Entity>) {
        self.is_working = true;
        self.current_task = Some(task);
        self.task_progress = 0.0;
        self.task_target = target;
    }
    
    pub fn update_progress(&mut self, delta: f32) -> bool {
        if self.is_working {
            self.task_progress += delta;
            if self.task_progress >= 1.0 {
                self.complete_task();
                return true;
            }
        }
        false
    }
    
    pub fn complete_task(&mut self) {
        self.is_working = false;
        self.current_task = None;
        self.task_progress = 0.0;
        self.task_target = None;
    }
    
    pub fn cancel_task(&mut self) {
        self.complete_task();
    }
}

/// Building ownership tracking
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct UnitOwnership {
    pub house: Option<Entity>,
    pub workplace: Option<Entity>,
    pub assigned_storage: Option<Entity>,
}

impl UnitOwnership {
    pub fn has_house(&self) -> bool {
        self.house.is_some()
    }
    
    pub fn assign_house(&mut self, entity: Entity) {
        self.house = Some(entity);
    }
    
    pub fn assign_workplace(&mut self, entity: Entity) {
        self.workplace = Some(entity);
    }
}