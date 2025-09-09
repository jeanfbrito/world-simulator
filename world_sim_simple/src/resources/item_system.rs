use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use super::ResourceType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    Resource(ResourceType),
    Tool(ToolType),
    Weapon(WeaponType),
    Armor(ArmorType),
    Consumable(ConsumableType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolType {
    Pickaxe,
    Axe,
    Shovel,
    Hammer,
    Saw,
    FishingRod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Bow,
    Spear,
    Dagger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Shield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsumableType {
    HealthPotion,
    EnergyPotion,
    Food(ResourceType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl ItemRarity {
    pub fn color(&self) -> Color {
        match self {
            Self::Common => Color::srgb(0.7, 0.7, 0.7),
            Self::Uncommon => Color::srgb(0.2, 0.8, 0.2),
            Self::Rare => Color::srgb(0.2, 0.4, 0.9),
            Self::Epic => Color::srgb(0.6, 0.2, 0.8),
            Self::Legendary => Color::srgb(1.0, 0.6, 0.1),
        }
    }
    
    pub fn value_multiplier(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.5,
            Self::Rare => 2.5,
            Self::Epic => 5.0,
            Self::Legendary => 10.0,
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub item_type: ItemType,
    pub name: String,
    pub description: String,
    pub rarity: ItemRarity,
    pub weight: f32,
    pub value: u32,
    pub durability: Option<f32>,
    pub max_durability: Option<f32>,
}

impl Item {
    pub fn new_resource(resource: ResourceType) -> Self {
        Self {
            item_type: ItemType::Resource(resource),
            name: format!("{:?}", resource),
            description: format!("A piece of {:?}", resource),
            rarity: ItemRarity::Common,
            weight: resource.weight(),
            value: resource.base_value(),
            durability: None,
            max_durability: None,
        }
    }
    
    pub fn new_tool(tool_type: ToolType, material: ResourceType) -> Self {
        let (durability, max_durability) = match material {
            ResourceType::Wood => (50.0, 50.0),
            ResourceType::Stone => (100.0, 100.0),
            ResourceType::IronIngot => (200.0, 200.0),
            ResourceType::GoldIngot => (40.0, 40.0),
            _ => (100.0, 100.0),
        };
        
        Self {
            item_type: ItemType::Tool(tool_type),
            name: format!("{:?} {:?}", material, tool_type),
            description: format!("A {:?} made of {:?}", tool_type, material),
            rarity: match material {
                ResourceType::Wood => ItemRarity::Common,
                ResourceType::Stone => ItemRarity::Common,
                ResourceType::IronIngot => ItemRarity::Uncommon,
                ResourceType::GoldIngot => ItemRarity::Rare,
                _ => ItemRarity::Common,
            },
            weight: 2.0,
            value: material.base_value() * 3,
            durability: Some(durability),
            max_durability: Some(max_durability),
        }
    }
    
    pub fn damage(&mut self, amount: f32) -> bool {
        if let Some(durability) = self.durability.as_mut() {
            *durability = (*durability - amount).max(0.0);
            *durability <= 0.0 // Returns true if broken
        } else {
            false
        }
    }
    
    pub fn repair(&mut self, amount: f32) {
        if let (Some(durability), Some(max)) = (self.durability.as_mut(), self.max_durability) {
            *durability = (*durability + amount).min(max);
        }
    }
    
    pub fn stack_size(&self) -> u32 {
        match self.item_type {
            ItemType::Resource(r) => r.stack_size(),
            ItemType::Tool(_) | ItemType::Weapon(_) | ItemType::Armor(_) => 1,
            ItemType::Consumable(_) => 10,
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub item: Item,
    pub count: u32,
}

impl ItemStack {
    pub fn new(item: Item, count: u32) -> Self {
        Self {
            count: count.min(item.stack_size()),
            item,
        }
    }
    
    pub fn add(&mut self, amount: u32) -> u32 {
        let max = self.item.stack_size();
        let can_add = (max - self.count).min(amount);
        self.count += can_add;
        amount - can_add // Returns overflow
    }
    
    pub fn remove(&mut self, amount: u32) -> u32 {
        let can_remove = self.count.min(amount);
        self.count -= can_remove;
        can_remove
    }
    
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    
    pub fn is_full(&self) -> bool {
        self.count >= self.item.stack_size()
    }
    
    pub fn total_weight(&self) -> f32 {
        self.item.weight * self.count as f32
    }
    
    pub fn total_value(&self) -> u32 {
        self.item.value * self.count
    }
}