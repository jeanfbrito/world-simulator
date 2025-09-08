//! Worker components for population units

use bevy_ecs::prelude::*;
use world_sim_interface::{EntityId, WorkerState, SettlementId, Position};

/// Component for worker entities
#[derive(Component, Debug, Clone)]
pub struct WorkerComponent {
    pub name: String,
    pub state: WorkerState,
    pub happiness: f32,
    pub hunger: f32,
    pub energy: f32,
    pub settlement: Option<SettlementId>,
    pub assigned_building: Option<EntityId>,
    pub current_task: Option<EntityId>,
}

impl WorkerComponent {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: WorkerState::Idle,
            happiness: 0.5,
            hunger: 0.3,
            energy: 1.0,
            settlement: None,
            assigned_building: None,
            current_task: None,
        }
    }
    
    pub fn with_settlement(mut self, settlement: SettlementId) -> Self {
        self.settlement = Some(settlement);
        self
    }
    
    pub fn update_needs(&mut self, delta_time: f32) {
        // Increase hunger over time
        self.hunger = (self.hunger + delta_time * 0.01).min(1.0);
        
        // Decrease energy when working
        if matches!(self.state, WorkerState::Working) {
            self.energy = (self.energy - delta_time * 0.02).max(0.0);
        } else {
            // Restore energy when idle
            self.energy = (self.energy + delta_time * 0.05).min(1.0);
        }
        
        // Update happiness based on needs
        let need_satisfaction = (1.0 - self.hunger) * self.energy;
        let happiness_delta = (need_satisfaction - 0.5) * delta_time * 0.1;
        self.happiness = (self.happiness + happiness_delta).clamp(0.0, 1.0);
    }
    
    pub fn consume_food(&mut self, amount: f32) {
        self.hunger = (self.hunger - amount).max(0.0);
    }
    
    pub fn is_hungry(&self) -> bool {
        self.hunger > 0.7
    }
    
    pub fn is_tired(&self) -> bool {
        self.energy < 0.3
    }
    
    pub fn can_work(&self) -> bool {
        !self.is_hungry() && !self.is_tired()
    }
    
    pub fn assign_task(&mut self, task: EntityId) {
        self.current_task = Some(task);
        self.state = WorkerState::Working;
    }
    
    pub fn complete_task(&mut self) {
        self.current_task = None;
        self.state = WorkerState::Idle;
    }
}

/// Component for worker movement
#[derive(Component, Debug, Clone)]
pub struct MovementComponent {
    pub target: Option<Position>,
    pub speed: f32,
    pub path: Vec<Position>,
    pub path_index: usize,
}

impl MovementComponent {
    pub fn new(speed: f32) -> Self {
        Self {
            target: None,
            speed,
            path: Vec::new(),
            path_index: 0,
        }
    }
    
    pub fn set_target(&mut self, target: Position) {
        self.target = Some(target);
        self.path.clear();
        self.path_index = 0;
    }
    
    pub fn set_path(&mut self, path: Vec<Position>) {
        self.path = path;
        self.path_index = 0;
        if !self.path.is_empty() {
            self.target = Some(self.path[self.path.len() - 1]);
        }
    }
    
    pub fn get_next_position(&mut self) -> Option<Position> {
        if self.path_index < self.path.len() {
            let pos = self.path[self.path_index];
            self.path_index += 1;
            Some(pos)
        } else {
            None
        }
    }
    
    pub fn has_reached_target(&self) -> bool {
        self.path_index >= self.path.len() && self.target.is_some()
    }
    
    pub fn clear(&mut self) {
        self.target = None;
        self.path.clear();
        self.path_index = 0;
    }
}