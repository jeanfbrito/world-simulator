use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};
use crate::components::{WorkProgress, WorkType, ResourceWork};

// Simple bevy_dogoap implementation for demonstration
// This will handle basic needs like eating and resting

// Actions as Components
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
#[reflect(Component)]
pub struct EatAction;

// RestAction removed - energy now recovers when idle

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
#[reflect(Component)]
pub struct GatherFoodAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
#[reflect(Component)]
pub struct WanderAction;

#[derive(Component, Clone, Reflect, ActionComponent)]
#[reflect(Component)]
pub struct MoveToResourceAction {
    pub target: Option<Entity>,
    pub resource_type: crate::resources::ResourceType,
}

impl Default for MoveToResourceAction {
    fn default() -> Self {
        Self {
            target: None,
            resource_type: crate::resources::ResourceType::Berries,
        }
    }
}

// State components using DatumComponent
#[derive(Component, Clone, DatumComponent)]
pub struct Satiety(pub f64);

#[derive(Component, Clone, DatumComponent)]
pub struct Energy(pub f64);

#[derive(Component, Clone, DatumComponent)]
pub struct FoodCount(pub f64);

#[derive(Component, Clone, DatumComponent)]
pub struct NearBerryBush(pub f64);  // 1.0 if near a berry bush, 0.0 otherwise

// System to handle EatAction execution
pub fn handle_eat_action(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    mut query: Query<(Entity, &EatAction, &mut Satiety, &mut FoodCount)>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, _action, mut hunger, mut food) in query.iter_mut() {
        if food.0 > 0.0 {
            food.0 -= 1.0;
            hunger.0 = (hunger.0 + 20.0).min(100.0);  // Restore 20 hunger (matches mutator)
            debug.log(DebugLevel::Info, "DOGOAP_ACTION", &format!("Worker ate food, hunger now: {:.1}, food left: {:.0}", hunger.0, food.0));
            commands.entity(entity).remove::<EatAction>();
        }
    }
}

// RestAction handler removed - energy now recovers automatically when idle

// System to handle GatherFoodAction execution
pub fn handle_gather_food_action(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    mut query: Query<(
        Entity, 
        &GatherFoodAction, 
        &mut FoodCount, 
        &mut Energy, 
        &NearBerryBush,
        Option<&mut crate::components::WorkProgress>,
        &crate::components::grid_position::GridPosition,
    )>,
    resource_query: Query<(Entity, &crate::components::grid_position::GridPosition, &crate::components::resource::ResourceNode)>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, _action, mut food, mut energy, near_bush, work_progress_opt, unit_pos) in query.iter_mut() {
        // Only gather if actually near a berry bush
        if near_bush.0 > 0.0 {
            // Find the nearest berry bush entity
            let mut closest_bush = None;
            let mut closest_distance = u32::MAX;
            
            for (resource_entity, resource_pos, resource_node) in resource_query.iter() {
                if resource_node.resource_type == crate::resources::ResourceType::Berries {
                    let distance = unit_pos.distance_to(resource_pos);
                    if distance <= 1 && distance < closest_distance && resource_node.can_harvest() {
                        closest_bush = Some(resource_entity);
                        closest_distance = distance;
                    }
                }
            }
            
            if let Some(bush_entity) = closest_bush {
                // Handle WorkProgress - either use existing or add new component
                if let Some(mut work_progress) = work_progress_opt {
                    // Only start work if not already working
                    if !work_progress.is_working {
                        // Start the actual work to gather berries
                        work_progress.start_work(
                            crate::components::WorkType::Gathering(crate::components::ResourceWork {
                                resource_type: crate::resources::ResourceType::Berries,
                                amount: 3,
                                tool_bonus: 1.0,
                            }),
                            30, // Takes 30 ticks (3 seconds) to gather
                            Some(bush_entity),
                        );
                        
                        debug.log(DebugLevel::Info, "GATHER", 
                            &format!("Started gathering work at ({}, {})", unit_pos.x, unit_pos.y));
                        
                        // Apply immediate energy cost
                        energy.0 = (energy.0 - 5.0).max(0.0);
                    }
                } else {
                    // No WorkProgress component, need to add it first
                    debug.log(DebugLevel::Info, "GATHER", 
                        &format!("Adding WorkProgress component to entity {:?}", entity));
                    
                    // Create and start work in new WorkProgress component
                    let mut new_work_progress = crate::components::WorkProgress::new();
                    new_work_progress.start_work(
                        crate::components::WorkType::Gathering(crate::components::ResourceWork {
                            resource_type: crate::resources::ResourceType::Berries,
                            amount: 3,
                            tool_bonus: 1.0,
                        }),
                        30, // Takes 30 ticks (3 seconds) to gather
                        Some(bush_entity),
                    );
                    
                    // Add WorkProgress and WorkSpeed components
                    commands.entity(entity).insert((
                        new_work_progress,
                        crate::components::WorkSpeed::default(),
                        crate::components::WorkQueue::new(10),
                    ));
                    
                    // Apply immediate energy cost
                    energy.0 = (energy.0 - 5.0).max(0.0);
                }
                
                debug.log(DebugLevel::Info, "DOGOAP_ACTION", 
                    &format!("Worker starting to gather berries from bush"));
            }
        }
        
        // Remove the action after setting up work
        commands.entity(entity).remove::<GatherFoodAction>();
    }
}

// System to handle WanderAction - makes peasants move randomly
pub fn handle_wander_action(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    mut query: Query<(
        Entity, 
        &WanderAction, 
        &crate::components::grid_position::GridPosition,
        &mut crate::components::grid_position::GridMovement,
        &mut crate::components::UnitMind
    )>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks  
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, _action, grid_pos, mut movement, mut mind) in query.iter_mut() {
        // Pick a random nearby location (5-10 tiles away)
        let range = 8;
        let new_x = (grid_pos.x as i32 + (rand::random::<i32>() % (range * 2)) - range).max(0) as u32;
        let new_y = (grid_pos.y as i32 + (rand::random::<i32>() % (range * 2)) - range).max(0) as u32;
        
        // Clamp to map bounds (assuming 64x64 map)
        let new_x = new_x.min(63);
        let new_y = new_y.min(63);
        
        // Set the movement target with proper path generation
        let target = crate::components::grid_position::GridPosition::new(new_x, new_y);
        movement.set_target_from(grid_pos, target);
        
        // Set the mind state
        *mind = crate::components::UnitMind::Wandering;
        
        debug.log(DebugLevel::Info, "DOGOAP_ACTION", 
            &format!("Worker wandering to ({}, {})", new_x, new_y));
        
        // Remove the action after setting movement
        commands.entity(entity).remove::<WanderAction>();
    }
}

// System to handle MoveToResourceAction - moves to a specific resource
pub fn handle_move_to_resource_action(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    mut query: Query<(
        Entity, 
        &MoveToResourceAction,
        &crate::components::grid_position::GridPosition,
        &mut crate::components::grid_position::GridMovement,
        &mut crate::components::UnitMind
    )>,
    resource_query: Query<(&crate::components::grid_position::GridPosition, &crate::components::resource::ResourceNode)>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, action, grid_pos, mut movement, mut mind) in query.iter_mut() {
        if let Some(target) = action.target {
            if let Ok((target_pos, _)) = resource_query.get(target) {
                // Move to the resource location with proper path generation
                movement.set_target_from(grid_pos, target_pos.clone());
                
                *mind = crate::components::UnitMind::GoingThere {
                    destination: format!("Berry bush at ({}, {})", target_pos.x, target_pos.y),
                };
                
                debug.log(DebugLevel::Info, "DOGOAP_ACTION", 
                    &format!("Worker moving to resource at ({}, {})", target_pos.x, target_pos.y));
            }
        }
        
        // Remove the action after setting movement
        commands.entity(entity).remove::<MoveToResourceAction>();
    }
}

// System to debug what actions are active on entities
pub fn debug_active_actions(
    query: Query<(
        Entity,
        Option<&EatAction>,
        Option<&WanderAction>,
        Option<&GatherFoodAction>,
        &Satiety,
        &Energy,
        &FoodCount,
    ), With<crate::components::UnitTag>>,
    debug: Res<DebugSystem>,
    sim_state: Res<crate::SimulationState>,
) {
    // Only log every 50 ticks to avoid spam
    if sim_state.tick % 50 != 0 {
        return;
    }
    
    for (entity, eat, wander, gather, hunger, energy, food) in query.iter() {
        let mut active_actions = Vec::new();
        if eat.is_some() { active_actions.push("Eat"); }
        if wander.is_some() { active_actions.push("Wander"); }
        if gather.is_some() { active_actions.push("Gather"); }
        
        if !active_actions.is_empty() {
            debug.log(DebugLevel::Info, "DOGOAP_ACTIVE", 
                &format!("Entity {:?} actions: {:?} | H:{:.1} E:{:.1} F:{:.0}", 
                    entity, active_actions, hunger.0, energy.0, food.0));
        }
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
        
        // Define the goal - keep satiety high (since lower values mean more hungry)
        // The satiety system: starts at 50, decreases over time (gets hungrier)
        // So we want to keep satiety ABOVE a threshold to avoid starvation
        let goal_not_hungry = Goal::from_reqs(&[
            Satiety::is_more(30.0),  // Trigger food gathering when satiety drops below 30
        ]);
        
        // Define actions with their preconditions and effects
        let eat_action = EatAction::new()
            .add_precondition(FoodCount::is_more(0.0))
            .add_mutator(Satiety::increase(20.0))  // Increased effect to make it worthwhile
            .add_mutator(FoodCount::decrease(1.0))
            .set_cost(1);
        
        // Wander action to explore and find berry bushes - simplified
        let wander_action = WanderAction::new()
            .add_precondition(NearBerryBush::is(0.0))  // Not near a bush
            .add_mutator(NearBerryBush::set(1.0))      // Will find a bush
            .set_cost(2);
        
        // Gather food action - only works when near a berry bush
        let gather_food_action = GatherFoodAction::new()
            .add_precondition(NearBerryBush::is(1.0)) // Must be near a bush
            .add_precondition(Energy::is_more(10.0))  // Need some energy to gather
            .add_mutator(FoodCount::increase(3.0))    // Get 3 food items when gathering
            .add_mutator(Energy::decrease(5.0))       // Costs some energy
            .set_cost(2);
        
        // Create the planner with the macro
        let (mut planner, components) = create_planner!({
            actions: [
                (EatAction, eat_action),
                (WanderAction, wander_action),
                (GatherFoodAction, gather_food_action),
            ],
            state: [
                Satiety(50.0),
                Energy(75.0),
                FoodCount(2.0),  // Start with some food
                NearBerryBush(0.0),  // Not near a bush initially
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
    mut query: Query<(
        &mut Satiety, 
        &mut Energy,
        Option<&crate::components::GridMovement>,
        Option<&crate::components::WorkProgress>,
    )>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (mut satiety, mut energy, movement, work) in query.iter_mut() {
        // Satiety decreases over time (lower means more hungry)
        // Decrease by 0.4 per tick (4 per second at 10 ticks per second)
        satiety.0 = (satiety.0 - 0.4).max(0.0);
        
        // Energy changes based on activity
        let is_moving = movement.map_or(false, |m| m.is_moving);
        
        // Determine energy consumption based on work type
        let energy_change = if let Some(w) = work {
            if w.is_working {
                // Different energy costs for different work types (reduced for balance)
                let cost = match &w.work_type {
                    Some(WorkType::Mining(_)) => -0.8,      // Mining is exhausting
                    Some(WorkType::Building(_)) => -0.6,    // Building is hard work
                    Some(WorkType::Farming(_)) => -0.5,     // Farming is tiring
                    Some(WorkType::Gathering(_)) => -0.4,   // Gathering moderate
                    Some(WorkType::Crafting(_)) => -0.25,   // Crafting is lighter
                    Some(WorkType::Research(_)) => -0.15,   // Research is mental
                    Some(WorkType::Repair(_)) => -0.5,      // Repair is physical
                    _ => -0.3,  // Generic work
                };
                
                if sim_state.tick % 20 == 0 {
                    let work_name = match &w.work_type {
                        Some(WorkType::Mining(_)) => "mining",
                        Some(WorkType::Building(_)) => "building",
                        Some(WorkType::Farming(_)) => "farming",
                        Some(WorkType::Gathering(_)) => "gathering",
                        Some(WorkType::Crafting(_)) => "crafting",
                        Some(WorkType::Research(_)) => "researching",
                        Some(WorkType::Repair(_)) => "repairing",
                        _ => "working",
                    };
                    debug.log(DebugLevel::Debug, "DOGOAP_STATE", 
                        &format!("Unit {}, energy: {:.1}, cost: {:.1}/tick", work_name, energy.0, cost));
                }
                cost
            } else if is_moving {
                -0.05  // Moving consumes very little energy (greatly reduced from -0.3)
            } else {
                0.5   // Idle recovers energy (increased)
            }
        } else if is_moving {
            if sim_state.tick % 20 == 0 {
                debug.log(DebugLevel::Debug, "DOGOAP_STATE", &format!("Unit moving, energy: {:.1}", energy.0));
            }
            -0.05  // Moving consumes very little energy (greatly reduced from -0.3)
        } else {
            0.5   // Idle recovers energy (increased)
        };
        
        // Apply energy change
        energy.0 = (energy.0 + energy_change).clamp(0.0, 100.0);
        
        // Log critical states
        if satiety.0 < 10.0 {
            debug.log(DebugLevel::Warn, "DOGOAP_STATE", "Worker is very hungry (low satiety)!");
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

// System to detect when peasants are near berry bushes
pub fn update_near_berry_bush(
    sim_state: Res<crate::SimulationState>,
    mut peasant_query: Query<(&crate::components::grid_position::GridPosition, &mut NearBerryBush), With<crate::components::UnitTag>>,
    resource_query: Query<(&crate::components::grid_position::GridPosition, &crate::components::resource::ResourceNode)>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (peasant_pos, mut near_bush) in peasant_query.iter_mut() {
        let mut found_bush = false;
        
        // Check all resource nodes for berry bushes
        for (resource_pos, resource_node) in resource_query.iter() {
            if resource_node.resource_type == crate::resources::ResourceType::Berries {
                // Calculate Manhattan distance
                let distance = peasant_pos.distance_to(resource_pos);
                
                // If within gathering range (1 tile), we're near a bush
                if distance <= 1 && resource_node.can_harvest() {
                    found_bush = true;
                    
                    // Only log state changes
                    if near_bush.0 < 0.5 {
                        debug.log(DebugLevel::Info, "DOGOAP_STATE", 
                            &format!("Worker found berry bush at distance {}", distance));
                    }
                    break;
                }
            }
        }
        
        // Update the state
        let old_value = near_bush.0;
        near_bush.0 = if found_bush { 1.0 } else { 0.0 };
        
        // Log when leaving bush proximity
        if old_value > 0.5 && near_bush.0 < 0.5 {
            debug.log(DebugLevel::Debug, "DOGOAP_STATE", "Worker left berry bush area");
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
            .register_type::<GatherFoodAction>()
            .register_type::<WanderAction>()
            .register_type::<MoveToResourceAction>()
            // Add our systems
            .add_systems(Update, (
                setup_dogoap_planners,
                update_needs_system,
                sync_inventory_to_food_count,  // Sync inventory berries to FoodCount
                update_near_berry_bush,         // Detect proximity to berry bushes
                debug_active_actions,           // Debug what actions are active
                handle_eat_action,
                handle_gather_food_action,
                handle_wander_action,
                handle_move_to_resource_action,
                // Sync dogoap values to UnitNeedsV2 for display
                crate::ai::shared_state::sync_dogoap_to_unit_needs,
            ));
        
        // CRITICAL: Register the DatumComponents with dogoap
        // This is required for the planner to find the components at runtime
        register_components!(
            app,
            vec![Satiety, Energy, FoodCount, NearBerryBush]
        );
    }
}