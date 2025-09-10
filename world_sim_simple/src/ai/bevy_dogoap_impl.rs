use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};

// Simple bevy_dogoap implementation for demonstration
// This will handle basic needs like eating and resting

// Actions as Components
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
#[reflect(Component)]
pub struct EatAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
#[reflect(Component)]
pub struct RestAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
#[reflect(Component)]
pub struct GatherFoodAction;

// State components using DatumComponent
#[derive(Component, Clone)]
pub struct Hunger(pub f32);

#[derive(Component, Clone)]
pub struct Energy(pub f32);

#[derive(Component, Clone)]
pub struct FoodCount(pub f64);

// System to handle EatAction execution
pub fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<(Entity, &EatAction, &mut Hunger, &mut FoodCount)>,
    debug: Res<DebugSystem>,
) {
    for (entity, _action, mut hunger, mut food) in query.iter_mut() {
        if food.0 > 0.0 {
            food.0 -= 1.0;
            hunger.0 = 100.0;  // Full
            debug.log(DebugLevel::Info, "DOGOAP_ACTION", "Worker ate food");
            commands.entity(entity).remove::<EatAction>();
        }
    }
}

// System to handle RestAction execution
pub fn handle_rest_action(
    mut commands: Commands,
    mut query: Query<(Entity, &RestAction, &mut Energy)>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    for (entity, _action, mut energy) in query.iter_mut() {
        energy.0 = (energy.0 + time.delta_secs() * 20.0).min(100.0);
        
        if energy.0 >= 100.0 {
            debug.log(DebugLevel::Info, "DOGOAP_ACTION", "Worker fully rested");
            commands.entity(entity).remove::<RestAction>();
        }
    }
}

// System to handle GatherFoodAction execution
pub fn handle_gather_food_action(
    mut commands: Commands,
    mut query: Query<(Entity, &GatherFoodAction, &mut FoodCount)>,
    debug: Res<DebugSystem>,
) {
    for (entity, _action, mut food) in query.iter_mut() {
        food.0 += 3.0;
        debug.log(DebugLevel::Info, "DOGOAP_ACTION", &format!("Gathered food, total: {}", food.0));
        commands.entity(entity).remove::<GatherFoodAction>();
    }
}

// System to set up GOAP planners for workers
pub fn setup_dogoap_planners(
    mut commands: Commands,
    query: Query<Entity, (With<crate::components::WorkerTag>, Without<Planner>)>,
    debug: Res<DebugSystem>,
) {
    for entity in query.iter() {
        debug.log(DebugLevel::Info, "DOGOAP", "Setting up planner for worker");
        
        // Define the goal - not be hungry
        let goal_not_hungry = Goal::from_reqs(&[
            Hunger::is_less(30.0),
        ]);
        
        // Define actions with their preconditions and effects
        let eat_action = EatAction::new()
            .add_precondition(FoodCount::is_more(0))
            .add_mutator(Hunger::set(100.0))
            .add_mutator(FoodCount::decrease(1.0));
        
        let gather_food = GatherFoodAction::new()
            .add_precondition(Energy::is_more(20.0))
            .add_mutator(FoodCount::increase(3.0));
        
        let rest_action = RestAction::new()
            .add_mutator(Energy::set(100.0));
        
        // Create the planner with the macro
        let (mut planner, components) = create_planner!({
            actions: [
                (EatAction, eat_action),
                (GatherFoodAction, gather_food),
                (RestAction, rest_action),
            ],
            state: [
                Hunger(50.0),
                Energy(75.0),
                FoodCount(2.0),
            ],
            goals: [goal_not_hungry],
        });
        
        // Configure the planner
        planner.always_plan = true;
        planner.remove_goal_on_no_plan_found = false;
        planner.current_goal = Some(goal_not_hungry.clone());
        
        // Add planner and components to the entity
        commands.entity(entity)
            .insert(planner)
            .insert(components);
    }
}

// System to update hunger and energy over time
pub fn update_needs_system(
    mut query: Query<(&mut Hunger, &mut Energy)>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    let dt = time.delta_secs();
    
    for (mut hunger, mut energy) in query.iter_mut() {
        // Hunger increases over time (lower is hungrier)
        hunger.0 = (hunger.0 - dt * 5.0).max(0.0);
        
        // Energy decreases over time
        energy.0 = (energy.0 - dt * 3.0).max(0.0);
        
        // Log critical states
        if hunger.0 < 10.0 {
            debug.log(DebugLevel::Warn, "DOGOAP_STATE", "Worker is very hungry!");
        }
        if energy.0 < 10.0 {
            debug.log(DebugLevel::Warn, "DOGOAP_STATE", "Worker is exhausted!");
        }
    }
}

// Plugin to register all bevy_dogoap systems
pub struct BevyDogoapPlugin;

impl Plugin for BevyDogoapPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add the main dogoap plugin
            .add_plugins(DogoapPlugin)
            // Register our components
            .register_type::<EatAction>()
            .register_type::<RestAction>()
            .register_type::<GatherFoodAction>()
            // Add our systems
            .add_systems(Update, (
                setup_dogoap_planners,
                update_needs_system,
                handle_eat_action,
                handle_rest_action,
                handle_gather_food_action,
            ));
    }
}