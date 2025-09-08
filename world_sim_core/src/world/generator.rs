//! World generation system

use bevy_ecs::prelude::*;
use world_sim_interface::{WorldConfig, Position, EntityType, ResourceType};
use crate::components::*;
use crate::resources::WorldState;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Generate a new world with terrain and resources
pub fn generate_world(
    mut commands: Commands,
    config: &WorldConfig,
    world_state: &mut WorldState,
) {
    let mut rng = if let Some(seed) = config.seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_entropy()
    };
    
    // Calculate resource counts
    let total_tiles = (config.width * config.height) as f32;
    let num_resources = (total_tiles * config.resource_density) as usize;
    
    // Generate trees
    let num_trees = num_resources / 2;
    for _ in 0..num_trees {
        let x = rng.gen_range(0..config.width as i32);
        let y = rng.gen_range(0..config.height as i32);
        spawn_resource(&mut commands, Position::new(x, y), ResourceType::Wood, 50);
    }
    
    // Generate berry bushes
    let num_berries = num_resources / 3;
    for _ in 0..num_berries {
        let x = rng.gen_range(0..config.width as i32);
        let y = rng.gen_range(0..config.height as i32);
        spawn_resource(&mut commands, Position::new(x, y), ResourceType::Food, 20);
    }
    
    // Generate stone deposits
    let num_stones = num_resources / 6;
    for _ in 0..num_stones {
        let x = rng.gen_range(0..config.width as i32);
        let y = rng.gen_range(0..config.height as i32);
        spawn_resource(&mut commands, Position::new(x, y), ResourceType::Stone, 30);
    }
}

/// Spawn a resource node at the given position
pub fn spawn_resource(
    commands: &mut Commands,
    position: Position,
    resource_type: ResourceType,
    amount: u32,
) -> Entity {
    commands.spawn((
        ResourceNodeComponent::new(resource_type, amount),
        PositionComponent::at(position),
    )).id()
}

/// Spawn a worker at the given position
pub fn spawn_worker(
    commands: &mut Commands,
    position: Position,
    name: String,
) -> Entity {
    commands.spawn((
        WorkerComponent::new(name),
        PositionComponent::at(position),
        InventoryComponent::new(20),
        MovementComponent::new(1.0),
        TaskQueueComponent::new(10),
    )).id()
}

/// Spawn starting workers for a new world
pub fn spawn_starting_workers(
    mut commands: Commands,
    config: &WorldConfig,
) {
    for i in 0..config.starting_workers {
        let pos = Position::new(
            (config.width as i32 / 2 + i as i32) % config.width as i32,
            config.height as i32 / 2,
        );
        spawn_worker(&mut commands, pos, format!("Worker {}", i + 1));
    }
}