//! World generation using Wave Function Collapse and other algorithms

use bevy::prelude::*;
use bevy_entitiles::algorithm::wfc::*;
use world_sim_interface::Position;
use super::{TileType, world_grid::WorldGrid, layers::LayerType};
use rand::Rng;

/// Configuration for world generation
#[derive(Resource)]
pub struct WorldGenConfig {
    pub width: u32,
    pub height: u32,
    pub seed: u64,
    pub biome_type: BiomeType,
    pub resource_density: f32,
}

impl Default for WorldGenConfig {
    fn default() -> Self {
        Self {
            width: 32,  // Small world for testing
            height: 32, // Small world for testing
            seed: 42,
            biome_type: BiomeType::Mixed,
            resource_density: 0.15,
        }
    }
}

/// Types of biomes for generation
#[derive(Debug, Clone, Copy)]
pub enum BiomeType {
    Forest,
    Mountain,
    Plains,
    Desert,
    Mixed,
}

/// Generate a world using Wave Function Collapse
pub fn generate_world_wfc(
    config: &WorldGenConfig,
    world_grid: &mut WorldGrid,
) {
    // Define tile adjacency rules for WFC
    let rules = create_wfc_rules(config.biome_type);
    
    // Generate base terrain
    generate_terrain_wfc(config, world_grid, &rules);
    
    // Add resources
    place_resources(config, world_grid);
    
    // Add initial structures
    place_starting_structures(config, world_grid);
}

/// Create WFC rules based on biome type
fn create_wfc_rules(biome: BiomeType) -> WfcRules {
    let mut rules = WfcRules::new();
    
    // Define which tiles can be adjacent
    match biome {
        BiomeType::Forest => {
            // Forest rules: mostly grass and trees
            rules.add_adjacency(TileType::Grass, TileType::Grass, 1.0);
            rules.add_adjacency(TileType::Grass, TileType::Tree, 0.7);
            rules.add_adjacency(TileType::Tree, TileType::Tree, 0.5);
            rules.add_adjacency(TileType::Grass, TileType::Water, 0.2);
        }
        BiomeType::Mountain => {
            // Mountain rules: stone and ore
            rules.add_adjacency(TileType::Stone, TileType::Stone, 1.0);
            rules.add_adjacency(TileType::Stone, TileType::Grass, 0.3);
            rules.add_adjacency(TileType::Stone, TileType::OreNode, 0.4);
        }
        BiomeType::Plains => {
            // Plains rules: mostly grass
            rules.add_adjacency(TileType::Grass, TileType::Grass, 1.0);
            rules.add_adjacency(TileType::Grass, TileType::Sand, 0.2);
            rules.add_adjacency(TileType::Grass, TileType::BerryBush, 0.3);
        }
        BiomeType::Desert => {
            // Desert rules: sand and stone
            rules.add_adjacency(TileType::Sand, TileType::Sand, 1.0);
            rules.add_adjacency(TileType::Sand, TileType::Stone, 0.4);
        }
        BiomeType::Mixed => {
            // Mixed biome: variety
            rules.add_adjacency(TileType::Grass, TileType::Grass, 0.8);
            rules.add_adjacency(TileType::Grass, TileType::Stone, 0.3);
            rules.add_adjacency(TileType::Grass, TileType::Sand, 0.2);
            rules.add_adjacency(TileType::Grass, TileType::Water, 0.3);
            rules.add_adjacency(TileType::Water, TileType::Water, 0.9);
            rules.add_adjacency(TileType::Stone, TileType::Stone, 0.7);
        }
    }
    
    rules
}

/// Simplified WFC rules structure (would use bevy_entitiles' actual WFC in production)
struct WfcRules {
    adjacencies: Vec<(TileType, TileType, f32)>,
}

impl WfcRules {
    fn new() -> Self {
        Self {
            adjacencies: Vec::new(),
        }
    }
    
    fn add_adjacency(&mut self, from: TileType, to: TileType, weight: f32) {
        self.adjacencies.push((from, to, weight));
    }
}

/// Generate terrain using WFC
fn generate_terrain_wfc(
    config: &WorldGenConfig,
    world_grid: &mut WorldGrid,
    _rules: &WfcRules,
) {
    let mut rng = rand::thread_rng();
    
    // Simple generation for now (would use actual WFC algorithm)
    for x in 0..config.width {
        for y in 0..config.height {
            let pos = Position::new(x as i32, y as i32);
            
            // Simple noise-based terrain generation as placeholder
            let noise_value = simple_noise(x as f32 / 10.0, y as f32 / 10.0, config.seed);
            
            let tile_type = match config.biome_type {
                BiomeType::Forest => {
                    if noise_value > 0.3 {
                        TileType::Grass
                    } else {
                        TileType::Water
                    }
                }
                BiomeType::Mountain => {
                    if noise_value > 0.5 {
                        TileType::Stone
                    } else {
                        TileType::Grass
                    }
                }
                BiomeType::Plains => {
                    if noise_value > 0.1 {
                        TileType::Grass
                    } else {
                        TileType::Water
                    }
                }
                BiomeType::Desert => {
                    if noise_value > 0.2 {
                        TileType::Sand
                    } else {
                        TileType::Stone
                    }
                }
                BiomeType::Mixed => {
                    if noise_value > 0.7 {
                        TileType::Stone
                    } else if noise_value > 0.3 {
                        TileType::Grass
                    } else if noise_value > 0.1 {
                        TileType::Sand
                    } else {
                        TileType::Water
                    }
                }
            };
            
            world_grid.set_terrain(pos, tile_type);
        }
    }
    
    // Add rivers
    generate_rivers(config, world_grid, &mut rng);
}

/// Generate rivers through the terrain
fn generate_rivers(
    config: &WorldGenConfig,
    world_grid: &mut WorldGrid,
    rng: &mut impl Rng,
) {
    // Generate 1-3 rivers
    let num_rivers = rng.gen_range(1..=3);
    
    for _ in 0..num_rivers {
        let start_x = rng.gen_range(0..config.width) as i32;
        let mut current = Position::new(start_x, 0);
        
        // Flow downward with some meandering
        while current.y < config.height as i32 {
            world_grid.set_terrain(current, TileType::Water);
            
            // Meander left or right
            let dx = rng.gen_range(-1..=1);
            current.x = (current.x + dx).clamp(0, config.width as i32 - 1);
            current.y += 1;
        }
    }
}

/// Place resources on the map
fn place_resources(
    config: &WorldGenConfig,
    world_grid: &mut WorldGrid,
) {
    let mut rng = rand::thread_rng();
    let num_resources = ((config.width * config.height) as f32 * config.resource_density) as u32;
    
    for _ in 0..num_resources {
        let x = rng.gen_range(0..config.width) as i32;
        let y = rng.gen_range(0..config.height) as i32;
        let pos = Position::new(x, y);
        
        // Only place resources on appropriate terrain
        if let Some(tile) = world_grid.get_tile(&pos) {
            let resource = match tile.terrain {
                TileType::Grass => {
                    if rng.gen_bool(0.7) {
                        Some(TileType::Tree)
                    } else {
                        Some(TileType::BerryBush)
                    }
                }
                TileType::Stone => {
                    if rng.gen_bool(0.3) {
                        Some(TileType::OreNode)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            
            if let Some(resource_type) = resource {
                world_grid.set_resource(pos, Some(resource_type));
            }
        }
    }
}

/// Place starting structures
fn place_starting_structures(
    config: &WorldGenConfig,
    world_grid: &mut WorldGrid,
) {
    // Find a suitable starting location (center of map for now)
    let center_x = config.width as i32 / 2;
    let center_y = config.height as i32 / 2;
    
    // Clear a small area for the settlement
    for dx in -3..=3 {
        for dy in -3..=3 {
            let pos = Position::new(center_x + dx, center_y + dy);
            world_grid.set_terrain(pos, TileType::Grass);
            world_grid.set_resource(pos, None);
        }
    }
    
    // Place initial buildings
    let storage_pos = Position::new(center_x, center_y);
    world_grid.set_building(storage_pos, Some(TileType::Storage));
    
    let workshop_pos = Position::new(center_x + 2, center_y);
    world_grid.set_building(workshop_pos, Some(TileType::Workshop));
    
    // Place some floor tiles around buildings
    for dx in -1..=1 {
        for dy in -1..=1 {
            let floor_pos1 = Position::new(center_x + dx, center_y + dy);
            let floor_pos2 = Position::new(center_x + 2 + dx, center_y + dy);
            
            if world_grid.get_tile(&floor_pos1).map(|t| t.building.is_none()).unwrap_or(false) {
                world_grid.set_building(floor_pos1, Some(TileType::Floor));
            }
            if world_grid.get_tile(&floor_pos2).map(|t| t.building.is_none()).unwrap_or(false) {
                world_grid.set_building(floor_pos2, Some(TileType::Floor));
            }
        }
    }
}

/// Simple noise function for terrain generation
fn simple_noise(x: f32, y: f32, seed: u64) -> f32 {
    // Very simple pseudo-noise based on position and seed
    let value = (x * 12.9898 + y * 78.233 + seed as f32 * 0.1).sin() * 43758.5453;
    (value - value.floor()).abs()
}

/// System to trigger world generation
pub fn trigger_world_generation_system(
    gen_config: Res<WorldGenConfig>,
    mut world_grid: ResMut<WorldGrid>,
    mut commands: Commands,
) {
    // Only generate once at startup (would have proper trigger logic)
    if world_grid.width == 0 {
        // Initialize world grid
        *world_grid = WorldGrid::new(gen_config.width, gen_config.height);
        
        // Generate world
        generate_world_wfc(&gen_config, &mut world_grid);
        
        // Mark generation complete
        commands.insert_resource(WorldGenerated);
    }
}

/// Marker resource for completed world generation
#[derive(Resource)]
pub struct WorldGenerated;