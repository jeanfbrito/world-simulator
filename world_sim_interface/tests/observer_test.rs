//! Test EngineObserver trait
//! This test MUST fail first (TDD)

use world_sim_interface::{EngineObserver, EngineEvent, WorldSnapshot, EntityType, Position, GlobalState, Season, Weather};
use std::sync::{Arc, Mutex};

struct TestObserver {
    events_received: Arc<Mutex<Vec<EngineEvent>>>,
    snapshots_received: Arc<Mutex<Vec<WorldSnapshot>>>,
}

impl EngineObserver for TestObserver {
    fn on_events(&mut self, events: &[EngineEvent]) {
        let mut received = self.events_received.lock().unwrap();
        received.extend_from_slice(events);
    }
    
    fn on_snapshot(&mut self, snapshot: &WorldSnapshot) {
        let mut received = self.snapshots_received.lock().unwrap();
        received.push(snapshot.clone());
    }
    
    fn wants_snapshots(&self) -> bool {
        true
    }
}

#[test]
fn test_observer_receives_events() {
    let events_received = Arc::new(Mutex::new(Vec::new()));
    let snapshots_received = Arc::new(Mutex::new(Vec::new()));
    
    let mut observer = TestObserver {
        events_received: events_received.clone(),
        snapshots_received: snapshots_received.clone(),
    };
    
    let test_events = vec![
        EngineEvent::TickCompleted {
            tick: 1,
            delta_time: 0.016,
        },
        EngineEvent::EntitySpawned {
            id: 42,
            entity_type: EntityType::Tree,
            position: Position { x: 10, y: 20 },
            components: Default::default(),
        },
    ];
    
    observer.on_events(&test_events);
    
    let received = events_received.lock().unwrap();
    assert_eq!(received.len(), 2);
}

#[test]
fn test_observer_receives_snapshots() {
    let events_received = Arc::new(Mutex::new(Vec::new()));
    let snapshots_received = Arc::new(Mutex::new(Vec::new()));
    
    let mut observer = TestObserver {
        events_received: events_received.clone(),
        snapshots_received: snapshots_received.clone(),
    };
    
    let snapshot = WorldSnapshot {
        tick: 100,
        entities: vec![],
        settlements: vec![],
        global: GlobalState {
            season: Season::Spring,
            weather: Weather::Clear,
            game_speed: 1.0,
        },
    };
    
    observer.on_snapshot(&snapshot);
    
    let received = snapshots_received.lock().unwrap();
    assert_eq!(received.len(), 1);
    assert_eq!(received[0].tick, 100);
}

#[test]
fn test_observer_wants_snapshots_flag() {
    let observer = TestObserver {
        events_received: Arc::new(Mutex::new(Vec::new())),
        snapshots_received: Arc::new(Mutex::new(Vec::new())),
    };
    
    assert!(observer.wants_snapshots());
}