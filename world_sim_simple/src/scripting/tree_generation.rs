use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use rand::Rng;
use crate::debug::{DebugSystem, DebugLevel};
use crate::{WorldMap, TileType, MAP_SIZE};
use crate::components::{NameComponent, PositionComponent};
use crate::TileEntity;
use crate::ai::TreeTag;

/// Component for trees generated through scripting
#[derive(Component)]
pub struct ScriptedTree {
    pub tree_type: String,
    pub wood_yield: u32,
    pub fruit_yield: u32,
    pub growth_stage: String,
    pub harvestable: bool,
    pub respawn_time: f32,
}

impl Default for ScriptedTree {
    fn default() -> Self {
        Self {
            tree_type: "oak".to_string(),
            wood_yield: 15,
            fruit_yield: 0,
            growth_stage: "mature".to_string(),
            harvestable: true,
            respawn_time: 60.0,
        }
    }
}

/// Event to trigger tree generation
#[derive(Event)]
pub struct GenerateTreesCommand {
    pub area: Option<(i32, i32, u32, u32)>, // x, y, width, height
    pub force_regenerate: bool,
}

/// Resource to track generated trees
#[derive(Resource, Default)]
pub struct TreeGenerationState {
    pub trees_generated: bool,
    pub generation_seed: u64,
}

/// System to handle tree generation through Lua scripts
pub fn generate_trees_system(
    mut commands: Commands,
    mut events: EventReader<GenerateTreesCommand>,
    world_map: Res<WorldMap>,
    mut tree_state: ResMut<TreeGenerationState>,
    asset_server: Res<AssetServer>,
    existing_trees: Query<&PositionComponent, With<TreeTag>>,
    debug: Res<DebugSystem>,
) {
    for event in events.read() {
        if tree_state.trees_generated && !event.force_regenerate {
            debug.log(
                DebugLevel::Info,
                "TREE_GEN",
                "Trees already generated, skipping"
            );
            continue;
        }

        debug.log(
            DebugLevel::Info,
            "TREE_GEN",
            "Starting scripted tree generation..."
        );

        // Load and execute tree generator script
        let script_handle: Handle<ScriptAsset> = asset_server.load("packs/stronghold/scripts/generators/tree_generator.lua");
        
        // Create map data for the Lua script
        let map_data = create_map_data_for_lua(&world_map, &existing_trees, event.area);
        
        // For now, we'll use a simplified approach since the full Lua integration 
        // requires more complex setup. We'll generate trees directly in Rust
        // but following the patterns from the Lua script.
        let generated_trees = generate_trees_rust_impl(&map_data, &debug);
        
        // Spawn the generated trees as entities
        for tree_data in generated_trees {
            spawn_tree_entity(&mut commands, tree_data, &debug);
        }
        
        tree_state.trees_generated = true;
        tree_state.generation_seed = rand::thread_rng().gen::<u64>();
        
        debug.log(
            DebugLevel::Info,
            "TREE_GEN",
            "Tree generation completed"
        );
    }
}

/// Create map data structure for Lua script consumption
fn create_map_data_for_lua(
    world_map: &WorldMap,
    existing_trees: &Query<&PositionComponent, With<TreeTag>>,
    area: Option<(i32, i32, u32, u32)>,
) -> MapData {
    let (start_x, start_y, width, height) = area.unwrap_or((0, 0, MAP_SIZE as u32, MAP_SIZE as u32));
    
    // Convert existing trees to positions
    let mut existing_tree_positions = Vec::new();
    for pos in existing_trees.iter() {
        existing_tree_positions.push((pos.x as usize, pos.y as usize));
    }
    
    MapData {
        width: width as usize,
        height: height as usize,
        start_x: start_x as usize,
        start_y: start_y as usize,
        terrain: convert_tilemap_to_terrain(&world_map.tiles),
        buildings: Vec::new(), // TODO: Add buildings when available
        existing_trees: existing_tree_positions,
    }
}

/// Convert TileType to terrain string for Lua compatibility
fn convert_tilemap_to_terrain(tiles: &Vec<Vec<TileType>>) -> Vec<Vec<String>> {
    tiles.iter()
        .map(|row| {
            row.iter()
                .map(|tile| match tile {
                    TileType::Grass => "grass".to_string(),
                    TileType::Stone => "stone".to_string(),
                    TileType::Sand => "sand".to_string(),
                    TileType::Water => "water".to_string(),
                    TileType::DeepWater => "water".to_string(),
                    TileType::Tree => "forest".to_string(),
                    _ => "grass".to_string(),
                })
                .collect()
        })
        .collect()
}

/// Simplified Rust implementation of tree generation logic
fn generate_trees_rust_impl(map_data: &MapData, debug: &DebugSystem) -> Vec<TreeData> {
    let mut trees = Vec::new();
    let tree_density = 0.15;
    let cluster_size = 3;
    let cluster_variation = 2;
    
    // Calculate number of clusters
    let total_area = map_data.width * map_data.height;
    let target_clusters = (total_area as f32 * tree_density / cluster_size as f32) as usize;
    
    debug.log(
        DebugLevel::Debug,
        "TREE_GEN",
        &format!("Generating {} tree clusters for {}x{} area", target_clusters, map_data.width, map_data.height)
    );
    
    for _ in 0..target_clusters {
        // Generate cluster center
        let cluster_x = rand::thread_rng().gen_range(5..map_data.width - 5);
        let cluster_y = rand::thread_rng().gen_range(5..map_data.height - 5);
        let cluster_tree_count = (cluster_size + rand::thread_rng().gen_range(-cluster_variation..=cluster_variation)).max(1) as usize;
        
        // Generate trees in this cluster
        for _ in 0..cluster_tree_count {
            if let Some(tree) = generate_tree_in_cluster(cluster_x, cluster_y, map_data) {
                trees.push(tree);
            }
        }
    }
    
    debug.log(
        DebugLevel::Debug,
        "TREE_GEN", 
        &format!("Generated {} trees total", trees.len())
    );
    
    trees
}

/// Generate a single tree within a cluster
fn generate_tree_in_cluster(cluster_x: usize, cluster_y: usize, map_data: &MapData) -> Option<TreeData> {
    // Try to place tree within 4 tiles of cluster center
    for _ in 0..10 { // Max 10 attempts
        let radius = rand::thread_rng().gen::<f32>() * 4.0;
        let angle = rand::thread_rng().gen::<f32>() * 2.0 * std::f32::consts::PI;
        let x = (cluster_x as f32 + radius * angle.cos()) as usize;
        let y = (cluster_y as f32 + radius * angle.sin()) as usize;
        
        if is_valid_tree_position(x, y, map_data) {
            let tree_type = select_tree_type(x, y, map_data);
            return Some(create_tree_data(x, y, tree_type));
        }
    }
    None
}

/// Check if a position is valid for tree placement
fn is_valid_tree_position(x: usize, y: usize, map_data: &MapData) -> bool {
    // Check bounds
    if x >= map_data.width || y >= map_data.height {
        return false;
    }
    
    // Check terrain
    let terrain = &map_data.terrain[y][x];
    let terrain_preference = match terrain.as_str() {
        "grass" => 1.0,
        "dirt" => 0.8,
        "forest" => 2.0,
        "sand" => 0.2,
        "stone" => 0.1,
        "water" => 0.0,
        "mountain" => 0.3,
        _ => 0.5,
    };
    
    if rand::thread_rng().gen::<f32>() > terrain_preference {
        return false;
    }
    
    // Check for existing trees (no overlaps)
    for (existing_x, existing_y) in &map_data.existing_trees {
        if *existing_x == x && *existing_y == y {
            return false;
        }
    }
    
    true
}

/// Select tree type based on terrain and randomness
fn select_tree_type(x: usize, y: usize, map_data: &MapData) -> &'static str {
    let terrain = &map_data.terrain[y][x];
    
    match terrain.as_str() {
        "mountain" => {
            if rand::thread_rng().gen::<f32>() < 0.7 { "pine" } else { "oak" }
        }
        "forest" => {
            match rand::thread_rng().gen_range(0..4) {
                0 => "oak",
                1 => "pine", 
                2 => "birch",
                _ => "oak",
            }
        }
        "grass" => {
            match rand::thread_rng().gen_range(0..10) {
                0..=3 => "oak",
                4..=6 => "birch",
                7..=8 => "pine",
                _ => "apple",
            }
        }
        _ => "oak"
    }
}

/// Create tree data structure
fn create_tree_data(x: usize, y: usize, tree_type: &str) -> TreeData {
    let (wood_yield, fruit_yield, respawn_time) = match tree_type {
        "oak" => (15, 0, 60.0),
        "pine" => (12, 0, 45.0),
        "birch" => (8, 0, 30.0),
        "apple" => (6, 5, 90.0),
        _ => (10, 0, 50.0),
    };
    
    TreeData {
        x,
        y,
        tree_type: tree_type.to_string(),
        wood_yield,
        fruit_yield,
        growth_stage: "mature".to_string(),
        harvestable: true,
        respawn_time,
    }
}

/// Spawn a tree entity from tree data
fn spawn_tree_entity(commands: &mut Commands, tree_data: TreeData, debug: &DebugSystem) {
    let world_x = (tree_data.x as f32 - MAP_SIZE as f32 / 2.0) * 10.0; // TILE_SIZE
    let world_y = (tree_data.y as f32 - MAP_SIZE as f32 / 2.0) * 10.0;
    
    commands.spawn((
        Sprite {
            color: Color::srgb(0.18, 0.31, 0.09),  // Dark green for trees
            custom_size: Some(Vec2::new(8.0, 8.0)),
            ..default()
        },
        Transform::from_xyz(world_x, world_y, 0.5),
        NameComponent::new(format!("{} Tree", tree_data.tree_type)),
        PositionComponent::from_tile(tree_data.x, tree_data.y),
        TileEntity { x: tree_data.x, y: tree_data.y },
        TreeTag,  // Marker for AI to find trees
        ScriptedTree {
            tree_type: tree_data.tree_type.clone(),
            wood_yield: tree_data.wood_yield,
            fruit_yield: tree_data.fruit_yield,
            growth_stage: tree_data.growth_stage,
            harvestable: tree_data.harvestable,
            respawn_time: tree_data.respawn_time,
        },
        // Add resource node for work system with tick-based regeneration
        crate::components::ResourceNode::tree(tree_data.wood_yield),
        crate::components::ResourceRegenerationTag,
        // Add grid position for work system
        crate::components::GridPosition {
            x: tree_data.x as u32,
            y: tree_data.y as u32,
        },
    ));
    
    debug.log(
        DebugLevel::Debug,
        "TREE_GEN",
        &format!("Spawned {} tree at ({}, {})", tree_data.tree_type, tree_data.x, tree_data.y)
    );
}

/// Map data structure for tree generation
#[derive(Debug)]
struct MapData {
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
    terrain: Vec<Vec<String>>,
    buildings: Vec<(usize, usize)>,
    existing_trees: Vec<(usize, usize)>,
}

/// Tree data structure from generation
#[derive(Debug)]
struct TreeData {
    x: usize,
    y: usize,
    tree_type: String,
    wood_yield: u32,
    fruit_yield: u32,
    growth_stage: String,
    harvestable: bool,
    respawn_time: f32,
}