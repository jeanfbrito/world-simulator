use crate::{BuildingComponent, TileEntity, BuildingType, WorldMap};
use crate::components::{GridPosition, NameComponent, PositionComponent, ResourceNode, GrowingResource, ResourceRegenerationTag};
use crate::packs::{PackSystem, EntityDefinition, registry::Registry};
use crate::resources::ResourceType;
use crate::TileType;
use crate::{MAP_SIZE, TILE_SIZE};
use bevy::prelude::*;
use colored::Colorize;
use rand::Rng;
use std::collections::HashMap;

/// System to spawn entities from pack definitions
pub struct EntitySpawnerPlugin;

impl Plugin for EntitySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_entities)
           .add_systems(Update, spawn_dynamic_entities);
    }
}

/// Spawn initial entities based on pack definitions
pub fn spawn_initial_entities(
    mut commands: Commands,
    pack_system: Option<Res<PackSystem>>,
    world_map: Option<Res<WorldMap>>,
) {
    if let Some(pack_system) = pack_system {
        println!("{}", "[SPAWN] Starting entity spawning from pack definitions...".cyan());

        // Get all entity definitions with spawn configurations
        let entities_to_spawn: Vec<_> = pack_system.entity_registry.get_all()
            .into_iter()
            .filter(|entity| {
                entity.spawn.as_ref().map_or(false, |spawn| spawn.initial_count.unwrap_or(0) > 0)
            })
            .collect();

        let mut total_spawned = 0;

        for entity_def in entities_to_spawn {
            if let Some(spawn_config) = &entity_def.spawn {
                let spawned_count = spawn_entity_type(
                    &mut commands,
                    entity_def,
                    spawn_config.initial_count.unwrap_or(0),
                    &pack_system,
                    world_map.as_deref(),
                );

                total_spawned += spawned_count;

                println!(
                    "{}",
                    format!("[SPAWN] Spawned {} {} entities from pack definition",
                        spawned_count, entity_def.name).green()
                );
            }
        }

        println!(
            "{}",
            format!("[SPAWN] Total entities spawned from pack definitions: {}", total_spawned).cyan()
        );
    } else {
        println!("{}", "[SPAWN] No pack system available - skipping pack-based spawning".yellow());
    }
}

/// Spawn dynamic entities during gameplay (wildlife respawns, etc.)
pub fn spawn_dynamic_entities(
    mut commands: Commands,
    pack_system: Option<Res<PackSystem>>,
    time: Res<Time>,
) {
    // This will handle dynamic spawning based on game conditions
    // For now, just a placeholder for future expansion
}

/// Spawn entities of a specific type from pack definition
fn spawn_entity_type(
    commands: &mut Commands,
    entity_def: &EntityDefinition,
    count: i32,
    pack_system: &PackSystem,
    world_map: Option<&WorldMap>,
) -> usize {
    let mut spawned = 0;
    let mut rng = rand::thread_rng();

    for _ in 0..count {
        if let Some(position) = find_valid_spawn_position(entity_def, &mut rng, world_map) {
            spawn_entity_at_position(commands, entity_def, position, pack_system);
            spawned += 1;
        }
    }

    spawned
}

/// Find a valid spawn position for an entity
fn find_valid_spawn_position(
    entity_def: &EntityDefinition,
    rng: &mut rand::rngs::ThreadRng,
    world_map: Option<&WorldMap>,
) -> Option<(usize, usize)> {
    let spawn_config = entity_def.spawn.as_ref()?;

    // Use spawn area if defined, otherwise use default
    if let Some(area) = &spawn_config.spawn_area {
        // Try multiple positions within the spawn area
        for _ in 0..50 {
            let x = rng.gen_range(area.min_x as usize..=area.max_x as usize);
            let y = rng.gen_range(area.min_y as usize..=area.max_y as usize);

            if x < MAP_SIZE && y < MAP_SIZE {
                // Check if position is valid (walkable, not occupied, etc.)
                if is_position_valid(x, y, entity_def, world_map) {
                    return Some((x, y));
                }
            }
        }
    } else {
        // Default spawn area - center of map
        for _ in 0..50 {
            let x = rng.gen_range(20..44);
            let y = rng.gen_range(20..44);

            // Check terrain validation
            if is_position_valid(x, y, entity_def, world_map) {
                return Some((x, y));
            }
        }
    }

    None
}

/// Check if a position is valid for spawning an entity
fn is_position_valid(
    x: usize,
    y: usize,
    entity_def: &EntityDefinition,
    world_map: Option<&WorldMap>,
) -> bool {
    // Basic bounds check
    if x >= MAP_SIZE || y >= MAP_SIZE {
        return false;
    }

    // Check if entity requires walkable terrain
    if let Some(spawn_config) = &entity_def.spawn {
        if spawn_config.require_walkable.unwrap_or(false) {
            if let Some(map) = world_map {
                if !map.tiles[y][x].is_walkable() {
                    return false;
                }
            } else {
                // If no world map is available, fall back to basic validation
                // This prevents spawning in water/deep water areas when map is not available
                let center = MAP_SIZE / 2;
                let dist = ((x as f32 - center as f32).powi(2) + (y as f32 - center as f32).powi(2)).sqrt();
                let max_dist = center as f32;

                // Prevent spawning in water/deep water areas (outside 75% of map radius)
                if dist > max_dist * 0.75 {
                    return false;
                }
            }
        }
    }

    // TODO: Add occupation checking when entity tracking is implemented
    // For now, assume position is not occupied

    true
}

/// Spawn a single entity at a specific position
fn spawn_entity_at_position(
    commands: &mut Commands,
    entity_def: &EntityDefinition,
    position: (usize, usize),
    pack_system: &PackSystem,
) {
    let (x, y) = position;

    // Create base components that all entities need
    let name_component = NameComponent::new(entity_def.name.clone());
    let position_component = PositionComponent::from_tile(x, y);
    let tile_entity = TileEntity { x, y };
    let grid_position = GridPosition { x: x as u32, y: y as u32 };

    // Spawn based on entity type
    match entity_def.entity_type.as_str() {
        "building" => spawn_building(commands, entity_def, name_component, position_component, tile_entity, grid_position, pack_system),
        "unit" => spawn_unit(commands, entity_def, name_component, position_component, tile_entity, grid_position, pack_system),
        "wildlife" => spawn_wildlife(commands, entity_def, name_component, position_component, tile_entity, grid_position, pack_system),
        "resource" => spawn_resource(commands, entity_def, name_component, position_component, tile_entity, grid_position, pack_system),
        _ => {
            println!(
                "{}",
                format!("[SPAWN] Unknown entity type: {} for {}", entity_def.entity_type, entity_def.name).yellow()
            );
        }
    }
}

/// Spawn a building entity
fn spawn_building(
    commands: &mut Commands,
    entity_def: &EntityDefinition,
    name: NameComponent,
    position: PositionComponent,
    tile_entity: TileEntity,
    grid_position: GridPosition,
    _pack_system: &PackSystem,
) {
    if let Some(building_props) = &entity_def.building {
        // Map building types to the enum
        let building_type = match entity_def.id.as_str() {
            "stockpile" => BuildingType::Stockpile,
            "granary" => BuildingType::Granary,
            "storage" => BuildingType::Storage,
            "warehouse" => BuildingType::Warehouse,
            _ => BuildingType::Storage, // Default fallback
        };

        // Create the building component
        let building_component = BuildingComponent::new(building_type, (grid_position.x as i32, grid_position.y as i32));

        // Spawn the building entity
        let pos_clone = grid_position.clone();
        commands.spawn((
            name,
            position,
            tile_entity,
            pos_clone,
            building_component,
        ));

        println!(
            "{}",
            format!("[SPAWN] Spawned building: {} at ({}, {})", entity_def.name, grid_position.x, grid_position.y).green()
        );
    } else {
        println!(
            "{}",
            format!("[SPAWN] Building {} has no building properties", entity_def.name).yellow()
        );
    }
}

/// Spawn a unit entity
fn spawn_unit(
    commands: &mut Commands,
    entity_def: &EntityDefinition,
    name: NameComponent,
    position: PositionComponent,
    tile_entity: TileEntity,
    grid_position: GridPosition,
    _pack_system: &PackSystem,
) {
    if let Some(unit_props) = &entity_def.unit {
        // Create unit-specific components based on the unit type
        let unit_tag = match entity_def.id.as_str() {
            "peasant" => crate::components::PeasantTag::new(),
            _ => {
                println!(
                    "{}",
                    format!("[SPAWN] Unknown unit type: {}", entity_def.id).yellow()
                );
                return;
            }
        };

        // Create unit inventory and needs
        let inventory = crate::components::UnitInventory::new();
        let needs = crate::components::UnitNeedsV2::new();

        // Create work components
        let work_progress = crate::components::WorkProgress::new();
        let work_speed = crate::components::WorkSpeed::default();
        let work_queue = crate::components::WorkQueue::new(10);

        // Spawn the unit entity
        let pos_clone = grid_position.clone();
        commands.spawn((
            name,
            position,
            tile_entity,
            pos_clone,
            unit_tag,
            inventory,
            needs,
            work_progress,
            work_speed,
            work_queue,
        ));

        println!(
            "{}",
            format!("[SPAWN] Spawned unit: {} at ({}, {})", entity_def.name, grid_position.x, grid_position.y).blue()
        );
    } else {
        println!(
            "{}",
            format!("[SPAWN] Unit {} has no unit properties", entity_def.name).yellow()
        );
    }
}

/// Spawn a wildlife entity
fn spawn_wildlife(
    commands: &mut Commands,
    entity_def: &EntityDefinition,
    name: NameComponent,
    position: PositionComponent,
    tile_entity: TileEntity,
    grid_position: GridPosition,
    _pack_system: &PackSystem,
) {
    // Create wildlife-specific components based on the wildlife type
    // For now, use UnitTag for wildlife until proper wildlife tags are defined
    let _wildlife_type = match entity_def.id.as_str() {
        "deer" => "deer",
        _ => {
            println!(
                "{}",
                format!("[SPAWN] Unknown wildlife type: {}", entity_def.id).yellow()
            );
            return;
        }
    };

    // Create basic wildlife components
    let needs = crate::components::UnitNeedsV2::new();

    // Spawn the wildlife entity
    let pos_clone = grid_position.clone();
    commands.spawn((
        name,
        position,
        tile_entity,
        pos_clone,
        crate::components::UnitTag, // Use UnitTag for wildlife for now
        needs,
    ));

    println!(
        "{}",
        format!("[SPAWN] Spawned wildlife: {} at ({}, {})", entity_def.name, grid_position.x, grid_position.y).magenta()
    );
}

/// Spawn a resource entity
fn spawn_resource(
    commands: &mut Commands,
    entity_def: &EntityDefinition,
    name: NameComponent,
    position: PositionComponent,
    tile_entity: TileEntity,
    grid_position: GridPosition,
    pack_system: &PackSystem,
) {
    // Parse resource properties from pack definition
    let (resource_type, initial_amount, max_amount) = match entity_def.id.as_str() {
        "tree" => (ResourceType::Wood, 10, 12),
        "berry_bush" | "berry_bush_corner" => (ResourceType::Berries, 3, 4),
        "stone_deposit" => (ResourceType::Stone, 8, 8),
        "iron_ore_deposit" => (ResourceType::IronOre, 6, 6),
        _ => {
            println!(
                "{}",
                format!("[SPAWN] Unknown resource type: {}", entity_def.id).yellow()
            );
            return;
        }
    };

    // Create resource node
    let mut resource_node = ResourceNode::new(resource_type, initial_amount);
    resource_node.max_amount = max_amount;

    // Get resource tag - only handle known tags for now
    let (tree_tag, berry_tag) = match entity_def.id.as_str() {
        "tree" => (Some(crate::ai::TreeTag), None),
        "berry_bush" | "berry_bush_corner" => (None, Some(crate::ai::BerryBushTag)),
        _ => {
            // For other resources, skip spawning tags for now
            println!(
                "{}",
                format!("[SPAWN] No specific tag for resource: {}", entity_def.id).yellow()
            );
            (None, None)
        }
    };

    // Create growing resource based on entity definition
    let growing_resource = match entity_def.id.as_str() {
        "tree" => Some(crate::components::GrowingResource::tree(initial_amount)),
        "berry_bush" | "berry_bush_corner" => Some(crate::components::GrowingResource::fruit_bush(initial_amount, max_amount)),
        _ => None,
    };

    // Spawn the resource entity
    let pos_clone = grid_position.clone();
    let mut entity = commands.spawn((
        name,
        position,
        tile_entity,
        pos_clone,
        resource_node,
        ResourceRegenerationTag,
    ));

    // Add growing resource if applicable
    if let Some(growing) = growing_resource {
        entity.insert(growing);
    }

    // Add resource tags if applicable
    if let Some(tree_tag) = tree_tag {
        entity.insert(tree_tag);
    }
    if let Some(berry_tag) = berry_tag {
        entity.insert(berry_tag);
    }

    println!(
        "{}",
        format!("[SPAWN] Spawned resource: {} at ({}, {}) with {} {}",
                entity_def.name, grid_position.x, grid_position.y, initial_amount, entity_def.id).green()
    );
}

/// Helper function to get spawn configuration from pack definitions
pub fn get_entities_to_spawn(pack_system: &PackSystem) -> Vec<&EntityDefinition> {
    pack_system.entity_registry.get_all()
        .into_iter()
        .filter(|entity| {
            entity.spawn.as_ref().map_or(false, |spawn| spawn.initial_count.unwrap_or(0) > 0)
        })
        .collect()
}

/// Helper function to check if an entity should be spawned dynamically
pub fn should_spawn_dynamically(entity_def: &EntityDefinition) -> bool {
    match entity_def.entity_type.as_str() {
        "wildlife" => true,
        "resource" => {
            // Resources that should respawn
            matches!(entity_def.id.as_str(), "tree" | "berry_bush")
        },
        _ => false,
    }
}