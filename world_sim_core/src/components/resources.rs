//! Resource node components for harvestable resources

use bevy_ecs::prelude::*;
use world_sim_interface::{ResourceType, EntityId};
use std::time::Duration;

/// Component for resource nodes that can be harvested
#[derive(Component, Debug, Clone)]
pub struct ResourceNodeComponent {
    pub resource_type: ResourceType,
    pub amount: u32,
    pub max_amount: u32,
    pub harvest_time: Duration,
    pub regeneration_rate: f32,
    pub regeneration_timer: f32,
}

impl ResourceNodeComponent {
    pub fn new(resource_type: ResourceType, amount: u32) -> Self {
        let (max_amount, harvest_time, regen_rate) = match resource_type {
            ResourceType::Wood => (100, Duration::from_secs(3), 0.1),
            ResourceType::Stone => (50, Duration::from_secs(4), 0.05),
            ResourceType::Food => (20, Duration::from_secs(2), 0.2),
            ResourceType::Water => (u32::MAX, Duration::from_secs(1), 1.0),
            ResourceType::Iron => (30, Duration::from_secs(5), 0.02),
            ResourceType::Gold => (10, Duration::from_secs(8), 0.01),
            _ => (50, Duration::from_secs(3), 0.1),
        };
        
        Self {
            resource_type,
            amount,
            max_amount,
            harvest_time,
            regeneration_rate: regen_rate,
            regeneration_timer: 0.0,
        }
    }
    
    pub fn harvest(&mut self, amount: u32) -> u32 {
        let harvested = amount.min(self.amount);
        self.amount -= harvested;
        harvested
    }
    
    pub fn regenerate(&mut self, delta_time: f32) {
        if self.amount < self.max_amount {
            self.regeneration_timer += delta_time;
            
            let regen_amount = (self.regeneration_timer * self.regeneration_rate) as u32;
            if regen_amount > 0 {
                self.amount = (self.amount + regen_amount).min(self.max_amount);
                self.regeneration_timer = 0.0;
            }
        }
    }
    
    pub fn is_depleted(&self) -> bool {
        self.amount == 0
    }
    
    pub fn is_full(&self) -> bool {
        self.amount == self.max_amount
    }
}

/// Component for entities actively harvesting a resource
#[derive(Component, Debug, Clone)]
pub struct HarvestingComponent {
    pub target_resource: EntityId,
    pub progress: f32,
    pub harvest_rate: f32,
}

impl HarvestingComponent {
    pub fn new(target: EntityId) -> Self {
        Self {
            target_resource: target,
            progress: 0.0,
            harvest_rate: 1.0,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) -> bool {
        self.progress += self.harvest_rate * delta_time;
        self.progress >= 1.0
    }
    
    pub fn reset(&mut self) {
        self.progress = 0.0;
    }
}