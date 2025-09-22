use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::packs::definitions::ResourceDefinition;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum ResourceType {
    // Core resources (hardcoded for backward compatibility)
    Wood,
    Stone,
    IronOre,
    CopperOre,
    GoldOre,
    Coal,
    Sand,
    Clay,
    IronIngot,
    CopperIngot,
    GoldIngot,
    Glass,
    Brick,
    Plank,
    Wheat,
    Corn,
    Berries,
    Fish,
    Meat,
    Bread,
    Firewood,
    Charcoal,
    Oil,
    Gem,
    Crystal,
    MagicDust,

    // Dynamic resources from packs
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum ResourceCategory {
    RawMaterial,
    ProcessedMaterial,
    Food,
    Energy,
    Special,
    Custom(String),
}

/// Global resource registry for dynamic pack-loaded resources
#[derive(Resource, Debug, Clone, Default)]
pub struct ResourceRegistry {
    pub definitions: HashMap<String, ResourceDefinition>,
    pub name_to_type: HashMap<String, ResourceType>,
    pub type_to_definition: HashMap<ResourceType, ResourceDefinition>,
}

impl ResourceRegistry {
    pub fn register_resource(&mut self, definition: ResourceDefinition) {
        let id = definition.id.clone();
        let resource_type = ResourceType::Custom(id.clone());

        self.definitions.insert(id.clone(), definition.clone());
        self.name_to_type.insert(id, resource_type.clone());
        self.type_to_definition.insert(resource_type, definition);
    }

    pub fn get_definition(&self, resource_type: &ResourceType) -> Option<&ResourceDefinition> {
        match resource_type {
            ResourceType::Custom(id) => self.definitions.get(id),
            _ => self.type_to_definition.get(resource_type),
        }
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<ResourceType> {
        self.name_to_type.get(name).cloned()
    }

    pub fn get_all_types(&self) -> Vec<ResourceType> {
        let mut types = vec![
            ResourceType::Wood,
            ResourceType::Stone,
            ResourceType::IronOre,
            ResourceType::CopperOre,
            ResourceType::GoldOre,
            ResourceType::Coal,
            ResourceType::Sand,
            ResourceType::Clay,
            ResourceType::IronIngot,
            ResourceType::CopperIngot,
            ResourceType::GoldIngot,
            ResourceType::Glass,
            ResourceType::Brick,
            ResourceType::Plank,
            ResourceType::Wheat,
            ResourceType::Corn,
            ResourceType::Berries,
            ResourceType::Fish,
            ResourceType::Meat,
            ResourceType::Bread,
            ResourceType::Firewood,
            ResourceType::Charcoal,
            ResourceType::Oil,
            ResourceType::Gem,
            ResourceType::Crystal,
            ResourceType::MagicDust,
        ];

        // Add custom resources
        for custom_id in self.definitions.keys() {
            types.push(ResourceType::Custom(custom_id.clone()));
        }

        types
    }
}

impl ResourceType {
    /// Get the category of this resource type
    pub fn category(&self) -> ResourceCategory {
        match self {
            Self::Wood
            | Self::Stone
            | Self::IronOre
            | Self::CopperOre
            | Self::GoldOre
            | Self::Coal
            | Self::Sand
            | Self::Clay => ResourceCategory::RawMaterial,

            Self::IronIngot
            | Self::CopperIngot
            | Self::GoldIngot
            | Self::Glass
            | Self::Brick
            | Self::Plank => ResourceCategory::ProcessedMaterial,

            Self::Wheat | Self::Corn | Self::Berries | Self::Fish | Self::Meat | Self::Bread => {
                ResourceCategory::Food
            }

            Self::Firewood | Self::Charcoal | Self::Oil => ResourceCategory::Energy,

            Self::Gem | Self::Crystal | Self::MagicDust => ResourceCategory::Special,

            Self::Custom(_) => ResourceCategory::Custom("dynamic".to_string()),
        }
    }

    /// Get base value from hardcoded defaults
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

            // Custom resources use registry
            Self::Custom(_) => 1, // Default value, will be overridden by registry
        }
    }

    /// Get stack size from hardcoded defaults
    pub fn stack_size(&self) -> u32 {
        match self.category() {
            ResourceCategory::RawMaterial => 100,
            ResourceCategory::ProcessedMaterial => 50,
            ResourceCategory::Food => 20,
            ResourceCategory::Energy => 40,
            ResourceCategory::Special => 10,
            ResourceCategory::Custom(_) => 50, // Default for custom
        }
    }

    /// Get weight from hardcoded defaults
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
            Self::Custom(_) => 1.0, // Default weight for custom
        }
    }

    /// Convert from string to resource type (for pack loading)
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "wood" => Some(ResourceType::Wood),
            "stone" => Some(ResourceType::Stone),
            "iron_ore" => Some(ResourceType::IronOre),
            "copper_ore" => Some(ResourceType::CopperOre),
            "gold_ore" => Some(ResourceType::GoldOre),
            "coal" => Some(ResourceType::Coal),
            "sand" => Some(ResourceType::Sand),
            "clay" => Some(ResourceType::Clay),
            "iron_ingot" => Some(ResourceType::IronIngot),
            "copper_ingot" => Some(ResourceType::CopperIngot),
            "gold_ingot" => Some(ResourceType::GoldIngot),
            "glass" => Some(ResourceType::Glass),
            "brick" => Some(ResourceType::Brick),
            "plank" => Some(ResourceType::Plank),
            "wheat" => Some(ResourceType::Wheat),
            "corn" => Some(ResourceType::Corn),
            "berries" => Some(ResourceType::Berries),
            "fish" => Some(ResourceType::Fish),
            "meat" => Some(ResourceType::Meat),
            "bread" => Some(ResourceType::Bread),
            "firewood" => Some(ResourceType::Firewood),
            "charcoal" => Some(ResourceType::Charcoal),
            "oil" => Some(ResourceType::Oil),
            "gem" => Some(ResourceType::Gem),
            "crystal" => Some(ResourceType::Crystal),
            "magic_dust" => Some(ResourceType::MagicDust),
            _ => Some(ResourceType::Custom(name.to_string())),
        }
    }

    /// Get the string name of the resource type
    pub fn name(&self) -> String {
        match self {
            ResourceType::Wood => "wood".to_string(),
            ResourceType::Stone => "stone".to_string(),
            ResourceType::IronOre => "iron_ore".to_string(),
            ResourceType::CopperOre => "copper_ore".to_string(),
            ResourceType::GoldOre => "gold_ore".to_string(),
            ResourceType::Coal => "coal".to_string(),
            ResourceType::Sand => "sand".to_string(),
            ResourceType::Clay => "clay".to_string(),
            ResourceType::IronIngot => "iron_ingot".to_string(),
            ResourceType::CopperIngot => "copper_ingot".to_string(),
            ResourceType::GoldIngot => "gold_ingot".to_string(),
            ResourceType::Glass => "glass".to_string(),
            ResourceType::Brick => "brick".to_string(),
            ResourceType::Plank => "plank".to_string(),
            ResourceType::Wheat => "wheat".to_string(),
            ResourceType::Corn => "corn".to_string(),
            ResourceType::Berries => "berries".to_string(),
            ResourceType::Fish => "fish".to_string(),
            ResourceType::Meat => "meat".to_string(),
            ResourceType::Bread => "bread".to_string(),
            ResourceType::Firewood => "firewood".to_string(),
            ResourceType::Charcoal => "charcoal".to_string(),
            ResourceType::Oil => "oil".to_string(),
            ResourceType::Gem => "gem".to_string(),
            ResourceType::Crystal => "crystal".to_string(),
            ResourceType::MagicDust => "magic_dust".to_string(),
            ResourceType::Custom(name) => name.clone(),
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceProperties {
    pub resource_type: ResourceType,
    pub quality: f32,            // 0.0 to 1.0
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
