use crate::resources::{Item, ItemRarity, ItemType, ResourceType, ToolType, ResourceRegistry, ItemRegistry};
use crate::packs::definitions::{RecipeDefinition, RecipeRequirement as PackRecipeRequirement, RecipeOutput as PackRecipeOutput, CraftingConfig};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeRequirement {
    pub item_type: ItemType,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeOutput {
    pub item: Item,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requirements: Vec<RecipeRequirement>,
    pub outputs: Vec<RecipeOutput>,
    pub crafting_time: f32,
    pub station_required: Option<super::CraftingStationType>,
}

impl Recipe {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        let id_str = id.into();
        let name_str = name.into();
        Self {
            id: id_str.clone(),
            name: name_str.clone(),
            description: format!("Recipe for {}", name_str),
            requirements: Vec::new(),
            outputs: Vec::new(),
            crafting_time: 5.0,
            station_required: None,
        }
    }

    pub fn with_requirement(mut self, item_type: ItemType, quantity: u32) -> Self {
        self.requirements.push(RecipeRequirement {
            item_type,
            quantity,
        });
        self
    }

    pub fn with_output(mut self, item: Item, quantity: u32) -> Self {
        self.outputs.push(RecipeOutput { item, quantity });
        self
    }

    pub fn with_time(mut self, time: f32) -> Self {
        self.crafting_time = time;
        self
    }

    pub fn with_station(mut self, station: super::CraftingStationType) -> Self {
        self.station_required = Some(station);
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn can_craft(&self, inventory: &HashMap<ItemType, u32>) -> bool {
        for req in &self.requirements {
            if inventory.get(&req.item_type).copied().unwrap_or(0) < req.quantity {
                return false;
            }
        }
        true
    }
}

#[derive(Resource, Default)]
pub struct RecipeRegistry {
    recipes: HashMap<String, Recipe>,
    recipes_by_station: HashMap<super::CraftingStationType, Vec<String>>,
    pack_recipes: HashMap<String, RecipeDefinition>, // Store pack-loaded recipes
}

impl RecipeRegistry {
    pub fn register(&mut self, recipe: Recipe) {
        info!("[RECIPES] Registering recipe: {}", recipe.name);

        if let Some(station) = recipe.station_required {
            self.recipes_by_station
                .entry(station)
                .or_default()
                .push(recipe.id.clone());
        }

        self.recipes.insert(recipe.id.clone(), recipe);
    }

    pub fn register_pack_recipe(&mut self, definition: RecipeDefinition, resource_registry: &ResourceRegistry, item_registry: &ItemRegistry) {
        info!("[RECIPES] Registering pack recipe: {}", definition.name);

        // Convert pack recipe definition to internal recipe
        let recipe = self.convert_pack_recipe(definition, resource_registry, item_registry);

        if let Some(station) = recipe.station_required {
            self.recipes_by_station
                .entry(station)
                .or_default()
                .push(recipe.id.clone());
        }

        self.recipes.insert(recipe.id.clone(), recipe);
    }

    pub fn register_pack_recipes(&mut self, definitions: Vec<RecipeDefinition>, resource_registry: &ResourceRegistry, item_registry: &ItemRegistry) {
        for definition in definitions {
            self.pack_recipes.insert(definition.id.clone(), definition.clone());
            self.register_pack_recipe(definition, resource_registry, item_registry);
        }
    }

    fn convert_pack_recipe(&self, definition: RecipeDefinition, resource_registry: &ResourceRegistry, item_registry: &ItemRegistry) -> Recipe {
        let mut recipe = Recipe::new(&definition.id, &definition.name)
            .with_description(definition.description.unwrap_or_default())
            .with_time(definition.crafting.time);

        // Convert requirements
        for req in definition.requirements {
            let item_type = ItemType::from_str(&req.item)
                .unwrap_or_else(|| ItemType::Custom(req.item.clone()));
            recipe = recipe.with_requirement(item_type, req.count as u32);
        }

        // Convert outputs
        for output in definition.outputs {
            let item_type = ItemType::from_str(&output.item)
                .unwrap_or_else(|| ItemType::Custom(output.item.clone()));

            // Create item based on type
            let item = self.create_item_from_type(item_type, item_registry);
            recipe = recipe.with_output(item, output.count as u32);
        }

        // Convert station requirement
        if let Some(station_name) = definition.crafting.station {
            if let Some(station_type) = self.parse_station_type(&station_name) {
                recipe = recipe.with_station(station_type);
            }
        }

        recipe
    }

    fn create_item_from_type(&self, item_type: ItemType, item_registry: &ItemRegistry) -> Item {
        match item_type {
            ItemType::Resource(resource_type) => Item::new_resource(resource_type),
            ItemType::Tool(tool_type) => {
                // Default material for tools
                let material = ResourceType::Wood;
                Item::new_tool(tool_type, material)
            },
            ItemType::Weapon(weapon_type) => {
                // Create a basic weapon item
                Item {
                    item_type: ItemType::Weapon(weapon_type.clone()),
                    name: format!("{:?}", weapon_type),
                    description: format!("A {:?}", weapon_type),
                    rarity: ItemRarity::Common,
                    weight: 1.0,
                    value: 10,
                    durability: Some(100.0),
                    max_durability: Some(100.0),
                }
            },
            ItemType::Armor(armor_type) => {
                // Create a basic armor item
                Item {
                    item_type: ItemType::Armor(armor_type.clone()),
                    name: format!("{:?}", armor_type),
                    description: format!("A {:?}", armor_type),
                    rarity: ItemRarity::Common,
                    weight: 2.0,
                    value: 15,
                    durability: Some(100.0),
                    max_durability: Some(100.0),
                }
            },
            ItemType::Consumable(consumable_type) => {
                // Create a basic consumable item
                Item {
                    item_type: ItemType::Consumable(consumable_type.clone()),
                    name: format!("{:?}", consumable_type),
                    description: format!("A {:?}", consumable_type),
                    rarity: ItemRarity::Common,
                    weight: 0.5,
                    value: 5,
                    durability: None,
                    max_durability: None,
                }
            },
            ItemType::Custom(name) => {
                // Try to get from item registry, otherwise create basic item
                if let Some(definition) = item_registry.get_definition(&ItemType::Custom(name.clone())) {
                    Item {
                        item_type: ItemType::Custom(name),
                        name: definition.name.clone(),
                        description: definition.description.clone().unwrap_or_default(),
                        rarity: ItemRarity::Common,
                        weight: definition.properties.weight,
                        value: definition.properties.value as u32,
                        durability: None,
                        max_durability: None,
                    }
                } else {
                    Item {
                        item_type: ItemType::Custom(name),
                        name: "Custom Item".to_string(),
                        description: "A custom item".to_string(),
                        rarity: ItemRarity::Common,
                        weight: 1.0,
                        value: 10,
                        durability: None,
                        max_durability: None,
                    }
                }
            }
        }
    }

    fn parse_station_type(&self, station_name: &str) -> Option<super::CraftingStationType> {
        match station_name {
            "workbench" => Some(super::CraftingStationType::Workbench),
            "furnace" => Some(super::CraftingStationType::Furnace),
            "anvil" => Some(super::CraftingStationType::Anvil),
            "kitchen" => Some(super::CraftingStationType::Kitchen),
            "sawmill" => Some(super::CraftingStationType::Sawmill),
            _ => None,
        }
    }

    pub fn get(&self, id: &str) -> Option<&Recipe> {
        self.recipes.get(id)
    }

    pub fn get_pack_definition(&self, id: &str) -> Option<&RecipeDefinition> {
        self.pack_recipes.get(id)
    }

    pub fn get_for_station(&self, station: super::CraftingStationType) -> Vec<&Recipe> {
        self.recipes_by_station
            .get(&station)
            .map(|ids| ids.iter().filter_map(|id| self.recipes.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn get_craftable(&self, inventory: &HashMap<ItemType, u32>) -> Vec<&Recipe> {
        self.recipes
            .values()
            .filter(|recipe| recipe.can_craft(inventory))
            .collect()
    }

    pub fn count(&self) -> usize {
        self.recipes.len()
    }

    pub fn pack_count(&self) -> usize {
        self.pack_recipes.len()
    }

    // Legacy method for backward compatibility
    pub fn register_default_recipes(&mut self) {
        use super::CraftingStationType;

        // Basic tools
        self.register(
            Recipe::new("wooden_pickaxe", "Wooden Pickaxe")
                .with_requirement(ItemType::Resource(ResourceType::Wood), 3)
                .with_requirement(ItemType::Resource(ResourceType::Stone), 2)
                .with_output(Item::new_tool(ToolType::Pickaxe, ResourceType::Wood), 1)
                .with_time(3.0),
        );

        self.register(
            Recipe::new("stone_pickaxe", "Stone Pickaxe")
                .with_requirement(ItemType::Resource(ResourceType::Wood), 2)
                .with_requirement(ItemType::Resource(ResourceType::Stone), 3)
                .with_output(Item::new_tool(ToolType::Pickaxe, ResourceType::Stone), 1)
                .with_time(5.0)
                .with_station(CraftingStationType::Workbench),
        );

        self.register(
            Recipe::new("iron_pickaxe", "Iron Pickaxe")
                .with_requirement(ItemType::Resource(ResourceType::Wood), 2)
                .with_requirement(ItemType::Resource(ResourceType::IronIngot), 3)
                .with_output(
                    Item::new_tool(ToolType::Pickaxe, ResourceType::IronIngot),
                    1,
                )
                .with_time(8.0)
                .with_station(CraftingStationType::Anvil),
        );

        // Processing recipes
        self.register(
            Recipe::new("planks", "Wood Planks")
                .with_requirement(ItemType::Resource(ResourceType::Wood), 1)
                .with_output(Item::new_resource(ResourceType::Plank), 4)
                .with_time(2.0),
        );

        self.register(
            Recipe::new("iron_ingot", "Iron Ingot")
                .with_requirement(ItemType::Resource(ResourceType::IronOre), 2)
                .with_requirement(ItemType::Resource(ResourceType::Coal), 1)
                .with_output(Item::new_resource(ResourceType::IronIngot), 1)
                .with_time(10.0)
                .with_station(CraftingStationType::Furnace),
        );

        self.register(
            Recipe::new("glass", "Glass")
                .with_requirement(ItemType::Resource(ResourceType::Sand), 2)
                .with_requirement(ItemType::Resource(ResourceType::Coal), 1)
                .with_output(Item::new_resource(ResourceType::Glass), 1)
                .with_time(5.0)
                .with_station(CraftingStationType::Furnace),
        );

        self.register(
            Recipe::new("brick", "Brick")
                .with_requirement(ItemType::Resource(ResourceType::Clay), 2)
                .with_requirement(ItemType::Resource(ResourceType::Coal), 1)
                .with_output(Item::new_resource(ResourceType::Brick), 2)
                .with_time(6.0)
                .with_station(CraftingStationType::Furnace),
        );

        // Food recipes
        self.register(
            Recipe::new("bread", "Bread")
                .with_requirement(ItemType::Resource(ResourceType::Wheat), 3)
                .with_output(Item::new_resource(ResourceType::Bread), 1)
                .with_time(4.0)
                .with_station(CraftingStationType::Kitchen),
        );

        // More tools
        self.register(
            Recipe::new("wooden_axe", "Wooden Axe")
                .with_requirement(ItemType::Resource(ResourceType::Wood), 3)
                .with_requirement(ItemType::Resource(ResourceType::Stone), 2)
                .with_output(Item::new_tool(ToolType::Axe, ResourceType::Wood), 1)
                .with_time(3.0),
        );

        self.register(
            Recipe::new("fishing_rod", "Fishing Rod")
                .with_requirement(ItemType::Resource(ResourceType::Wood), 3)
                .with_output(Item::new_tool(ToolType::FishingRod, ResourceType::Wood), 1)
                .with_time(4.0)
                .with_station(CraftingStationType::Workbench),
        );
    }
}
