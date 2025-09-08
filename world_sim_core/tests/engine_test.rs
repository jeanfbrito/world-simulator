//! Test SimulationEngine trait
//! This test MUST fail first (TDD)

use world_sim_core::SimulationEngine;
use world_sim_interface::{EngineCommand, WorldConfig, Position, BuildingType};

#[test]
fn test_engine_creation() {
    let mut engine = SimulationEngine::new();
    
    let config = WorldConfig {
        width: 100,
        height: 100,
        seed: Some(42),
        resource_density: 0.1,
        starting_workers: 5,
        seasons_enabled: false,
        resource_regeneration: true,
    };
    
    engine.new_world(config).expect("Should create world");
    
    let snapshot = engine.snapshot();
    assert_eq!(snapshot.entities.len(), 5, "Should have 5 starting workers");
}

#[test]
fn test_engine_tick() {
    let mut engine = SimulationEngine::new();
    engine.new_world(WorldConfig::default()).unwrap();
    
    let initial_tick = engine.snapshot().tick;
    engine.tick(0.016); // 60 FPS
    let after_tick = engine.snapshot().tick;
    
    assert_eq!(after_tick, initial_tick + 1);
}

#[test]
fn test_engine_command_execution() {
    let mut engine = SimulationEngine::new();
    engine.new_world(WorldConfig::default()).unwrap();
    
    let cmd = EngineCommand::ConstructBuilding {
        building_type: BuildingType::House,
        position: Position { x: 50, y: 50 },
        workers: vec![],
    };
    
    let result = engine.execute_command(cmd);
    assert!(result.success);
}

#[test]
fn test_engine_determinism() {
    let config = WorldConfig {
        seed: Some(12345),
        ..Default::default()
    };
    
    let mut engine1 = SimulationEngine::new();
    engine1.new_world(config.clone()).unwrap();
    
    let mut engine2 = SimulationEngine::new();
    engine2.new_world(config).unwrap();
    
    // Run same commands on both
    for _ in 0..10 {
        engine1.tick();
        engine2.tick();
    }
    
    // Should produce identical state
    let snap1 = engine1.snapshot();
    let snap2 = engine2.snapshot();
    assert_eq!(snap1.tick, snap2.tick);
    assert_eq!(snap1.entities.len(), snap2.entities.len());
}