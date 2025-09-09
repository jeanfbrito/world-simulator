use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::debug::{DebugSystem, DebugLevel};

#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct EnergyComponent {
    pub current: f32,
    pub maximum: f32,
    pub drain_rate: f32,
    pub recover_rate: f32,
}

impl EnergyComponent {
    pub fn new(maximum: f32) -> Self {
        Self {
            current: maximum,
            maximum,
            drain_rate: 0.5,  // Per second when working
            recover_rate: 1.0, // Per second when resting
        }
    }
    
    pub fn consume(&mut self, amount: f32, entity_name: &str, debug: &DebugSystem) -> bool {
        if self.current >= amount {
            self.current -= amount;
            debug.log(
                DebugLevel::Debug,
                "ENERGY",
                &format!("{}: Energy {} (-{})", entity_name, self.current, amount)
            );
            true
        } else {
            debug.log(
                DebugLevel::Warn,
                "ENERGY",
                &format!("{}: Insufficient energy ({}/{})", entity_name, self.current, amount)
            );
            false
        }
    }
    
    pub fn is_exhausted(&self) -> bool {
        self.current < self.maximum * 0.1
    }
    
    pub fn percentage(&self) -> f32 {
        (self.current / self.maximum) * 100.0
    }
}