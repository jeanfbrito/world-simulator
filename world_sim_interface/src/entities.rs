//! Entity type definitions

use serde::{Deserialize, Serialize};

/// Types of entities in the simulation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum EntityType {
    // Resource nodes
    Tree,
    BerryBush,
    StoneDeposit,
    IronOre,
    
    // Units
    Worker,
    
    // Buildings
    House,
    Stockpile,
    Granary,
    Sawmill,
    Quarry,
    Farm,
    
    // Items
    ResourceItem(ResourceType),
}

/// Types of resources in the game
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResourceType {
    // Raw materials
    Wood,
    Stone,
    Iron,
    Berries,
    Wheat,
    Food, // Generic food
    
    // Processed materials
    Planks,
    Bread,
    Tools,
}

/// Types of buildings that can be constructed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BuildingType {
    House,
    Stockpile,
    Granary,
    Sawmill,
    Quarry,
    Farm,
    Bakery,
    Smithy,
}

/// Reasons why an entity was destroyed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DestroyReason {
    Harvested,
    Demolished,
    Decayed,
    Attacked,
}

/// Reasons for population changes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PopulationChangeReason {
    Birth,
    Death,
    Immigration,
    Emigration,
}

/// Seasons in the game
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

/// Weather conditions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Weather {
    Clear,
    Rain,
    Snow,
    Storm,
}

/// Types of tasks workers can perform
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    Idle,
    Harvesting,
    Building,
    Crafting,
    Hauling,
    Farming,
}

/// Task assignments for workers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskAssignment {
    Harvest { target: super::EntityId },
    Build { building: super::EntityId },
    Craft { recipe: String, building: super::EntityId },
    Haul { from: super::Position, to: super::Position, resource: ResourceType },
    Farm { field: super::EntityId },
    Idle,
}