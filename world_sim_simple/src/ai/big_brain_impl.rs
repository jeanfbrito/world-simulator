use bevy::prelude::*;
use big_brain::prelude::*;
use crate::ai::shared_state::*;
use crate::debug::{DebugSystem, DebugLevel};
use bevy_dogoap::prelude::Planner;

// Define AIState as a simple container for hunger/energy values
#[derive(Component, Clone, Debug)]
pub struct AIState {
    pub hunger: f32,
    pub energy: f32,
}

// Scorers - evaluate the world and decide what's important

#[derive(Debug, Clone, Component, ScorerBuilder)]
pub struct HungerScorer;

#[derive(Debug, Clone, Component, ScorerBuilder)]
pub struct EnergyScorer;

#[derive(Debug, Clone, Component, ScorerBuilder)]
pub struct ResourceScorer;

// PHASE 2: IdleScorer for baseline GOAP planning
#[derive(Debug, Clone, Component, ScorerBuilder)]
pub struct IdleScorer;

// Actions - what the AI can do

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct EatQuickAction;

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct RestQuickAction;

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct PanicGatherAction;

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct MoveToTargetAction {
    pub target: Option<crate::components::GridPosition>,
}

// PHASE 2: StartPlanningAction Bridge to connect big-brain to GOAP
#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct StartPlanningAction;

// Scorer systems - evaluate current state

pub fn hunger_scorer_system(
    stats_query: Query<&AIState>,
    mut query: Query<(&Actor, &mut Score), With<HungerScorer>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(stats) = stats_query.get(*actor) {
            // Critical hunger levels get high scores
            let hunger_score = 1.0 - stats.hunger;
            score.set(hunger_score);
        }
    }
}

pub fn energy_scorer_system(
    stats_query: Query<&AIState>,
    mut query: Query<(&Actor, &mut Score), With<EnergyScorer>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(stats) = stats_query.get(*actor) {
            // Critical energy levels get high scores
            let energy_score = 1.0 - stats.energy;
            score.set(energy_score);
        }
    }
}

pub fn resource_scorer_system(
    resource_query: Query<(&HasFood, &HasWood, &HasStone)>,
    mut query: Query<(&Actor, &mut Score), With<ResourceScorer>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok((food, wood, stone)) = resource_query.get(*actor) {
            // Score based on resource scarcity
            let food_score: f32 = if food.0 < 2 { 0.8 } else { 0.0 };
            let wood_score: f32 = if wood.0 < 5 { 0.3 } else { 0.0 };
            let stone_score: f32 = if stone.0 < 3 { 0.2 } else { 0.0 };

            score.set((food_score + wood_score + stone_score).min(1.0));
        }
    }
}

// PHASE 2: IdleScorer system for baseline GOAP planning
pub fn idle_scorer_system(
    mut query: Query<(&Actor, &mut Score), With<IdleScorer>>,
) {
    for (_, mut score) in query.iter_mut() {
        // Always have a baseline score to trigger planning when not in crisis
        score.set(0.1);
    }
}

// Action systems - execute the chosen actions

pub fn eat_quick_action_system(
    mut stats_query: Query<(&mut AIState, &mut HasFood)>,
    mut query: Query<(&Actor, &mut ActionState), With<EatQuickAction>>,
    debug: Res<DebugSystem>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        if let Ok((mut stats, mut food)) = stats_query.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    if food.0 > 0 {
                        food.0 -= 1;
                        stats.hunger = (stats.hunger + 0.5).min(1.0);
                        debug.log(DebugLevel::Info, "BIG_BRAIN", "Quick eat!");
                        *state = ActionState::Success;
                    } else {
                        *state = ActionState::Failure;
                    }
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

pub fn rest_quick_action_system(
    mut stats_query: Query<&mut AIState>,
    mut query: Query<(&Actor, &mut ActionState), With<RestQuickAction>>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        if let Ok(mut stats) = stats_query.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    stats.energy = (stats.energy + time.delta_secs() * 0.5).min(1.0);
                    
                    if stats.energy >= 0.6 {
                        debug.log(DebugLevel::Info, "BIG_BRAIN", "Quick rest complete");
                        *state = ActionState::Success;
                    }
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

pub fn panic_gather_action_system(
    mut resource_query: Query<&mut HasFood>,
    mut query: Query<(&Actor, &mut ActionState), With<PanicGatherAction>>,
    debug: Res<DebugSystem>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        if let Ok(mut food) = resource_query.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    // Desperately gather any nearby food
                    food.0 += 2;
                    debug.log(DebugLevel::Warn, "BIG_BRAIN", "Panic gathering food!");
                    *state = ActionState::Success;
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

// Movement action system - executes movement to targets
pub fn move_to_target_action_system(
    mut movement_query: Query<(
        &mut crate::components::GridMovement,
        &crate::components::GridPosition,
        &mut crate::components::UnitMind,
    )>,
    mut query: Query<(&Actor, &mut ActionState, &MoveToTargetAction)>,
    debug: Res<DebugSystem>,
) {
    for (Actor(actor), mut state, action) in query.iter_mut() {
        if let Ok((mut movement, grid_pos, mut mind)) = movement_query.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    if let Some(ref target) = action.target {
                        // Check if we're already at target
                        if grid_pos.x == target.x && grid_pos.y == target.y {
                            *state = ActionState::Success;
                            debug.log(DebugLevel::Info, "BIG_BRAIN", "Already at target");
                        } else {
                            // Set movement target
                            movement.set_target(target.clone());
                            *mind = crate::components::UnitMind::GoingThere {
                                destination: format!("({}, {})", target.x, target.y),
                            };
                            *state = ActionState::Executing;
                            debug.log(DebugLevel::Info, "BIG_BRAIN", 
                                &format!("Moving to ({}, {})", target.x, target.y));
                        }
                    } else {
                        *state = ActionState::Failure;
                    }
                }
                ActionState::Executing => {
                    // Check if movement is complete
                    if !movement.is_moving {
                        *state = ActionState::Success;
                        *mind = crate::components::UnitMind::Idle;
                        debug.log(DebugLevel::Info, "BIG_BRAIN", "Reached target");
                    }
                }
                ActionState::Cancelled => {
                    movement.stop();
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

// PHASE 2: StartPlanningAction system - bridge to GOAP
pub fn start_planning_action_system(
    mut query: Query<(&Actor, &mut ActionState), With<StartPlanningAction>>,
    mut planner_query: Query<&mut Planner>,
    debug: Res<DebugSystem>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        match *state {
            ActionState::Requested => {
                if let Ok(mut planner) = planner_query.get_mut(*actor) {
                    // Force replanning by clearing current plan
                    planner.current_plan.clear();
                    debug.log(DebugLevel::Info, "BIG_BRAIN", "Triggered GOAP replanning");
                    *state = ActionState::Success;
                } else {
                    // No planner found, fail
                    *state = ActionState::Failure;
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

// PHASE 3: Create a Thinker for hybrid AI (big-brain + GOAP)
pub fn create_reactive_thinker() -> ThinkerBuilder {
    Thinker::build()
        .picker(FirstToScore { threshold: 0.7 })  // High threshold for emergencies
        // Critical responses (score > 0.7)
        .when(HungerScorer, EatQuickAction)
        .when(EnergyScorer, RestQuickAction)
        .when(ResourceScorer, PanicGatherAction)
        // Fallback to GOAP planning (always scores 0.1)
        .when(IdleScorer, StartPlanningAction)
}

// PHASE 3: System to add Thinkers to all units with GOAP planners
pub fn setup_reactive_thinkers_system(
    mut commands: Commands,
    query: Query<Entity, (With<Planner>, Without<Thinker>)>,  // Changed condition
    debug: Res<DebugSystem>,
) {
    for entity in query.iter() {
        debug.log(DebugLevel::Info, "BIG_BRAIN", "Setting up hybrid AI thinker");

        let thinker = create_reactive_thinker();
        commands.entity(entity).insert((
            thinker,
            AIState { hunger: 0.5, energy: 0.5 },  // Initial state
        ));
    }
}

// Plugin to register all big-brain systems
pub struct BigBrainAIPlugin;

impl Plugin for BigBrainAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BigBrainPlugin::new(PreUpdate))
            .add_systems(PreUpdate, (
                hunger_scorer_system,
                energy_scorer_system,
                resource_scorer_system,
                idle_scorer_system,  // PHASE 2: Add IdleScorer
            ).in_set(BigBrainSet::Scorers))
            .add_systems(PreUpdate, (
                eat_quick_action_system,
                rest_quick_action_system,
                panic_gather_action_system,
                move_to_target_action_system,
                start_planning_action_system,  // PHASE 2: Add StartPlanningAction
            ).in_set(BigBrainSet::Actions))
            .add_systems(Update, setup_reactive_thinkers_system);
    }
}