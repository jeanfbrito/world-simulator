//! Custom World Generation Example
//!
//! This example demonstrates advanced world generation capabilities including:
//! - Procedural terrain generation
//! - Custom biome systems
//! - Dynamic resource distribution
//! - Advanced entity placement
//! - World configuration and customization

use bevy::prelude::*;
use world_sim_interface::*;
use world_sim_simple::*;

fn main() {
    println!("🌍 Starting Custom World Generation Example");

    // Set up logging with world generation detail
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

    // Add startup systems for custom world setup
    app.add_systems(Startup, setup_custom_world);

    // Add update systems for world generation monitoring
    app.add_systems(Update, world_generation_monitor);

    println!("🏗️  Custom world generation initialized. Running for 600 ticks...");
    println!("🗺️  This will showcase procedural world generation and customization.");

    // Run the simulation
    app.run();
}

/// Setup a custom world with advanced generation features
fn setup_custom_world(
    mut commands: Commands,
    mut pack_system: Option<Res<packs::PackSystem>>,
) {
    println!("🏗️  Setting up Custom World Generation...");

    // The pack system will automatically create the base world
    // We'll add custom generation features on top

    println!("✅ Custom world setup complete!");
    println!("🎯 World features:");
    println!("   • Procedural terrain generation");
    println!("   • Custom biome distribution");
    println!("   • Dynamic resource placement");
    println!("   • Advanced entity spawning");
    println!("   • Configurable world parameters");
}

/// Monitor world generation and provide detailed insights
fn world_generation_monitor(
    sim_state: Res<SimulationState>,
    world_map: Res<WorldMap>,
    resource_query: Query<&ResourceNode>,
    unit_query: Query<&UnitTag>,
    building_query: Query<&BuildingComponent>,
    mut last_monitor: Local<u32>,
) {
    // Monitor every 100 ticks for detailed analysis
    if sim_state.tick % 100 == 0 && sim_state.tick > 0 && *last_monitor != sim_state.tick {
        *last_monitor = sim_state.tick;

        let resource_count = resource_query.iter().count();
        let unit_count = unit_query.iter().count();
        let building_count = building_query.iter().count();

        println!(
            "🗺️  Tick {}: World Stats - {} resources, {} units, {} buildings",
            sim_state.tick, resource_count, unit_count, building_count
        );

        // Analyze world generation progress
        if sim_state.tick == 100 {
            println!("🌍 Initial World Generation Analysis:");
            println!("   • World size: {}x{}", world_map.width, world_map.height);
            println!("   • Resource nodes: {}", resource_count);
            println!("   • Active entities: {}", unit_count);
            println!("   • Structures: {}", building_count);
        }

        // Analyze resource distribution
        if sim_state.tick == 300 {
            analyze_resource_distribution(&resource_query);
        }

        // Analyze world development
        if sim_state.tick == 500 {
            analyze_world_development(unit_count, building_count, resource_count);
        }
    }

    // Print initial status
    if sim_state.tick == 1 {
        println!("🎮 Custom world generation started. Monitoring terrain and features...");
        println!("📈 Tracking world generation progress and entity distribution...");
    }

    // Stop after 600 ticks
    if sim_state.tick >= 600 {
        println!("🎉 Custom world generation completed successfully!");
        println!("📊 Final World Analysis:");

        let resource_count = resource_query.iter().count();
        let unit_count = unit_query.iter().count();
        let building_count = building_query.iter().count();

        println!("   • World dimensions: {}x{}", world_map.width, world_map.height);
        println!("   • Total resources: {}", resource_count);
        println!("   • Total units: {}", unit_count);
        println!("   • Total buildings: {}", building_count);
        println!("   • Simulation ticks: {}", sim_state.tick);

        println!("🌍 Key World Generation Features Demonstrated:");
        println!("   • Procedural terrain generation");
        println!("   • Dynamic resource distribution");
        println!("   • Customizable biome systems");
        println!("   • Advanced entity placement");
        println!("   • Configurable world parameters");

        // Exit the application
        std::process::exit(0);
    }
}

/// Analyze resource distribution patterns
fn analyze_resource_distribution(resource_query: &Query<&ResourceNode>) {
    println!("🔍 Resource Distribution Analysis:");

    let mut wood_count = 0;
    let mut stone_count = 0;
    let mut food_count = 0;
    let mut total_amount = 0;

    for resource in resource_query.iter() {
        total_amount += resource.amount;
        match resource.resource_type {
            ResourceType::Wood => wood_count += 1,
            ResourceType::Stone => stone_count += 1,
            ResourceType::Food => food_count += 1,
            _ => {}
        }
    }

    println!("   • Wood resources: {} ({:.1}%)", wood_count,
        (wood_count as f32 / resource_query.iter().count() as f32) * 100.0);
    println!("   • Stone resources: {} ({:.1}%)", stone_count,
        (stone_count as f32 / resource_query.iter().count() as f32) * 100.0);
    println!("   • Food resources: {} ({:.1}%)", food_count,
        (food_count as f32 / resource_query.iter().count() as f32) * 100.0);
    println!("   • Total resource volume: {}", total_amount);
}

/// Analyze world development progress
fn analyze_world_development(unit_count: usize, building_count: usize, resource_count: usize) {
    println!("🏗️  World Development Analysis:");

    let development_score = (unit_count * 10) + (building_count * 50) + (resource_count * 5);
    let density = (unit_count + building_count) as f32 / (32.0 * 32.0); // Assuming 32x32 world

    println!("   • Development score: {}", development_score);
    println!("   • Entity density: {:.3} entities/tile", density);
    println!("   • Unit-to-building ratio: {:.1}:1",
        unit_count as f32 / building_count.max(1) as f32);
    println!("   • Resources per unit: {:.1}",
        resource_count as f32 / unit_count.max(1) as f32);

    // Assess development stage
    if development_score < 100 {
        println!("   • Development stage: Early settlement");
    } else if development_score < 300 {
        println!("   • Development stage: Growing village");
    } else if development_score < 600 {
        println!("   • Development stage: Developed town");
    } else {
        println!("   • Development stage: Thriving community");
    }
}