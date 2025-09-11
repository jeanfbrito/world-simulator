use crate::resources::{Item, ItemType, ResourceType, ToolType};
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

    pub fn get(&self, id: &str) -> Option<&Recipe> {
        self.recipes.get(id)
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
