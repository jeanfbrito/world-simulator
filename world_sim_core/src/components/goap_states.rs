//! GOAP state components for AI decision making

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_dogoap::prelude::*;
use world_sim_interface::ResourceType;

// Worker needs states

/// Tracks worker hunger level (0.0 = not hungry, 100.0 = starving)
#[derive(Component, Clone, DatumComponent)]
pub struct IsHungry(pub f64);

/// Tracks worker energy level (0.0 = exhausted, 100.0 = fully rested)
#[derive(Component, Clone, DatumComponent)]
pub struct HasEnergy(pub f64);

/// Whether worker is currently at a resource node
#[derive(Component, Clone, DatumComponent)]
pub struct AtResource(pub bool);

/// Whether worker is at a storage building
#[derive(Component, Clone, DatumComponent)]
pub struct AtStorage(pub bool);

/// Whether worker is at their home/house
#[derive(Component, Clone, DatumComponent)]
pub struct AtHome(pub bool);

/// Whether worker is currently working on a task
#[derive(Component, Clone, DatumComponent)]
pub struct IsWorking(pub bool);

/// Whether worker is idle
#[derive(Component, Clone, DatumComponent)]
pub struct IsIdle(pub bool);

// Resource states

/// Tracks if worker has specific resource type and amount
#[derive(Component, Clone)]
pub struct HasWood(pub u32);

#[derive(Component, Clone)]
pub struct HasFood(pub u32);

#[derive(Component, Clone)]
pub struct HasStone(pub u32);

/// Whether inventory is full
#[derive(Component, Clone, DatumComponent)]
pub struct InventoryFull(pub bool);

/// Whether inventory is empty
#[derive(Component, Clone, DatumComponent)]
pub struct InventoryEmpty(pub bool);

// Building states

/// Whether a specific building type exists nearby
#[derive(Component, Clone, DatumComponent)]
pub struct HouseAvailable(pub bool);

#[derive(Component, Clone, DatumComponent)]
pub struct StorageAvailable(pub bool);

#[derive(Component, Clone, DatumComponent)]
pub struct FarmAvailable(pub bool);

// Settlement states

/// Number of workers in settlement
#[derive(Component, Clone)]
pub struct PopulationCount(pub i32);

/// Total food available in settlement storage
#[derive(Component, Clone)]
pub struct SettlementFood(pub u32);

/// Total wood available in settlement storage
#[derive(Component, Clone)]
pub struct SettlementWood(pub u32);

/// Total stone available in settlement storage  
#[derive(Component, Clone)]
pub struct SettlementStone(pub u32);

// Task completion states

/// Whether current harvest task is complete
#[derive(Component, Clone, DatumComponent)]
pub struct HarvestComplete(pub bool);

/// Whether current building task is complete
#[derive(Component, Clone, DatumComponent)]
pub struct BuildingComplete(pub bool);

/// Whether current delivery task is complete
#[derive(Component, Clone, DatumComponent)]
pub struct DeliveryComplete(pub bool);

// Location enum for more complex navigation
#[derive(Clone, Default, Reflect, Copy, EnumDatum)]
pub enum WorkerLocation {
    #[default]
    Unknown,
    Home,
    Storage,
    Farm,
    Forest,
    Quarry,
    Field,
    Construction,
}

/// Current location of the worker
#[derive(Component, Clone, EnumComponent)]
pub struct AtLocation(pub WorkerLocation);

// Helper functions for state management
impl IsHungry {
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 100.0))
    }
    
    pub fn is_hungry(&self) -> bool {
        self.0 > 50.0
    }
    
    pub fn is_starving(&self) -> bool {
        self.0 > 80.0
    }
}

impl HasEnergy {
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 100.0))
    }
    
    pub fn is_tired(&self) -> bool {
        self.0 < 30.0
    }
    
    pub fn is_exhausted(&self) -> bool {
        self.0 < 10.0
    }
}