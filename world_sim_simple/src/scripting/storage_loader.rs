/// Storage building loader from Lua scripts
/// 
/// Loads storage building definitions from pack scripts
/// and spawns them in the world based on data.

use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use crate::components::{
    StorageBuilding, Stockpile, Warehouse, Silo,
    GridPosition, NameComponent, StorageUpdateTag
};
use crate::resources::ResourceType;
use crate::TileEntity;
use colored::Colorize;
use std::collections::HashMap;

/// Storage building definition loaded from script
#[derive(Debug, Clone)]
pub struct StorageBuildingDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub capacity_per_tile: u32,
    pub total_capacity: u32,
    pub storage_type: String,
    pub allowed_resources: Option<Vec<String>>,
    pub priority: i32,
    pub build_time_ticks: u32,
    pub requires_workers: bool,
    pub required_workers: u32,
    pub protection_level: f32,
    pub decay_rate: f32,
}

/// Resource to store loaded storage building definitions
#[derive(Resource, Default)]
pub struct StorageBuildingRegistry {
    pub buildings: HashMap<String, StorageBuildingDef>,
}

/// Component to mark script entities that contain storage definitions
#[derive(Component)]
pub struct StorageScriptTag;

/// Event to trigger loading storage buildings from scripts
#[derive(Event)]
pub struct LoadStorageBuildingsEvent;

/// Event to spawn a storage building
#[derive(Event)]
pub struct SpawnStorageBuildingEvent {
    pub building_id: String,
    pub position: GridPosition,
}

/// System to load storage building definitions from Lua scripts
pub fn load_storage_buildings_system(
    mut commands: Commands,
    mut events: EventReader<LoadStorageBuildingsEvent>,
    asset_server: Res<AssetServer>,
    mut registry: ResMut<StorageBuildingRegistry>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    for _ in events.read() {
        debug.log(
            DebugLevel::Info,
            "STORAGE",
            "Loading storage building definitions from scripts"
        );
        
        // Load the storage buildings script
        let script_handle: Handle<ScriptAsset> = asset_server.load(
            "packs/stronghold/scripts/buildings/storage_buildings.lua"
        );
        
        // Create entity to hold the script
        commands.spawn((
            ScriptCollection::<LuaScript> {
                scripts: vec![Script::new(
                    "storage_buildings.lua".to_string(),
                    script_handle,
                )],
            },
            StorageScriptTag,
        ));
        
        println!("{} Loading storage building definitions from Lua scripts",
            "📜".yellow());
    }
}

/// System to process loaded storage building scripts
pub fn process_storage_scripts_system(
    mut scripts: Query<&mut ScriptCollection<LuaScript>, With<StorageScriptTag>>,
    mut registry: ResMut<StorageBuildingRegistry>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    for mut script_collection in scripts.iter_mut() {
        for script in script_collection.scripts.iter_mut() {
            // Get Lua context
            let ctx = script.get_context().expect("Failed to get Lua context");
            
            ctx.scope(|scope| {
                // Get the storage_buildings table
                let buildings_table: mlua::Table = ctx.globals()
                    .get("storage_buildings")
                    .unwrap_or_else(|_| ctx.create_table().unwrap());
                
                // Iterate through all building definitions
                for pair in buildings_table.pairs::<String, mlua::Table>() {
                    if let Ok((id, building_data)) = pair {
                        // Parse building definition
                        let def = parse_building_definition(&id, building_data);
                        
                        if let Ok(building_def) = def {
                            debug.log(
                                DebugLevel::Debug,
                                "STORAGE",
                                &format!("Loaded storage building: {} ({}x{}, capacity: {})",
                                    building_def.name, 
                                    building_def.width,
                                    building_def.height,
                                    building_def.total_capacity)
                            );
                            
                            registry.buildings.insert(id.clone(), building_def);
                        }
                    }
                }
            });
        }
    }
    
    if !registry.buildings.is_empty() {
        println!("{} Loaded {} storage building types",
            "✅".green(),
            registry.buildings.len());
    }
}

/// Parse a building definition from Lua table
fn parse_building_definition(
    id: &str, 
    table: mlua::Table
) -> mlua::Result<StorageBuildingDef> {
    // Get size
    let size_table: mlua::Table = table.get("size")?;
    let width: u32 = size_table.get("width")?;
    let height: u32 = size_table.get("height")?;
    
    // Get allowed resources if specified
    let allowed_resources = if let Ok(resources_table) = table.get::<mlua::Table>("allowed_resources") {
        let mut resources = Vec::new();
        for value in resources_table.sequence_values::<String>() {
            if let Ok(resource) = value {
                resources.push(resource);
            }
        }
        Some(resources)
    } else {
        None
    };
    
    Ok(StorageBuildingDef {
        id: id.to_string(),
        name: table.get("name")?,
        description: table.get("description")?,
        width,
        height,
        capacity_per_tile: table.get("capacity_per_tile")?,
        total_capacity: table.get("total_capacity")?,
        storage_type: table.get("storage_type")?,
        allowed_resources,
        priority: table.get("priority").unwrap_or(0),
        build_time_ticks: table.get("build_time_ticks")?,
        requires_workers: table.get("requires_workers").unwrap_or(false),
        required_workers: table.get("required_workers").unwrap_or(0),
        protection_level: table.get("protection_level").unwrap_or(0.0),
        decay_rate: table.get("decay_rate").unwrap_or(0.0),
    })
}

/// System to spawn storage buildings from events
pub fn spawn_storage_building_system(
    mut commands: Commands,
    mut events: EventReader<SpawnStorageBuildingEvent>,
    registry: Res<StorageBuildingRegistry>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    for event in events.read() {
        if let Some(def) = registry.buildings.get(&event.building_id) {
            // Create base storage component
            let mut storage = StorageBuilding::new(def.total_capacity);
            storage.priority = def.priority;
            
            // Set allowed resources if specialized
            if let Some(ref allowed) = def.allowed_resources {
                let resource_types: Vec<ResourceType> = allowed.iter()
                    .filter_map(|s| parse_resource_type(s))
                    .collect();
                
                if !resource_types.is_empty() {
                    storage = StorageBuilding::specialized(
                        def.total_capacity,
                        resource_types
                    );
                    storage.priority = def.priority;
                }
            }
            
            // Spawn entity with appropriate components
            let mut entity = commands.spawn((
                NameComponent::new(def.name.clone()),
                GridPosition {
                    x: event.position.x,
                    y: event.position.y,
                },
                TileEntity {
                    x: event.position.x as usize,
                    y: event.position.y as usize,
                },
                storage,
                StorageUpdateTag,
            ));
            
            // Add specific building type components
            match def.storage_type.as_str() {
                "general" if !def.requires_workers => {
                    entity.insert(Stockpile::new(def.width, def.height));
                }
                "general" if def.requires_workers => {
                    let mut warehouse = Warehouse::new();
                    warehouse.required_workers = def.required_workers;
                    warehouse.protection_level = def.protection_level;
                    entity.insert(warehouse);
                }
                "specialized" => {
                    // Could add Silo component for specialized storage
                    if let Some(ref allowed) = def.allowed_resources {
                        if allowed.len() == 1 {
                            if let Some(resource_type) = parse_resource_type(&allowed[0]) {
                                entity.insert(Silo::new(resource_type, def.total_capacity));
                            }
                        }
                    }
                }
                _ => {}
            }
            
            println!("{} Spawned {} at ({}, {})",
                "🏗️".green(),
                def.name.cyan(),
                event.position.x,
                event.position.y
            );
            
            debug.log(
                DebugLevel::Info,
                "STORAGE",
                &format!("Spawned {} at ({}, {})",
                    def.name, event.position.x, event.position.y)
            );
        } else {
            debug.log(
                DebugLevel::Error,
                "STORAGE",
                &format!("Unknown storage building type: {}", event.building_id)
            );
        }
    }
}

/// Helper to parse resource type from string
fn parse_resource_type(s: &str) -> Option<ResourceType> {
    match s {
        "wood" => Some(ResourceType::Wood),
        "stone" => Some(ResourceType::Stone),
        "iron" | "iron_ore" => Some(ResourceType::Iron),
        "berries" => Some(ResourceType::Berries),
        "wheat" => Some(ResourceType::Wheat),
        "meat" => Some(ResourceType::Meat),
        _ => None,
    }
}

/// System to spawn initial storage buildings from script data
pub fn spawn_initial_storage_system(
    mut events: EventWriter<SpawnStorageBuildingEvent>,
    registry: Res<StorageBuildingRegistry>,
    existing: Query<Entity, With<StorageBuilding>>,
    sim_state: Res<crate::SimulationState>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only spawn once when registry is loaded
    if sim_state.tick != 10 || existing.iter().count() > 0 || registry.buildings.is_empty() {
        return;
    }
    
    debug.log(
        DebugLevel::Info,
        "STORAGE",
        "Spawning initial storage buildings from script data"
    );
    
    // Spawn a stockpile at center
    events.send(SpawnStorageBuildingEvent {
        building_id: "stockpile".to_string(),
        position: GridPosition { x: 32, y: 32 },
    });
    
    // Spawn a warehouse nearby
    events.send(SpawnStorageBuildingEvent {
        building_id: "warehouse".to_string(),
        position: GridPosition { x: 37, y: 32 },
    });
    
    // Spawn a granary for food
    events.send(SpawnStorageBuildingEvent {
        building_id: "granary".to_string(),
        position: GridPosition { x: 32, y: 37 },
    });
}