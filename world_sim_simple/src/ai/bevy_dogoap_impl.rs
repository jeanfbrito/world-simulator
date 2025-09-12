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
#[derive(Component, Clone, DatumComponent)]
pub struct Hunger(pub f64);

#[derive(Component, Clone, DatumComponent)]
pub struct Energy(pub f64);

#[derive(Component, Clone, DatumComponent)]
pub struct FoodCount(pub f64);

// System to handle EatAction execution
pub fn handle_eat_action(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    mut query: Query<(Entity, &EatAction, &mut Hunger, &mut FoodCount)>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, _action, mut hunger, mut food) in query.iter_mut() {
        if food.0 > 0.0 {
            food.0 -= 1.0;
            hunger.0 = (hunger.0 + 10.0).min(100.0);  // Restore 10 hunger, max 100
            debug.log(DebugLevel::Info, "DOGOAP_ACTION", &format!("Worker ate food, hunger now: {:.1}", hunger.0));
            commands.entity(entity).remove::<EatAction>();
        }
    }
}

// System to handle RestAction execution
pub fn handle_rest_action(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    mut query: Query<(Entity, &RestAction, &mut Energy)>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, _action, mut energy) in query.iter_mut() {
        // Recover 5 energy per tick while resting
        energy.0 = (energy.0 + 5.0).min(100.0);
        
        if energy.0 >= 100.0 {
            debug.log(DebugLevel::Info, "DOGOAP_ACTION", "Worker fully rested");
            commands.entity(entity).remove::<RestAction>();
        }
    }
}

// System to handle GatherFoodAction execution
pub fn handle_gather_food_action(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    query: Query<(Entity, &GatherFoodAction)>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, _action) in query.iter() {
        // IMPORTANT: This action doesn't actually add food!
        // Food comes from the actual berry harvesting through the work system.
        // This action just signals that the peasant should gather food.
        // The actual gathering happens via:
        // 1. Peasant finds berry bush
        // 2. Starts WorkType::Gathering 
        // 3. Work system harvests berries and adds to inventory
        // 4. Inventory gets synced to FoodCount
        
        debug.log(DebugLevel::Info, "DOGOAP_ACTION", "GatherFoodAction triggered - peasant will look for berries");
        
        // Remove the action since it's just a trigger
        // The actual gathering will be handled by the work system
        commands.entity(entity).remove::<GatherFoodAction>();
    }
}

// System to set up GOAP planners for workers
pub fn setup_dogoap_planners(
    mut commands: Commands,
    query: Query<Entity, (With<crate::components::UnitTag>, Without<Planner>)>,
    debug: Res<DebugSystem>,
) {
    for entity in query.iter() {
        debug.log(DebugLevel::Info, "DOGOAP", "Setting up planner for worker");
        
        // Define the goal - not be hungry
        // In dogoap: 0 = starving, 100 = full
        // So we want hunger to be MORE than 70 (well fed)
        let goal_not_hungry = Goal::from_reqs(&[
            Hunger::is_more(70.0),
        ]);
        
        // Define actions with their preconditions and effects
        let eat_action = EatAction::new()
            .add_precondition(FoodCount::is_more(0.0))
            .add_mutator(Hunger::increase(10.0))  // Matches what handle_eat_action actually does
            .add_mutator(FoodCount::decrease(1.0));
        
        // IMPORTANT: gather_food doesn't directly increase FoodCount!
        // It triggers the peasant to find berries and start harvesting.
        // The actual food comes from the work system.
        // We can't use this in a plan that expects immediate food.
        // Instead, we'll remove it from the planner for now.
        
        let rest_action = RestAction::new()
            .add_mutator(Energy::set(100.0));
        
        // Create the planner with the macro
        let (mut planner, components) = create_planner!({
            actions: [
                (EatAction, eat_action),
                // GatherFoodAction removed - it doesn't have immediate effects
                // Food comes from the actual work system
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

// System to update hunger and energy over time (on simulation ticks)
pub fn update_needs_system(
    sim_state: Res<crate::SimulationState>,
    mut query: Query<(&mut Hunger, &mut Energy)>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (mut hunger, mut energy) in query.iter_mut() {
        // Hunger increases over time (lower is hungrier)
        // Decrease by 0.4 per tick (4 per second at 10 ticks per second)
        hunger.0 = (hunger.0 - 0.4).max(0.0);
        
        // Energy decreases over time  
        // Decrease by 0.5 per tick (5 per second at 10 ticks per second)
        energy.0 = (energy.0 - 0.5).max(0.0);
        
        // Log critical states
        if hunger.0 < 10.0 {
            debug.log(DebugLevel::Warn, "DOGOAP_STATE", "Worker is very hungry!");
        }
        if energy.0 < 10.0 {
            debug.log(DebugLevel::Warn, "DOGOAP_STATE", "Worker is exhausted!");
        }
    }
}

// System to sync inventory berries to FoodCount for GOAP
pub fn sync_inventory_to_food_count(
    sim_state: Res<crate::SimulationState>,
    mut query: Query<(&crate::components::UnitInventory, &mut FoodCount)>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (inventory, mut food_count) in query.iter_mut() {
        // Get the actual berry count from inventory
        let berry_count = inventory.get_amount(crate::resources::ResourceType::Berries) as f64;
        
        // Only log if there's a change
        if (food_count.0 - berry_count).abs() > 0.01 {
            debug.log(DebugLevel::Debug, "DOGOAP_SYNC", 
                &format!("Syncing FoodCount: {} -> {} from inventory", food_count.0, berry_count));
        }
        
        // Update FoodCount to match actual inventory
        food_count.0 = berry_count;
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
                sync_inventory_to_food_count,  // Sync inventory berries to FoodCount
                handle_eat_action,
                handle_rest_action,
                handle_gather_food_action,
                // Sync dogoap values to UnitNeedsV2 for display
                crate::ai::shared_state::sync_dogoap_to_unit_needs,
            ));
    }
}