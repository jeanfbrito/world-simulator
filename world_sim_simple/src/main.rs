use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
// use bevy_dogoap::prelude::*; // Temporarily disabled for testing
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;
use colored::Colorize;

mod websocket;
mod simulation;
mod debug;
mod debug_cli;
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

use websocket::WebSocketPlugin;
use debug::{DebugPlugin, DebugSystem};
use debug_cli::DebugCLI;
use components::{
    ComponentsPlugin, PositionComponent, HealthComponent, 
    NameComponent, EnergyComponent, WorkerTag, WorkerStats
};
use resources::create_starter_inventory;
use plugin::{PluginManager, plugin_init_system};
use plugins::{WorldPlugin, SimulationPlugin as SimPlugin};
use tilemap::TilemapPlugin;
use resources::ResourcesPlugin;
use buildings::{BuildingsPlugin, BuildingComponent, BuildingType};
use crafting::CraftingPlugin;
use ai::{AIPlugin, WorkerAI, goap_actions::{WorldState, StateValue}};
use save_load::SaveLoadPlugin;
use performance::PerformancePlugin;
use scripting::ScriptingPlugin;

pub const MAP_SIZE: usize = 64;
const TILE_SIZE: f32 = 10.0;

fn main() {
    // Initialize env_logger for terminal output
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "World Simulator - Simple".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        // .add_plugins(DogoapPlugin) // Temporarily disabled for testing
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
        .add_plugins(ScriptingPlugin)
        .init_resource::<WorldMap>()
        .init_resource::<SimulationState>()
        .init_resource::<SelectedTile>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, (setup_debug_cli, plugin_init_system))
        .add_systems(Update, (
            // UI systems (sequential, can't parallelize with rendering)
            ui_system,
            tile_interaction_system,
        ).chain())
        .add_systems(Update, (
            // Simulation systems (can run in parallel)
            simulation_system,
            render_map_system,
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
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            running: true,  // Start simulation running automatically
            tick: 0,
            speed: 1.0,
            accumulated_time: 0.0,
            changed: false,
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

#[derive(Resource, Default)]
struct SelectedTile {
    position: Option<(usize, usize)>,
}

#[derive(Component)]
pub struct TileEntity {
    pub x: usize,
    pub y: usize,
}

// Worker entity is now composed of multiple components instead of a single struct

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
    
    // Spawn initial tiles as entities for rendering
    let world_map = WorldMap::default();
    
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let tile_type = world_map.tiles[y][x];
            let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            
            commands.spawn((
                Sprite {
                    color: tile_type.color(),
                    custom_size: Some(Vec2::new(TILE_SIZE - 1.0, TILE_SIZE - 1.0)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.0),
                TileEntity { x, y },
            ));
        }
    }
    
    // Spawn a few workers
    let mut rng = rand::thread_rng();
    for i in 0..5 {
        let x = rng.gen_range(20..44);
        let y = rng.gen_range(20..44);
        
        if world_map.tiles[y][x].is_walkable() {
            let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            
            let worker_entity = commands.spawn((
                // Rendering
                Sprite {
                    color: Color::srgb(1.0, 0.75, 0.0),
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.6, TILE_SIZE * 0.6)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 1.0),
                // Core components
                NameComponent::new(format!("Worker {}", i + 1)),
                PositionComponent::from_tile(x, y),
                HealthComponent::new(100.0),
                EnergyComponent::new(100.0),
                // Worker-specific components
                WorkerTag,
                WorkerStats::default(),
                // AI
                WorkerAI::new(),
                WorldState::new(),  // GOAP world state
                // Inventory
                create_starter_inventory(),
                // Tile tracking
                TileEntity { x, y },
            ))
            .insert((
                // GOAP states (initial values)
                components::IsHungry(0.0),
                components::HasEnergy(1.0),
                components::IsWorking(false),
                components::IsIdle(true),
                components::HasWood(0),  // No initial wood, must gather
                components::HasHouse(false),  // Worker starts without a house
                components::HasFood(5),  // Start with some food to survive initially
                components::HasStone(0),  // No initial stone, must gather
                components::InventoryFull(false),
                components::InventoryEmpty(false),  // Not empty since has food
            ))
            .insert((
                // Location states
                components::AtResource(false),
                components::AtStorage(false),
                components::AtHome(false),
                components::AtCraftingStation(false),
                // Building ownership and availability
                components::HasHouse(false),  // Workers start without a house
                components::StorageAvailable(true),  // We spawned stockpile
                components::HouseAvailable(false),
                components::WorkshopAvailable(false),
                components::FarmAvailable(false),
            ))
            .insert({
                let mut ws = WorldState::new();
                // Initialize with current component values
                ws.set("is_hungry", StateValue::Float(0.0));
                ws.set("has_energy", StateValue::Float(1.0));
                ws.set("has_wood", StateValue::Int(10));  // From inventory
                ws.set("has_food", StateValue::Int(20));  // Berries from inventory  
                ws.set("has_stone", StateValue::Int(5));  // From inventory
                ws.set("has_house", StateValue::Bool(false));
                ws.set("at_storage", StateValue::Bool(false));
                ws.set("inventory_full", StateValue::Bool(false));
                ws
            })  // Add GOAP world state for planning with initial values
            .id();
            
            // Log worker creation with component-based architecture
            println!("{}", format!("[SPAWN] Worker {} at ({}, {}) - Components: Name, Position, Health, Energy, WorkerTag, WorkerStats, GOAP States", 
                i + 1, x, y).green());
        }
    }
    
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
            Sprite {
                color: Color::srgb(0.7, 0.6, 0.2), // Yellow-brown for granary
                custom_size: Some(Vec2::new(TILE_SIZE * 2.0, TILE_SIZE * 2.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 0.5),
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
                Sprite {
                    color: Color::srgb(0.18, 0.31, 0.09),  // Dark green for trees
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.5),
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
                Sprite {
                    color: Color::srgb(0.4, 0.4, 0.4),  // Gray for rocks
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.7, TILE_SIZE * 0.7)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.5),
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
                Sprite {
                    color: Color::srgb(0.8, 0.1, 0.4),
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.5, TILE_SIZE * 0.5)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.5),
                NameComponent::new("Berry Bush".to_string()),
                PositionComponent::from_tile(x, y),
                components::ResourceNode {
                    resource_type: resources::ResourceType::Berries,
                    amount: 10,
                    max_amount: 10,
                    respawn_time: 30.0,
                    time_since_depletion: 0.0,
                },
                TileEntity { x, y },
                ai::BerryBushTag,  // Add marker for AI to find berries
            ));
            
            println!("{}", format!("[SPAWN] Berry Bush at ({}, {})", x, y).magenta());
        }
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    mut sim_state: ResMut<SimulationState>,
    selected_tile: Res<SelectedTile>,
    world_map: Res<WorldMap>,
    debug_system: Res<DebugSystem>,
    performance_metrics: Res<performance::PerformanceMetrics>,
    settlement_state: Res<components::SettlementState>,
    workers: Query<(&NameComponent, &components::IsHungry, &components::HasEnergy, &components::HasWood, &components::HasFood, &components::HasHouse), With<WorkerTag>>,
) {
    egui::SidePanel::left("controls").show(contexts.ctx_mut().unwrap(), |ui| {
        ui.heading("World Simulator");
        
        ui.separator();
        ui.label(format!("Tick: {}", sim_state.tick));
        
        if ui.button(if sim_state.running { "⏸ Pause" } else { "▶ Play" }).clicked() {
            sim_state.running = !sim_state.running;
        }
        
        if ui.button("Step").clicked() && !sim_state.running {
            sim_state.tick += 1;
        }
        
        if ui.button("Reset").clicked() {
            sim_state.tick = 0;
            sim_state.running = false;
        }
        
        ui.add(egui::Slider::new(&mut sim_state.speed, 0.1..=5.0).text("Speed"));
        
        ui.separator();
        ui.heading("Selected Tile");
        
        if let Some((x, y)) = selected_tile.position {
            ui.label(format!("Position: ({}, {})", x, y));
            ui.label(format!("Type: {:?}", world_map.tiles[y][x]));
            ui.label(format!("Walkable: {}", world_map.tiles[y][x].is_walkable()));
        } else {
            ui.label("No tile selected");
        }
        
        ui.separator();
        ui.heading("Settlement Resources");
        ui.label(format!("🪵 Wood: {}", settlement_state.wood_supply));
        ui.label(format!("🪨 Stone: {}", settlement_state.stone_supply));
        ui.label(format!("🍎 Food: {}", settlement_state.food_supply));
        ui.label(format!("🏠 Buildings: {}", settlement_state.building_count));
        
        ui.separator();
        ui.heading("Workers Status");
        
        for (name, hunger, energy, wood, food, has_house) in workers.iter() {
            ui.collapsing(&name.display_name, |ui| {
                ui.horizontal(|ui| {
                    let hunger_color = if hunger.0 > 0.7 { egui::Color32::RED } 
                                     else if hunger.0 > 0.4 { egui::Color32::YELLOW } 
                                     else { egui::Color32::GREEN };
                    ui.colored_label(hunger_color, format!("Hunger: {:.0}%", hunger.0 * 100.0));
                });
                
                ui.horizontal(|ui| {
                    let energy_color = if energy.0 < 0.3 { egui::Color32::RED }
                                     else if energy.0 < 0.6 { egui::Color32::YELLOW }
                                     else { egui::Color32::GREEN };
                    ui.colored_label(energy_color, format!("Energy: {:.0}%", energy.0 * 100.0));
                });
                
                ui.label(format!("Inventory: 🪵{} 🍎{}", wood.0, food.0));
                ui.label(format!("House: {}", if has_house.0 { "✅" } else { "❌" }));
            });
        }
        
        ui.separator();
        ui.heading("Map Generation");
        
        if ui.button("Generate Random").clicked() {
            // Would regenerate map here
        }
        
        if ui.button("Generate Island").clicked() {
            // Would regenerate as island
        }
        
        if ui.button("Generate Forest").clicked() {
            // Would generate forest
        }
    });
    
    // Right panel for logs
    egui::SidePanel::right("logs")
        .default_width(400.0)
        .show(contexts.ctx_mut().unwrap(), |ui| {
            ui.heading("System Logs");
            
            // Performance metrics at top
            let (fps, frame_ms, _, _) = performance_metrics.get_stats();
            ui.horizontal(|ui| {
                ui.label(format!("FPS: {:.1}", fps));
                ui.separator();
                ui.label(format!("Frame: {:.1}ms", frame_ms));
                if fps < 30.0 {
                    ui.separator();
                    ui.colored_label(egui::Color32::RED, "⚠ Low FPS");
                }
            });
            
            ui.separator();
            
            // Category filter toggles
            ui.label("Category Filters:");
            ui.horizontal_wrapped(|ui| {
                let categories = debug_system.get_known_categories();
                for category in categories {
                    let is_enabled = debug_system.is_category_enabled(&category);
                    let button_text = if is_enabled {
                        format!("✓ {}", category)
                    } else {
                        format!("  {}", category)
                    };
                    
                    let button = if is_enabled {
                        ui.small_button(&button_text)
                    } else {
                        ui.add(egui::Button::new(&button_text).small().fill(egui::Color32::from_gray(60)))
                    };
                    
                    if button.clicked() {
                        debug_system.toggle_category(&category);
                    }
                }
            });
            
            ui.separator();
            
            // Scrollable log area
            egui::ScrollArea::vertical()
                .max_height(ui.available_height())
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    let logs = debug_system.get_recent_logs(100);
                    
                    for log in logs {
                        let color = match log.level {
                            debug::DebugLevel::Error => egui::Color32::RED,
                            debug::DebugLevel::Warn => egui::Color32::YELLOW,
                            debug::DebugLevel::Info => egui::Color32::GREEN,
                            debug::DebugLevel::Debug => egui::Color32::LIGHT_BLUE,
                            debug::DebugLevel::Trace => egui::Color32::GRAY,
                        };
                        
                        ui.horizontal(|ui| {
                            // Timestamp
                            ui.monospace(format!("[{:6.2}]", log.timestamp));
                            
                            // Category
                            ui.colored_label(egui::Color32::LIGHT_GRAY, format!("[{}]", log.category));
                            
                            // Message with color based on level
                            ui.colored_label(color, &log.message);
                        });
                    }
                });
        });
}

fn tile_interaction_system(
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera.single() else { return };
    
    if let Some(cursor_pos) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                let tile_x = ((world_pos.x / TILE_SIZE) + (MAP_SIZE as f32 / 2.0)) as usize;
                let tile_y = ((world_pos.y / TILE_SIZE) + (MAP_SIZE as f32 / 2.0)) as usize;
                
                if tile_x < MAP_SIZE && tile_y < MAP_SIZE {
                    selected_tile.position = Some((tile_x, tile_y));
                }
            }
        }
    }
}

fn simulation_system(
    time: Res<Time>,
    mut sim_state: ResMut<SimulationState>,
    workers: Query<(&mut Transform, &mut TileEntity), With<WorkerTag>>,
    world_map: Res<WorldMap>,
) {
    if !sim_state.running {
        return;
    }
    
    sim_state.accumulated_time += time.delta_secs() * sim_state.speed;
    
    if sim_state.accumulated_time >= 1.0 {
        sim_state.accumulated_time = 0.0;
        sim_state.tick += 1;
        
        // Movement is now handled by the AI task execution system
        // Workers will move according to their GOAP plans
    }
}

fn render_map_system(
    mut tiles: Query<(&mut Sprite, &TileEntity), Without<WorkerTag>>,
    world_map: Res<WorldMap>,
) {
    for (mut sprite, tile_entity) in tiles.iter_mut() {
        sprite.color = world_map.tiles[tile_entity.y][tile_entity.x].color();
    }
}

fn setup_debug_cli(debug: Res<DebugSystem>) {
    let cli = DebugCLI::new(debug.get_command_sender());
    cli.start();
}