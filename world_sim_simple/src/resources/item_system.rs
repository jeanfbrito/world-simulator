use super::ResourceType;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::packs::definitions::ItemDefinition;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    Resource(ResourceType),
    Tool(ToolType),
    Weapon(WeaponType),
    Armor(ArmorType),
    Consumable(ConsumableType),
    Custom(String), // For dynamically loaded items
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolType {
    Pickaxe,
    Axe,
    Shovel,
    Hammer,
    Saw,
    FishingRod,
    Custom(String), // For custom tools
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Bow,
    Spear,
    Dagger,
    Custom(String), // For custom weapons
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Shield,
    Custom(String), // For custom armor
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsumableType {
    HealthPotion,
    EnergyPotion,
    Food(ResourceType),
    Custom(String), // For custom consumables
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
        let resource_clone = resource.clone();
        Self {
            item_type: ItemType::Resource(resource_clone.clone()),
            name: format!("{:?}", resource_clone),
            description: format!("A piece of {:?}", resource_clone),
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
            item_type: ItemType::Tool(tool_type.clone()),
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
        match &self.item_type {
            ItemType::Resource(r) => r.stack_size(),
            ItemType::Tool(_) | ItemType::Weapon(_) | ItemType::Armor(_) => 1,
            ItemType::Consumable(_) => 10,
            ItemType::Custom(_) => 50,
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

/// Global item registry for dynamic pack-loaded items
#[derive(Resource, Debug, Clone, Default)]
pub struct ItemRegistry {
    pub definitions: HashMap<String, ItemDefinition>,
    pub name_to_type: HashMap<String, ItemType>,
    pub type_to_definition: HashMap<ItemType, ItemDefinition>,
}

impl ItemRegistry {
    pub fn register_item(&mut self, definition: ItemDefinition) {
        let id = definition.id.clone();
        let item_type = ItemType::Custom(id.clone());

        self.definitions.insert(id.clone(), definition.clone());
        self.name_to_type.insert(id, item_type.clone());
        self.type_to_definition.insert(item_type, definition);
    }

    pub fn get_definition(&self, item_type: &ItemType) -> Option<&ItemDefinition> {
        match item_type {
            ItemType::Custom(id) => self.definitions.get(id),
            _ => self.type_to_definition.get(item_type),
        }
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<ItemType> {
        self.name_to_type.get(name).cloned()
    }

    pub fn get_all_types(&self) -> Vec<ItemType> {
        let mut types = vec![];

        // Add all resource items (hardcoded for now)
        use crate::resources::ResourceType;
        types.extend([
            ItemType::Resource(ResourceType::Wood),
            ItemType::Resource(ResourceType::Stone),
            ItemType::Resource(ResourceType::IronOre),
            ItemType::Resource(ResourceType::CopperOre),
            ItemType::Resource(ResourceType::GoldOre),
            ItemType::Resource(ResourceType::Coal),
            ItemType::Resource(ResourceType::Sand),
            ItemType::Resource(ResourceType::Clay),
            ItemType::Resource(ResourceType::IronIngot),
            ItemType::Resource(ResourceType::CopperIngot),
            ItemType::Resource(ResourceType::GoldIngot),
            ItemType::Resource(ResourceType::Glass),
            ItemType::Resource(ResourceType::Brick),
            ItemType::Resource(ResourceType::Plank),
            ItemType::Resource(ResourceType::Wheat),
            ItemType::Resource(ResourceType::Corn),
            ItemType::Resource(ResourceType::Berries),
            ItemType::Resource(ResourceType::Fish),
            ItemType::Resource(ResourceType::Meat),
            ItemType::Resource(ResourceType::Bread),
            ItemType::Resource(ResourceType::Firewood),
            ItemType::Resource(ResourceType::Charcoal),
            ItemType::Resource(ResourceType::Oil),
            ItemType::Resource(ResourceType::Gem),
            ItemType::Resource(ResourceType::Crystal),
            ItemType::Resource(ResourceType::MagicDust),
        ]);

        // Add all tool types
        types.extend([
            ItemType::Tool(ToolType::Pickaxe),
            ItemType::Tool(ToolType::Axe),
            ItemType::Tool(ToolType::Shovel),
            ItemType::Tool(ToolType::Hammer),
            ItemType::Tool(ToolType::Saw),
            ItemType::Tool(ToolType::FishingRod),
        ]);

        // Add all weapon types
        types.extend([
            ItemType::Weapon(WeaponType::Sword),
            ItemType::Weapon(WeaponType::Bow),
            ItemType::Weapon(WeaponType::Spear),
            ItemType::Weapon(WeaponType::Dagger),
        ]);

        // Add all armor types
        types.extend([
            ItemType::Armor(ArmorType::Helmet),
            ItemType::Armor(ArmorType::Chestplate),
            ItemType::Armor(ArmorType::Leggings),
            ItemType::Armor(ArmorType::Boots),
            ItemType::Armor(ArmorType::Shield),
        ]);

        // Add all consumable types
        types.extend([
            ItemType::Consumable(ConsumableType::HealthPotion),
            ItemType::Consumable(ConsumableType::EnergyPotion),
        ]);

        // Add custom items
        for custom_id in self.definitions.keys() {
            types.push(ItemType::Custom(custom_id.clone()));
        }

        types
    }
}

impl ToolType {
    /// Convert from string to tool type (for pack loading)
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "pickaxe" => Some(ToolType::Pickaxe),
            "axe" => Some(ToolType::Axe),
            "shovel" => Some(ToolType::Shovel),
            "hammer" => Some(ToolType::Hammer),
            "saw" => Some(ToolType::Saw),
            "fishing_rod" => Some(ToolType::FishingRod),
            _ => Some(ToolType::Custom(name.to_string())),
        }
    }

    /// Get the string name of the tool type
    pub fn name(&self) -> String {
        match self {
            ToolType::Pickaxe => "pickaxe".to_string(),
            ToolType::Axe => "axe".to_string(),
            ToolType::Shovel => "shovel".to_string(),
            ToolType::Hammer => "hammer".to_string(),
            ToolType::Saw => "saw".to_string(),
            ToolType::FishingRod => "fishing_rod".to_string(),
            ToolType::Custom(name) => name.clone(),
        }
    }
}

impl WeaponType {
    /// Convert from string to weapon type (for pack loading)
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "sword" => Some(WeaponType::Sword),
            "bow" => Some(WeaponType::Bow),
            "spear" => Some(WeaponType::Spear),
            "dagger" => Some(WeaponType::Dagger),
            _ => Some(WeaponType::Custom(name.to_string())),
        }
    }

    /// Get the string name of the weapon type
    pub fn name(&self) -> String {
        match self {
            WeaponType::Sword => "sword".to_string(),
            WeaponType::Bow => "bow".to_string(),
            WeaponType::Spear => "spear".to_string(),
            WeaponType::Dagger => "dagger".to_string(),
            WeaponType::Custom(name) => name.clone(),
        }
    }
}

impl ArmorType {
    /// Convert from string to armor type (for pack loading)
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "helmet" => Some(ArmorType::Helmet),
            "chestplate" => Some(ArmorType::Chestplate),
            "leggings" => Some(ArmorType::Leggings),
            "boots" => Some(ArmorType::Boots),
            "shield" => Some(ArmorType::Shield),
            _ => Some(ArmorType::Custom(name.to_string())),
        }
    }

    /// Get the string name of the armor type
    pub fn name(&self) -> String {
        match self {
            ArmorType::Helmet => "helmet".to_string(),
            ArmorType::Chestplate => "chestplate".to_string(),
            ArmorType::Leggings => "leggings".to_string(),
            ArmorType::Boots => "boots".to_string(),
            ArmorType::Shield => "shield".to_string(),
            ArmorType::Custom(name) => name.clone(),
        }
    }
}

impl ConsumableType {
    /// Convert from string to consumable type (for pack loading)
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "health_potion" => Some(ConsumableType::HealthPotion),
            "energy_potion" => Some(ConsumableType::EnergyPotion),
            _ => {
                // Try to parse as food resource (hardcoded for now)
                use crate::resources::ResourceType;
                if let Some(resource_type) = ResourceType::from_str(name) {
                    Some(ConsumableType::Food(resource_type))
                } else {
                    Some(ConsumableType::Custom(name.to_string()))
                }
            }
        }
    }

    /// Get the string name of the consumable type
    pub fn name(&self) -> String {
        match self {
            ConsumableType::HealthPotion => "health_potion".to_string(),
            ConsumableType::EnergyPotion => "energy_potion".to_string(),
            ConsumableType::Food(resource_type) => resource_type.name(),
            ConsumableType::Custom(name) => name.clone(),
        }
    }
}

impl ItemType {
    /// Convert from string to item type (for pack loading)
    pub fn from_str(name: &str) -> Option<Self> {
        // Try to parse as tool
        if let Some(tool_type) = ToolType::from_str(name) {
            return Some(ItemType::Tool(tool_type));
        }

        // Try to parse as weapon
        if let Some(weapon_type) = WeaponType::from_str(name) {
            return Some(ItemType::Weapon(weapon_type));
        }

        // Try to parse as armor
        if let Some(armor_type) = ArmorType::from_str(name) {
            return Some(ItemType::Armor(armor_type));
        }

        // Try to parse as consumable
        if let Some(consumable_type) = ConsumableType::from_str(name) {
            return Some(ItemType::Consumable(consumable_type));
        }

        // Try to parse as resource (hardcoded for now)
        if let Some(resource_type) = ResourceType::from_str(name) {
            return Some(ItemType::Resource(resource_type));
        }

        // Fallback to custom item
        Some(ItemType::Custom(name.to_string()))
    }

    /// Get the string name of the item type
    pub fn name(&self) -> String {
        match self {
            ItemType::Resource(resource_type) => resource_type.name(),
            ItemType::Tool(tool_type) => tool_type.name(),
            ItemType::Weapon(weapon_type) => weapon_type.name(),
            ItemType::Armor(armor_type) => armor_type.name(),
            ItemType::Consumable(consumable_type) => consumable_type.name(),
            ItemType::Custom(name) => name.clone(),
        }
    }

    /// Get the category of this item type
    pub fn category(&self) -> String {
        match self {
            ItemType::Resource(_) => "resource".to_string(),
            ItemType::Tool(_) => "tool".to_string(),
            ItemType::Weapon(_) => "weapon".to_string(),
            ItemType::Armor(_) => "armor".to_string(),
            ItemType::Consumable(_) => "consumable".to_string(),
            ItemType::Custom(_) => "custom".to_string(),
        }
    }
}
