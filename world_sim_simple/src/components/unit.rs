use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Universal marker component for all controllable/simulated units
/// This replaces both WorkerTag and PeasantTag for a unified system
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct UnitTag;

/// Defines the type/class of unit for specialized behavior
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub enum UnitType {
    /// Basic worker unit (peasant in Stronghold, villager in AoE, etc.)
    Worker,
    /// Military combat unit
    Military,
    /// Special or hero unit
    Special,
    /// Custom type defined by game pack
    Custom(String),
}

impl Default for UnitType {
    fn default() -> Self {
        UnitType::Worker
    }
}

/// Unit-specific stats (consolidates WorkerStats functionality)
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct UnitStats {
    pub work_speed: f32,
    pub move_speed: f32,
    pub carry_capacity: f32,
    pub experience: u32,
    pub level: u32,
}

impl Default for UnitStats {
    fn default() -> Self {
        Self {
            work_speed: 1.0,
            move_speed: 1.0,
            carry_capacity: 10.0,
            experience: 0,
            level: 1,
        }
    }
}