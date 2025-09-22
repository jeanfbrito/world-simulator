use bevy::asset::AssetPlugin;
use bevy::prelude::*;
// Removed bevy_egui import for headless operation
// use bevy_dogoap::prelude::*; // Temporarily disabled for testing
use crate::components::UnitTag;
use colored::Colorize;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[path = "debug/ai_monitor.rs"]
mod ai_monitor;
mod debug;
mod legacy_simulation;
mod simulation;
mod websocket;
// // mod debug_cli; // Disabled for headless operation
mod ai;
mod buildings;
mod components;
mod crafting;
mod packs;
mod performance;
mod plugin;
mod plugins;
mod resources;
mod save_load;
mod scripting;
mod spawning;
mod systems;
mod tilemap;

use debug::DebugPlugin;
// use debug_cli::DebugCLI; // Disabled for headless operation
use ai::AIPlugin;
use buildings::{BuildingComponent, BuildingType, BuildingsPlugin};
use components::{ComponentsPlugin, NameComponent, PositionComponent};
use crafting::CraftingPlugin;
use packs::{PackSystemPlugin, Registry};
use performance::PerformancePlugin;
use plugin::{plugin_init_system, PluginManager};
use plugins::{SimulationPlugin as SimPlugin, WorldPlugin};
use resources::ResourcesPlugin;
use save_load::SaveLoadPlugin;
use spawning::SpawningPlugin;
use systems::SystemsPlugin;
use tilemap::TilemapPlugin;
use websocket::WebSocketPlugin;

// Import the new tick-based simulation module

pub const MAP_SIZE: usize = 64;
const TILE_SIZE: f32 = 10.0;

fn main() {
    println!("🚀 Starting World Simulator (Headless Mode)");

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    println!("📦 Initializing Bevy App...");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: None,
            exit_condition: bevy::window::ExitCondition::DontExit,
            close_when_requested: false,
        })) // Use DefaultPlugins for proper time support but disable window
        // Removed EguiPlugin for headless operation
        // .add_plugins(DogoapPlugin) // Temporarily disabled for testing
        .add_plugins(simulation::TickSimulationPlugin) // Enable the new tick event system
        .add_plugins(WebSocketPlugin) // Re-enabled for web viewer
        .add_plugins(DebugPlugin)
        .add_plugins(ComponentsPlugin)
        .add_plugins(PackSystemPlugin) // Load data-driven content
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
        .add_plugins(SystemsPlugin)  // Add the new systems plugin (includes work systems)
        // .add_plugins(ScriptingPlugin) // Disabled for headless operation - requires Diagnostics resource
        .init_resource::<WorldMap>()
        .init_resource::<SimulationState>()
        // Removed SelectedTile resource initialization for headless operation
        .add_systems(Startup, setup)
        .add_systems(PostStartup, plugin_init_system)
        // Removed UI systems for headless operation
        .add_systems(Update, (
            // Simulation systems (can run in parallel) - headless mode
            // simulation_system, // Disabled - using new TickSimulationPlugin
            ai_monitor::simple_ai_monitor_system,
        ))
        // .add_systems(PostUpdate, reset_tick_flag_system) // Not needed with new tick system
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
    
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Grass | TileType::Sand | TileType::Floor | TileType::Door => true,
            TileType::Stone => true, // Can walk on stone tiles
            TileType::Water | TileType::DeepWater => false,
            TileType::Tree | TileType::Ore | TileType::Berry => false, // Resources block movement
            TileType::Wall | TileType::Blocked => false,
            TileType::Storage | TileType::Workshop => false, // Buildings block movement
            TileType::Empty => true,
        }
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
                let dist = ((x as f32 - center as f32).powi(2)
                    + (y as f32 - center as f32).powi(2))
                .sqrt();
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
            let x = rng.gen_range(10..MAP_SIZE - 10);
            let y = rng.gen_range(10..MAP_SIZE - 10);
            if tiles[y][x] == TileType::Grass {
                tiles[y][x] = if rng.gen_bool(0.5) {
                    TileType::Tree
                } else {
                    TileType::Ore
                };
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
            running: true, // Start simulation running automatically
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

fn setup(
    mut commands: Commands,
    pack_system: Option<Res<packs::PackSystem>>,
) {
    // Removed Camera2d for headless operation

    // Initialize world map for headless operation (no tile sprites)
    let world_map = WorldMap::default();

    // Peasant spawning is now handled by SpawningPlugin
    let mut rng = rand::thread_rng();

    // Spawn stockpile (for wood/stone storage) at center of map
    {
        let x = 32;
        let y = 32;
        // No rendering components needed - we're not rendering
        
        commands.spawn((
            BuildingComponent::new(BuildingType::Stockpile, (x as i32, y as i32)),
            NameComponent::new("Central Stockpile".to_string()),
            PositionComponent::from_tile(x, y),
            TileEntity { x, y },
        ));

        // Mark it as complete (pre-built)
        commands.spawn((BuildingComponent {
            building_type: BuildingType::Stockpile,
            health: 200.0,
            max_health: 200.0,
            construction_progress: 1.0,
            is_active: true,
            position: (x as i32, y as i32),
        },));

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
        commands.spawn((BuildingComponent {
            building_type: BuildingType::Granary,
            health: 200.0,
            max_health: 200.0,
            construction_progress: 1.0,
            is_active: true,
            position: (x as i32, y as i32),
        },));

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
                ai::TreeTag, // Marker for AI to find trees
            ));

            println!("{}", format!("[SPAWN] Tree at ({}, {})", x, y).green());
        }
    }

    // Rocks removed - focusing on food resources only for Phase 3.5

    // Add many berry bushes spread across the map to encourage exploration
    // Spawn berry bushes in clusters for more realistic distribution

    // Try to get berry bush data from pack system
    let berry_bush_def = pack_system
        .as_ref()
        .and_then(|ps| ps.resource_registry.get("berry_bush"));

    if let Some(def) = berry_bush_def {
        println!(
            "{}",
            format!("[PACK] Using berry bush definition from pack: {}", def.name).cyan()
        );
    }

    // Spawn scattered berry bushes across the whole map
    let mut spawned_bushes = 0;
    let target_bushes = 25;
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 500; // Prevent infinite loop

    while spawned_bushes < target_bushes && attempts < MAX_ATTEMPTS {
        attempts += 1;
        let x = rng.gen_range(5..59); // Expanded range to cover more of the map
        let y = rng.gen_range(5..59);

        if world_map.tiles[y][x] == TileType::Grass {
            let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;

            // Use pack data if available, otherwise use defaults
            let berry_count = if let Some(def) = berry_bush_def {
                if let Some(harvestable) = &def.harvestable {
                    rng.gen_range(harvestable.r#yield[0].min..=harvestable.r#yield[0].max)
                } else {
                    rng.gen_range(1..4)
                }
            } else {
                rng.gen_range(1..4)
            };

            commands.spawn((
                NameComponent::new(berry_bush_def.map(|d| d.name.clone()).unwrap_or("Berry Bush".to_string())),
                PositionComponent::from_tile(x, y),
                components::ResourceNode::fruit_bush(berry_count as u32), // Use pack data for berry count
                components::GrowingResource::fruit_bush(berry_count as u32, 10), // GrowingResource for berry growth
                components::ResourceRegenerationTag,
                components::GridPosition {
                    x: x as u32,
                    y: y as u32,
                },
                TileEntity { x, y },
                ai::BerryBushTag, // Add marker for AI to find berries
            ));

            println!(
                "{}",
                format!("[SPAWN] {} at ({}, {}) with {} berries (from pack data)",
                    berry_bush_def.map(|d| &d.name[..]).unwrap_or("Berry Bush"),
                    x, y, berry_count
                ).magenta()
            );
            spawned_bushes += 1;
        }
    }

    if spawned_bushes < target_bushes {
        println!(
            "{}",
            format!(
                "[SPAWN] Only spawned {} of {} berry bushes (terrain limited)",
                spawned_bushes, target_bushes
            )
            .yellow()
        );
    }

    // Add berry bush clusters in corners to encourage long-distance travel
    let corners = [(8, 8), (8, 56), (56, 8), (56, 56)];
    for (corner_x, corner_y) in corners.iter() {
        let mut corner_spawned = 0;
        let target_per_corner = 3;
        let mut corner_attempts = 0;

        while corner_spawned < target_per_corner && corner_attempts < 50 {
            corner_attempts += 1;
            let x = corner_x + rng.gen_range(0..5) as usize;
            let y = corner_y + rng.gen_range(0..5) as usize;

            if x < MAP_SIZE && y < MAP_SIZE && world_map.tiles[y][x] == TileType::Grass {
                // Use pack data for corner bushes too
                let berry_count = if let Some(def) = berry_bush_def {
                    if let Some(harvestable) = &def.harvestable {
                        rng.gen_range(harvestable.r#yield[0].min..=harvestable.r#yield[0].max)
                    } else {
                        rng.gen_range(1..4)
                    }
                } else {
                    rng.gen_range(1..4)
                };

                commands.spawn((
                    NameComponent::new(format!("{} (Corner)",
                        berry_bush_def.map(|d| &d.name[..]).unwrap_or("Berry Bush")
                    )),
                    PositionComponent::from_tile(x, y),
                    components::ResourceNode::fruit_bush(berry_count as u32), // Use pack data
                    components::GrowingResource::fruit_bush(berry_count as u32, 10), // GrowingResource for berry growth
                    components::ResourceRegenerationTag,
                    components::GridPosition {
                        x: x as u32,
                        y: y as u32,
                    },
                    TileEntity { x, y },
                    ai::BerryBushTag,
                ));

                println!(
                    "{}",
                    format!("[SPAWN] Corner Berry Bush at ({}, {})", x, y).bright_magenta()
                );
                corner_spawned += 1;
            }
        }

        if corner_spawned < target_per_corner {
            println!(
                "{}",
                format!(
                    "[SPAWN] Corner ({}, {}) only spawned {} of {} bushes",
                    corner_x, corner_y, corner_spawned, target_per_corner
                )
                .yellow()
            );
        }
    }
}

// Removed ui_system for headless operation

// Removed tile_interaction_system for headless operation

fn simulation_system(
    time: Res<Time>,
    mut sim_state: ResMut<SimulationState>,
    mut tick_events: EventWriter<simulation::SimulationTickEvent>,
    workers: Query<&mut TileEntity, With<UnitTag>>,
    world_map: Res<WorldMap>,
) {
    if !sim_state.running {
        return;
    }

    let delta = time.delta_secs();
    sim_state.accumulated_time += delta * sim_state.speed;

    // We want 10 ticks per second, so tick every 0.1 seconds
    const TICK_RATE: f32 = 0.1; // 10 TPS

    // Debug: Show that the system is running (only occasionally)
    if sim_state.tick % 10 == 0 && sim_state.accumulated_time < 0.02 {
        println!(
            "Simulation system called, delta: {}, accumulated: {}, tick: {}",
            delta, sim_state.accumulated_time, sim_state.tick
        );
    }

    if sim_state.accumulated_time >= TICK_RATE {
        sim_state.accumulated_time -= TICK_RATE; // Use subtraction to keep remainder
        sim_state.tick += 1;
        sim_state.just_ticked = true; // Keep for compatibility
        
        // Send tick event
        tick_events.write(simulation::SimulationTickEvent { tick: sim_state.tick as u64 });

        // Log tick for easy reading
        println!(
            "{}",
            format!("\n=== TICK {} ===", sim_state.tick).bright_blue()
        );

        // Movement is now handled by the AI task execution system
        // Workers will move according to their GOAP plans
    }
}

// System to reset the tick flag at the end of the frame
fn reset_tick_flag_system(mut sim_state: ResMut<SimulationState>) {
    // Reset the flag after all systems have had a chance to check it
    if sim_state.just_ticked {
        sim_state.just_ticked = false;
    }
}

// Removed render_map_system for headless operation

// Debug CLI disabled for headless operation
// fn setup_debug_cli(debug: Res<DebugSystem>) {
//     let cli = DebugCLI::new(debug.get_command_sender());
//     cli.start();
// }
