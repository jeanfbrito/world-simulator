//! Test population management functionality
//! This test MUST fail first (TDD)

use world_sim_core::SimulationEngine;
use world_sim_interface::{
    WorldConfig, EntityType, Position, EngineCommand,
    BuildingType, ResourceType, WorkerState
};

#[test]
fn test_worker_needs_food() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 30,
        height: 30,
        starting_workers: 3,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    // Run simulation without food
    for _ in 0..100 {
        engine.tick();
    }
    
    let snapshot = engine.snapshot();
    
    // Workers should have hunger status
    for worker in snapshot.entities.iter()
        .filter(|e| matches!(e.entity_type, EntityType::Worker))
    {
        let hunger = worker.components
            .get("hunger")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        
        assert!(hunger > 0.5, "Workers should be hungry after time");
    }
}

#[test]
fn test_worker_consumes_food() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 20,
        height: 20,
        starting_workers: 1,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Give worker food
    let give_food = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![(ResourceType::Food, 10)].into_iter().collect(),
    };
    engine.execute_command(give_food);
    
    // Run simulation
    for _ in 0..50 {
        engine.tick();
    }
    
    // Check food was consumed
    let snapshot = engine.snapshot();
    let worker_after = snapshot.entities
        .iter()
        .find(|e| e.id == worker.id)
        .unwrap();
    
    let inventory = worker_after.components
        .get("inventory")
        .expect("Worker should have inventory");
    
    let food_remaining: u32 = inventory
        .get("food")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    
    assert!(food_remaining < 10, "Worker should consume food over time");
    
    let hunger = worker_after.components
        .get("hunger")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);
    
    assert!(hunger < 0.5, "Worker should not be hungry with food");
}

#[test]
fn test_house_increases_population_cap() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 40,
        height: 40,
        starting_workers: 2,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    // Get initial population cap
    let initial_cap = engine.get_population_cap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Build house
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![
            (ResourceType::Wood, 10),
            (ResourceType::Stone, 5),
        ].into_iter().collect(),
    };
    engine.execute_command(give_resources);
    
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::House,
        position: Position::new(20, 20),
    };
    engine.execute_command(build_cmd);
    
    // Let construction complete
    for _ in 0..20 {
        engine.tick();
    }
    
    // Check population cap increased
    let new_cap = engine.get_population_cap();
    assert!(new_cap > initial_cap, "House should increase population cap");
}

#[test]
fn test_spawn_new_worker() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 30,
        height: 30,
        starting_workers: 2,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let initial_workers = snapshot.entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Worker))
        .count();
    
    // Build house for population cap
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![
            (ResourceType::Wood, 10),
            (ResourceType::Stone, 5),
            (ResourceType::Food, 20),
        ].into_iter().collect(),
    };
    engine.execute_command(give_resources);
    
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::House,
        position: Position::new(15, 15),
    };
    engine.execute_command(build_cmd);
    
    // Let house complete
    for _ in 0..20 {
        engine.tick();
    }
    
    // Spawn new worker
    let spawn_cmd = EngineCommand::SpawnWorker {
        position: Position::new(16, 16),
        settlement_id: None,
    };
    
    let result = engine.execute_command(spawn_cmd);
    assert!(result.success, "Should spawn worker with available housing");
    
    let snapshot = engine.snapshot();
    let new_workers = snapshot.entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Worker))
        .count();
    
    assert_eq!(new_workers, initial_workers + 1, "Should have spawned one worker");
}

#[test]
fn test_worker_happiness() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 25,
        height: 25,
        starting_workers: 2,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Check initial happiness
    let initial_happiness = worker.components
        .get("happiness")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.5);
    
    // Build house (increases happiness)
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![
            (ResourceType::Wood, 10),
            (ResourceType::Stone, 5),
        ].into_iter().collect(),
    };
    engine.execute_command(give_resources);
    
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::House,
        position: Position::new(12, 12),
    };
    engine.execute_command(build_cmd);
    
    // Give food (increases happiness)
    let give_food = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![(ResourceType::Food, 15)].into_iter().collect(),
    };
    engine.execute_command(give_food);
    
    // Let time pass
    for _ in 0..30 {
        engine.tick();
    }
    
    let snapshot = engine.snapshot();
    let worker_after = snapshot.entities
        .iter()
        .find(|e| e.id == worker.id)
        .unwrap();
    
    let final_happiness = worker_after.components
        .get("happiness")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    
    assert!(final_happiness > initial_happiness, 
            "Worker happiness should increase with housing and food");
}

#[test]
fn test_worker_idle_state() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 20,
        height: 20,
        starting_workers: 1,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Check initial state
    let state = worker.components
        .get("state")
        .and_then(|v| serde_json::from_value::<WorkerState>(v.clone()).ok())
        .unwrap_or(WorkerState::Idle);
    
    assert_eq!(state, WorkerState::Idle, "Worker should start idle");
    
    // Give task
    let harvest_cmd = EngineCommand::Harvest {
        worker_id: worker.id,
        resource_id: 999, // Non-existent resource
    };
    engine.execute_command(harvest_cmd);
    
    engine.tick();
    
    let snapshot = engine.snapshot();
    let worker_after = snapshot.entities
        .iter()
        .find(|e| e.id == worker.id)
        .unwrap();
    
    let state_after = worker_after.components
        .get("state")
        .and_then(|v| serde_json::from_value::<WorkerState>(v.clone()).ok())
        .unwrap_or(WorkerState::Idle);
    
    // Should return to idle if task fails
    assert_ne!(state_after, WorkerState::Working, 
               "Worker should not be working on invalid task");
}

#[test]
fn test_population_limit() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 15,
        height: 15,
        starting_workers: 1,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    // Try to spawn many workers without housing
    let mut spawn_results = Vec::new();
    for i in 0..10 {
        let spawn_cmd = EngineCommand::SpawnWorker {
            position: Position::new(i as i32, i as i32),
            settlement_id: None,
        };
        spawn_results.push(engine.execute_command(spawn_cmd));
    }
    
    // Most spawns should fail due to population limit
    let successful_spawns = spawn_results.iter()
        .filter(|r| r.success)
        .count();
    
    assert!(successful_spawns < 5, 
            "Should not spawn unlimited workers without housing");
    
    let snapshot = engine.snapshot();
    let worker_count = snapshot.entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Worker))
        .count();
    
    assert!(worker_count <= engine.get_population_cap(),
            "Worker count should not exceed population cap");
}