//! Example demonstrating hybrid GOAP + Utility AI system

use bevy::prelude::*;
use world_sim_core::{
    SimulationPlugin,
    ai::{spawn_hybrid_worker, AICoordinator, AIMode},
    components::*,
    ResourceType, Position,
};
use big_brain::prelude::*;

fn main() {
    let mut app = App::new();
    
    // Minimal plugins for headless simulation
    app.add_plugins(MinimalPlugins)
       .add_plugins(SimulationPlugin)
       .add_systems(Startup, setup)
       .add_systems(Update, (
           print_ai_status,
           spawn_threats,
           spawn_opportunities,
       ))
       .run();
}

fn setup(
    mut commands: Commands,
) {
    println!("Hybrid AI Demonstration");
    println!("========================");
    println!("This example shows GOAP and Utility AI working together:");
    println!("- GOAP handles long-term goals (harvest wood, build storage)");
    println!("- Utility AI handles immediate needs and interrupts");
    println!();
    
    // Create world configuration
    let config = world_sim_core::WorldConfig {
        width: 100,
        height: 100,
        starting_workers: 0,
        resource_density: 0.15,
        ..Default::default()
    };
    
    // Initialize world state
    commands.insert_resource(world_sim_core::resources::WorldState::new(config));
    
    // Spawn hybrid AI workers
    for i in 0..2 {
        let position = Position {
            x: 25 + i * 30,
            y: 50,
        };
        
        let worker = spawn_hybrid_worker(
            &mut commands,
            position,
            format!("HybridWorker_{}", i + 1),
        );
        
        println!("Spawned Hybrid Worker_{} with both GOAP and Utility AI", i + 1);
    }
    
    // Spawn resources
    spawn_initial_resources(&mut commands);
    
    // Spawn storage for workers to use
    spawn_storage(&mut commands);
    
    println!("\nWorkers will:");
    println!("1. Use GOAP to plan resource gathering");
    println!("2. Use Utility AI to handle hunger/fatigue");
    println!("3. Flee from threats using Utility AI");
    println!("4. Grab opportunities when available");
    println!("5. Help each other when in need");
}

fn spawn_initial_resources(commands: &mut Commands) {
    // Spawn wood resources
    for i in 0..8 {
        commands.spawn((
            ResourceNodeComponent::new(ResourceType::Wood, 25),
            PositionComponent::new(10 + i * 12, 20),
            Name::new("Tree"),
        ));
    }
    
    // Spawn food resources
    for i in 0..5 {
        commands.spawn((
            ResourceNodeComponent::new(ResourceType::Food, 20),
            PositionComponent::new(10 + i * 15, 80),
            Name::new("BerryBush"),
        ));
    }
    
    // Spawn some valuable resources (opportunities)
    for i in 0..3 {
        commands.spawn((
            ResourceNodeComponent::new(ResourceType::Stone, 30),
            PositionComponent::new(70 + i * 10, 50),
            world_sim_core::ai::scorers::Opportunity { value: 50.0 },
            Name::new("GemDeposit"),
        ));
    }
}

fn spawn_storage(commands: &mut Commands) {
    commands.spawn((
        StorageComponent::new(200),
        PositionComponent::new(50, 50),
        Name::new("MainStorage"),
    ));
}

fn spawn_threats(
    mut commands: Commands,
    threats: Query<Entity, With<world_sim_core::ai::scorers::Threat>>,
    time: Res<Time>,
) {
    static mut THREAT_TIMER: f32 = 0.0;
    
    unsafe {
        THREAT_TIMER += time.delta_secs();
        
        // Spawn a threat every 30 seconds
        if THREAT_TIMER > 30.0 && threats.iter().count() == 0 {
            THREAT_TIMER = 0.0;
            
            let x = (rand::random::<u32>() % 80) as i32 + 10;
            let y = (rand::random::<u32>() % 80) as i32 + 10;
            
            commands.spawn((
                world_sim_core::ai::scorers::Threat { danger_level: 0.9 },
                PositionComponent::new(x, y),
                Name::new("Wolf"),
            ));
            
            println!("\n!!! THREAT SPAWNED: Wolf at ({}, {}) !!!", x, y);
            println!("Workers should flee using Utility AI!");
        }
        
        // Remove threat after 10 seconds
        if THREAT_TIMER > 10.0 && THREAT_TIMER < 11.0 {
            for entity in threats.iter() {
                commands.entity(entity).despawn();
                println!("Threat removed - workers should resume GOAP goals");
            }
        }
    }
}

fn spawn_opportunities(
    mut commands: Commands,
    opportunities: Query<Entity, With<world_sim_core::ai::scorers::Opportunity>>,
    time: Res<Time>,
) {
    static mut OPPORTUNITY_TIMER: f32 = 0.0;
    
    unsafe {
        OPPORTUNITY_TIMER += time.delta_secs();
        
        // Spawn valuable opportunities occasionally
        if OPPORTUNITY_TIMER > 20.0 && opportunities.iter().count() < 2 {
            OPPORTUNITY_TIMER = 0.0;
            
            let x = (rand::random::<u32>() % 60) as i32 + 20;
            let y = (rand::random::<u32>() % 60) as i32 + 20;
            
            commands.spawn((
                ResourceNodeComponent::new(ResourceType::Stone, 50),
                world_sim_core::ai::scorers::Opportunity { value: 80.0 },
                PositionComponent::new(x, y),
                Name::new("RareResource"),
            ));
            
            println!("\n*** OPPORTUNITY: Rare resource at ({}, {}) ***", x, y);
        }
    }
}

fn print_ai_status(
    query: Query<(
        &Name,
        &WorkerComponent,
        &IsHungry,
        &HasEnergy,
        &PositionComponent,
        &AICoordinator,
        Option<&Score>,
        Option<&ActionState>,
    )>,
    time: Res<Time>,
) {
    static mut LAST_PRINT: f32 = 0.0;
    
    unsafe {
        LAST_PRINT += time.delta_secs();
        if LAST_PRINT < 3.0 {
            return;
        }
        LAST_PRINT = 0.0;
    }
    
    println!("\n=== AI Status Update ===");
    
    for (name, worker, hunger, energy, pos, coordinator, score, action_state) in query.iter() {
        let mode_str = match coordinator.mode {
            AIMode::UtilityDriven => "UTILITY (urgent needs)",
            AIMode::GoalDriven => "GOAP (following plan)",
            AIMode::Hybrid => "HYBRID (balanced)",
        };
        
        let utility_score = score.map(|s| s.0).unwrap_or(0.0);
        let action_str = action_state.map(|s| format!("{:?}", s)).unwrap_or("None".to_string());
        
        println!("{}: Mode={} Pos({},{}) H:{:.0} E:{:.0} UtilityScore:{:.2} Action:{}",
            name.as_str(),
            mode_str,
            pos.x, pos.y,
            hunger.0,
            energy.0,
            utility_score,
            action_str,
        );
        
        // Show AI decision reasoning
        if coordinator.mode == AIMode::UtilityDriven {
            println!("  -> Utility AI handling critical need (score > {})", 
                coordinator.interrupt_threshold);
        } else if coordinator.goap_paused {
            println!("  -> GOAP paused, will resume when utility score < 0.3");
        }
    }
    
    println!("========================");
}