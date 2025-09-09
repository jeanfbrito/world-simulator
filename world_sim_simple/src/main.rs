use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
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
use buildings::BuildingsPlugin;
use crafting::CraftingPlugin;
use ai::{AIPlugin, WorkerAI};
use save_load::SaveLoadPlugin;

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
        .add_plugins(EguiPlugin)
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
        .init_resource::<WorldMap>()
        .init_resource::<SimulationState>()
        .init_resource::<SelectedTile>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, (setup_debug_cli, plugin_init_system))
        .add_systems(Update, (
            ui_system,
            tile_interaction_system,
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
            running: false,
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
    commands.spawn(Camera2dBundle::default());
    
    // Spawn initial tiles as entities for rendering
    let world_map = WorldMap::default();
    
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let tile_type = world_map.tiles[y][x];
            let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
            
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: tile_type.color(),
                        custom_size: Some(Vec2::new(TILE_SIZE - 1.0, TILE_SIZE - 1.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_x, world_y, 0.0),
                    ..default()
                },
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
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.75, 0.0),
                        custom_size: Some(Vec2::new(TILE_SIZE * 0.6, TILE_SIZE * 0.6)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_x, world_y, 1.0),
                    ..default()
                },
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
                // Inventory
                create_starter_inventory(),
                // Tile tracking
                TileEntity { x, y },
            )).id();
            
            // Log worker creation with component-based architecture
            println!("{}", format!("[SPAWN] Worker {} at ({}, {}) - Components: Name, Position, Health, Energy, WorkerTag, WorkerStats", 
                i + 1, x, y).green());
        }
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    mut sim_state: ResMut<SimulationState>,
    selected_tile: Res<SelectedTile>,
    world_map: Res<WorldMap>,
) {
    egui::SidePanel::left("controls").show(contexts.ctx_mut(), |ui| {
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
}

fn tile_interaction_system(
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera.single();
    
    if let Some(cursor_pos) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
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
    mut workers: Query<(&mut Transform, &mut TileEntity), With<WorkerTag>>,
    world_map: Res<WorldMap>,
) {
    if !sim_state.running {
        return;
    }
    
    sim_state.accumulated_time += time.delta_seconds() * sim_state.speed;
    
    if sim_state.accumulated_time >= 1.0 {
        sim_state.accumulated_time = 0.0;
        sim_state.tick += 1;
        
        // Simple random movement for workers
        let mut rng = rand::thread_rng();
        for (mut transform, mut tile_entity) in workers.iter_mut() {
            if rng.gen_bool(0.3) {
                let dx = rng.gen_range(-1..=1);
                let dy = rng.gen_range(-1..=1);
                
                let new_x = (tile_entity.x as i32 + dx).max(0).min(MAP_SIZE as i32 - 1) as usize;
                let new_y = (tile_entity.y as i32 + dy).max(0).min(MAP_SIZE as i32 - 1) as usize;
                
                if world_map.tiles[new_y][new_x].is_walkable() {
                    tile_entity.x = new_x;
                    tile_entity.y = new_y;
                    
                    let world_x = (new_x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
                    let world_y = (new_y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
                    transform.translation.x = world_x;
                    transform.translation.y = world_y;
                }
            }
        }
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