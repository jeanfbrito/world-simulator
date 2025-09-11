use bevy::prelude::*;
use bevy::asset::AssetPlugin;
// Removed bevy_egui import for headless operation
// use bevy_dogoap::prelude::*; // Temporarily disabled for testing
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;
use colored::Colorize;

mod websocket;
mod legacy_simulation;
mod simulation;
mod debug;
#[path = "debug/ai_monitor.rs"]
mod ai_monitor;
// // mod debug_cli; // Disabled for headless operation
mod components;
mod plugin;
mod plugins;
mod tilemap;
mod resources;
mod buildings;
mod crafting;
mod ai;
mod save_load;
mod performance;
mod scripting;
mod spawning;
mod systems;

use websocket::WebSocketPlugin;
use debug::DebugPlugin;
// use debug_cli::DebugCLI; // Disabled for headless operation
use components::{
    ComponentsPlugin, PositionComponent, 
    NameComponent, WorkerTag
};
use plugin::{PluginManager, plugin_init_system};
use plugins::{WorldPlugin, SimulationPlugin as SimPlugin};
use tilemap::TilemapPlugin;
use resources::ResourcesPlugin;
use buildings::{BuildingsPlugin, BuildingComponent, BuildingType};
use crafting::CraftingPlugin;
use ai::AIPlugin;
use save_load::SaveLoadPlugin;
use performance::PerformancePlugin;
use scripting::ScriptingPlugin;
use spawning::SpawningPlugin;
use systems::SystemsPlugin;

// Import the new tick-based simulation module
use simulation::TickSimulationPlugin;

pub const MAP_SIZE: usize = 64;
const TILE_SIZE: f32 = 10.0;

fn main() {
    // Initialize env_logger for terminal output
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    App::new()
        .add_plugins(MinimalPlugins) // Headless operation - no window, no rendering
        .add_plugins(AssetPlugin::default()) // Add asset system for scripting without rendering
        // Removed EguiPlugin for headless operation
        // .add_plugins(DogoapPlugin) // Temporarily disabled for testing
        .add_plugins(TickSimulationPlugin) // Core tick-based simulation
        .add_plugins(WebSocketPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(ComponentsPlugin)
        .init_resource::<PluginManager>()
        .add_plugins(WorldPlugin)
        .add_plugins(SimPlugin)
        .add_plugins(TilemapPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(BuildingsPlugin)
        .add_plugins(CraftingPlugin)
        .add_plugins(AIPlugin)
        .add_plugins(SaveLoadPlugin)
        .add_plugins(PerformancePlugin)
        .add_plugins(SpawningPlugin)
        .add_plugins(SystemsPlugin)  // Add the new systems plugin
        // .add_plugins(ScriptingPlugin) // Disabled for headless operation - requires Diagnostics resource
        .init_resource::<WorldMap>()
        .init_resource::<SimulationState>()
        // Removed SelectedTile resource initialization for headless operation
        .add_systems(Startup, setup)
        .add_systems(PostStartup, plugin_init_system)
        // Removed UI systems for headless operation
        .add_systems(Update, (
            // Simulation systems (can run in parallel) - headless mode
            simulation_system,
            systems::update_unit_needs_system,  // Keep old system for compatibility
            systems::sync_needs_to_worldstate_system,
            ai_monitor::simple_ai_monitor_system,
        ))
        .run();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Grass,
    Stone,
    Sand,
    Water,
    DeepWater,
    Tree,
    Ore,
    Berry,
    Wall,
    Floor,
    Door,
    Storage,
    Workshop,
    Empty,
    Blocked,
}

impl TileType {
    fn color(&self) -> Color {
        match self {
            TileType::Grass => Color::srgb(0.29, 0.36, 0.14),
            TileType::Stone => Color::srgb(0.4, 0.4, 0.4),
            TileType::Sand => Color::srgb(0.82, 0.71, 0.55),
            TileType::Water => Color::srgb(0.12, 0.42, 0.66),
            TileType::DeepWater => Color::srgb(0.05, 0.31, 0.55),
            TileType::Tree => Color::srgb(0.18, 0.31, 0.09),
            TileType::Ore => Color::srgb(0.55, 0.27, 0.07),
            TileType::Berry => Color::srgb(0.55, 0.0, 0.32),
            TileType::Wall => Color::srgb(0.27, 0.27, 0.27),
            TileType::Floor => Color::srgb(0.55, 0.45, 0.33),
            TileType::Door => Color::srgb(0.4, 0.26, 0.13),
            TileType::Storage => Color::srgb(0.8, 0.52, 0.25),
            TileType::Workshop => Color::srgb(0.44, 0.26, 0.08),
            TileType::Empty => Color::BLACK,
            TileType::Blocked => Color::srgb(0.13, 0.13, 0.13),
        }
    }
    
    fn is_walkable(&self) -> bool {
        !matches!(self, 
            TileType::Water | 
            TileType::DeepWater | 
            TileType::Wall | 
            TileType::Blocked |
            TileType::Tree |
            TileType::Ore
        )
    }
}

#[derive(Resource)]
pub struct WorldMap {
    pub tiles: Vec<Vec<TileType>>,
    entities: HashMap<(usize, usize), Vec<Entity>>,
}

impl Default for WorldMap {
    fn default() -> Self {
        let mut tiles = vec![vec![TileType::Grass; MAP_SIZE]; MAP_SIZE];
        
        // Generate simple island
        let center = MAP_SIZE / 2;
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let dist = ((x as f32 - center as f32).powi(2) + 
                           (y as f32 - center as f32).powi(2)).sqrt();
                let max_dist = center as f32;
                
                if dist > max_dist * 0.9 {
                    tiles[y][x] = TileType::DeepWater;
                } else if dist > max_dist * 0.75 {
                    tiles[y][x] = TileType::Water;
                } else if dist > max_dist * 0.6 {
                    tiles[y][x] = TileType::Sand;
                }
            }
        }
        
        // Add some random features
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let x = rng.gen_range(10..MAP_SIZE-10);
            let y = rng.gen_range(10..MAP_SIZE-10);
            if tiles[y][x] == TileType::Grass {
                tiles[y][x] = if rng.gen_bool(0.5) { TileType::Tree } else { TileType::Ore };
            }
        }
        
        Self {
            tiles,
            entities: HashMap::new(),
        }
    }
}

#[derive(Resource)]
pub struct SimulationState {
    pub running: bool,
    pub tick: u32,
    pub speed: f32,
    accumulated_time: f32,
    changed: bool,
    pub just_ticked: bool,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            running: true,  // Start simulation running automatically
            tick: 0,
            speed: 1.0,
            accumulated_time: 0.0,
            changed: false,
            just_ticked: false,
        }
    }
}

impl SimulationState {
    pub fn set_changed(&mut self) {
        self.changed = true;
    }
    
    pub fn is_changed(&self) -> bool {
        self.changed
    }
    
    pub fn clear_changed(&mut self) {
        self.changed = false;
    }
}

// Removed SelectedTile resource for headless operation

#[derive(Component)]
pub struct TileEntity {
    pub x: usize,
    pub y: usize,
}

// Worker entity is now composed of multiple components instead of a single struct

fn setup(mut commands: Commands) {
    // Removed Camera2d for headless operation
    
    // Initialize world map for headless operation (no tile sprites)
    let world_map = WorldMap::default();
    
    // Peasant spawning is now handled by SpawningPlugin
    let mut rng = rand::thread_rng();
    
    // Spawn stockpile (for wood/stone storage) at center of map
    {
        let x = 32;
        let y = 32;
        let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
        let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
        
        commands.spawn((
            Sprite {
                color: Color::srgb(0.5, 0.3, 0.1), // Brown for stockpile
                custom_size: Some(Vec2::new(TILE_SIZE * 2.0, TILE_SIZE * 2.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 0.5),
            BuildingComponent::new(BuildingType::Stockpile, (x as i32, y as i32)),
            NameComponent::new("Central Stockpile".to_string()),
            PositionComponent::from_tile(x, y),
            TileEntity { x, y },
        ));
        
        // Mark it as complete (pre-built)
        commands.spawn((
            BuildingComponent {
                building_type: BuildingType::Stockpile,
                health: 200.0,
                max_health: 200.0,
                construction_progress: 1.0,
                is_active: true,
                position: (x as i32, y as i32),
            },
        ));
        
        println!("{}", format!("[SPAWN] Stockpile at ({}, {})", x, y).cyan());
    }
    
    // Spawn granary (for food storage) near stockpile
    {
        let x = 35;
        let y = 32;
        let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
        let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
        
        commands.spawn((
            BuildingComponent::new(BuildingType::Granary, (x as i32, y as i32)),
            NameComponent::new("Central Granary".to_string()),
            PositionComponent::from_tile(x, y),
            TileEntity { x, y },
        ));
        
        // Mark it as complete (pre-built)
        commands.spawn((
            BuildingComponent {
                building_type: BuildingType::Granary,
                health: 200.0,
                max_health: 200.0,
                construction_progress: 1.0,
                is_active: true,
                position: (x as i32, y as i32),
            },
        ));
        
        println!("{}", format!("[SPAWN] Granary at ({}, {})", x, y).cyan());
    }
    
    // Spawn trees as entities (for wood harvesting)
    for _ in 0..15 {
        let x = rng.gen_range(10..54);
        let y = rng.gen_range(10..54);
        
        if world_map.tiles[y][x] == TileType::Grass {
            let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            
            commands.spawn((
                NameComponent::new("Tree".to_string()),
                PositionComponent::from_tile(x, y),
                TileEntity { x, y },
                ai::TreeTag,  // Marker for AI to find trees
            ));
            
            println!("{}", format!("[SPAWN] Tree at ({}, {})", x, y).green());
        }
    }
    
    // Spawn rocks as entities (for stone harvesting)
    for _ in 0..12 {
        let x = rng.gen_range(10..54);
        let y = rng.gen_range(10..54);
        
        if world_map.tiles[y][x] == TileType::Grass {
            let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            
            commands.spawn((
                NameComponent::new("Rock".to_string()),
                PositionComponent::from_tile(x, y),
                TileEntity { x, y },
                ai::RockTag,  // Marker for AI to find rocks
            ));
            
            println!("{}", format!("[SPAWN] Rock at ({}, {})", x, y).bright_black());
        }
    }
    
    // Add some berry bushes as resources
    for _ in 0..10 {
        let x = rng.gen_range(15..49);
        let y = rng.gen_range(15..49);
        
        if world_map.tiles[y][x] == TileType::Grass {
            let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            
            commands.spawn((
                NameComponent::new("Berry Bush".to_string()),
                PositionComponent::from_tile(x, y),
                components::ResourceNode {
                    resource_type: resources::ResourceType::Berries,
                    amount: 10,
                    max_amount: 10,
                    yield_amount: 3,
                    respawn_time: 30.0,
                    time_since_depletion: 0.0,
                },
                components::GridPosition { x: x as u32, y: y as u32 },
                TileEntity { x, y },
                ai::BerryBushTag,  // Add marker for AI to find berries
            ));
            
            println!("{}", format!("[SPAWN] Berry Bush at ({}, {})", x, y).magenta());
        }
    }
}

// Removed ui_system for headless operation

// Removed tile_interaction_system for headless operation

fn simulation_system(
    time: Res<Time>,
    mut sim_state: ResMut<SimulationState>,
    workers: Query<(&mut Transform, &mut TileEntity), With<WorkerTag>>,
    world_map: Res<WorldMap>,
) {
    if !sim_state.running {
        return;
    }
    
    // Reset the just_ticked flag at the beginning of each frame
    sim_state.just_ticked = false;
    
    sim_state.accumulated_time += time.delta_secs() * sim_state.speed;
    
    if sim_state.accumulated_time >= 1.0 {
        sim_state.accumulated_time = 0.0;
        sim_state.tick += 1;
        sim_state.just_ticked = true; // Set the flag when a tick occurs
        
        // Log tick for easy reading
        println!("{}", format!("\n=== TICK {} ===", sim_state.tick).bright_blue());
        
        // Movement is now handled by the AI task execution system
        // Workers will move according to their GOAP plans
    }
}

// Removed render_map_system for headless operation

// Debug CLI disabled for headless operation
// fn setup_debug_cli(debug: Res<DebugSystem>) {
//     let cli = DebugCLI::new(debug.get_command_sender());
//     cli.start();
// }