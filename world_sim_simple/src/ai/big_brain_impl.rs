use bevy::prelude::*;
use big_brain::prelude::*;
use crate::ai::shared_state::*;
use crate::debug::{DebugSystem, DebugLevel};

// Scorers - evaluate the world and decide what's important

#[derive(Debug, Clone, Component, ScorerBuilder)]
pub struct HungerScorer;

#[derive(Debug, Clone, Component, ScorerBuilder)]
pub struct EnergyScorer;

#[derive(Debug, Clone, Component, ScorerBuilder)]
pub struct ResourceScorer;

// Actions - what the AI can do

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct EatQuickAction;

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct RestQuickAction;

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct PanicGatherAction;

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
            let food_score = if food.0 < 2 { 0.8 } else { 0.0 };
            let wood_score = if wood.0 < 5 { 0.3 } else { 0.0 };
            let stone_score = if stone.0 < 3 { 0.2 } else { 0.0 };
            
            score.set((food_score + wood_score + stone_score).min(1.0));
        }
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

// Create a Thinker for reactive behaviors
pub fn create_reactive_thinker() -> ThinkerBuilder {
    Thinker::build()
        .picker(FirstToScore { threshold: 0.6 })
        // Critical hunger response
        .when(HungerScorer, EatQuickAction)
        // Critical energy response
        .when(EnergyScorer, RestQuickAction)
        // Resource shortage response
        .when(ResourceScorer, PanicGatherAction)
}

// System to add Thinkers to workers in reactive mode
pub fn setup_reactive_thinkers_system(
    mut commands: Commands,
    query: Query<(Entity, &AIMode), (Without<Thinker>, With<AIState>)>,
    debug: Res<DebugSystem>,
) {
    for (entity, ai_mode) in query.iter() {
        if *ai_mode == AIMode::Reactive {
            debug.log(DebugLevel::Info, "BIG_BRAIN", "Setting up reactive thinker");
            
            let thinker = create_reactive_thinker();
            commands.entity(entity).insert(thinker);
        }
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
            ).in_set(BigBrainSet::Scorers))
            .add_systems(PreUpdate, (
                eat_quick_action_system,
                rest_quick_action_system,
                panic_gather_action_system,
            ).in_set(BigBrainSet::Actions))
            .add_systems(Update, setup_reactive_thinkers_system);
    }
}