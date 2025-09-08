//! Entity type definitions

use serde::{Deserialize, Serialize};

/// Types of entities in the simulation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    Building(BuildingType),
    
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
    Gold,
    Water,
    Berries,
    Wheat,
    Food, // Generic food
    Livestock,
    
    // Processed materials
    Planks,
    Bread,
    Tools,
    Leather,
}

/// Types of buildings that can be constructed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BuildingType {
    House,
    Stockpile,
    Granary,
    Sawmill,
    Quarry,
    Mine,
    Farm,
    Bakery,
    Smithy,
    Barracks,
    Workshop,
    Market,
    Well,
    Tavern,
    Temple,
    Wall,
    Tower,
    Gate,
    Warehouse,
    Brewery,
    Butcher,
    Fishery,
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

/// Worker states
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkerState {
    Idle,
    Working,
    Moving,
    Sleeping,
    Eating,
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

/// Recipe identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecipeId(String);

impl RecipeId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for RecipeId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for RecipeId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Recipe definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: String,
    pub inputs: std::collections::HashMap<ResourceType, u32>,
    pub outputs: std::collections::HashMap<ResourceType, u32>,
    pub duration_ticks: u64,
    pub required_building: Option<BuildingType>,
}