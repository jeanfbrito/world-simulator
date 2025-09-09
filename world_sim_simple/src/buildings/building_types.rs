use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::resources::{ResourceType, ItemType};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingType {
    // Storage
    Storage,
    Warehouse,
    Stockpile,  // For wood, stone, and other raw materials
    Granary,    // For food storage
    
    // Production
    Lumbermill,
    Quarry,
    Mine,
    Farm,
    
    // Processing
    Smelter,
    Workshop,
    Kitchen,
    
    // Residential
    House,
    Barracks,
    
    // Defense
    WallSection,
    Tower,
    Gate,
}

impl BuildingType {
    pub fn name(&self) -> &str {
        match self {
            Self::Storage => "Storage",
            Self::Warehouse => "Warehouse",
            Self::Stockpile => "Stockpile",
            Self::Granary => "Granary",
            Self::Lumbermill => "Lumbermill",
            Self::Quarry => "Quarry",
            Self::Mine => "Mine",
            Self::Farm => "Farm",
            Self::Smelter => "Smelter",
            Self::Workshop => "Workshop",
            Self::Kitchen => "Kitchen",
            Self::House => "House",
            Self::Barracks => "Barracks",
            Self::WallSection => "Wall Section",
            Self::Tower => "Tower",
            Self::Gate => "Gate",
        }
    }
    
    pub fn size(&self) -> BuildingSize {
        match self {
            Self::Storage => BuildingSize::Small,
            Self::House => BuildingSize::Small,
            Self::WallSection => BuildingSize::Small,
            Self::Stockpile => BuildingSize::Medium,
            Self::Granary => BuildingSize::Medium,
            
            Self::Lumbermill | Self::Quarry | Self::Kitchen => BuildingSize::Medium,
            Self::Workshop | Self::Farm => BuildingSize::Medium,
            
            Self::Warehouse | Self::Mine | Self::Smelter => BuildingSize::Large,
            Self::Barracks | Self::Tower | Self::Gate => BuildingSize::Large,
        }
    }
    
    pub fn requirements(&self) -> BuildingRequirements {
        let mut resources = HashMap::new();
        
        match self {
            Self::Storage => {
                resources.insert(ResourceType::Wood, 20);
                resources.insert(ResourceType::Stone, 10);
            },
            Self::Stockpile => {
                resources.insert(ResourceType::Wood, 10);
                resources.insert(ResourceType::Stone, 5);
            },
            Self::Granary => {
                resources.insert(ResourceType::Wood, 15);
                resources.insert(ResourceType::Stone, 10);
            },
            Self::Warehouse => {
                resources.insert(ResourceType::Wood, 50);
                resources.insert(ResourceType::Stone, 30);
                resources.insert(ResourceType::IronIngot, 10);
            },
            Self::Lumbermill => {
                resources.insert(ResourceType::Wood, 30);
                resources.insert(ResourceType::Stone, 20);
                resources.insert(ResourceType::IronIngot, 5);
            },
            Self::Quarry => {
                resources.insert(ResourceType::Wood, 20);
                resources.insert(ResourceType::Stone, 40);
                resources.insert(ResourceType::IronIngot, 10);
            },
            Self::Mine => {
                resources.insert(ResourceType::Wood, 40);
                resources.insert(ResourceType::Stone, 60);
                resources.insert(ResourceType::IronIngot, 20);
            },
            Self::Farm => {
                resources.insert(ResourceType::Wood, 25);
                resources.insert(ResourceType::Stone, 5);
            },
            Self::Smelter => {
                resources.insert(ResourceType::Stone, 50);
                resources.insert(ResourceType::Clay, 20);
                resources.insert(ResourceType::IronIngot, 15);
            },
            Self::Workshop => {
                resources.insert(ResourceType::Wood, 35);
                resources.insert(ResourceType::Stone, 25);
                resources.insert(ResourceType::IronIngot, 10);
            },
            Self::Kitchen => {
                resources.insert(ResourceType::Wood, 20);
                resources.insert(ResourceType::Stone, 15);
                resources.insert(ResourceType::Clay, 10);
            },
            Self::House => {
                resources.insert(ResourceType::Wood, 15);
                resources.insert(ResourceType::Stone, 10);
            },
            Self::Barracks => {
                resources.insert(ResourceType::Wood, 30);
                resources.insert(ResourceType::Stone, 40);
                resources.insert(ResourceType::IronIngot, 10);
            },
            Self::WallSection => {
                resources.insert(ResourceType::Stone, 30);
            },
            Self::Tower => {
                resources.insert(ResourceType::Stone, 50);
                resources.insert(ResourceType::Wood, 20);
            },
            Self::Gate => {
                resources.insert(ResourceType::Stone, 40);
                resources.insert(ResourceType::IronIngot, 20);
            },
        }
        
        BuildingRequirements {
            resources,
            build_time: self.build_time(),
        }
    }
    
    fn build_time(&self) -> f32 {
        match self.size() {
            BuildingSize::Small => 10.0,
            BuildingSize::Medium => 20.0,
            BuildingSize::Large => 30.0,
        }
    }
    
    pub fn production(&self) -> Option<ProductionInfo> {
        match self {
            Self::Lumbermill => Some(ProductionInfo {
                input: vec![],
                output: vec![(ResourceType::Wood, 1)],
                rate: 1.0,
            }),
            Self::Quarry => Some(ProductionInfo {
                input: vec![],
                output: vec![(ResourceType::Stone, 1)],
                rate: 0.8,
            }),
            Self::Mine => Some(ProductionInfo {
                input: vec![],
                output: vec![(ResourceType::IronOre, 1)],
                rate: 0.5,
            }),
            Self::Farm => Some(ProductionInfo {
                input: vec![],
                output: vec![(ResourceType::Wheat, 2)],
                rate: 0.3,
            }),
            Self::Smelter => Some(ProductionInfo {
                input: vec![(ResourceType::IronOre, 2), (ResourceType::Coal, 1)],
                output: vec![(ResourceType::IronIngot, 1)],
                rate: 0.4,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingSize {
    Small,  // 1x1
    Medium, // 2x2
    Large,  // 3x3
}

impl BuildingSize {
    pub fn tiles(&self) -> usize {
        match self {
            Self::Small => 1,
            Self::Medium => 2,
            Self::Large => 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingRequirements {
    pub resources: HashMap<ResourceType, u32>,
    pub build_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionInfo {
    pub input: Vec<(ResourceType, u32)>,
    pub output: Vec<(ResourceType, u32)>,
    pub rate: f32, // Items per second
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct BuildingComponent {
    pub building_type: BuildingType,
    pub health: f32,
    pub max_health: f32,
    pub construction_progress: f32,
    pub is_active: bool,
    pub position: (i32, i32),
}

impl BuildingComponent {
    pub fn new(building_type: BuildingType, position: (i32, i32)) -> Self {
        let max_health = match building_type.size() {
            BuildingSize::Small => 100.0,
            BuildingSize::Medium => 200.0,
            BuildingSize::Large => 300.0,
        };
        
        Self {
            building_type,
            health: max_health,
            max_health,
            construction_progress: 0.0,
            is_active: false,
            position,
        }
    }
    
    pub fn is_complete(&self) -> bool {
        self.construction_progress >= 1.0
    }
    
    pub fn damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
        if self.health == 0.0 {
            self.is_active = false;
        }
    }
    
    pub fn repair(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }
}