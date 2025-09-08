//! Test building construction functionality
//! This test MUST fail first (TDD)

use world_sim_core::{SimulationEngine, WorldState};
use world_sim_interface::{
    WorldConfig, EntityType, Position, EngineCommand,
    BuildingType, ResourceType, CommandResult
};
use std::collections::HashMap;

#[test]
fn test_construct_house() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 50,
        height: 50,
        seed: Some(100),
        starting_workers: 2,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    // Give workers resources for building
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Add resources to worker (simulating they harvested)
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![
            (ResourceType::Wood, 10),
            (ResourceType::Stone, 5),
        ].into_iter().collect(),
    };
    
    engine.execute_command(give_resources);
    
    // Issue build command
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::House,
        position: Position::new(25, 25),
    };
    
    let result = engine.execute_command(build_cmd);
    assert!(result.success, "Build command should succeed with resources");
    
    // Let construction proceed
    for _ in 0..20 {
        engine.tick();
    }
    
    // Check house was built
    let snapshot = engine.snapshot();
    let house = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Building(BuildingType::House)));
    
    assert!(house.is_some(), "House should be constructed");
    assert_eq!(house.unwrap().position, Position::new(25, 25), 
               "House should be at specified position");
}

#[test]
fn test_building_requires_resources() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 30,
        height: 30,
        starting_workers: 1,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Try to build without resources
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::Sawmill,
        position: Position::new(15, 15),
    };
    
    let result = engine.execute_command(build_cmd);
    assert!(!result.success, "Should fail to build without resources");
    assert!(result.message.unwrap().contains("resources") || 
            result.message.unwrap().contains("insufficient"),
            "Should indicate resource requirement");
}

#[test]
fn test_stockpile_stores_resources() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 40,
        height: 40,
        starting_workers: 2,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Give resources to build stockpile
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![(ResourceType::Wood, 5)].into_iter().collect(),
    };
    engine.execute_command(give_resources);
    
    // Build stockpile
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::Stockpile,
        position: Position::new(20, 20),
    };
    
    engine.execute_command(build_cmd);
    
    // Let construction complete
    for _ in 0..15 {
        engine.tick();
    }
    
    let snapshot = engine.snapshot();
    let stockpile = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Building(BuildingType::Stockpile)))
        .expect("Stockpile should exist");
    
    // Give worker more resources
    let worker2 = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker) && e.id != worker.id)
        .unwrap();
    
    let give_more = EngineCommand::GiveResources {
        entity_id: worker2.id,
        resources: vec![
            (ResourceType::Wood, 20),
            (ResourceType::Food, 10),
        ].into_iter().collect(),
    };
    engine.execute_command(give_more);
    
    // Store in stockpile
    let store_cmd = EngineCommand::Store {
        worker_id: worker2.id,
        building_id: stockpile.id,
    };
    
    engine.execute_command(store_cmd);
    
    // Let storage happen
    for _ in 0..10 {
        engine.tick();
    }
    
    // Check stockpile has resources
    let snapshot = engine.snapshot();
    let stockpile_after = snapshot.entities
        .iter()
        .find(|e| e.id == stockpile.id)
        .unwrap();
    
    let storage = stockpile_after.components
        .get("storage")
        .expect("Stockpile should have storage");
    
    assert!(storage.get("wood").is_some(), "Stockpile should store wood");
    assert!(storage.get("food").is_some(), "Stockpile should store food");
}

#[test]
fn test_sawmill_processes_wood() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 30,
        height: 30,
        starting_workers: 2,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Give resources to build sawmill
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![
            (ResourceType::Wood, 15),
            (ResourceType::Stone, 10),
        ].into_iter().collect(),
    };
    engine.execute_command(give_resources);
    
    // Build sawmill
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::Sawmill,
        position: Position::new(15, 15),
    };
    
    engine.execute_command(build_cmd);
    
    // Let construction complete
    for _ in 0..20 {
        engine.tick();
    }
    
    let snapshot = engine.snapshot();
    let sawmill = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Building(BuildingType::Sawmill)))
        .expect("Sawmill should exist");
    
    // Give raw wood to process
    let give_wood = EngineCommand::GiveResources {
        entity_id: sawmill.id,
        resources: vec![(ResourceType::Wood, 10)].into_iter().collect(),
    };
    engine.execute_command(give_wood);
    
    // Assign worker to sawmill
    let assign_cmd = EngineCommand::AssignWorker {
        worker_id: worker.id,
        building_id: sawmill.id,
    };
    engine.execute_command(assign_cmd);
    
    // Let processing happen
    for _ in 0..30 {
        engine.tick();
    }
    
    // Check sawmill produced planks
    let snapshot = engine.snapshot();
    let sawmill_after = snapshot.entities
        .iter()
        .find(|e| e.id == sawmill.id)
        .unwrap();
    
    let output = sawmill_after.components
        .get("output")
        .expect("Sawmill should have output");
    
    let planks: u32 = output
        .get("planks")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    
    assert!(planks > 0, "Sawmill should produce planks from wood");
}

#[test]
fn test_building_collision() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 20,
        height: 20,
        starting_workers: 2,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Give resources for two buildings
    let give_resources = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![(ResourceType::Wood, 30)].into_iter().collect(),
    };
    engine.execute_command(give_resources);
    
    // Build first building
    let build1 = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::House,
        position: Position::new(10, 10),
    };
    engine.execute_command(build1);
    
    // Let first building complete
    for _ in 0..15 {
        engine.tick();
    }
    
    // Try to build on same spot
    let build2 = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::House,
        position: Position::new(10, 10),
    };
    
    let result = engine.execute_command(build2);
    assert!(!result.success, "Should not build on occupied position");
}