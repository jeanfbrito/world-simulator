// Version 2 Pack Loader - Simpler approach without unsafe code
// Collects all definitions then registers them after loading

use super::{PackSystem, PackError, ResourceDefinition, ItemDefinition, RecipeDefinition, EntityDefinition};
use super::registry::{Registry, ResourceRegistry, ItemRegistry, RecipeRegistry, EntityRegistry};
use bevy::prelude::*;
use mlua::prelude::*;
use std::fs;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

// Storage for pending registrations
pub struct PendingRegistrations {
    resources: Arc<Mutex<VecDeque<ResourceDefinition>>>,
    items: Arc<Mutex<VecDeque<ItemDefinition>>>,
    recipes: Arc<Mutex<VecDeque<RecipeDefinition>>>,
    entities: Arc<Mutex<VecDeque<EntityDefinition>>>,
}

impl PendingRegistrations {
    pub fn new() -> Self {
        Self {
            resources: Arc::new(Mutex::new(VecDeque::new())),
            items: Arc::new(Mutex::new(VecDeque::new())),
            recipes: Arc::new(Mutex::new(VecDeque::new())),
            entities: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

impl PackSystem {
    /// Load all files in a category
    pub fn load_category(&mut self, category: &str) -> Result<(), PackError> {
        let category_path = self.pack_path.join("data").join(category);

        if !category_path.exists() {
            info!("[PACK] Category '{}' path does not exist, skipping", category);
            return Ok(());
        }

        // Walk through all .lua files in the category
        let mut loaded_count = 0;
        for entry in WalkDir::new(&category_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                debug!("[PACK] Loading file: {:?}", path);
                self.load_lua_file(path)?;
                loaded_count += 1;
            }
        }

        info!("[PACK] Loaded {} files from category '{}'", loaded_count, category);
        Ok(())
    }

    /// Load and execute a single Lua file
    fn load_lua_file(&mut self, path: &std::path::Path) -> Result<(), PackError> {
        let content = fs::read_to_string(path)?;

        self.lua.load(&content)
            .set_name(&format!("{:?}", path))
            .exec()
            .map_err(|e| PackError::LuaError {
                file: path.to_path_buf(),
                error: e,
            })?;

        Ok(())
    }

    /// Bind Lua API functions with pending registrations
    pub fn bind_lua_api(&mut self) -> Result<(), PackError> {
        let pending = PendingRegistrations::new();
        let debug = self.metadata.config.debug;

        // Bind register_resource
        {
            let resources = pending.resources.clone();
            self.lua.globals().set("register_resource",
                self.lua.create_function(move |lua, table: LuaTable| {
                    let def = parse_resource_definition(lua, table)?;
                    if debug {
                        eprintln!("[PACK] Queuing resource: {}", def.id);
                    }
                    resources.lock().unwrap().push_back(def);
                    Ok(())
                })?
            )?;
        }

        // Bind register_item
        {
            let items = pending.items.clone();
            self.lua.globals().set("register_item",
                self.lua.create_function(move |lua, table: LuaTable| {
                    let def = parse_item_definition(lua, table)?;
                    if debug {
                        eprintln!("[PACK] Queuing item: {}", def.id);
                    }
                    items.lock().unwrap().push_back(def);
                    Ok(())
                })?
            )?;
        }

        // Bind register_recipe
        {
            let recipes = pending.recipes.clone();
            self.lua.globals().set("register_recipe",
                self.lua.create_function(move |lua, table: LuaTable| {
                    let def = parse_recipe_definition(lua, table)?;
                    if debug {
                        eprintln!("[PACK] Queuing recipe: {}", def.id);
                    }
                    recipes.lock().unwrap().push_back(def);
                    Ok(())
                })?
            )?;
        }

        // Bind register_entity
        {
            let entities = pending.entities.clone();
            self.lua.globals().set("register_entity",
                self.lua.create_function(move |lua, table: LuaTable| {
                    let def = parse_entity_definition(lua, table)?;
                    if debug {
                        eprintln!("[PACK] Queuing entity: {}", def.id);
                    }
                    entities.lock().unwrap().push_back(def);
                    Ok(())
                })?
            )?;
        }

        // Store pending registrations for later processing
        self.pending_registrations = Some(pending);

        Ok(())
    }

    /// Process all pending registrations after loading
    pub fn process_pending_registrations(&mut self) -> Result<(), PackError> {
        if let Some(pending) = self.pending_registrations.take() {
            // Process resources
            let resources = pending.resources.lock().unwrap();
            for def in resources.iter() {
                let id = def.id.clone();
                self.resource_registry.register(id, def.clone())?;
            }

            // Process items
            let items = pending.items.lock().unwrap();
            for def in items.iter() {
                let id = def.id.clone();
                self.item_registry.register(id, def.clone())?;
            }

            // Process recipes
            let recipes = pending.recipes.lock().unwrap();
            for def in recipes.iter() {
                let id = def.id.clone();
                self.recipe_registry.register(id, def.clone())?;
            }

            // Process entities
            let entities = pending.entities.lock().unwrap();
            for def in entities.iter() {
                let id = def.id.clone();
                self.entity_registry.register(id, def.clone())?;
            }

            info!("[PACK] Processed {} resources, {} items, {} recipes, {} entities",
                resources.len(), items.len(), recipes.len(), entities.len());
        }

        Ok(())
    }
}


// Parser functions (same as before)
fn parse_resource_definition(_lua: &Lua, table: LuaTable) -> LuaResult<ResourceDefinition> {
    Ok(ResourceDefinition {
        id: table.get("id")?,
        name: table.get("name")?,
        description: table.get("description").ok(),
        category: table.get("category")?,
        properties: parse_resource_properties(&table.get("properties")?)?,
        harvestable: table.get("harvestable").ok()
            .and_then(|t: LuaTable| parse_harvestable_config(&t).ok()),
        spawn: table.get("spawn").ok()
            .and_then(|t: LuaTable| parse_spawn_config(&t).ok()),
        visuals: table.get("visuals").ok()
                        .and_then(|t: LuaTable| parse_visual_config(&t).ok()),
    })
}

fn parse_resource_properties(table: &LuaTable) -> LuaResult<super::ResourceProperties> {
    Ok(super::ResourceProperties {
        weight: table.get("weight")?,
        stack_size: table.get("stack_size")?,
        base_value: table.get("base_value")?,
        quality: table.get("quality").ok(),
        durability: table.get("durability").ok(),
    })
}

fn parse_harvestable_config(table: &LuaTable) -> LuaResult<super::HarvestableConfig> {
    let yields: Vec<LuaTable> = table.get("yield")?;
    let yield_configs = yields.into_iter()
        .map(|t| Ok(super::YieldConfig {
            item: t.get("item")?,
            min: t.get("min")?,
            max: t.get("max")?,
        }))
        .collect::<LuaResult<Vec<_>>>()?;

    Ok(super::HarvestableConfig {
        tool_required: table.get("tool_required").ok(),
        r#yield: yield_configs,
        respawn_time: table.get("respawn_time").ok(),
        growth_stages: table.get("growth_stages").ok(),
        stage_time: table.get("stage_time").ok(),
        requires_water: table.get("requires_water").ok(),
    })
}

fn parse_spawn_config(table: &LuaTable) -> LuaResult<super::SpawnConfig> {
    let cluster_table: LuaTable = table.get("cluster_size")?;

    Ok(super::SpawnConfig {
        biomes: table.get("biomes")?,
        frequency: table.get("frequency")?,
        cluster_size: super::ClusterSize {
            min: cluster_table.get("min")?,
            max: cluster_table.get("max")?,
        },
        min_distance: table.get("min_distance").ok(),
    })
}

fn parse_visual_config(table: &LuaTable) -> LuaResult<super::VisualConfig> {
    Ok(super::VisualConfig {
        sprite: table.get("sprite").ok(),
        color_variation: table.get("color_variation").ok(),
        size_variation: table.get("size_variation").ok(),
    })
}

fn parse_item_definition(_lua: &Lua, table: LuaTable) -> LuaResult<ItemDefinition> {
    Ok(ItemDefinition {
        id: table.get("id")?,
        name: table.get("name")?,
        description: table.get("description").ok(),
        category: table.get("category")?,
        properties: parse_item_properties(&table.get("properties")?)?,
        tool: table.get("tool").ok()
            .and_then(|t: LuaTable| parse_tool_properties(&t).ok()),
        consumable: table.get("consumable").ok()
            .and_then(|t: LuaTable| parse_consumable_properties(&t).ok()),
        tags: table.get("tags").ok(),
    })
}

fn parse_item_properties(table: &LuaTable) -> LuaResult<super::ItemProperties> {
    Ok(super::ItemProperties {
        weight: table.get("weight")?,
        stack_size: table.get("stack_size")?,
        value: table.get("value")?,
        rarity: table.get("rarity").ok(),
        tradeable: table.get("tradeable").ok(),
    })
}

fn parse_tool_properties(table: &LuaTable) -> LuaResult<super::ToolProperties> {
    let repair_cost: Option<Vec<LuaTable>> = table.get("repair_cost").ok();
    let repair_configs = repair_cost.map(|costs| {
        costs.into_iter()
            .map(|t| Ok(super::RepairCost {
                item: t.get("item")?,
                count: t.get("count")?,
            }))
            .collect::<LuaResult<Vec<_>>>()
    }).transpose()?;

    Ok(super::ToolProperties {
        tool_type: table.get("type")?,
        material: table.get("material")?,
        durability: table.get("durability")?,
        max_durability: table.get("max_durability")?,
        efficiency: table.get("efficiency")?,
        repairable: table.get("repairable").ok(),
        repair_cost: repair_configs,
        can_harvest: table.get("can_harvest").ok(),
    })
}

fn parse_consumable_properties(table: &LuaTable) -> LuaResult<super::ConsumableProperties> {
    let effects: Vec<LuaTable> = table.get("effects")?;
    let effect_configs = effects.into_iter()
        .map(|t| Ok(super::ConsumableEffect {
            effect_type: t.get("type")?,
            amount: t.get("amount")?,
            duration: t.get("duration").ok(),
        }))
        .collect::<LuaResult<Vec<_>>>()?;

    Ok(super::ConsumableProperties {
        effects: effect_configs,
        cooldown: table.get("cooldown").ok(),
        perishable: table.get("perishable").ok(),
        perish_time: table.get("perish_time").ok(),
    })
}

fn parse_recipe_definition(_lua: &Lua, table: LuaTable) -> LuaResult<RecipeDefinition> {
    let requirements: Vec<LuaTable> = table.get("requirements")?;
    let requirement_configs = requirements.into_iter()
        .map(|t| Ok(super::RecipeRequirement {
            item: t.get("item")?,
            count: t.get("count")?,
            consume: t.get("consume").ok(),
        }))
        .collect::<LuaResult<Vec<_>>>()?;

    let outputs: Vec<LuaTable> = table.get("outputs")?;
    let output_configs = outputs.into_iter()
        .map(|t| Ok(super::RecipeOutput {
            item: t.get("item")?,
            count: t.get("count")?,
            chance: t.get("chance").ok(),
        }))
        .collect::<LuaResult<Vec<_>>>()?;

    let crafting_table: LuaTable = table.get("crafting")?;
    let skill_required: Option<LuaTable> = crafting_table.get("skill_required").ok();
    let skills = skill_required.and_then(|t| {
        let mut map = std::collections::HashMap::new();
        for pair in t.pairs::<String, i32>() {
            if let Ok((k, v)) = pair {
                map.insert(k, v);
            }
        }
        Some(map)
    });

    Ok(RecipeDefinition {
        id: table.get("id")?,
        name: table.get("name")?,
        description: table.get("description").ok(),
        category: table.get("category")?,
        requirements: requirement_configs,
        outputs: output_configs,
        crafting: super::CraftingConfig {
            time: crafting_table.get("time")?,
            station: crafting_table.get("station").ok(),
            skill_required: skills,
            unlock_condition: crafting_table.get("unlock_condition").ok(),
        },
        tags: table.get("tags").ok(),
    })
}

fn parse_entity_definition(_lua: &Lua, table: LuaTable) -> LuaResult<EntityDefinition> {
    let properties_table: LuaTable = table.get("properties")?;
    let size_table: Option<LuaTable> = properties_table.get("size").ok();
    let size = size_table.and_then(|t| {
        let x: Option<i32> = t.get("x").ok();
        let y: Option<i32> = t.get("y").ok();
        match (x, y) {
            (Some(x), Some(y)) => Some(super::EntitySize { x, y }),
            _ => None,
        }
    });

    Ok(EntityDefinition {
        id: table.get("id")?,
        name: table.get("name")?,
        entity_type: table.get("type")?,
        description: table.get("description").ok(),
        properties: super::EntityProperties {
            health: properties_table.get("health")?,
            max_health: properties_table.get("max_health")?,
            size,
        },
        unit: table.get("unit").ok()
            .and_then(|t: LuaTable| parse_unit_properties(&t).ok()),
        building: table.get("building").ok()
            .and_then(|t: LuaTable| parse_building_properties(&t).ok()),
        spawn: table.get("spawn").ok()
            .and_then(|t: LuaTable| parse_entity_spawn_config(&t).ok()),
        visuals: table.get("visuals").ok()
                        .and_then(|t: LuaTable| parse_visual_config(&t).ok()),
        tags: table.get("tags").ok(),
    })
}

fn parse_unit_properties(table: &LuaTable) -> LuaResult<super::UnitProperties> {
    let needs_table: LuaTable = table.get("needs")?;
    let inventory_table: LuaTable = table.get("inventory")?;

    let starting_items: Option<Vec<LuaTable>> = inventory_table.get("starting_items").ok();
    let starting_configs = starting_items.map(|items| {
        items.into_iter()
            .map(|t| Ok(super::StartingItem {
                item: t.get("item")?,
                count: t.get("count")?,
            }))
            .collect::<LuaResult<Vec<_>>>()
    }).transpose()?;

    let skills: Option<LuaTable> = table.get("skills").ok();
    let skill_map = skills.and_then(|t| {
        let mut map = std::collections::HashMap::new();
        for pair in t.pairs::<String, i32>() {
            if let Ok((k, v)) = pair {
                map.insert(k, v);
            }
        }
        Some(map)
    });

    Ok(super::UnitProperties {
        ticks_per_tile: table.get("ticks_per_tile")?,
        energy: table.get("energy")?,
        max_energy: table.get("max_energy")?,
        needs: super::UnitNeeds {
            hunger_decay: needs_table.get("hunger_decay")?,
            energy_decay: needs_table.get("energy_decay")?,
            thirst_decay: needs_table.get("thirst_decay").ok(),
        },
        inventory: super::UnitInventory {
            slots: inventory_table.get("slots")?,
            starting_items: starting_configs,
        },
        behaviors: table.get("behaviors")?,
        work_speed: table.get("work_speed").ok(),
        skills: skill_map,
    })
}

fn parse_building_properties(table: &LuaTable) -> LuaResult<super::BuildingProperties> {
    let construction_table: LuaTable = table.get("construction")?;
    let requirements: Vec<LuaTable> = construction_table.get("requirements")?;
    let requirement_configs = requirements.into_iter()
        .map(|t| Ok(super::ConstructionRequirement {
            item: t.get("item")?,
            count: t.get("count")?,
        }))
        .collect::<LuaResult<Vec<_>>>()?;

    let storage = table.get("storage").ok()
                .map(|t: LuaTable| -> LuaResult<super::StorageConfig> {
            Ok(super::StorageConfig {
                capacity: t.get("capacity")?,
                allowed_items: t.get("allowed_items").ok(),
            })
        }).transpose()?;

    let production = table.get("production").ok()
                .map(|t: LuaTable| -> LuaResult<super::ProductionConfig> {
            Ok(super::ProductionConfig {
                recipes: t.get("recipes")?,
                speed_multiplier: t.get("speed_multiplier").ok(),
            })
        }).transpose()?;

    Ok(super::BuildingProperties {
        construction: super::ConstructionConfig {
            requirements: requirement_configs,
            time: construction_table.get("time")?,
            builder_required: construction_table.get("builder_required").ok(),
        },
        storage,
        production,
    })
}

fn parse_entity_spawn_config(table: &LuaTable) -> LuaResult<super::EntitySpawnConfig> {
    let spawn_area = table.get("spawn_area").ok()
                .map(|t: LuaTable| -> LuaResult<super::SpawnArea> {
            Ok(super::SpawnArea {
                min_x: t.get("min_x")?,
                max_x: t.get("max_x")?,
                min_y: t.get("min_y")?,
                max_y: t.get("max_y")?,
            })
        }).transpose()?;

    Ok(super::EntitySpawnConfig {
        initial_count: table.get("initial_count").ok(),
        spawn_area,
        require_walkable: table.get("require_walkable").ok(),
        preferred_terrain: table.get("preferred_terrain").ok(),
        avoided_terrain: table.get("avoided_terrain").ok(),
        preferred_biomes: table.get("preferred_biomes").ok(),
        min_fertility: table.get("min_fertility").ok(),
        max_elevation: table.get("max_elevation").ok(),
        moisture_range: table.get::<Option<LuaTable>>("moisture_range").ok()
            .flatten()
            .and_then(|t| parse_spawn_range(&t).ok()),
        temperature_range: table.get::<Option<LuaTable>>("temperature_range").ok()
            .flatten()
            .and_then(|t| parse_spawn_range(&t).ok()),
    })
}

/// Parse spawn range from Lua table
fn parse_spawn_range(table: &LuaTable) -> LuaResult<super::SpawnRange> {
    Ok(super::SpawnRange {
        min: table.get("min")?,
        max: table.get("max")?,
    })
}