//! Test deterministic simulation functionality
//! This test MUST fail first (TDD)

use world_sim_core::SimulationEngine;
use world_sim_interface::{
    WorldConfig, EntityType, Position, EngineCommand,
    BuildingType, ResourceType, WorldSnapshot
};
use std::collections::HashMap;

#[test]
fn test_deterministic_world_generation() {
    let config = WorldConfig {
        width: 50,
        height: 50,
        seed: Some(12345),
        resource_density: 0.15,
        starting_workers: 5,
        ..Default::default()
    };
    
    // Create multiple engines with same config
    let mut engines = Vec::new();
    for _ in 0..3 {
        let mut engine = SimulationEngine::new();
        engine.new_world(config.clone()).unwrap();
        engines.push(engine);
    }
    
    // All should have identical initial state
    let snapshots: Vec<WorldSnapshot> = engines.iter()
        .map(|e| e.snapshot())
        .collect();
    
    for i in 1..snapshots.len() {
        assert_eq!(snapshots[0].entities.len(), snapshots[i].entities.len(),
                   "Should have same number of entities");
        
        // Check each entity matches
        for j in 0..snapshots[0].entities.len() {
            assert_eq!(snapshots[0].entities[j].position, 
                      snapshots[i].entities[j].position,
                      "Entity positions should match");
            assert_eq!(snapshots[0].entities[j].entity_type,
                      snapshots[i].entities[j].entity_type,
                      "Entity types should match");
        }
    }
}

#[test]
fn test_deterministic_simulation() {
    let config = WorldConfig {
        width: 30,
        height: 30,
        seed: Some(99999),
        starting_workers: 3,
        resource_density: 0.1,
        ..Default::default()
    };
    
    // Create two identical simulations
    let mut engine1 = SimulationEngine::new();
    let mut engine2 = SimulationEngine::new();
    
    engine1.new_world(config.clone()).unwrap();
    engine2.new_world(config.clone()).unwrap();
    
    // Issue same commands to both
    let commands = vec![
        EngineCommand::Move {
            entity_id: 1,
            target: Position::new(10, 10),
        },
        EngineCommand::Harvest {
            worker_id: 2,
            resource_id: 10,
        },
    ];
    
    for cmd in &commands {
        engine1.execute_command(cmd.clone());
        engine2.execute_command(cmd.clone());
    }
    
    // Run same number of ticks
    for _ in 0..50 {
        engine1.tick();
        engine2.tick();
    }
    
    // States should be identical
    let snap1 = engine1.snapshot();
    let snap2 = engine2.snapshot();
    
    assert_eq!(snap1.tick, snap2.tick, "Should be at same tick");
    assert_eq!(snap1.entities.len(), snap2.entities.len(),
               "Should have same entities");
    
    // Check all entity states match
    for (e1, e2) in snap1.entities.iter().zip(snap2.entities.iter()) {
        assert_eq!(e1.id, e2.id, "Entity IDs should match");
        assert_eq!(e1.position, e2.position, "Positions should match");
        assert_eq!(e1.entity_type, e2.entity_type, "Types should match");
        assert_eq!(e1.components, e2.components, "Components should match");
    }
}

#[test]
fn test_deterministic_random_events() {
    let config = WorldConfig {
        width: 40,
        height: 40,
        seed: Some(7777),
        resource_regeneration: true,
        seasons_enabled: true,
        ..Default::default()
    };
    
    let mut engine1 = SimulationEngine::new();
    let mut engine2 = SimulationEngine::new();
    
    engine1.new_world(config.clone()).unwrap();
    engine2.new_world(config.clone()).unwrap();
    
    // Run many ticks to trigger random events
    for _ in 0..200 {
        engine1.tick();
        engine2.tick();
    }
    
    let snap1 = engine1.snapshot();
    let snap2 = engine2.snapshot();
    
    // Even with random events, should be deterministic
    assert_eq!(snap1.global.season, snap2.global.season,
               "Seasons should match");
    assert_eq!(snap1.global.weather, snap2.global.weather,
               "Weather should match");
}

#[test]
fn test_deterministic_pathfinding() {
    let config = WorldConfig {
        width: 25,
        height: 25,
        seed: Some(4444),
        starting_workers: 2,
        ..Default::default()
    };
    
    let mut engine1 = SimulationEngine::new();
    let mut engine2 = SimulationEngine::new();
    
    engine1.new_world(config.clone()).unwrap();
    engine2.new_world(config.clone()).unwrap();
    
    // Move workers to same destination
    let move_cmd = EngineCommand::Move {
        entity_id: 1,
        target: Position::new(20, 20),
    };
    
    engine1.execute_command(move_cmd.clone());
    engine2.execute_command(move_cmd);
    
    // Step through pathfinding
    for _ in 0..30 {
        engine1.tick();
        engine2.tick();
        
        // Check positions match at each step
        let snap1 = engine1.snapshot();
        let snap2 = engine2.snapshot();
        
        let worker1 = snap1.entities.iter()
            .find(|e| e.id == 1).unwrap();
        let worker2 = snap2.entities.iter()
            .find(|e| e.id == 1).unwrap();
        
        assert_eq!(worker1.position, worker2.position,
                   "Pathfinding should be deterministic");
    }
}

#[test]
fn test_deterministic_resource_consumption() {
    let config = WorldConfig {
        width: 20,
        height: 20,
        seed: Some(3333),
        starting_workers: 3,
        resource_density: 0.2,
        ..Default::default()
    };
    
    let mut engine1 = SimulationEngine::new();
    let mut engine2 = SimulationEngine::new();
    
    engine1.new_world(config.clone()).unwrap();
    engine2.new_world(config.clone()).unwrap();
    
    // Give same resources to workers
    for i in 1..=3 {
        let give_cmd = EngineCommand::GiveResources {
            entity_id: i,
            resources: vec![(ResourceType::Food, 10)].into_iter().collect(),
        };
        engine1.execute_command(give_cmd.clone());
        engine2.execute_command(give_cmd);
    }
    
    // Run simulation
    for _ in 0..100 {
        engine1.tick();
        engine2.tick();
    }
    
    // Check resource consumption matches
    let snap1 = engine1.snapshot();
    let snap2 = engine2.snapshot();
    
    for i in 1..=3 {
        let worker1 = snap1.entities.iter()
            .find(|e| e.id == i).unwrap();
        let worker2 = snap2.entities.iter()
            .find(|e| e.id == i).unwrap();
        
        let food1 = worker1.components.get("inventory")
            .and_then(|inv| inv.get("food"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        
        let food2 = worker2.components.get("inventory")
            .and_then(|inv| inv.get("food"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        
        assert_eq!(food1, food2, "Food consumption should be deterministic");
    }
}

#[test]
fn test_save_load_determinism() {
    let config = WorldConfig {
        width: 35,
        height: 35,
        seed: Some(8888),
        starting_workers: 4,
        ..Default::default()
    };
    
    let mut engine1 = SimulationEngine::new();
    engine1.new_world(config).unwrap();
    
    // Run some simulation
    for _ in 0..50 {
        engine1.tick();
    }
    
    // Save state
    let save_data = engine1.save_state().expect("Should save state");
    
    // Load into new engine
    let mut engine2 = SimulationEngine::new();
    engine2.load_state(save_data).expect("Should load state");
    
    // Continue simulation
    for _ in 0..50 {
        engine1.tick();
        engine2.tick();
    }
    
    // States should match
    let snap1 = engine1.snapshot();
    let snap2 = engine2.snapshot();
    
    assert_eq!(snap1.tick, snap2.tick);
    assert_eq!(snap1.entities.len(), snap2.entities.len());
    assert_eq!(snap1.global.season, snap2.global.season);
}

#[test]
fn test_deterministic_system_order() {
    let config = WorldConfig {
        width: 15,
        height: 15,
        seed: Some(1111),
        starting_workers: 2,
        ..Default::default()
    };
    
    // Create multiple engines
    let mut engines = Vec::new();
    for _ in 0..5 {
        let mut engine = SimulationEngine::new();
        engine.new_world(config.clone()).unwrap();
        engines.push(engine);
    }
    
    // Run many ticks with complex interactions
    for _ in 0..100 {
        for engine in &mut engines {
            engine.tick();
        }
    }
    
    // All should have identical state
    let reference = engines[0].snapshot();
    for engine in &engines[1..] {
        let snapshot = engine.snapshot();
        assert_eq!(reference.tick, snapshot.tick);
        assert_eq!(reference.entities.len(), snapshot.entities.len());
        
        // Verify entity order is preserved
        for (r, s) in reference.entities.iter().zip(snapshot.entities.iter()) {
            assert_eq!(r.id, s.id, "Entity order should be deterministic");
        }
    }
}