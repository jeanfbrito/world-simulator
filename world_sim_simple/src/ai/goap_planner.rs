use bevy::prelude::*;
use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Ordering;
use crate::ai::goap_actions::{GoapAction, WorldState, StateValue, ActionSet, ActionPlan};
use crate::debug::{DebugSystem, DebugLevel};

/// A node in the planning graph
#[derive(Clone, Debug)]
struct PlanNode {
    state: WorldState,
    action: Option<GoapAction>,
    parent: Option<Box<PlanNode>>,
    g_cost: f32, // Cost from start
    h_cost: f32, // Heuristic cost to goal
    f_cost: f32, // Total cost (g + h)
}

impl PlanNode {
    fn new(state: WorldState, action: Option<GoapAction>, parent: Option<Box<PlanNode>>, g_cost: f32, h_cost: f32) -> Self {
        Self {
            state,
            action,
            parent,
            g_cost,
            h_cost,
            f_cost: g_cost + h_cost,
        }
    }
}

impl PartialEq for PlanNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost == other.f_cost
    }
}

impl Eq for PlanNode {}

impl Ord for PlanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap
        other.f_cost.partial_cmp(&self.f_cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for PlanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Simple GOAP planner using A* search
pub struct GoapPlanner {
    max_depth: usize,
}

impl Default for GoapPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl GoapPlanner {
    pub fn new() -> Self {
        Self {
            max_depth: 10, // Maximum planning depth to prevent infinite loops
        }
    }
    
    /// Create a plan to achieve the goal state from the current state
    pub fn plan(
        &self,
        current_state: &WorldState,
        goal_state: &WorldState,
        action_set: &ActionSet,
        debug: &DebugSystem,
    ) -> Option<ActionPlan> {
        debug.log(DebugLevel::Debug, "GOAP", "Starting planning...");
        
        // Use A* search to find the optimal action sequence
        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        
        // Start with the current state
        let start_node = PlanNode::new(
            current_state.clone(),
            None,
            None,
            0.0,
            self.calculate_heuristic(current_state, goal_state),
        );
        
        open_set.push(start_node);
        
        let mut iterations = 0;
        
        while !open_set.is_empty() && iterations < 100 {
            iterations += 1;
            
            let current_node = open_set.pop().unwrap();
            
            // Check if we've reached the goal
            if self.is_goal_satisfied(&current_node.state, goal_state) {
                debug.log(
                    DebugLevel::Info,
                    "GOAP",
                    &format!("Plan found after {} iterations", iterations)
                );
                return Some(self.reconstruct_plan(current_node));
            }
            
            // Skip if we've already explored this state
            let state_hash = self.hash_state(&current_node.state);
            if closed_set.contains(&state_hash) {
                continue;
            }
            closed_set.insert(state_hash);
            
            // Don't go too deep
            if current_node.g_cost > self.max_depth as f32 {
                continue;
            }
            
            // Explore all valid actions from this state
            for action in action_set.get_valid_actions(&current_node.state) {
                let mut new_state = current_node.state.clone();
                new_state.apply_action(action);
                
                let new_g_cost = current_node.g_cost + action.cost;
                let new_h_cost = self.calculate_heuristic(&new_state, goal_state);
                
                let new_node = PlanNode::new(
                    new_state,
                    Some(action.clone()),
                    Some(Box::new(current_node.clone())),
                    new_g_cost,
                    new_h_cost,
                );
                
                open_set.push(new_node);
            }
        }
        
        debug.log(
            DebugLevel::Warning,
            "GOAP",
            &format!("No plan found after {} iterations", iterations)
        );
        None
    }
    
    /// Calculate heuristic cost (number of unsatisfied goals)
    fn calculate_heuristic(&self, current: &WorldState, goal: &WorldState) -> f32 {
        let mut unsatisfied = 0.0;
        
        for (key, goal_value) in &goal.states {
            if let Some(current_value) = current.get(key) {
                if !self.values_match(current_value, goal_value) {
                    unsatisfied += 1.0;
                }
            } else {
                unsatisfied += 1.0; // Missing state
            }
        }
        
        unsatisfied
    }
    
    /// Check if the goal state is satisfied
    fn is_goal_satisfied(&self, current: &WorldState, goal: &WorldState) -> bool {
        for (key, goal_value) in &goal.states {
            if let Some(current_value) = current.get(key) {
                if !self.values_match(current_value, goal_value) {
                    return false;
                }
            } else {
                return false; // Missing required state
            }
        }
        true
    }
    
    fn values_match(&self, current: &StateValue, goal: &StateValue) -> bool {
        match (current, goal) {
            (StateValue::Bool(c), StateValue::Bool(g)) => c == g,
            (StateValue::Float(c), StateValue::Float(g)) => (c - g).abs() < 0.1,
            (StateValue::Int(c), StateValue::Int(g)) => c >= g,
            _ => false,
        }
    }
    
    /// Reconstruct the action plan from the goal node
    fn reconstruct_plan(&self, mut node: PlanNode) -> ActionPlan {
        let mut actions = Vec::new();
        
        while let Some(action) = node.action {
            actions.push(action);
            if let Some(parent) = node.parent {
                node = *parent;
            } else {
                break;
            }
        }
        
        actions.reverse();
        ActionPlan::new(actions)
    }
    
    /// Create a simple hash of the world state for closed set checking
    fn hash_state(&self, state: &WorldState) -> String {
        let mut items: Vec<_> = state.states.iter().collect();
        items.sort_by_key(|(k, _)| k.as_str());
        
        let mut hash = String::new();
        for (key, value) in items {
            hash.push_str(key);
            hash.push('=');
            match value {
                StateValue::Bool(b) => hash.push_str(&b.to_string()),
                StateValue::Float(f) => hash.push_str(&format!("{:.1}", f)),
                StateValue::Int(i) => hash.push_str(&i.to_string()),
            }
            hash.push(';');
        }
        hash
    }
}

/// System to create plans for agents that need them
pub fn goap_planning_system(
    mut commands: Commands,
    mut agents: Query<(Entity, &WorldState, Option<&ActionPlan>), Without<ActionPlan>>,
    action_set: Res<ActionSet>,
    debug: Res<DebugSystem>,
) {
    let planner = GoapPlanner::new();
    
    for (entity, current_state, existing_plan) in agents.iter_mut() {
        // Skip if already has a plan
        if existing_plan.is_some() {
            continue;
        }
        
        // Define a simple goal: collect wood and store it
        let mut goal = WorldState::new();
        goal.set("has_wood", StateValue::Int(0)); // Want to have stored wood (empty inventory)
        goal.set("at_storage", StateValue::Bool(true)); // Want to be at storage
        
        // Create a plan
        if let Some(plan) = planner.plan(current_state, &goal, &action_set, &debug) {
            debug.log(
                DebugLevel::Info,
                "GOAP",
                &format!("Created plan with {} actions", plan.actions.len())
            );
            
            // Log the plan
            for (i, action) in plan.actions.iter().enumerate() {
                debug.log(
                    DebugLevel::Debug,
                    "GOAP",
                    &format!("  {}. {}", i + 1, action.name)
                );
            }
            
            commands.entity(entity).insert(plan);
        }
    }
}

/// System to execute GOAP plans
pub fn goap_execution_system(
    mut agents: Query<(&mut ActionPlan, &mut WorldState, Entity)>,
    debug: Res<DebugSystem>,
) {
    for (mut plan, mut world_state, entity) in agents.iter_mut() {
        if plan.is_complete() {
            continue;
        }
        
        if let Some(action) = plan.current_action() {
            // Check if action is still valid
            if action.is_valid(&world_state) {
                debug.log(
                    DebugLevel::Debug,
                    "GOAP",
                    &format!("Executing action: {}", action.name)
                );
                
                // Apply the action's effects
                world_state.apply_action(action);
                
                // Move to next action
                plan.advance();
            } else {
                debug.log(
                    DebugLevel::Warning,
                    "GOAP",
                    &format!("Action {} no longer valid, replanning needed", action.name)
                );
                // TODO: Trigger replanning
            }
        }
    }
}