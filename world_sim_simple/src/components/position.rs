use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,  // For future 3D support
}

impl PositionComponent {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, z: 0.0 }
    }
    
    pub fn from_tile(tile_x: usize, tile_y: usize) -> Self {
        Self {
            x: tile_x as f32,
            y: tile_y as f32,
            z: 0.0,
        }
    }
    
    pub fn to_tile(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
    
    pub fn distance_to(&self, other: &PositionComponent) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}