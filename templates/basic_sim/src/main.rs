//! Basic World Simulation Example
//!
//! This example demonstrates how to create a simple world simulation
//! using the world-simulator framework.

use std::time::Duration;
use tracing::{info, warn, error};
use world_sim::prelude::*;

mod config;
mod systems;
mod components;

use config::SimulationConfig;
use systems::*;

/// Main simulation application
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    init_logging();

    info!("Starting Basic World Simulation");

    // Load configuration
    let config = load_config()?;

    // Create simulation
    let mut simulation = create_simulation(&config).await?;

    // Run simulation loop
    run_simulation(&mut simulation, &config).await?;

    info!("Simulation completed successfully");
    Ok(())
}

/// Initialize logging system
fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .init();
}

/// Load simulation configuration
fn load_config() -> Result<SimulationConfig, Box<dyn std::error::Error>> {
    info!("Loading configuration...");

    // Try to load from file, fall back to defaults
    let config = SimulationConfig::load_from_file("config.toml")
        .unwrap_or_else(|_| {
            warn!("Could not load config.toml, using defaults");
            SimulationConfig::default()
        });

    info!("Configuration loaded: {:?}", config);
    Ok(config)
}

/// Create and initialize simulation
async fn create_simulation(config: &SimulationConfig) -> Result<Simulation, Box<dyn std::error::Error>> {
    info!("Creating simulation...");

    // Create simulation builder
    let mut builder = Simulation::builder()
        .with_world_size(config.world.width, config.world.height)
        .with_tick_duration(Duration::from_millis(config.tick_duration_ms))
        .with_random_seed(config.seed.unwrap_or_else(|| rand::random()));

    // Add systems based on configuration
    if config.systems.movement.enabled {
        builder = builder.add_system(MovementSystem::new());
    }

    if config.systems.gathering.enabled {
        builder = builder.add_system(ResourceGatheringSystem::new());
    }

    if config.systems.crafting.enabled {
        builder = builder.add_system(CraftingSystem::new());
    }

    if config.systems.ai.enabled {
        builder = builder.add_system(AISystem::new());
    }

    // Build simulation
    let mut simulation = builder.build()?;

    // Initialize simulation
    simulation.initialize().await?;

    // Spawn initial entities
    spawn_initial_entities(&mut simulation, config).await?;

    info!("Simulation created and initialized");
    Ok(simulation)
}

/// Spawn initial entities in the simulation
async fn spawn_initial_entities(
    simulation: &mut Simulation,
    config: &SimulationConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Spawning initial entities...");

    // Spawn peasants
    for i in 0..config.entities.peasant_count {
        let position = Position::new(
            rand::random::<f32>() * config.world.width as f32,
            rand::random::<f32>() * config.world.height as f32,
        );

        let peasant = simulation.spawn_entity("peasant")?
            .with_component(position)
            .with_component(Inventory::new(10))
            .with_component(Movement::new(1.0))
            .with_component(GatheringSkill::new(0.5));

        info!("Spawned peasant {} at position {:?}", i, peasant.position());
    }

    // Spawn resources
    spawn_resources(simulation, config).await?;

    info!("Initial entities spawned");
    Ok(())
}

/// Spawn resources in the world
async fn spawn_resources(
    simulation: &mut Simulation,
    config: &SimulationConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Spawning resources...");

    let resource_types = &config.resources.types;

    for resource_type in resource_types {
        let count = (config.world.width * config.world.height) as f32 * resource_type.density;

        for _ in 0..count as usize {
            let x = rand::random::<f32>() * config.world.width as f32;
            let y = rand::random::<f32>() * config.world.height as f32;
            let amount = rand::thread_rng().gen_range(1..=resource_type.max_per_tile);

            simulation.spawn_resource(resource_type.name.clone(), x, y, amount)?;
        }
    }

    info!("Resources spawned");
    Ok(())
}

/// Run the main simulation loop
async fn run_simulation(
    simulation: &mut Simulation,
    config: &SimulationConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting simulation loop...");

    let mut tick_count = 0;
    let max_ticks = config.simulation.max_ticks.unwrap_or(1000);
    let mut last_stats_update = std::time::Instant::now();
    let stats_interval = Duration::from_secs(5);

    // Main simulation loop
    while tick_count < max_ticks {
        // Execute simulation tick
        let tick_start = std::time::Instant::now();
        let result = simulation.tick().await;
        let tick_duration = tick_start.elapsed();

        if let Err(e) = result {
            error!("Simulation tick failed: {}", e);
            break;
        }

        tick_count += 1;

        // Print periodic statistics
        if last_stats_update.elapsed() >= stats_interval {
            print_simulation_stats(simulation, tick_count).await?;
            last_stats_update = std::time::Instant::now();
        }

        // Check for exit conditions
        if should_exit_simulation(simulation, tick_count, config).await? {
            info!("Exit condition met, stopping simulation");
            break;
        }

        // Control simulation speed
        if tick_duration < Duration::from_millis(config.tick_duration_ms) {
            tokio::time::sleep(Duration::from_millis(config.tick_duration_ms) - tick_duration).await;
        }
    }

    // Print final statistics
    print_final_stats(simulation, tick_count).await?;

    Ok(())
}

/// Print current simulation statistics
async fn print_simulation_stats(
    simulation: &Simulation,
    tick_count: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let stats = simulation.get_statistics().await?;

    info!(
        "Tick {}: {} entities, {} resources, {:.2} FPS",
        tick_count,
        stats.entity_count,
        stats.resource_count,
        stats.fps
    );

    // Print resource distribution
    for (resource_type, amount) in &stats.resources {
        info!("  {}: {}", resource_type, amount);
    }

    Ok(())
}

/// Print final simulation statistics
async fn print_final_stats(
    simulation: &Simulation,
    tick_count: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let stats = simulation.get_statistics().await?;
    let duration = stats.simulation_duration;

    info!("=== Final Simulation Statistics ===");
    info!("Total ticks: {}", tick_count);
    info!("Duration: {:?}", duration);
    info!("Average FPS: {:.2}", stats.fps);
    info!("Total entities: {}", stats.entity_count);
    info!("Total resources: {}", stats.resource_count);

    // Calculate totals
    let total_gathered: u32 = stats.gathering_stats.values().sum();
    let total_crafted: u32 = stats.crafting_stats.values().sum();

    info!("Total resources gathered: {}", total_gathered);
    info!("Total items crafted: {}", total_crafted);

    // Performance metrics
    if duration.as_secs() > 0 {
        let ticks_per_second = tick_count as f64 / duration.as_secs_f64();
        info!("Ticks per second: {:.2}", ticks_per_second);
    }

    Ok(())
}

/// Check if simulation should exit
async fn should_exit_simulation(
    simulation: &Simulation,
    tick_count: u64,
    config: &SimulationConfig,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Check user input (in a real application)
    // For now, just check tick count

    // Check if all entities are idle
    let stats = simulation.get_statistics().await?;
    let active_entities = stats.entity_count - stats.idle_entities;

    if active_entities == 0 && tick_count > 100 {
        info!("All entities idle, ending simulation");
        return Ok(true);
    }

    // Check resource depletion
    let total_resources: u32 = stats.resources.values().sum();
    if total_resources == 0 && tick_count > 50 {
        info!("All resources depleted, ending simulation");
        return Ok(true);
    }

    Ok(false)
}

/// Handle keyboard input (simplified)
async fn handle_keyboard_input(simulation: &mut Simulation) -> Result<(), Box<dyn std::error::Error>> {
    // In a real application, this would handle actual keyboard input
    // For this example, we'll just ignore it

    // Example of what you might do:
    // - Press 'q' to quit
    // - Press 'p' to pause/unpause
    // - Press 's' to save state
    // - Press 'r' to reset simulation

    Ok(())
}

/// Simulation signal handler
async fn handle_signals() {
    // Set up signal handlers for graceful shutdown
    // This would typically handle SIGINT, SIGTERM, etc.

    tokio::signal::ctrl_c().await.unwrap();
    info!("Received shutdown signal");
}

/// Clean up simulation resources
async fn cleanup_simulation(simulation: &mut Simulation) -> Result<(), Box<dyn std::error::Error>> {
    info!("Cleaning up simulation...");

    // Save final state if needed
    if simulation.should_save_state() {
        let saved_state = simulation.save_state().await?;
        info!("Simulation state saved");
    }

    // Clean up systems
    simulation.shutdown().await?;

    info!("Simulation cleanup completed");
    Ok(())
}