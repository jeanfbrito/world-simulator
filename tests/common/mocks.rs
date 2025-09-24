//! Mock objects and utilities for testing
//!
//! This module provides mock implementations of various components
//! and systems to facilitate isolated unit testing.

use world_sim_interface::*;
use world_sim_simple::*;
use std::collections::HashMap;

/// Mock simulation state for testing
#[derive(Debug, Clone)]
pub struct MockSimulationState {
    pub tick: u32,
    pub running: bool,
    pub paused: bool,
    pub entities: HashMap<Entity, MockEntity>,
}

impl Default for MockSimulationState {
    fn default() -> Self {
        Self {
            tick: 0,
            running: false,
            paused: false,
            entities: HashMap::new(),
        }
    }
}

impl MockSimulationState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.running = true;
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.paused = false;
    }

    pub fn advance_tick(&mut self) {
        if self.running && !self.paused {
            self.tick += 1;
        }
    }

    pub fn add_entity(&mut self, entity: MockEntity) -> Entity {
        let id = Entity::from_raw(self.entities.len() as u32);
        self.entities.insert(id, entity);
        id
    }

    pub fn get_entity(&self, id: Entity) -> Option<&MockEntity> {
        self.entities.get(&id)
    }
}

/// Mock entity for testing
#[derive(Debug, Clone)]
pub struct MockEntity {
    pub id: Entity,
    pub name: String,
    pub components: HashMap<String, Box<dyn std::any::Any>>,
}

impl MockEntity {
    pub fn new(id: Entity, name: String) -> Self {
        Self {
            id,
            name,
            components: HashMap::new(),
        }
    }

    pub fn add_component<T: 'static>(&mut self, component: T) {
        self.components.insert(
            std::any::type_name::<T>().to_string(),
            Box::new(component),
        );
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        self.components
            .get(&std::any::type_name::<T>().to_string())
            .and_then(|c| c.downcast_ref::<T>())
    }
}

/// Mock world for testing
#[derive(Debug)]
pub struct MockWorld {
    pub state: MockSimulationState,
    pub resources: HashMap<String, u32>,
    pub events: Vec<MockEvent>,
}

impl MockWorld {
    pub fn new() -> Self {
        Self {
            state: MockSimulationState::new(),
            resources: HashMap::new(),
            events: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, resource_type: &str, amount: u32) {
        *self.resources.entry(resource_type.to_string()).or_insert(0) += amount;
    }

    pub fn get_resource(&self, resource_type: &str) -> u32 {
        self.resources.get(resource_type).copied().unwrap_or(0)
    }

    pub fn add_event(&mut self, event: MockEvent) {
        self.events.push(event);
    }

    pub fn get_events(&self) -> &[MockEvent] {
        &self.events
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    pub fn update(&mut self) {
        self.state.advance_tick();

        // Process basic entity updates
        for entity in self.state.entities.values_mut() {
            // Simple hunger decay
            if let Some(mut hunger) = entity.get_component::<f32>().cloned() {
                hunger = (hunger - 0.1).max(0.0);
                entity.components.insert(
                    "hunger".to_string(),
                    Box::new(hunger),
                );
            }
        }
    }
}

/// Mock event for testing
#[derive(Debug, Clone)]
pub struct MockEvent {
    pub event_type: String,
    pub timestamp: u32,
    pub data: HashMap<String, String>,
}

impl MockEvent {
    pub fn new(event_type: String) -> Self {
        Self {
            event_type,
            timestamp: 0,
            data: HashMap::new(),
        }
    }

    pub fn with_data(mut self, key: String, value: String) -> Self {
        self.data.insert(key, value);
        self
    }
}

/// Mock AI system for testing
#[derive(Debug)]
pub struct MockAISystem {
    pub decisions: Vec<String>,
    pub planning_time: u32,
}

impl MockAISystem {
    pub fn new() -> Self {
        Self {
            decisions: Vec::new(),
            planning_time: 0,
        }
    }

    pub fn plan(&mut self, world: &MockWorld) -> String {
        self.planning_time += 1;

        // Simple mock decision making
        let decision = if world.get_resource("food") < 50 {
            "gather_food".to_string()
        } else if world.get_resource("wood") < 30 {
            "gather_wood".to_string()
        } else {
            "idle".to_string()
        };

        self.decisions.push(decision.clone());
        decision
    }

    pub fn get_last_decision(&self) -> Option<&str> {
        self.decisions.last().map(|s| s.as_str())
    }

    pub fn clear_decisions(&mut self) {
        self.decisions.clear();
    }
}

/// Mock resource system for testing
#[derive(Debug)]
pub struct MockResourceSystem {
    pub resources: HashMap<String, u32>,
    pub regeneration_rates: HashMap<String, f32>,
}

impl MockResourceSystem {
    pub fn new() -> Self {
        let mut system = Self {
            resources: HashMap::new(),
            regeneration_rates: HashMap::new(),
        };

        // Initialize with default resources
        system.add_resource("food", 100, 0.1);
        system.add_resource("wood", 50, 0.05);
        system.add_resource("stone", 30, 0.02);

        system
    }

    pub fn add_resource(&mut self, resource_type: &str, amount: u32, regeneration_rate: f32) {
        self.resources.insert(resource_type.to_string(), amount);
        self.regeneration_rates.insert(resource_type.to_string(), regeneration_rate);
    }

    pub fn get_resource(&self, resource_type: &str) -> u32 {
        self.resources.get(resource_type).copied().unwrap_or(0)
    }

    pub fn harvest_resource(&mut self, resource_type: &str, amount: u32) -> bool {
        if let Some(current) = self.resources.get_mut(resource_type) {
            if *current >= amount {
                *current -= amount;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn update(&mut self) {
        // Regenerate resources
        for (resource_type, rate) in &self.regeneration_rates {
            if let Some(current) = self.resources.get_mut(resource_type) {
                *current = (*current as f32 * (1.0 + rate)).min(1000.0) as u32;
            }
        }
    }
}

/// Mock networking system for testing
#[derive(Debug)]
pub struct MockNetworkingSystem {
    pub connected: bool,
    pub messages: Vec<String>,
    pub sent_messages: Vec<String>,
    pub connection_errors: Vec<String>,
}

impl MockNetworkingSystem {
    pub fn new() -> Self {
        Self {
            connected: false,
            messages: Vec::new(),
            sent_messages: Vec::new(),
            connection_errors: Vec::new(),
        }
    }

    pub fn connect(&mut self) -> Result<(), String> {
        self.connected = true;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn send_message(&mut self, message: String) -> Result<(), String> {
        if !self.connected {
            return Err("Not connected".to_string());
        }

        self.sent_messages.push(message);
        Ok(())
    }

    pub fn receive_message(&mut self) -> Option<String> {
        self.messages.pop()
    }

    pub fn simulate_incoming_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn simulate_connection_error(&mut self, error: String) {
        self.connection_errors.push(error);
        self.connected = false;
    }
}

/// Mock persistence system for testing
#[derive(Debug)]
pub struct MockPersistenceSystem {
    pub saved_states: Vec<String>,
    pub load_errors: Vec<String>,
    pub save_errors: Vec<String>,
}

impl MockPersistenceSystem {
    pub fn new() -> Self {
        Self {
            saved_states: Vec::new(),
            load_errors: Vec::new(),
            save_errors: Vec::new(),
        }
    }

    pub fn save_state(&mut self, state: String) -> Result<(), String> {
        self.saved_states.push(state);
        Ok(())
    }

    pub fn load_state(&mut self) -> Result<String, String> {
        if let Some(state) = self.saved_states.last() {
            Ok(state.clone())
        } else {
            Err("No saved state available".to_string())
        }
    }

    pub fn simulate_save_error(&mut self, error: String) {
        self.save_errors.push(error);
    }

    pub fn simulate_load_error(&mut self, error: String) {
        self.load_errors.push(error);
    }

    pub fn get_saved_states(&self) -> &[String] {
        &self.saved_states
    }
}

/// Test helpers for mock systems
pub struct MockTestHelpers;

impl MockTestHelpers {
    pub fn create_basic_world() -> MockWorld {
        let mut world = MockWorld::new();
        world.state.start();
        world.add_resource("food", 100);
        world.add_resource("wood", 50);
        world
    }

    pub fn create_entity_with_components(id: u32, name: &str) -> MockEntity {
        let mut entity = MockEntity::new(Entity::from_raw(id), name.to_string());
        entity.add_component(PositionComponent { x: 0.0, y: 0.0 });
        entity.add_component(UnitStats::default());
        entity.add_component(UnitTag);
        entity
    }

    pub fn simulate_ticks(world: &mut MockWorld, ticks: u32) {
        for _ in 0..ticks {
            world.update();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_simulation_state() {
        let mut state = MockSimulationState::new();
        assert_eq!(state.tick, 0);
        assert!(!state.running);

        state.start();
        assert!(state.running);

        state.advance_tick();
        assert_eq!(state.tick, 1);

        state.pause();
        assert!(state.paused);

        state.advance_tick();
        assert_eq!(state.tick, 1); // Should not advance when paused
    }

    #[test]
    fn test_mock_resource_system() {
        let mut system = MockResourceSystem::new();

        assert_eq!(system.get_resource("food"), 100);

        assert!(system.harvest_resource("food", 50));
        assert_eq!(system.get_resource("food"), 50);

        assert!(!system.harvest_resource("food", 100));

        system.update(); // Should regenerate some food
        assert!(system.get_resource("food") > 50);
    }

    #[test]
    fn test_mock_networking_system() {
        let mut system = MockNetworkingSystem::new();

        assert!(!system.connected);

        assert!(system.connect().is_ok());
        assert!(system.connected);

        assert!(system.send_message("test".to_string()).is_ok());
        assert_eq!(system.sent_messages.len(), 1);

        system.simulate_incoming_message("response".to_string());
        assert_eq!(system.receive_message(), Some("response".to_string()));
    }
}