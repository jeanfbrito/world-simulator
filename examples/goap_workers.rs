//! Example demonstrating GOAP-based intelligent workers

use bevy::prelude::*;
use world_sim_core::{
    SimulationEngine, WorldConfig, SimulationPlugin,
    ai::spawn_worker_with_goap,
    components::*,
    ResourceType, Position,
};
use bevy_dogoap::prelude::*;

fn main() {
    let mut app = App::new();
    
    // Minimal plugins for headless simulation
    app.add_plugins(MinimalPlugins)
       .add_plugins(SimulationPlugin)
       .add_systems(Startup, setup)
       .add_systems(Update, (
           print_worker_status,
           spawn_resources,
       ))
       .run();
}

fn setup(
    mut commands: Commands,
) {
    println!("Starting GOAP Workers Simulation");
    println!("==================================");
    
    // Create world configuration
    let config = WorldConfig {
        width: 50,
        height: 50,
        starting_workers: 0, // We'll spawn with GOAP
        resource_density: 0.1,
        ..Default::default()
    };
    
    // Initialize world state
    commands.insert_resource(world_sim_core::resources::WorldState::new(config));
    
    // Spawn workers with GOAP AI at different positions
    for i in 0..3 {
        let position = Position {
            x: 10 + i * 15,
            y: 25,
        };
        
        spawn_worker_with_goap(
            &mut commands,
            position,
            format!("Worker_{}", i + 1),
        );
        
        println!("Spawned Worker_{} at ({}, {})", i + 1, position.x, position.y);
    }
    
    // Spawn some initial resources
    spawn_initial_resources(&mut commands);
    
    // Spawn a storage building
    spawn_storage_building(&mut commands);
}

fn spawn_initial_resources(commands: &mut Commands) {
    // Spawn wood resources (trees)
    for i in 0..5 {
        commands.spawn((
            ResourceNodeComponent::new(ResourceType::Wood, 20),
            PositionComponent::new(5 + i * 8, 10),
            Name::new("Tree"),
        ));
    }
    
    // Spawn food resources (berry bushes)
    for i in 0..3 {
        commands.spawn((
            ResourceNodeComponent::new(ResourceType::Food, 15),
            PositionComponent::new(5 + i * 10, 40),
            Name::new("BerryBush"),
        ));
    }
    
    println!("Spawned 5 trees and 3 berry bushes");
}

fn spawn_storage_building(commands: &mut Commands) {
    commands.spawn((
        StorageComponent::new(100),
        PositionComponent::new(25, 25),
        Name::new("Storage"),
    ));
    
    println!("Spawned storage building at (25, 25)");
}

fn print_worker_status(
    query: Query<(
        &Name,
        &WorkerComponent,
        &IsHungry,
        &HasEnergy,
        &HasWood,
        &HasFood,
        &PositionComponent,
        &Planner,
    )>,
    time: Res<Time>,
) {
    static mut LAST_PRINT: f32 = 0.0;
    
    unsafe {
        LAST_PRINT += time.delta_secs();
        if LAST_PRINT < 2.0 {
            return;
        }
        LAST_PRINT = 0.0;
    }
    
    println!("\n--- Worker Status ---");
    for (name, worker, hunger, energy, wood, food, pos, planner) in query.iter() {
        let current_action = get_current_action(&planner);
        
        println!("{}: Pos({},{}) H:{:.0} E:{:.0} Wood:{} Food:{} State:{:?} Action:{}",
            name.as_str(),
            pos.x, pos.y,
            hunger.0,
            energy.0,
            wood.0,
            food.0,
            worker.state,
            current_action,
        );
    }
}

fn get_current_action(planner: &Planner) -> String {
    // This is a simplified version - in reality you'd check which action component is active
    if planner.current_plan.is_some() {
        "Planning..."
    } else {
        "Idle"
    }.to_string()
}

fn spawn_resources(
    mut commands: Commands,
    resources: Query<Entity, With<ResourceNodeComponent>>,
    time: Res<Time>,
) {
    static mut SPAWN_TIMER: f32 = 0.0;
    
    unsafe {
        SPAWN_TIMER += time.delta_secs();
        if SPAWN_TIMER < 10.0 {
            return;
        }
        SPAWN_TIMER = 0.0;
    }
    
    // Count current resources
    let resource_count = resources.iter().count();
    
    // Spawn new resources if we're running low
    if resource_count < 5 {
        // Spawn a new tree
        let x = (rand::random::<u32>() % 40) as i32 + 5;
        let y = (rand::random::<u32>() % 40) as i32 + 5;
        
        commands.spawn((
            ResourceNodeComponent::new(ResourceType::Wood, 15),
            PositionComponent::new(x, y),
            Name::new("Tree"),
        ));
        
        println!("Spawned new tree at ({}, {})", x, y);
    }
    
    // Spawn food occasionally
    if resource_count < 8 && rand::random::<f32>() > 0.5 {
        let x = (rand::random::<u32>() % 40) as i32 + 5;
        let y = (rand::random::<u32>() % 40) as i32 + 5;
        
        commands.spawn((
            ResourceNodeComponent::new(ResourceType::Food, 10),
            PositionComponent::new(x, y),
            Name::new("BerryBush"),
        ));
        
        println!("Spawned new berry bush at ({}, {})", x, y);
    }
}