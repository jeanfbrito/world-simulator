//! Test world generation functionality
//! This test MUST fail first (TDD)

use world_sim_core::{SimulationEngine, WorldState};
use world_sim_interface::{WorldConfig, EntityType, Position};

#[test]
fn test_world_generates_terrain() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 50,
        height: 50,
        seed: Some(12345),
        resource_density: 0.1,
        starting_workers: 3,
        seasons_enabled: false,
        resource_regeneration: true,
    };
    
    engine.new_world(config).expect("Should create world");
    
    let snapshot = engine.snapshot();
    
    // Should have generated some trees
    let trees: Vec<_> = snapshot.entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Tree))
        .collect();
    
    assert!(trees.len() > 0, "Should have generated trees");
    assert!(trees.len() < 250, "Shouldn't fill entire map with trees");
}

#[test]
fn test_world_generates_berry_bushes() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 30,
        height: 30,
        seed: Some(54321),
        resource_density: 0.15,
        ..Default::default()
    };
    
    engine.new_world(config).expect("Should create world");
    
    let snapshot = engine.snapshot();
    
    let berries: Vec<_> = snapshot.entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::BerryBush))
        .collect();
    
    assert!(berries.len() > 0, "Should have generated berry bushes");
}

#[test]
fn test_world_spawns_starting_workers() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        starting_workers: 5,
        ..Default::default()
    };
    
    engine.new_world(config).expect("Should create world");
    
    let snapshot = engine.snapshot();
    
    let workers: Vec<_> = snapshot.entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Worker))
        .collect();
    
    assert_eq!(workers.len(), 5, "Should spawn exactly 5 starting workers");
}

#[test]
fn test_world_generation_is_deterministic() {
    let config = WorldConfig {
        width: 20,
        height: 20,
        seed: Some(999),
        resource_density: 0.2,
        ..Default::default()
    };
    
    let mut engine1 = SimulationEngine::new();
    engine1.new_world(config.clone()).unwrap();
    
    let mut engine2 = SimulationEngine::new();
    engine2.new_world(config).unwrap();
    
    let snap1 = engine1.snapshot();
    let snap2 = engine2.snapshot();
    
    assert_eq!(snap1.entities.len(), snap2.entities.len());
    
    // Check that entities are in same positions
    for (e1, e2) in snap1.entities.iter().zip(snap2.entities.iter()) {
        assert_eq!(e1.position, e2.position);
        assert_eq!(e1.entity_type, e2.entity_type);
    }
}