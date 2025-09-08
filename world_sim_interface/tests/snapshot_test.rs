//! Test WorldSnapshot structure
//! This test MUST fail first (TDD)

use world_sim_interface::{WorldSnapshot, EntitySnapshot, SettlementSnapshot, GlobalState};
use world_sim_interface::{EntityId, EntityType, Position, ResourceType, Season, Weather};
use std::collections::HashMap;

#[test]
fn test_world_snapshot_structure() {
    let snapshot = WorldSnapshot {
        tick: 1000,
        entities: vec![
            EntitySnapshot {
                id: 1,
                entity_type: EntityType::Tree,
                position: Position { x: 10, y: 20 },
                components: HashMap::new(),
            },
            EntitySnapshot {
                id: 2,
                entity_type: EntityType::Worker,
                position: Position { x: 15, y: 25 },
                components: HashMap::new(),
            },
        ],
        settlements: vec![
            SettlementSnapshot {
                id: 1,
                name: "Main Settlement".to_string(),
                position: Position { x: 50, y: 50 },
                population: 10,
                happiness: 75.0,
                resources: HashMap::from([
                    (ResourceType::Wood, 100),
                    (ResourceType::Food, 50),
                ]),
                buildings: vec![1, 2, 3],
            },
        ],
        global: GlobalState {
            season: Season::Spring,
            weather: Weather::Clear,
            game_speed: 1.0,
        },
    };
    
    assert_eq!(snapshot.tick, 1000);
    assert_eq!(snapshot.entities.len(), 2);
    assert_eq!(snapshot.settlements.len(), 1);
}

#[test]
fn test_entity_snapshot_serialization() {
    let entity = EntitySnapshot {
        id: 42,
        entity_type: EntityType::House,
        position: Position { x: 30, y: 40 },
        components: HashMap::from([
            ("health".to_string(), serde_json::json!(100)),
            ("owner".to_string(), serde_json::json!("player1")),
        ]),
    };
    
    let json = serde_json::to_string(&entity).expect("Should serialize");
    assert!(json.contains("\"id\":42"));
    assert!(json.contains("House"));
    
    let deserialized: EntitySnapshot = serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(deserialized.id, 42);
}

#[test]
fn test_global_state() {
    let state = GlobalState {
        season: Season::Winter,
        weather: Weather::Snow,
        game_speed: 2.0,
    };
    
    let json = serde_json::to_string(&state).expect("Should serialize");
    assert!(json.contains("Winter"));
    assert!(json.contains("Snow"));
    assert!(json.contains("2.0"));
}