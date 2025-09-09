use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::*;

/// Represents a GOAP action that can be performed by an agent
#[derive(Clone, Debug)]
pub struct GoapAction {
    pub name: String,
    pub cost: f32,
    pub preconditions: HashMap<String, StateValue>,
    pub effects: HashMap<String, StateValue>,
}

/// Value types for GOAP states
#[derive(Clone, Debug, PartialEq)]
pub enum StateValue {
    Bool(bool),
    Float(f64),
    Int(u32),
}

impl GoapAction {
    pub fn new(name: &str, cost: f32) -> Self {
        Self {
            name: name.to_string(),
            cost,
            preconditions: HashMap::new(),
            effects: HashMap::new(),
        }
    }
    
    pub fn with_precondition(mut self, key: &str, value: StateValue) -> Self {
        self.preconditions.insert(key.to_string(), value);
        self
    }
    
    pub fn with_effect(mut self, key: &str, value: StateValue) -> Self {
        self.effects.insert(key.to_string(), value);
        self
    }
    
    /// Check if this action can be performed given the current world state
    pub fn is_valid(&self, world_state: &WorldState) -> bool {
        for (key, required_value) in &self.preconditions {
            if let Some(current_value) = world_state.get(key) {
                if !self.check_condition(current_value, required_value) {
                    return false;
                }
            } else {
                return false; // Missing required state
            }
        }
        true
    }
    
    fn check_condition(&self, current: &StateValue, required: &StateValue) -> bool {
        match (current, required) {
            (StateValue::Bool(c), StateValue::Bool(r)) => c == r,
            (StateValue::Float(c), StateValue::Float(r)) => (c - r).abs() < 0.01,
            (StateValue::Int(c), StateValue::Int(r)) => c >= r, // For resources, current must be >= required
            _ => false,
        }
    }
}

/// Represents the current world state for GOAP planning
#[derive(Component, Clone, Debug, Default)]
pub struct WorldState {
    pub states: HashMap<String, StateValue>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: &str, value: StateValue) {
        self.states.insert(key.to_string(), value);
    }
    
    pub fn get(&self, key: &str) -> Option<&StateValue> {
        self.states.get(key)
    }
    
    /// Apply the effects of an action to this world state
    pub fn apply_action(&mut self, action: &GoapAction) {
        for (key, value) in &action.effects {
            // For resource counts (has_wood, has_stone, has_food), add to existing value
            if key.starts_with("has_") {
                if let StateValue::Int(add_amount) = value {
                    if let Some(StateValue::Int(current)) = self.states.get(key) {
                        // Add to existing amount
                        self.states.insert(key.clone(), StateValue::Int(current + add_amount));
                    } else {
                        // Set initial amount
                        self.states.insert(key.clone(), value.clone());
                    }
                } else {
                    // Non-int resource or other state, just set it
                    self.states.insert(key.clone(), value.clone());
                }
            } else {
                // For non-resource states, just set the value
                self.states.insert(key.clone(), value.clone());
            }
        }
    }
    
    /// Create world state from entity components
    pub fn from_entity(
        entity: Entity,
        query: &Query<(
            Option<&IsHungry>,
            Option<&HasEnergy>,
            Option<&IsWorking>,
            Option<&HasWood>,
            Option<&HasFood>,
            Option<&HasStone>,
            Option<&AtResource>,
            Option<&AtStorage>,
            Option<&InventoryFull>,
            Option<&HasHouse>,
        )>,
    ) -> Self {
        let mut state = WorldState::new();
        
        if let Ok((hungry, energy, working, wood, food, stone, at_resource, at_storage, inv_full, has_house)) = query.get(entity) {
            if let Some(h) = hungry {
                state.set("is_hungry", StateValue::Float(h.0));
            }
            if let Some(e) = energy {
                state.set("has_energy", StateValue::Float(e.0));
            }
            if let Some(w) = working {
                state.set("is_working", StateValue::Bool(w.0));
            }
            if let Some(w) = wood {
                state.set("has_wood", StateValue::Int(w.0));
            }
            if let Some(f) = food {
                state.set("has_food", StateValue::Int(f.0));
            }
            if let Some(s) = stone {
                state.set("has_stone", StateValue::Int(s.0));
            }
            if let Some(ar) = at_resource {
                state.set("at_resource", StateValue::Bool(ar.0));
            }
            if let Some(as_) = at_storage {
                state.set("at_storage", StateValue::Bool(as_.0));
            }
            if let Some(if_) = inv_full {
                state.set("inventory_full", StateValue::Bool(if_.0));
            }
            if let Some(hh) = has_house {
                state.set("has_house", StateValue::Bool(hh.0));
            }
        }
        
        state
    }
}

/// Collection of all available GOAP actions
#[derive(Resource)]
pub struct ActionSet {
    pub actions: Vec<GoapAction>,
}

impl Default for ActionSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionSet {
    /// Get all actions that are valid for the given world state
    pub fn get_valid_actions(&self, state: &WorldState) -> Vec<&GoapAction> {
        self.actions
            .iter()
            .filter(|action| action.is_valid(state))
            .collect()
    }
    
    pub fn new() -> Self {
        let mut actions = Vec::new();
        
        // Move to resource action
        actions.push(
            GoapAction::new("move_to_resource", 1.0)
                .with_precondition("has_energy", StateValue::Float(0.2))
                .with_effect("at_resource", StateValue::Bool(true))
                .with_effect("at_storage", StateValue::Bool(false))
        );
        
        // Harvest resource action
        actions.push(
            GoapAction::new("harvest_resource", 2.0)
                .with_precondition("at_resource", StateValue::Bool(true))
                .with_precondition("inventory_full", StateValue::Bool(false))
                .with_effect("has_wood", StateValue::Int(5))
        );
        
        // Move to storage action
        actions.push(
            GoapAction::new("move_to_storage", 1.0)
                .with_precondition("has_energy", StateValue::Float(0.2))
                .with_effect("at_storage", StateValue::Bool(true))
                .with_effect("at_resource", StateValue::Bool(false))
        );
        
        // Store resources action
        actions.push(
            GoapAction::new("store_resources", 1.0)
                .with_precondition("at_storage", StateValue::Bool(true))
                .with_precondition("has_wood", StateValue::Int(1))
                .with_effect("has_wood", StateValue::Int(0))
                .with_effect("inventory_full", StateValue::Bool(false))
        );
        
        // Eat food action
        actions.push(
            GoapAction::new("eat_food", 0.5)
                .with_precondition("has_food", StateValue::Int(1))
                .with_precondition("is_hungry", StateValue::Float(0.3))
                .with_effect("is_hungry", StateValue::Float(0.0))
                .with_effect("has_energy", StateValue::Float(1.0))
        );
        
        // Rest action (when energy is low)
        actions.push(
            GoapAction::new("rest", 0.1)
                .with_precondition("has_energy", StateValue::Float(0.5)) // Can rest when below 50% energy
                .with_effect("has_energy", StateValue::Float(1.0)) // Restores to full energy
                .with_effect("is_working", StateValue::Bool(false))
        );
        
        // Gather food action (find berry bushes)
        actions.push(
            GoapAction::new("gather_food", 1.5)
                .with_precondition("has_energy", StateValue::Float(0.3)) // Need some energy to gather
                .with_precondition("has_food", StateValue::Int(0)) // Only gather if we don't have food
                .with_effect("has_food", StateValue::Int(3)) // Gather 3 food items
        );
        
        // Cut wood from trees (when need more wood)
        actions.push(
            GoapAction::new("cut_wood", 2.5)
                .with_precondition("has_energy", StateValue::Float(0.4))
                // No wood precondition - can cut wood anytime
                .with_effect("has_wood", StateValue::Int(10))
        );
        
        // Build house action
        actions.push(
            GoapAction::new("build_house", 8.0)
                .with_precondition("has_wood", StateValue::Int(15)) // Need 15 wood for house
                .with_precondition("has_stone", StateValue::Int(10)) // Need 10 stone for house
                .with_precondition("has_energy", StateValue::Float(0.5))
                .with_effect("has_house", StateValue::Bool(true))
                .with_effect("has_wood", StateValue::Int(0))
                .with_effect("has_stone", StateValue::Int(0))
        );
        
        // Get wood from stockpile
        actions.push(
            GoapAction::new("get_wood_from_stockpile", 1.0)
                .with_precondition("at_storage", StateValue::Bool(true))
                .with_precondition("has_wood", StateValue::Int(0))
                .with_effect("has_wood", StateValue::Int(15))
        );
        
        // Get stone from stockpile  
        actions.push(
            GoapAction::new("get_stone_from_stockpile", 1.0)
                .with_precondition("at_storage", StateValue::Bool(true))
                .with_precondition("has_stone", StateValue::Int(0))
                .with_effect("has_stone", StateValue::Int(10))
        );
        
        // Quarry stone (when need more stone)
        actions.push(
            GoapAction::new("quarry_stone", 3.0)
                .with_precondition("has_energy", StateValue::Float(0.4))
                // No stone precondition - can quarry stone anytime
                .with_effect("has_stone", StateValue::Int(10))
        );
        
        // Build structure action (generic for other buildings)
        actions.push(
            GoapAction::new("build_structure", 5.0)
                .with_precondition("has_wood", StateValue::Int(10))
                .with_precondition("has_stone", StateValue::Int(5))
                .with_precondition("has_energy", StateValue::Float(0.5))
                .with_effect("has_wood", StateValue::Int(0))
                .with_effect("has_stone", StateValue::Int(0))
        );
        
        Self { actions }
    }
}

/// Component to store an agent's current action plan
#[derive(Component, Clone, Debug)]
pub struct ActionPlan {
    pub actions: Vec<GoapAction>,
    pub current_index: usize,
}

impl ActionPlan {
    pub fn new(actions: Vec<GoapAction>) -> Self {
        Self {
            actions,
            current_index: 0,
        }
    }
    
    pub fn is_complete(&self) -> bool {
        self.current_index >= self.actions.len()
    }
    
    pub fn current_action(&self) -> Option<&GoapAction> {
        self.actions.get(self.current_index)
    }
    
    pub fn advance(&mut self) {
        self.current_index += 1;
    }
}