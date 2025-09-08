//! Inventory component for entities that can carry items

use bevy_ecs::prelude::*;
use world_sim_interface::ResourceType;
use std::collections::HashMap;

/// Component for entities that can carry resources
#[derive(Component, Debug, Clone)]
pub struct InventoryComponent {
    pub capacity: u32,
    pub resources: HashMap<ResourceType, u32>,
}

impl InventoryComponent {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            resources: HashMap::new(),
        }
    }
    
    pub fn add_resource(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        let current_weight = self.total_weight();
        let available_capacity = self.capacity.saturating_sub(current_weight);
        let amount_to_add = amount.min(available_capacity);
        
        if amount_to_add > 0 {
            *self.resources.entry(resource_type).or_insert(0) += amount_to_add;
        }
        
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
    
    pub fn transfer_to(&mut self, other: &mut InventoryComponent, resource_type: ResourceType, amount: u32) -> u32 {
        let available = self.get_amount(resource_type);
        let to_transfer = amount.min(available);
        
        if to_transfer > 0 {
            let removed = self.remove_resource(resource_type, to_transfer);
            let added = other.add_resource(resource_type, removed);
            
            // If the other inventory couldn't accept all, put back what wasn't transferred
            if added < removed {
                self.add_resource(resource_type, removed - added);
            }
            
            added
        } else {
            0
        }
    }
    
    pub fn transfer_all_to(&mut self, other: &mut InventoryComponent) -> HashMap<ResourceType, u32> {
        let mut transferred = HashMap::new();
        
        // Collect resources to transfer to avoid borrow issues
        let resources_to_transfer: Vec<(ResourceType, u32)> = 
            self.resources.iter().map(|(&r, &a)| (r, a)).collect();
        
        for (resource_type, amount) in resources_to_transfer {
            let transferred_amount = self.transfer_to(other, resource_type, amount);
            if transferred_amount > 0 {
                transferred.insert(resource_type, transferred_amount);
            }
        }
        
        transferred
    }
    
    pub fn get_amount(&self, resource_type: ResourceType) -> u32 {
        self.resources.get(&resource_type).copied().unwrap_or(0)
    }
    
    pub fn has_resource(&self, resource_type: ResourceType, amount: u32) -> bool {
        self.get_amount(resource_type) >= amount
    }
    
    pub fn has_resources(&self, requirements: &HashMap<ResourceType, u32>) -> bool {
        requirements.iter().all(|(resource, amount)| {
            self.has_resource(*resource, *amount)
        })
    }
    
    pub fn consume_resources(&mut self, requirements: &HashMap<ResourceType, u32>) -> bool {
        // First check if we have all requirements
        if !self.has_resources(requirements) {
            return false;
        }
        
        // Consume the resources
        for (resource, amount) in requirements {
            self.remove_resource(*resource, *amount);
        }
        
        true
    }
    
    pub fn total_weight(&self) -> u32 {
        self.resources.values().sum()
    }
    
    pub fn space_available(&self) -> u32 {
        self.capacity.saturating_sub(self.total_weight())
    }
    
    pub fn is_full(&self) -> bool {
        self.total_weight() >= self.capacity
    }
    
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
    
    pub fn clear(&mut self) {
        self.resources.clear();
    }
    
    pub fn list_resources(&self) -> Vec<(ResourceType, u32)> {
        self.resources
            .iter()
            .map(|(&resource, &amount)| (resource, amount))
            .collect()
    }
}

/// Component for tracking carried weight (for more complex weight systems)
#[derive(Component, Debug, Clone)]
pub struct CarryWeightComponent {
    pub current_weight: f32,
    pub max_weight: f32,
    pub movement_penalty: f32,
}

impl CarryWeightComponent {
    pub fn new(max_weight: f32) -> Self {
        Self {
            current_weight: 0.0,
            max_weight,
            movement_penalty: 0.0,
        }
    }
    
    pub fn update_weight(&mut self, weight: f32) {
        self.current_weight = weight.min(self.max_weight);
        
        // Calculate movement penalty based on encumbrance
        let encumbrance = self.current_weight / self.max_weight;
        self.movement_penalty = if encumbrance > 0.75 {
            0.5  // 50% speed reduction when heavily encumbered
        } else if encumbrance > 0.5 {
            0.25 // 25% speed reduction when moderately encumbered
        } else {
            0.0  // No penalty when lightly loaded
        };
    }
    
    pub fn can_carry(&self, additional_weight: f32) -> bool {
        self.current_weight + additional_weight <= self.max_weight
    }
    
    pub fn is_encumbered(&self) -> bool {
        self.current_weight > self.max_weight * 0.5
    }
    
    pub fn is_overloaded(&self) -> bool {
        self.current_weight >= self.max_weight
    }
}