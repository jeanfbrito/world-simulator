use bevy::prelude::*;
use crate::resources::ResourceType;

#[derive(Component, Clone, Debug)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub amount: u32,
    pub max_amount: u32,
    pub yield_amount: u32,  // Amount gained per gathering action
    pub respawn_time: f32,
    pub time_since_depletion: f32,
}

impl ResourceNode {
    pub fn new(resource_type: ResourceType, amount: u32) -> Self {
        Self {
            resource_type,
            amount,
            max_amount: amount,
            yield_amount: 5,  // Default yield per gathering
            respawn_time: 30.0,
            time_since_depletion: 0.0,
        }
    }
    
    pub fn harvest(&mut self, amount: u32) -> u32 {
        let harvested = amount.min(self.amount);
        self.amount -= harvested;
        if self.amount == 0 {
            self.time_since_depletion = 0.0;
        }
        harvested
    }
    
    pub fn update(&mut self, delta: f32) {
        if self.amount == 0 {
            self.time_since_depletion += delta;
            if self.time_since_depletion >= self.respawn_time {
                self.amount = self.max_amount;
                self.time_since_depletion = 0.0;
            }
        }
    }
}