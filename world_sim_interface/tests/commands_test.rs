//! Test EngineCommand serialization and structure
//! This test MUST fail first (TDD)

use serde_json;
use world_sim_interface::{EngineCommand, EntityId, Position, BuildingType, TaskAssignment};

#[test]
fn test_harvest_command_serialization() {
    let cmd = EngineCommand::HarvestResource {
        worker_ids: vec![1, 2, 3],
        target_id: 42,
    };
    
    let json = serde_json::to_string(&cmd).expect("Should serialize");
    assert!(json.contains("HarvestResource"));
    assert!(json.contains("worker_ids"));
}

#[test]
fn test_construct_building_command() {
    let cmd = EngineCommand::ConstructBuilding {
        building_type: BuildingType::House,
        position: Position { x: 50, y: 50 },
        workers: vec![5, 6],
    };
    
    let json = serde_json::to_string(&cmd).expect("Should serialize");
    let deserialized: EngineCommand = serde_json::from_str(&json).expect("Should deserialize");
    
    match deserialized {
        EngineCommand::ConstructBuilding { position, .. } => {
            assert_eq!(position.x, 50);
            assert_eq!(position.y, 50);
        }
        _ => panic!("Wrong command type"),
    }
}

#[test]
fn test_query_area_command() {
    let cmd = EngineCommand::QueryArea {
        min: Position { x: 0, y: 0 },
        max: Position { x: 100, y: 100 },
    };
    
    let json = serde_json::to_string(&cmd).expect("Should serialize");
    assert!(json.contains("QueryArea"));
}

#[test]
fn test_command_with_optional_fields() {
    let cmd = EngineCommand::AssignWorker {
        worker_id: 7,
        task: TaskAssignment::Idle,
    };
    
    let json = serde_json::to_string(&cmd).expect("Should serialize");
    let deserialized: EngineCommand = serde_json::from_str(&json).expect("Should deserialize");
    
    match deserialized {
        EngineCommand::AssignWorker { worker_id, .. } => assert_eq!(worker_id, 7),
        _ => panic!("Wrong command type"),
    }
}