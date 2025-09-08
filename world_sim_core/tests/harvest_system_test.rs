//! Test harvest system functionality
//! This test MUST fail first (TDD)

use world_sim_core::{SimulationEngine, WorldState};
use world_sim_interface::{
    WorldConfig, EntityType, Position, EngineCommand, 
    ResourceType, CommandResult
};

#[test]
fn test_worker_can_harvest_trees() {
    let mut engine = SimulationEngine::new();
    
    // Create world with specific setup
    let config = WorldConfig {
        width: 10,
        height: 10,
        seed: Some(999),
        resource_density: 0.2,
        starting_workers: 1,
        ..Default::default()
    };
    
    engine.new_world(config).expect("Should create world");
    
    // Find a tree and worker
    let snapshot = engine.snapshot();
    let tree = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Tree))
        .expect("Should have at least one tree");
    
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .expect("Should have worker");
    
    // Issue harvest command
    let cmd = EngineCommand::Harvest {
        worker_id: worker.id,
        resource_id: tree.id,
    };
    
    let result = engine.execute_command(cmd);
    assert!(result.success, "Harvest command should succeed");
    
    // Advance simulation
    for _ in 0..10 {
        engine.tick();
    }
    
    // Check worker has wood in inventory
    let snapshot = engine.snapshot();
    let worker_after = snapshot.entities
        .iter()
        .find(|e| e.id == worker.id)
        .expect("Worker should still exist");
    
    let inventory = worker_after.components
        .get("inventory")
        .expect("Worker should have inventory");
    
    let wood_count: u32 = inventory
        .get("wood")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    
    assert!(wood_count > 0, "Worker should have harvested wood");
}

#[test]
fn test_harvest_requires_proximity() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 100,
        height: 100,
        seed: Some(123),
        starting_workers: 1,
        resource_density: 0.01, // Low density for controlled test
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Find a tree far from worker
    let distant_tree = snapshot.entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Tree))
        .max_by_key(|e| {
            e.position.distance_squared(&worker.position)
        })
        .expect("Should have trees");
    
    // Try to harvest distant tree
    let cmd = EngineCommand::Harvest {
        worker_id: worker.id,
        resource_id: distant_tree.id,
    };
    
    let result = engine.execute_command(cmd);
    
    // Worker should move towards tree but not harvest immediately
    for _ in 0..3 {
        engine.tick();
    }
    
    let snapshot = engine.snapshot();
    let worker_after = snapshot.entities
        .iter()
        .find(|e| e.id == worker.id)
        .unwrap();
    
    let inventory = worker_after.components.get("inventory");
    assert!(inventory.is_none() || 
            inventory.unwrap().get("wood").is_none(),
            "Should not harvest immediately when far away");
}

#[test]
fn test_berry_bush_harvest() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 20,
        height: 20,
        seed: Some(777),
        resource_density: 0.3,
        starting_workers: 2,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    
    let berry_bush = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::BerryBush))
        .expect("Should have berry bushes");
    
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Harvest berries
    let cmd = EngineCommand::Harvest {
        worker_id: worker.id,
        resource_id: berry_bush.id,
    };
    
    engine.execute_command(cmd);
    
    // Let worker complete harvest
    for _ in 0..15 {
        engine.tick();
    }
    
    let snapshot = engine.snapshot();
    let worker_after = snapshot.entities
        .iter()
        .find(|e| e.id == worker.id)
        .unwrap();
    
    let inventory = worker_after.components
        .get("inventory")
        .expect("Worker should have inventory");
    
    let food_count: u32 = inventory
        .get("food")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    
    assert!(food_count > 0, "Worker should have harvested food from berries");
}

#[test]
fn test_resource_depletion() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 10,
        height: 10,
        seed: Some(444),
        resource_density: 0.1,
        starting_workers: 3,
        resource_regeneration: false, // Disable regeneration
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let tree = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Tree))
        .expect("Should have trees");
    
    // Multiple workers harvest same tree
    for worker in snapshot.entities.iter()
        .filter(|e| matches!(e.entity_type, EntityType::Worker))
        .take(3)
    {
        let cmd = EngineCommand::Harvest {
            worker_id: worker.id,
            resource_id: tree.id,
        };
        engine.execute_command(cmd);
    }
    
    // Let harvesting complete
    for _ in 0..50 {
        engine.tick();
    }
    
    // Tree should be depleted
    let snapshot = engine.snapshot();
    let tree_after = snapshot.entities
        .iter()
        .find(|e| e.id == tree.id);
    
    assert!(
        tree_after.is_none() || 
        tree_after.unwrap().components
            .get("resource_amount")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) == 0,
        "Tree should be depleted after multiple harvests"
    );
}