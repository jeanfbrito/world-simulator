//! Test EngineEvent serialization and structure
//! This test MUST fail first (TDD)

use serde_json;
use world_sim_interface::{EngineEvent, EntityId, Position, EntityType, ResourceType};

#[test]
fn test_entity_spawned_event_serialization() {
    let event = EngineEvent::EntitySpawned {
        id: 42,
        entity_type: EntityType::Tree,
        position: Position { x: 10, y: 20 },
        components: Default::default(),
    };
    
    let json = serde_json::to_string(&event).expect("Should serialize");
    assert!(json.contains("EntitySpawned"));
    assert!(json.contains("\"id\":42"));
}

#[test]
fn test_resource_harvested_event() {
    let event = EngineEvent::ResourceHarvested {
        worker_id: 1,
        resource_id: 2,
        resource_type: ResourceType::Wood,
        amount: 5,
    };
    
    let json = serde_json::to_string(&event).expect("Should serialize");
    let deserialized: EngineEvent = serde_json::from_str(&json).expect("Should deserialize");
    
    match deserialized {
        EngineEvent::ResourceHarvested { amount, .. } => assert_eq!(amount, 5),
        _ => panic!("Wrong event type"),
    }
}

#[test]
fn test_building_completed_event() {
    let event = EngineEvent::BuildingCompleted {
        building_id: 100,
        building_type: BuildingType::House,
    };
    
    let json = serde_json::to_string(&event).expect("Should serialize");
    assert!(json.contains("BuildingCompleted"));
}

#[test]
fn test_tick_completed_event() {
    let event = EngineEvent::TickCompleted {
        tick: 1234,
        delta_time: 0.016,
    };
    
    let bytes = bincode::serialize(&event).expect("Should serialize to binary");
    assert!(bytes.len() > 0);
}