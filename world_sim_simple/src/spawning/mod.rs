use bevy::prelude::*;
use colored::Colorize;
use crate::components::{
    PositionComponent, HealthComponent, NameComponent, EnergyComponent,
    WorkerTag, WorkerStats, PeasantTag, PeasantConfig,
    UnitNeeds, UnitInventory, UnitLocation, UnitWorkState, UnitOwnership
};
use crate::ai::{WorkerAI, WorldState, StateValue};
use crate::TileEntity;
use rand::Rng;

/// Plugin for unit spawning systems
pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initial_unit_spawn_system);
    }
}

/// Spawns initial units at the start of the game
fn initial_unit_spawn_system(
    mut commands: Commands,
    world_map: Res<crate::WorldMap>
) {
    let mut rng = rand::thread_rng();
    let mut spawned = 0;
    
    // Spawn 5 peasants with consolidated components
    while spawned < 5 {
        let x = rng.gen_range(20..44);
        let y = rng.gen_range(20..44);
        
        // Check if tile is walkable
        if world_map.tiles[y][x].is_walkable() {
            spawn_peasant(&mut commands, spawned + 1, x, y);
            spawned += 1;
            
            println!("{}", format!("[SPAWN] Peasant {} at ({}, {}) with consolidated components", 
                spawned, x, y).green());
        }
    }
    
    println!("{}", "[SPAWN] Created 5 initial peasants with consolidated state components".green());
}

/// Factory function to spawn a peasant with all necessary components
pub fn spawn_peasant(commands: &mut Commands, id: usize, x: usize, y: usize) -> Entity {
    let peasant_config = PeasantConfig::default();
    
    let entity = commands.spawn((
        // Identity
        NameComponent::new(format!("Peasant {}", id)),
        
        // Position
        PositionComponent::from_tile(x, y),
        TileEntity { x, y },
        
        // Basic stats (keeping for compatibility)
        HealthComponent::new(peasant_config.health),
        EnergyComponent::new(100.0),
        
        // Worker components
        WorkerTag,
        WorkerStats::default(),
        PeasantTag::with_config(peasant_config.clone()),
    )).id();
    
    // Add additional components separately to avoid bundle size limit
    commands.entity(entity).insert((
        // NEW: Consolidated state components
        UnitNeeds::new(),
        UnitInventory::with_starting_items(),
        UnitLocation::new(x, y),
        UnitWorkState::default(),
        UnitOwnership::default(),
        
        // AI components
        WorkerAI::new(),
        create_initial_world_state(),
        crate::ai::BehaviorCycle::default(),
    ));
    
    entity
}

/// Creates the initial GOAP world state for a unit
fn create_initial_world_state() -> WorldState {
    let mut state = WorldState::new();
    
    // Initialize from consolidated components
    state.set("is_hungry", StateValue::Float(0.3));  // Slightly hungry
    state.set("has_energy", StateValue::Float(1.0));  // Full energy
    state.set("has_wood", StateValue::Int(2));        // Starting wood
    state.set("has_food", StateValue::Int(5));        // Starting food
    state.set("has_stone", StateValue::Int(0));       // No stone initially
    state.set("has_house", StateValue::Bool(false));  // No house
    state.set("at_storage", StateValue::Bool(false));
    state.set("at_resource", StateValue::Bool(false));
    state.set("at_home", StateValue::Bool(false));
    state.set("inventory_full", StateValue::Bool(false));
    
    state
}

/// Spawn configuration loaded from Lua scripts
#[derive(Debug, Clone)]
pub struct SpawnConfig {
    pub unit_type: String,
    pub count: usize,
    pub spawn_area: SpawnArea,
}

#[derive(Debug, Clone)]
pub struct SpawnArea {
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

impl SpawnArea {
    pub fn random_position(&self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        (
            rng.gen_range(self.min_x..self.max_x),
            rng.gen_range(self.min_y..self.max_y),
        )
    }
}