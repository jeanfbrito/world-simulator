// Pack Loader
// Handles loading Lua files and binding API functions

use super::{PackSystem, PackError, ResourceDefinition, ItemDefinition, RecipeDefinition, EntityDefinition, Registry};
use bevy::prelude::*;
use mlua::prelude::*;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

impl PackSystem {
    /// Load all files in a category
    pub fn load_category(&mut self, category: &str) -> Result<(), PackError> {
        let category_path = self.pack_path.join("data").join(category);

        if !category_path.exists() {
            // It's okay if a category doesn't exist
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
        let content = fs::read_to_string(path)
            .map_err(|e| PackError::IoError(e))?;

        self.lua.load(&content)
            .set_name(&format!("{:?}", path))
            .exec()
            .map_err(|e| PackError::LuaError {
                file: path.to_path_buf(),
                error: e,
            })?;

        Ok(())
    }

    /// Bind Lua API functions
    pub fn bind_lua_api(&mut self) -> Result<(), PackError> {
        // Clone metadata for use in closures
        let debug = self.metadata.config.debug;

        // Create references that will be captured by closures
        // We'll use interior mutability pattern with a channel for thread safety
        let (tx_resource, rx_resource) = std::sync::mpsc::channel::<ResourceDefinition>();
        let (tx_item, rx_item) = std::sync::mpsc::channel::<ItemDefinition>();
        let (tx_recipe, rx_recipe) = std::sync::mpsc::channel::<RecipeDefinition>();
        let (tx_entity, rx_entity) = std::sync::mpsc::channel::<EntityDefinition>();

        // Store receivers for later processing
        let receivers = super::RegistrationReceivers {
            resource: rx_resource,
            item: rx_item,
            recipe: rx_recipe,
            entity: rx_entity,
        };

        // Bind register_resource function
        {
            let tx = tx_resource.clone();
            let debug = debug;
            self.lua.globals().set("register_resource",
                self.lua.create_function(move |lua, table: LuaTable| {
                    let def = parse_resource_definition(lua, table)?;
                    if debug {
                        eprintln!("[PACK] Registering resource: {}", def.id);
                    }
                    tx.send(def).map_err(|e| LuaError::external(e))?;
                    Ok(())
                })?
            )?;
        }

        // Bind register_item function
        {
            let tx = tx_item.clone();
            let debug = debug;
            self.lua.globals().set("register_item",
                self.lua.create_function(move |lua, table: LuaTable| {
                    let def = parse_item_definition(lua, table)?;
                    if debug {
                        eprintln!("[PACK] Registering item: {}", def.id);
                    }
                    tx.send(def).map_err(|e| LuaError::external(e))?;
                    Ok(())
                })?
            )?;
        }

        // Bind register_recipe function
        {
            let tx = tx_recipe.clone();
            let debug = debug;
            self.lua.globals().set("register_recipe",
                self.lua.create_function(move |lua, table: LuaTable| {
                    let def = parse_recipe_definition(lua, table)?;
                    if debug {
                        eprintln!("[PACK] Registering recipe: {}", def.id);
                    }
                    tx.send(def).map_err(|e| LuaError::external(e))?;
                    Ok(())
                })?
            )?;
        }

        // Bind register_entity function
        {
            let tx = tx_entity.clone();
            let debug = debug;
            self.lua.globals().set("register_entity",
                self.lua.create_function(move |lua, table: LuaTable| {
                    let def = parse_entity_definition(lua, table)?;
                    if debug {
                        eprintln!("[PACK] Registering entity: {}", def.id);
                    }
                    tx.send(def).map_err(|e| LuaError::external(e))?;
                    Ok(())
                })?
            )?;
        }

        // Store receivers for processing after loading
        self.receivers = Some(receivers);

        Ok(())
    }

    /// Process all pending registrations from Lua
    pub fn process_registrations(&mut self) -> Result<(), PackError> {
        if let Some(receivers) = self.receivers.take() {
            // Process resources
            while let Ok(def) = receivers.resource.try_recv() {
                let id = def.id.clone();
                self.resource_registry.register(id, def)?;
            }

            // Process items
            while let Ok(def) = receivers.item.try_recv() {
                let id = def.id.clone();
                self.item_registry.register(id, def)?;
            }

            // Process recipes
            while let Ok(def) = receivers.recipe.try_recv() {
                let id = def.id.clone();
                self.recipe_registry.register(id, def)?;
            }

            // Process entities
            while let Ok(def) = receivers.entity.try_recv() {
                let id = def.id.clone();
                self.entity_registry.register(id, def)?;
            }
        }
        Ok(())
    }
}



/// Parse a resource definition from a Lua table
fn parse_resource_definition(_lua: &Lua, table: LuaTable) -> LuaResult<ResourceDefinition> {
    Ok(ResourceDefinition {
        id: table.get("id")?,
        name: table.get("name")?,
        description: table.get("description").ok(),
        category: table.get("category")?,
        properties: parse_resource_properties(&table.get::<_, LuaTable>("properties")?)?,
        harvestable: table.get::<_, Option<LuaTable>>("harvestable").ok()
            .flatten()
            .and_then(|t| parse_harvestable_config(&t).ok()),
        spawn: table.get::<_, Option<LuaTable>>("spawn").ok()
            .flatten()
            .and_then(|t| parse_spawn_config(&t).ok()),
        visuals: table.get::<_, Option<LuaTable>>("visuals").ok()
            .flatten()
            .and_then(|t| parse_visual_config(&t).ok()),
    })
}

/// Parse resource properties
fn parse_resource_properties(table: &LuaTable) -> LuaResult<super::ResourceProperties> {
    Ok(super::ResourceProperties {
        weight: table.get("weight")?,
        stack_size: table.get("stack_size")?,
        base_value: table.get("base_value")?,
        quality: table.get("quality").ok(),
        durability: table.get("durability").ok(),
    })
}

/// Parse harvestable configuration
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

/// Parse spawn configuration
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

/// Parse visual configuration
fn parse_visual_config(table: &LuaTable) -> LuaResult<super::VisualConfig> {
    Ok(super::VisualConfig {
        sprite: table.get("sprite").ok(),
        color_variation: table.get("color_variation").ok(),
        size_variation: table.get("size_variation").ok(),
    })
}

/// Parse item definition
fn parse_item_definition(_lua: &Lua, table: LuaTable) -> LuaResult<ItemDefinition> {
    Ok(ItemDefinition {
        id: table.get("id")?,
        name: table.get("name")?,
        description: table.get("description").ok(),
        category: table.get("category")?,
        properties: parse_item_properties(&table.get::<_, LuaTable>("properties")?)?,
        tool: table.get::<_, Option<LuaTable>>("tool").ok()
            .flatten()
            .and_then(|t| parse_tool_properties(&t).ok()),
        consumable: table.get::<_, Option<LuaTable>>("consumable").ok()
            .flatten()
            .and_then(|t| parse_consumable_properties(&t).ok()),
        tags: table.get("tags").ok(),
    })
}

/// Parse item properties
fn parse_item_properties(table: &LuaTable) -> LuaResult<super::ItemProperties> {
    Ok(super::ItemProperties {
        weight: table.get("weight")?,
        stack_size: table.get("stack_size")?,
        value: table.get("value")?,
        rarity: table.get("rarity").ok(),
        tradeable: table.get("tradeable").ok(),
    })
}

/// Parse tool properties
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

/// Parse consumable properties
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

/// Parse recipe definition
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
    let skills = skill_required.map(|t| {
        let mut map = std::collections::HashMap::new();
        for pair in t.pairs::<String, i32>() {
            if let Ok((k, v)) = pair {
                map.insert(k, v);
            }
        }
        map
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

/// Parse entity definition
fn parse_entity_definition(_lua: &Lua, table: LuaTable) -> LuaResult<EntityDefinition> {
    let properties_table: LuaTable = table.get("properties")?;
    let size_table: Option<LuaTable> = properties_table.get("size").ok();
    let size = size_table.map(|t| Ok(super::EntitySize {
        x: t.get("x")?,
        y: t.get("y")?,
    })).transpose()?;

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
        unit: table.get::<_, Option<LuaTable>>("unit").ok()
            .flatten()
            .and_then(|t| parse_unit_properties(&t).ok()),
        building: table.get::<_, Option<LuaTable>>("building").ok()
            .flatten()
            .and_then(|t| parse_building_properties(&t).ok()),
        spawn: table.get::<_, Option<LuaTable>>("spawn").ok()
            .flatten()
            .and_then(|t| parse_entity_spawn_config(&t).ok()),
        visuals: table.get::<_, Option<LuaTable>>("visuals").ok()
            .flatten()
            .and_then(|t| parse_visual_config(&t).ok()),
        tags: table.get("tags").ok(),
    })
}

/// Parse unit properties
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
    let skill_map = skills.map(|t| {
        let mut map = std::collections::HashMap::new();
        for pair in t.pairs::<String, i32>() {
            if let Ok((k, v)) = pair {
                map.insert(k, v);
            }
        }
        map
    });

    Ok(super::UnitProperties {
        movement_speed: table.get("movement_speed")?,
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

/// Parse building properties
fn parse_building_properties(table: &LuaTable) -> LuaResult<super::BuildingProperties> {
    let construction_table: LuaTable = table.get("construction")?;
    let requirements: Vec<LuaTable> = construction_table.get("requirements")?;
    let requirement_configs = requirements.into_iter()
        .map(|t| Ok(super::ConstructionRequirement {
            item: t.get("item")?,
            count: t.get("count")?,
        }))
        .collect::<LuaResult<Vec<_>>>()?;

    let storage = table.get::<_, Option<LuaTable>>("storage").ok()
        .flatten()
        .map(|t| Ok(super::StorageConfig {
            capacity: t.get("capacity")?,
            allowed_items: t.get("allowed_items").ok(),
        })).transpose()?;

    let production = table.get::<_, Option<LuaTable>>("production").ok()
        .flatten()
        .map(|t| Ok(super::ProductionConfig {
            recipes: t.get("recipes")?,
            speed_multiplier: t.get("speed_multiplier").ok(),
        })).transpose()?;

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

/// Parse entity spawn configuration
fn parse_entity_spawn_config(table: &LuaTable) -> LuaResult<super::EntitySpawnConfig> {
    let spawn_area = table.get::<_, Option<LuaTable>>("spawn_area").ok()
        .flatten()
        .map(|t| Ok(super::SpawnArea {
            min_x: t.get("min_x")?,
            max_x: t.get("max_x")?,
            min_y: t.get("min_y")?,
            max_y: t.get("max_y")?,
        })).transpose()?;

    // Parse biome and terrain preferences
    let preferred_terrain = table.get::<_, Option<Vec<String>>>("preferred_terrain").ok()
        .flatten();
    let avoided_terrain = table.get::<_, Option<Vec<String>>>("avoided_terrain").ok()
        .flatten();
    let preferred_biomes = table.get::<_, Option<Vec<String>>>("preferred_biomes").ok()
        .flatten();
    let min_fertility = table.get::<_, Option<f32>>("min_fertility").ok()
        .flatten();
    let max_elevation = table.get::<_, Option<f32>>("max_elevation").ok()
        .flatten();

    // Parse optional ranges
    let moisture_range = table.get::<_, Option<LuaTable>>("moisture_range").ok()
        .flatten()
        .map(|t| Ok(super::SpawnRange {
            min: t.get("min")?,
            max: t.get("max")?,
        })).transpose()?;

    let temperature_range = table.get::<_, Option<LuaTable>>("temperature_range").ok()
        .flatten()
        .map(|t| Ok(super::SpawnRange {
            min: t.get("min")?,
            max: t.get("max")?,
        })).transpose()?;

    Ok(super::EntitySpawnConfig {
        initial_count: table.get("initial_count").ok(),
        spawn_area,
        require_walkable: table.get("require_walkable").ok(),
        preferred_terrain,
        avoided_terrain,
        preferred_biomes,
        min_fertility,
        max_elevation,
        moisture_range,
        temperature_range,
    })
}