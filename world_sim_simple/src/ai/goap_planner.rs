use crate::ai::goap_actions::{ActionPlan, ActionSet, GoapAction, StateValue, WorldState};
use crate::debug::{DebugLevel, DebugSystem};
use bevy::prelude::*;
use colored::Colorize;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

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
    fn new(
        state: WorldState,
        action: Option<GoapAction>,
        parent: Option<Box<PlanNode>>,
        g_cost: f32,
        h_cost: f32,
    ) -> Self {
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
        other
            .f_cost
            .partial_cmp(&self.f_cost)
            .unwrap_or(Ordering::Equal)
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
                    DebugLevel::Debug,
                    "GOAP_PLANNER",
                    &format!(
                        "✓ Solution found after {} iterations (depth: {:.0})",
                        iterations, current_node.g_cost
                    ),
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
            let valid_actions = action_set.get_valid_actions(&current_node.state);

            // Debug: Show all actions and why they might not be valid
            if iterations == 1 {
                debug.log(DebugLevel::Info, "GOAP_DEBUG", "Checking action validity:");
                for action in &action_set.actions {
                    let is_valid = action.is_valid(&current_node.state);
                    
                    // Extra debug for harvest_resource
                    if action.name == "harvest_resource" && !is_valid {
                        let at_resource = current_node.state.get("at_resource")
                            .map(|v| format!("{:?}", v))
                            .unwrap_or_else(|| "missing".to_string());
                        let inventory_full = current_node.state.get("inventory_full")
                            .map(|v| format!("{:?}", v))
                            .unwrap_or_else(|| "missing".to_string());
                        debug.log(
                            DebugLevel::Info,
                            "GOAP_DEBUG",
                            &format!("    harvest_resource failed: at_resource={}, inventory_full={}", 
                                at_resource, inventory_full),
                        );
                    }
                    
                    debug.log(
                        DebugLevel::Info,
                        "GOAP_DEBUG",
                        &format!(
                            "  {} {}: {}",
                            if is_valid { "✓" } else { "✗" },
                            action.name,
                            if !is_valid {
                                let mut missing = Vec::new();
                                for (key, req) in &action.preconditions {
                                    if let Some(_curr) = current_node.state.get(key) {
                                        // Already handled in check_condition
                                    } else {
                                        missing.push(format!("{} missing", key));
                                    }
                                }
                                if missing.is_empty() {
                                    "preconditions not met"
                                } else {
                                    &missing.join(", ")
                                }
                            } else {
                                "valid"
                            }
                        ),
                    );
                }
            }

            debug.log(
                DebugLevel::Debug,
                "GOAP_ACTIONS",
                &format!(
                    "Valid actions available: {:?}",
                    valid_actions.iter().map(|a| &a.name).collect::<Vec<_>>()
                ),
            );

            for action in valid_actions {
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
            DebugLevel::Info,
            "GOAP",
            &format!("No plan found after {} iterations", iterations),
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
            (StateValue::IntDelta(_), _) | (_, StateValue::IntDelta(_)) => {
                // IntDelta is only for effects, not goals
                false
            }
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
                StateValue::IntDelta(d) => hash.push_str(&format!("Δ{}", d)),
            }
            hash.push(';');
        }
        hash
    }
}

/// System to create plans for agents that need them
pub fn goap_planning_system(
    mut commands: Commands,
    mut agents: Query<(Entity, &WorldState, Option<&ActionPlan>), With<crate::components::UnitTag>>,
    action_set: Res<ActionSet>,
    debug: Res<DebugSystem>,
) {
    let agent_count = agents.iter().count();
    debug.log(
        DebugLevel::Info,
        "GOAP_PLANNING",
        &format!(
            "🤖 GOAP planning system running - found {} agents",
            agent_count
        ),
    );

    let planner = GoapPlanner::new();

    for (entity, current_state, existing_plan) in agents.iter_mut() {
        // Skip if already has an active (non-completed) plan
        if let Some(plan) = existing_plan {
            if !plan.is_complete() {
                continue;
            }
        }

        debug.log(
            DebugLevel::Info,
            "GOAP_PLANNING",
            &format!("Creating plan for entity {:?}", entity),
        );

        // Create dynamic goals based on current needs
        let mut goal = WorldState::new();

        // Check if worker has a house (priority 1 - basic shelter)
        // Don't set has_house as immediate goal - let it emerge from resource gathering
        // This is handled later in the resource gathering section

        // Priority 1: CRITICAL - If completely exhausted, must rest first (can't do anything without energy)
        if let Some(StateValue::Float(energy)) = current_state.get("has_energy") {
            if *energy <= 0.0 {
                goal.set("has_energy", StateValue::Float(1.0)); // Want full energy
                debug.log(
                    DebugLevel::Info,
                    "GOAP_GOAL",
                    &format!(
                        "⚠️ CRITICAL: {} exhausted! Must rest immediately (energy: {:.2})",
                        entity, energy
                    ),
                );
            }
        }

        // Priority 2: If starving AND have food, eat
        if goal.states.is_empty() {
            if let Some(StateValue::Float(hunger)) = current_state.get("is_hungry") {
                if *hunger > 0.3 {
                    // Peasants need food more urgently
                    if let Some(StateValue::Int(food)) = current_state.get("has_food") {
                        if *food > 0 {
                            goal.set("is_hungry", StateValue::Float(0.0)); // Want to not be hungry
                            debug.log(
                                DebugLevel::Info,
                                "GOAP_GOAL",
                                &format!(
                                    "🍞 URGENT: Needs food! (hunger: {:.2}, has {} food)",
                                    hunger, food
                                ),
                            );
                        } else {
                            // Need to gather food first
                            goal.set("has_food", StateValue::Int(3)); // Get some food
                            debug.log(
                                DebugLevel::Info,
                                "GOAP_GOAL",
                                &format!(
                                    "🍞 Hungry but no food! Need to gather (hunger: {:.2})",
                                    hunger
                                ),
                            );
                        }
                    }
                }
            }
        }

        // Priority 3: If low energy, rest
        if goal.states.is_empty() {
            if let Some(StateValue::Float(energy)) = current_state.get("has_energy") {
                if *energy < 0.3 {
                    goal.set("has_energy", StateValue::Float(1.0)); // Want full energy
                    debug.log(
                        DebugLevel::Info,
                        "GOAP_GOAL",
                        &format!("😴 Low energy, need rest (energy: {:.2})", energy),
                    );
                }
            }
        }

        // If no urgent needs, work on gathering resources for building a house
        if goal.states.is_empty() {
            // If we don't have a house yet, gather resources to build one
            if let Some(StateValue::Bool(has_house)) = current_state.get("has_house") {
                if !has_house {
                    // Check current resources and set goals for what we're missing
                    let current_wood = current_state
                        .get("has_wood")
                        .and_then(|v| {
                            if let StateValue::Int(n) = v {
                                Some(*n)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(0);
                    let current_stone = current_state
                        .get("has_stone")
                        .and_then(|v| {
                            if let StateValue::Int(n) = v {
                                Some(*n)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(0);

                    // Be more incremental - start with smaller goals
                    if current_wood < 5 {
                        goal.set("has_wood", StateValue::Int(5)); // Get some wood first
                        debug.log(
                            DebugLevel::Info,
                            "GOAP_GOAL",
                            &format!("Goal: Get wood ({}/5)", current_wood),
                        );
                    } else if current_stone < 5 {
                        goal.set("has_stone", StateValue::Int(5)); // Get some stone first
                        debug.log(
                            DebugLevel::Info,
                            "GOAP_GOAL",
                            &format!("Goal: Get stone ({}/5)", current_stone),
                        );
                    } else if current_wood < 15 {
                        goal.set("has_wood", StateValue::Int(15)); // Get enough wood for house
                        debug.log(
                            DebugLevel::Info,
                            "GOAP_GOAL",
                            &format!("Goal: Get more wood ({}/15)", current_wood),
                        );
                    } else if current_stone < 10 {
                        goal.set("has_stone", StateValue::Int(10)); // Get enough stone for house
                        debug.log(
                            DebugLevel::Info,
                            "GOAP_GOAL",
                            &format!("Goal: Get more stone ({}/10)", current_stone),
                        );
                    } else {
                        // We have all resources, now build the house
                        goal.set("has_house", StateValue::Bool(true));
                        debug.log(
                            DebugLevel::Info,
                            "GOAP_GOAL",
                            "Goal: Build house (have all resources)",
                        );
                    }
                }
            }
        }

        // Default idle goal - just rest
        if goal.states.is_empty() {
            goal.set("has_energy", StateValue::Float(1.0)); // Want full energy
            debug.log(
                DebugLevel::Info,
                "GOAP_GOAL",
                "Goal: Rest and maintain energy",
            );
        }

        // Log current state for debugging
        debug.log(
            DebugLevel::Info,
            "GOAP_DECISION",
            &format!(
                "Agent analyzing state - Hungry: {:?}, Energy: {:?}, Has Wood: {:?}",
                current_state.get("is_hungry"),
                current_state.get("has_energy"),
                current_state.get("has_wood")
            ),
        );

        // Create a plan
        if let Some(plan) = planner.plan(current_state, &goal, &action_set, &debug) {
            let action_names: Vec<String> = plan.actions.iter().map(|a| a.name.clone()).collect();
            let total_cost: f32 = plan.actions.iter().map(|a| a.cost).sum();
            debug.log(
                DebugLevel::Info,
                "GOAP_PLAN",
                &format!(
                    "✓ Decision made: {} (Total cost: {:.1})",
                    action_names.join(" → "),
                    total_cost
                ),
            );

            // Log the plan details
            for (i, action) in plan.actions.iter().enumerate() {
                debug.log(
                    DebugLevel::Debug,
                    "GOAP_PLAN",
                    &format!(
                        "  Step {}. {} (cost: {:.1})",
                        i + 1,
                        action.name,
                        action.cost
                    ),
                );
            }

            commands.entity(entity).insert(plan);
        }
    }
}

/// System to execute GOAP plans
pub fn goap_execution_system(
    mut agents: Query<(
        &mut ActionPlan,
        &mut WorldState,
        &mut crate::components::UnitNeeds,
        &mut crate::components::UnitInventory,
        Entity,
        &crate::components::NameComponent,
    )>,
    debug: Res<DebugSystem>,
    sim_state: Res<crate::SimulationState>,
) {
    // Only execute on simulation ticks
    if !sim_state.just_ticked {
        return;
    }

    for (mut plan, mut world_state, mut needs, mut inventory, entity, name) in agents.iter_mut() {
        // Clear completed plans to trigger replanning
        if plan.is_complete() {
            debug.log(
                DebugLevel::Info,
                "GOAP",
                &format!("{} completed plan, clearing for replanning", name.name),
            );
            *plan = ActionPlan::new(Vec::new());
            continue;
        }

        if let Some(action) = plan.current_action() {
            // Check if action is still valid
            if action.is_valid(&world_state) {
                debug.log(
                    DebugLevel::Info,
                    "GOAP",
                    &format!("{} executing: {}", name.name, action.name),
                );

                // Apply effects to actual components based on action
                let should_advance = match action.name.as_str() {
                    "eat_food" => {
                        // Actually consume food from inventory
                        if inventory.remove_item(crate::resources::ResourceType::Berries, 1) {
                            needs.hunger = 0.0; // Reset hunger
                            needs.energy = 1.0; // Restore energy
                            // Update the world state to reflect the change
                            world_state.set("is_hungry", StateValue::Float(0.0));
                            world_state.set("has_energy", StateValue::Float(1.0));
                            // Update food count in world state
                            let food_count = inventory.get_amount(crate::resources::ResourceType::Berries);
                            world_state.set("has_food", StateValue::Int(food_count));
                            println!("🍎 {} ate food! Hunger reset to 0", name.name.green());
                        }
                        true // Instant action, advance immediately
                    }
                    "rest" => {
                        needs.energy = 1.0; // Restore energy
                        // Update the world state to reflect the change
                        world_state.set("has_energy", StateValue::Float(1.0));
                        println!("😴 {} rested! Energy restored", name.name.blue());
                        true // Instant action, advance immediately
                    }
                    "gather_food" | "cut_wood" | "gather_stone" | "harvest_resource" => {
                        // These require work to complete, don't advance yet
                        // The work system will advance when work completes
                        false
                    }
                    _ => {
                        // Other actions handled by task system
                        false // Assume they need work
                    }
                };

                // Only apply effects and advance for instant actions
                // Actions that require work (gather_food, cut_wood, etc.) 
                // should have their effects applied by the task execution system
                if should_advance {
                    // Apply the action's effects to world state immediately
                    world_state.apply_action(action);
                    plan.advance();
                }
                // For non-instant actions, effects will be applied when work completes
            } else {
                debug.log(
                    DebugLevel::Info,
                    "GOAP",
                    &format!(
                        "Action {} no longer valid for {}, replanning needed",
                        action.name, name.name
                    ),
                );
                // Clear the plan to trigger replanning
                *plan = ActionPlan::new(Vec::new());
            }
        }
    }
}
