// Data Definitions
// Structures for all data types loaded from Lua

use serde::{Deserialize, Serialize};

/// Resource definition loaded from Lua
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub properties: ResourceProperties,
    pub harvestable: Option<HarvestableConfig>,
    pub spawn: Option<SpawnConfig>,
    pub visuals: Option<VisualConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceProperties {
    pub weight: f32,
    pub stack_size: i32,
    pub base_value: i32,
    pub quality: Option<f32>,
    pub durability: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableConfig {
    pub tool_required: Option<String>,
    pub r#yield: Vec<YieldConfig>,
    pub respawn_time: Option<f32>,
    pub growth_stages: Option<i32>,
    pub stage_time: Option<f32>,
    pub requires_water: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldConfig {
    pub item: String,
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnConfig {
    pub biomes: Vec<String>,
    pub frequency: f32,
    pub cluster_size: ClusterSize,
    pub min_distance: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterSize {
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualConfig {
    pub sprite: Option<String>,
    pub color_variation: Option<bool>,
    pub size_variation: Option<Vec<f32>>,
}

/// Item definition loaded from Lua
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub properties: ItemProperties,
    pub tool: Option<ToolProperties>,
    pub consumable: Option<ConsumableProperties>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemProperties {
    pub weight: f32,
    pub stack_size: i32,
    pub value: i32,
    pub rarity: Option<String>,
    pub tradeable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolProperties {
    pub tool_type: String,
    pub material: String,
    pub durability: f32,
    pub max_durability: f32,
    pub efficiency: f32,
    pub repairable: Option<bool>,
    pub repair_cost: Option<Vec<RepairCost>>,
    pub can_harvest: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairCost {
    pub item: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableProperties {
    pub effects: Vec<ConsumableEffect>,
    pub cooldown: Option<f32>,
    pub perishable: Option<bool>,
    pub perish_time: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableEffect {
    pub effect_type: String,
    pub amount: f32,
    pub duration: Option<f32>,
}

/// Recipe definition loaded from Lua
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub requirements: Vec<RecipeRequirement>,
    pub outputs: Vec<RecipeOutput>,
    pub crafting: CraftingConfig,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeRequirement {
    pub item: String,
    pub count: i32,
    pub consume: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeOutput {
    pub item: String,
    pub count: i32,
    pub chance: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingConfig {
    pub time: f32,
    pub station: Option<String>,
    pub skill_required: Option<std::collections::HashMap<String, i32>>,
    pub unlock_condition: Option<String>,
}

/// Entity definition loaded from Lua
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDefinition {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub description: Option<String>,
    pub properties: EntityProperties,
    pub unit: Option<UnitProperties>,
    pub building: Option<BuildingProperties>,
    pub spawn: Option<EntitySpawnConfig>,
    pub visuals: Option<VisualConfig>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityProperties {
    pub health: f32,
    pub max_health: f32,
    pub size: Option<EntitySize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySize {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitProperties {
    pub movement_speed: f32,
    pub energy: f32,
    pub max_energy: f32,
    pub needs: UnitNeeds,
    pub inventory: UnitInventory,
    pub behaviors: Vec<String>,
    pub work_speed: Option<f32>,
    pub skills: Option<std::collections::HashMap<String, i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitNeeds {
    pub hunger_decay: f32,
    pub energy_decay: f32,
    pub thirst_decay: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitInventory {
    pub slots: i32,
    pub starting_items: Option<Vec<StartingItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartingItem {
    pub item: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingProperties {
    pub construction: ConstructionConfig,
    pub storage: Option<StorageConfig>,
    pub production: Option<ProductionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionConfig {
    pub requirements: Vec<ConstructionRequirement>,
    pub time: f32,
    pub builder_required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionRequirement {
    pub item: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub capacity: i32,
    pub allowed_items: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    pub recipes: Vec<String>,
    pub speed_multiplier: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySpawnConfig {
    pub initial_count: Option<i32>,
    pub spawn_area: Option<SpawnArea>,
    pub require_walkable: Option<bool>,

    // Biome and terrain-specific spawning
    #[serde(default)]
    pub preferred_terrain: Option<Vec<String>>,
    #[serde(default)]
    pub avoided_terrain: Option<Vec<String>>,
    #[serde(default)]
    pub preferred_biomes: Option<Vec<String>>,
    #[serde(default)]
    pub min_fertility: Option<f32>,
    #[serde(default)]
    pub max_elevation: Option<f32>,
    #[serde(default)]
    pub moisture_range: Option<SpawnRange>,
    #[serde(default)]
    pub temperature_range: Option<SpawnRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRange {
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnArea {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}