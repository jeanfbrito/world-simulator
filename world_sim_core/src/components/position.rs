//! Position component for entities in the world

use bevy_ecs::prelude::*;
use world_sim_interface::Position;

/// Component that stores an entity's position in the world
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionComponent {
    pub position: Position,
}

impl PositionComponent {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: Position::new(x, y),
        }
    }
    
    pub fn at(position: Position) -> Self {
        Self { position }
    }
    
    pub fn distance_to(&self, other: &PositionComponent) -> f32 {
        let dx = (self.position.x - other.position.x) as f32;
        let dy = (self.position.y - other.position.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }
    
    pub fn distance_squared_to(&self, other: &PositionComponent) -> i32 {
        self.position.distance_squared(&other.position)
    }
    
    pub fn is_adjacent_to(&self, other: &PositionComponent) -> bool {
        let dx = (self.position.x - other.position.x).abs();
        let dy = (self.position.y - other.position.y).abs();
        dx <= 1 && dy <= 1
    }
    
    pub fn move_towards(&mut self, target: &Position, max_distance: i32) {
        let dx = target.x - self.position.x;
        let dy = target.y - self.position.y;
        
        let distance = ((dx * dx + dy * dy) as f32).sqrt();
        if distance <= max_distance as f32 {
            self.position = *target;
        } else {
            let ratio = max_distance as f32 / distance;
            self.position.x += (dx as f32 * ratio) as i32;
            self.position.y += (dy as f32 * ratio) as i32;
        }
    }
}