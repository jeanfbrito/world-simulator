//! Basic simulation example
//!
//! This example demonstrates how to set up and run a basic world simulation
//! with AI agents, resources, and buildings. It shows the core functionality
//! of the world simulator in a simple, easy-to-understand way.

use bevy::prelude::*;
use world_sim_interface::*;
use world_sim_simple::*;

fn main() {
    println!("🚀 Starting Basic World Simulation Example");
    
    // Set up logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    // Create and run the simulation
    let mut app = App::new();
    
    // Configure for headless operation
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: None,
        exit_condition: bevy::window::ExitCondition::DontExit,
        close_when_requested: false,
    }));
    
    // Add core simulation plugins
    app.add_plugins(simulation::TickSimulationPlugin);
    app.add_plugins(ComponentsPlugin);
    app.add_plugins(PackSystemPlugin);
    app.add_plugins(WorldPlugin);
    app.add_plugins(SimPlugin);
    app.add_plugins(TilemapPlugin);
    app.add_plugins(ResourcesPlugin);
    app.add_plugins(BuildingsPlugin);
    app.add_plugins(CraftingPlugin);
    app.add_plugins(AIPlugin);
    app.add_plugins(SaveLoadPlugin);
    app.add_plugins(PerformancePlugin);
    app.add_plugins(SystemsPlugin);
    
    // Initialize resources
    app.init_resource::<WorldMap>();
    app.init_resource::<SimulationState>();
    
    // Add startup systems
    app.add_systems(Startup, setup_basic_simulation);
    
    // Add update systems for monitoring
    app.add_systems(Update, simulation_monitor_system);
    
    println!("🌍 Basic simulation initialized. Running for 500 ticks...");
    
    // Run the simulation
    app.run();
}

/// Setup a basic simulation with default world and entities
fn setup_basic_simulation(
    mut commands: Commands,
    mut pack_system: Option<Res<packs::PackSystem>>,
) {
    println!("🏗️  Setting up basic simulation...");
    
    // The world map and entities are automatically created by the pack system
    // and initialization systems. We just need to let them run.
    
    println!("✅ Basic simulation setup complete!");
}

/// Monitor system that tracks simulation progress and prints status updates
fn simulation_monitor_system(
    sim_state: Res<SimulationState>,
    world_map: Res<WorldMap>,
    unit_query: Query<&UnitTag>,
    resource_query: Query<&ResourceNode>,
    building_query: Query<&BuildingComponent>,
) {
    // Only print status every 50 ticks to avoid spam
    if sim_state.tick % 50 == 0 && sim_state.tick > 0 {
        let unit_count = unit_query.iter().count();
        let resource_count = resource_query.iter().count();
        let building_count = building_query.iter().count();
        
        println!(
            "📊 Tick {}: {} units, {} resources, {} buildings",
            sim_state.tick, unit_count, resource_count, building_count
        );
        
        // Stop after 500 ticks for this example
        if sim_state.tick >= 500 {
            println!("🎉 Basic simulation example completed successfully!");
            println!("📈 Final state:");
            println!("   • Total simulation ticks: {}", sim_state.tick);
            println!("   • Active units: {}", unit_count);
            println!("   • Available resources: {}", resource_count);
            println!("   • Constructed buildings: {}", building_count);
            
            // Exit the application
            std::process::exit(0);
        }
    }
    
    // Print initial status
    if sim_state.tick == 1 {
        println!("🎮 Simulation started. Monitoring progress...");
    }
}
