use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component for worker entities
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct WorkerTag;

/// Worker-specific data
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct WorkerStats {
    pub work_speed: f32,
    pub carry_capacity: f32,
    pub experience: u32,
}

impl Default for WorkerStats {
    fn default() -> Self {
        Self {
            work_speed: 1.0,
            carry_capacity: 10.0,
            experience: 0,
        }
    }
}