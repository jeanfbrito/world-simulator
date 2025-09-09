use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    // Raw Materials
    Wood,
    Stone,
    IronOre,
    CopperOre,
    GoldOre,
    Coal,
    Sand,
    Clay,
    
    // Processed Materials
    IronIngot,
    CopperIngot,
    GoldIngot,
    Glass,
    Brick,
    Plank,
    
    // Food Resources
    Wheat,
    Corn,
    Berries,
    Fish,
    Meat,
    Bread,
    
    // Energy Resources
    Firewood,
    Charcoal,
    Oil,
    
    // Special Resources
    Gem,
    Crystal,
    MagicDust,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceCategory {
    RawMaterial,
    ProcessedMaterial,
    Food,
    Energy,
    Special,
}

impl ResourceType {
    pub fn category(&self) -> ResourceCategory {
        match self {
            Self::Wood | Self::Stone | Self::IronOre | Self::CopperOre 
            | Self::GoldOre | Self::Coal | Self::Sand | Self::Clay => ResourceCategory::RawMaterial,
            
            Self::IronIngot | Self::CopperIngot | Self::GoldIngot 
            | Self::Glass | Self::Brick | Self::Plank => ResourceCategory::ProcessedMaterial,
            
            Self::Wheat | Self::Corn | Self::Berries | Self::Fish 
            | Self::Meat | Self::Bread => ResourceCategory::Food,
            
            Self::Firewood | Self::Charcoal | Self::Oil => ResourceCategory::Energy,
            
            Self::Gem | Self::Crystal | Self::MagicDust => ResourceCategory::Special,
        }
    }
    
    pub fn base_value(&self) -> u32 {
        match self {
            // Raw materials
            Self::Wood => 2,
            Self::Stone => 1,
            Self::Sand => 1,
            Self::Clay => 2,
            Self::Coal => 5,
            Self::IronOre => 8,
            Self::CopperOre => 6,
            Self::GoldOre => 20,
            
            // Processed materials
            Self::Plank => 5,
            Self::Brick => 4,
            Self::Glass => 6,
            Self::IronIngot => 15,
            Self::CopperIngot => 12,
            Self::GoldIngot => 40,
            
            // Food
            Self::Wheat => 3,
            Self::Corn => 3,
            Self::Berries => 2,
            Self::Fish => 5,
            Self::Meat => 8,
            Self::Bread => 10,
            
            // Energy
            Self::Firewood => 3,
            Self::Charcoal => 8,
            Self::Oil => 15,
            
            // Special
            Self::Gem => 50,
            Self::Crystal => 30,
            Self::MagicDust => 100,
        }
    }
    
    pub fn stack_size(&self) -> u32 {
        match self.category() {
            ResourceCategory::RawMaterial => 100,
            ResourceCategory::ProcessedMaterial => 50,
            ResourceCategory::Food => 20,
            ResourceCategory::Energy => 40,
            ResourceCategory::Special => 10,
        }
    }
    
    pub fn weight(&self) -> f32 {
        match self {
            Self::Stone | Self::IronOre | Self::CopperOre | Self::GoldOre => 2.0,
            Self::Wood | Self::Coal => 1.5,
            Self::IronIngot | Self::CopperIngot | Self::GoldIngot => 1.8,
            Self::Sand | Self::Clay => 1.2,
            Self::Glass | Self::Brick => 1.0,
            Self::Plank => 0.8,
            Self::Wheat | Self::Corn | Self::Berries => 0.2,
            Self::Fish | Self::Meat => 0.5,
            Self::Bread => 0.3,
            Self::Firewood | Self::Charcoal => 0.6,
            Self::Oil => 0.4,
            Self::Gem | Self::Crystal => 0.3,
            Self::MagicDust => 0.1,
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceProperties {
    pub resource_type: ResourceType,
    pub quality: f32, // 0.0 to 1.0
    pub durability: Option<f32>, // For tools/equipment
}

impl ResourceProperties {
    pub fn new(resource_type: ResourceType) -> Self {
        Self {
            resource_type,
            quality: 1.0,
            durability: None,
        }
    }
    
    pub fn with_quality(mut self, quality: f32) -> Self {
        self.quality = quality.clamp(0.0, 1.0);
        self
    }
    
    pub fn value(&self) -> u32 {
        (self.resource_type.base_value() as f32 * self.quality) as u32
    }
}