//! Core types used throughout the simulation engine

use serde::{Deserialize, Serialize};

/// Unique identifier for entities
pub type EntityId = u64;

/// Unique identifier for settlements
pub type SettlementId = u64;

/// Unique identifier for players
pub type PlayerId = u64;

/// Simulation tick counter
pub type Tick = u64;

/// 2D position in the world
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    pub fn distance_squared(&self, other: &Position) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}