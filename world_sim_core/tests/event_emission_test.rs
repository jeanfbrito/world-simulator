//! Test event emission functionality
//! This test MUST fail first (TDD)

use world_sim_core::SimulationEngine;
use world_sim_interface::{
    WorldConfig, EntityType, Position, EngineCommand,
    EngineEvent, EngineObserver, BuildingType, ResourceType
};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

#[derive(Clone)]
struct TestObserver {
    events: Arc<Mutex<VecDeque<EngineEvent>>>,
}

impl TestObserver {
    fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    fn get_events(&self) -> Vec<EngineEvent> {
        self.events.lock().unwrap().drain(..).collect()
    }
    
    fn has_event<F>(&self, predicate: F) -> bool 
    where
        F: Fn(&EngineEvent) -> bool
    {
        self.events.lock().unwrap().iter().any(predicate)
    }
}

impl EngineObserver for TestObserver {
    fn on_event(&mut self, event: EngineEvent) {
        self.events.lock().unwrap().push_back(event);
    }
}

#[test]
fn test_world_created_event() {
    let mut engine = SimulationEngine::new();
    let observer = TestObserver::new();
    
    engine.add_observer(Box::new(observer.clone()));
    
    let config = WorldConfig {
        width: 25,
        height: 25,
        seed: Some(42),
        ..Default::default()
    };
    
    engine.new_world(config.clone()).unwrap();
    
    let events = observer.get_events();
    
    assert!(events.iter().any(|e| matches!(e, EngineEvent::WorldCreated { .. })),
            "Should emit WorldCreated event");
    
    if let Some(EngineEvent::WorldCreated { width, height, seed }) = 
        events.iter().find(|e| matches!(e, EngineEvent::WorldCreated { .. }))
    {
        assert_eq!(*width, config.width);
        assert_eq!(*height, config.height);
        assert_eq!(*seed, config.seed);
    }
}

#[test]
fn test_entity_spawned_events() {
    let mut engine = SimulationEngine::new();
    let observer = TestObserver::new();
    
    engine.add_observer(Box::new(observer.clone()));
    
    let config = WorldConfig {
        width: 30,
        height: 30,
        starting_workers: 3,
        resource_density: 0.1,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let events = observer.get_events();
    
    // Should emit events for workers
    let worker_spawns = events.iter()
        .filter(|e| matches!(e, EngineEvent::EntitySpawned { 
            entity_type: EntityType::Worker, .. 
        }))
        .count();
    
    assert_eq!(worker_spawns, 3, "Should emit event for each worker spawn");
    
    // Should emit events for resources
    let resource_spawns = events.iter()
        .filter(|e| matches!(e, EngineEvent::EntitySpawned { 
            entity_type: EntityType::Tree | EntityType::BerryBush | EntityType::StoneDeposit, .. 
        }))
        .count();
    
    assert!(resource_spawns > 0, "Should emit events for resource spawns");
}

#[test]
fn test_harvest_events() {
    let mut engine = SimulationEngine::new();
    let observer = TestObserver::new();
    
    engine.add_observer(Box::new(observer.clone()));
    
    let config = WorldConfig {
        width: 20,
        height: 20,
        starting_workers: 1,
        resource_density: 0.2,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    let tree = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Tree))
        .unwrap();
    
    // Clear initial events
    observer.get_events();
    
    // Issue harvest command
    let cmd = EngineCommand::Harvest {
        worker_id: worker.id,
        resource_id: tree.id,
    };
    engine.execute_command(cmd);
    
    // Let harvest proceed
    for _ in 0..20 {
        engine.tick();
    }
    
    let events = observer.get_events();
    
    // Should have harvest started event
    assert!(events.iter().any(|e| matches!(e, EngineEvent::HarvestStarted { .. })),
            "Should emit HarvestStarted event");
    
    // Should have harvest completed event
    assert!(events.iter().any(|e| matches!(e, EngineEvent::HarvestCompleted { .. })),
            "Should emit HarvestCompleted event");
    
    // Should have resource collected event
    assert!(events.iter().any(|e| matches!(e, EngineEvent::ResourceCollected { .. })),
            "Should emit ResourceCollected event");
}

#[test]
fn test_building_events() {
    let mut engine = SimulationEngine::new();
    let observer = TestObserver::new();
    
    engine.add_observer(Box::new(observer.clone()));
    
    let config = WorldConfig {
        width: 25,
        height: 25,
        starting_workers: 1,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Clear initial events
    observer.get_events();
    
    // Give resources
    let give_cmd = EngineCommand::GiveResources {
        entity_id: worker.id,
        resources: vec![
            (ResourceType::Wood, 10),
            (ResourceType::Stone, 5),
        ].into_iter().collect(),
    };
    engine.execute_command(give_cmd);
    
    // Build house
    let build_cmd = EngineCommand::Build {
        builder_id: worker.id,
        building_type: BuildingType::House,
        position: Position::new(12, 12),
    };
    engine.execute_command(build_cmd);
    
    // Let construction proceed
    for _ in 0..25 {
        engine.tick();
    }
    
    let events = observer.get_events();
    
    // Should have construction started event
    assert!(events.iter().any(|e| matches!(e, EngineEvent::ConstructionStarted { .. })),
            "Should emit ConstructionStarted event");
    
    // Should have construction completed event
    assert!(events.iter().any(|e| matches!(e, EngineEvent::ConstructionCompleted { .. })),
            "Should emit ConstructionCompleted event");
    
    // Should have entity spawned for building
    assert!(events.iter().any(|e| matches!(e, EngineEvent::EntitySpawned { 
        entity_type: EntityType::Building(_), .. 
    })), "Should emit EntitySpawned for building");
}

#[test]
fn test_tick_events() {
    let mut engine = SimulationEngine::new();
    let observer = TestObserver::new();
    
    engine.add_observer(Box::new(observer.clone()));
    
    let config = WorldConfig::default();
    engine.new_world(config).unwrap();
    
    // Clear initial events
    observer.get_events();
    
    // Run some ticks
    for _ in 0..5 {
        engine.tick();
    }
    
    let events = observer.get_events();
    
    // Should have tick events
    let tick_events: Vec<_> = events.iter()
        .filter_map(|e| match e {
            EngineEvent::Tick { tick } => Some(*tick),
            _ => None
        })
        .collect();
    
    assert_eq!(tick_events.len(), 5, "Should emit event for each tick");
    
    // Ticks should be sequential
    for i in 0..4 {
        assert_eq!(tick_events[i] + 1, tick_events[i + 1], 
                   "Ticks should be sequential");
    }
}

#[test]
fn test_multiple_observers() {
    let mut engine = SimulationEngine::new();
    
    let observer1 = TestObserver::new();
    let observer2 = TestObserver::new();
    
    engine.add_observer(Box::new(observer1.clone()));
    engine.add_observer(Box::new(observer2.clone()));
    
    let config = WorldConfig {
        width: 15,
        height: 15,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    let events1 = observer1.get_events();
    let events2 = observer2.get_events();
    
    // Both observers should receive events
    assert!(!events1.is_empty(), "Observer 1 should receive events");
    assert!(!events2.is_empty(), "Observer 2 should receive events");
    
    // Both should receive same number of events
    assert_eq!(events1.len(), events2.len(), 
               "All observers should receive same events");
}

#[test]
fn test_command_events() {
    let mut engine = SimulationEngine::new();
    let observer = TestObserver::new();
    
    engine.add_observer(Box::new(observer.clone()));
    
    let config = WorldConfig::default();
    engine.new_world(config).unwrap();
    
    let snapshot = engine.snapshot();
    let worker = snapshot.entities
        .iter()
        .find(|e| matches!(e.entity_type, EntityType::Worker))
        .unwrap();
    
    // Clear initial events
    observer.get_events();
    
    // Issue various commands
    let move_cmd = EngineCommand::Move {
        entity_id: worker.id,
        target: Position::new(10, 10),
    };
    engine.execute_command(move_cmd);
    
    let events = observer.get_events();
    
    // Should emit command received event
    assert!(events.iter().any(|e| matches!(e, EngineEvent::CommandReceived { .. })),
            "Should emit CommandReceived event");
    
    // Should emit command executed event
    assert!(events.iter().any(|e| matches!(e, EngineEvent::CommandExecuted { .. })),
            "Should emit CommandExecuted event");
}

#[test]
fn test_state_change_events() {
    let mut engine = SimulationEngine::new();
    let observer = TestObserver::new();
    
    engine.add_observer(Box::new(observer.clone()));
    
    let config = WorldConfig {
        width: 20,
        height: 20,
        starting_workers: 1,
        seasons_enabled: true,
        ..Default::default()
    };
    
    engine.new_world(config).unwrap();
    
    // Clear initial events
    observer.get_events();
    
    // Run many ticks to trigger season change
    for _ in 0..500 {
        engine.tick();
    }
    
    let events = observer.get_events();
    
    // Should have season change event
    assert!(events.iter().any(|e| matches!(e, EngineEvent::SeasonChanged { .. })),
            "Should emit SeasonChanged event when seasons enabled");
}