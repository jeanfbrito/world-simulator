use crate::components::*;
use bevy::prelude::*;
use std::collections::HashMap;

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
    IntDelta(i32), // For consumption/production changes
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
            (StateValue::Float(c), StateValue::Float(r)) => {
                // For needs like hunger/energy, use threshold checks based on action name
                // This is a bit hacky but works for our current action set
                // Better solution would be to have a separate threshold type
                match self.name.as_str() {
                    "eat_food" => *c >= *r, // Can eat when hunger is >= threshold (more hungry)
                    "rest" => *c <= *r,     // Can rest when energy is <= threshold (less energy)
                    "move_to_resource" | "move_to_storage" | "harvest_resource" | "cut_wood"
                    | "quarry_stone" | "build_house" | "build_structure" | "gather_food" => {
                        *c >= *r
                    } // Need energy >= threshold for actions
                    _ => *c >= *r,          // Default: current must be >= required
                }
            }
            (StateValue::Int(c), StateValue::Int(r)) => c >= r, // For resources, current must be >= required
            (StateValue::Int(c), StateValue::IntDelta(d)) => {
                // For consumption, check if we have enough
                if *d < 0 {
                    *c >= (-d) as u32
                } else {
                    true // Can always add resources
                }
            }
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
            match value {
                StateValue::IntDelta(delta) => {
                    // Handle resource changes (consumption/production)
                    if let Some(StateValue::Int(current)) = self.states.get(key) {
                        let new_amount = (*current as i32 + delta).max(0) as u32;
                        self.states.insert(key.clone(), StateValue::Int(new_amount));
                    } else if *delta > 0 {
                        // Starting from 0 if adding resources
                        self.states
                            .insert(key.clone(), StateValue::Int(*delta as u32));
                    }
                }
                StateValue::Int(amount) => {
                    // For absolute values (like rewards from gathering)
                    if key.starts_with("has_") {
                        // For resources, add to existing
                        if let Some(StateValue::Int(current)) = self.states.get(key) {
                            self.states
                                .insert(key.clone(), StateValue::Int(current + amount));
                        } else {
                            self.states.insert(key.clone(), value.clone());
                        }
                    } else {
                        // For other states, just set
                        self.states.insert(key.clone(), value.clone());
                    }
                }
                _ => {
                    // For Bool and Float, just set the value
                    self.states.insert(key.clone(), value.clone());
                }
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

        if let Ok((
            hungry,
            energy,
            working,
            wood,
            food,
            stone,
            at_resource,
            at_storage,
            inv_full,
            has_house,
        )) = query.get(entity)
        {
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

        // Move to resource action - DISABLED for survival-only mode
        // actions.push(
        //     GoapAction::new("move_to_resource", 1.0)
        //         .with_precondition("has_energy", StateValue::Float(0.2))
        //         .with_effect("at_resource", StateValue::Bool(true))
        //         .with_effect("at_storage", StateValue::Bool(false)),
        // );

        // Harvest resource action - DISABLED for survival-only mode
        // actions.push(
        //     GoapAction::new("harvest_resource", 2.0)
        //         .with_precondition("at_resource", StateValue::Bool(true))
        //         .with_precondition("inventory_full", StateValue::Bool(false))
        //         .with_effect("has_wood", StateValue::Int(5)),
        // );

        // Move to storage action - DISABLED for survival-only mode
        // actions.push(
        //     GoapAction::new("move_to_storage", 1.0)
        //         .with_precondition("has_energy", StateValue::Float(0.2))
        //         .with_effect("at_storage", StateValue::Bool(true))
        //         .with_effect("at_resource", StateValue::Bool(false)),
        // );

        // Store resources action (stores all carried resources) - DISABLED
        // actions.push(
        //     GoapAction::new("store_resources", 1.0)
        //         .with_precondition("at_storage", StateValue::Bool(true))
        //         .with_precondition("has_wood", StateValue::Int(1))
        //         .with_effect("has_wood", StateValue::IntDelta(-999))  // Clear all wood
        //         .with_effect("inventory_full", StateValue::Bool(false)),
        // );

        // Eat food action (consumes 1 food)
        // NOTE: Removed is_hungry precondition - if we have food and are hungry, we should eat!
        actions.push(
            GoapAction::new("eat_food", 0.5)
                .with_precondition("has_food", StateValue::Int(1))
                // No hunger precondition - the goal system will trigger this when needed
                .with_effect("is_hungry", StateValue::Float(0.0))
                .with_effect("has_energy", StateValue::Float(1.0))
                .with_effect("has_food", StateValue::IntDelta(-1)), // Consume 1 food
        );

        // Rest action (when energy is low)
        actions.push(
            GoapAction::new("rest", 0.1)
                .with_precondition("has_energy", StateValue::Float(0.5)) // Can rest when below 50% energy
                .with_effect("has_energy", StateValue::Float(1.0)) // Restores to full energy
                .with_effect("is_working", StateValue::Bool(false)),
        );

        // Gather food action (find berry bushes)
        actions.push(
            GoapAction::new("gather_food", 1.5)
                .with_precondition("has_energy", StateValue::Float(0.3)) // Need some energy to gather
                // Note: removed has_food precondition - peasants can gather more food anytime
                .with_effect("has_food", StateValue::Int(3)), // Gather 3 food items
        );

        // Cut wood from trees (when need more wood) - DISABLED for survival-only mode
        // actions.push(
        //     GoapAction::new("cut_wood", 2.5)
        //         .with_precondition("has_energy", StateValue::Float(0.4))
        //         // No wood precondition - can cut wood anytime
        //         .with_effect("has_wood", StateValue::Int(10)),
        // );

        // Build house action (consumes resources)
        actions.push(
            GoapAction::new("build_house", 8.0)
                .with_precondition("has_wood", StateValue::Int(15)) // Need 15 wood for house
                .with_precondition("has_stone", StateValue::Int(10)) // Need 10 stone for house
                .with_precondition("has_energy", StateValue::Float(0.5))
                .with_effect("has_house", StateValue::Bool(true))
                .with_effect("has_wood", StateValue::IntDelta(-15))  // Consume 15 wood
                .with_effect("has_stone", StateValue::IntDelta(-10)), // Consume 10 stone
        );

        // Get wood from stockpile
        actions.push(
            GoapAction::new("get_wood_from_stockpile", 1.0)
                .with_precondition("at_storage", StateValue::Bool(true))
                .with_precondition("has_wood", StateValue::Int(0))
                .with_effect("has_wood", StateValue::Int(15)),
        );

        // Get stone from stockpile
        actions.push(
            GoapAction::new("get_stone_from_stockpile", 1.0)
                .with_precondition("at_storage", StateValue::Bool(true))
                .with_precondition("has_stone", StateValue::Int(0))
                .with_effect("has_stone", StateValue::Int(10)),
        );

        // Quarry stone (when need more stone)
        actions.push(
            GoapAction::new("quarry_stone", 3.0)
                .with_precondition("has_energy", StateValue::Float(0.4))
                // No stone precondition - can quarry stone anytime
                .with_effect("has_stone", StateValue::Int(10)),
        );

        // Build structure action (generic for other buildings)
        actions.push(
            GoapAction::new("build_structure", 5.0)
                .with_precondition("has_wood", StateValue::Int(10))
                .with_precondition("has_stone", StateValue::Int(5))
                .with_precondition("has_energy", StateValue::Float(0.5))
                .with_effect("has_wood", StateValue::IntDelta(-10))  // Consume 10 wood
                .with_effect("has_stone", StateValue::IntDelta(-5)), // Consume 5 stone
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
