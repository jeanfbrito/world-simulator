use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};
use crate::components::{WorkProgress, WorkType, ResourceWork};

// System set for GOAP action processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct GoapActionSet;

// Simple bevy_dogoap implementation for demonstration
// This will handle basic needs like eating and resting

// Actions as Components
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
#[reflect(Component)]
pub struct EatAction;

// NapAction for recovering energy when drowsy
#[derive(Component, Clone, Reflect, ActionComponent)]
#[reflect(Component)]
pub struct NapAction {
    pub ticks_remaining: u32,
    pub started: bool,
}

impl Default for NapAction {
    fn default() -> Self {
        Self {
            ticks_remaining: 50, // 5 seconds at 10 TPS
            started: false,
        }
    }
}

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
    pub started: bool,
    pub target_position: Option<(u32, u32)>,
    pub target_entity: Option<Entity>,
}

impl Default for MoveToResourceAction {
    fn default() -> Self {
        Self {
            target: None,
            resource_type: crate::resources::ResourceType::Berries,
            started: false,
            target_position: None,
            target_entity: None,
        }
    }
}

impl MoveToResourceAction {
    pub fn new() -> Self {
        Self::default()
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

// System to handle NapAction execution
pub fn handle_nap_action(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut NapAction, &mut Energy, Option<&mut crate::components::GridMovement>)>,
    mut planner_query: Query<&mut Planner>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }

    for (entity, mut nap_action, mut energy, movement_opt) in query.iter_mut() {
        // Initialize nap if first tick
        if !nap_action.started {
            nap_action.started = true;

            // Stop movement during nap
            if let Some(mut movement) = movement_opt {
                movement.is_moving = false;
                movement.path.clear();
            }

            debug.log(DebugLevel::Info, "DOGOAP_ACTION",
                &format!("Worker starting nap for {} ticks, current energy: {:.1}",
                    nap_action.ticks_remaining, energy.0));
        }

        // Recover energy during nap (DOGOAP uses 0-100 scale)
        let recovery = 1.6;  // 1.6% per tick (vs 0.5% idle recovery)
        energy.0 = (energy.0 + recovery).min(100.0);

        // Decrease remaining ticks
        nap_action.ticks_remaining = nap_action.ticks_remaining.saturating_sub(1);

        // Check if nap is complete
        if nap_action.ticks_remaining == 0 || energy.0 >= 80.0 {
            debug.log(DebugLevel::Info, "DOGOAP_ACTION",
                &format!("Worker finished napping, energy restored to: {:.1}", energy.0));

            // Remove action and force replanning
            commands.entity(entity).remove::<NapAction>();

            if let Ok(mut planner) = planner_query.get_mut(entity) {
                planner.current_plan.clear();
            }
        }
    }
}

// PHASE 4: System to handle GatherFoodAction execution with proper completion detection
pub fn handle_gather_food_action(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &GatherFoodAction,
        &mut FoodCount,
        &mut Energy,
        &NearBerryBush,
        Option<&crate::components::WorkProgress>,  // Changed to non-mut
        &crate::components::grid_position::GridPosition,
    )>,
    mut planner_query: Query<&mut Planner>,
    resource_query: Query<(Entity, &crate::components::grid_position::GridPosition, &crate::components::resource::ResourceNode)>,
    debug: Res<DebugSystem>,
) {
    if !sim_state.just_ticked {
        return;
    }

    for (entity, _action, mut food, mut energy, near_bush, work_progress_opt, unit_pos) in query.iter_mut() {
        // DEBUG: Log entry into gather handler
        debug.log(DebugLevel::Info, "GATHER_CHECK",
            &format!("Checking gather for entity {:?}, near_bush: {}, work_progress: {}",
                entity, near_bush.0, work_progress_opt.is_some()));
        // PHASE 4: Check if work is complete FIRST
        if let Some(work_progress) = work_progress_opt {
            if work_progress.progress_counter >= crate::simulation::MAX_WORK_PROGRESS && !work_progress.is_working {
                // Work finished! Get amount from work type and update state
                let gathered = if let Some(work_type) = &work_progress.work_type {
                    match work_type {
                        crate::components::WorkType::Gathering(resource_work) => resource_work.amount,
                        _ => 1, // Default amount
                    }
                } else {
                    1 // Default amount
                };

                food.0 += gathered as f64;

                debug.log(DebugLevel::Info, "DOGOAP_ACTION",
                    &format!("Gathering complete! Got {} berries", gathered));

                // Remove action and force replanning
                commands.entity(entity).remove::<GatherFoodAction>();

                // Clear the work progress
                commands.entity(entity).remove::<crate::components::WorkProgress>();

                if let Ok(mut planner) = planner_query.get_mut(entity) {
                    planner.current_plan.clear();
                }
                continue;
            }
        }
        // Only gather if actually near a berry bush
        if near_bush.0 > 0.0 {
            debug.log(DebugLevel::Info, "GATHER",
                &format!("Entity {:?} is near berry bush, checking for work", entity));
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
                // Handle WorkProgress - but check if work can be started
                if work_progress_opt.is_none() {
                    // No WorkProgress component, need to add it first
                    debug.log(DebugLevel::Info, "GATHER",
                        &format!("Adding WorkProgress component to entity {:?} at bush {:?}", entity, bush_entity));

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

                    debug.log(DebugLevel::Info, "DOGOAP_ACTION",
                        &format!("Worker starting to gather berries from bush {:?}", bush_entity));
                }
                // If WorkProgress exists and is working, just wait for completion
                // If WorkProgress exists but isn't working, the work system will handle it
                else {
                    debug.log(DebugLevel::Info, "GATHER",
                        &format!("Entity {:?} already has WorkProgress, work_in_progress: {}",
                            entity, work_progress_opt.map_or(false, |wp| wp.is_working)));
                }
            } else {
                debug.log(DebugLevel::Debug, "GATHER", "No berry bush found near peasant");
                // No berry bush found, remove action and replan
                commands.entity(entity).remove::<GatherFoodAction>();
                if let Ok(mut planner) = planner_query.get_mut(entity) {
                    planner.current_plan.clear();
                }
            }
        }
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

// Function to find the closest unclaimed berry bush
pub fn find_closest_unclaimed_berry_bush(
    unit_pos: &crate::components::grid_position::GridPosition,
    berry_bush_query: &Query<(Entity, &crate::components::grid_position::GridPosition, &crate::components::resource::ResourceNode), With<crate::ai::BerryBushTag>>,
    current_tick: u32,
) -> Option<(Entity, crate::components::grid_position::GridPosition, u32)> {
    let mut closest = None;
    let mut min_distance = u32::MAX;

    for (entity, bush_pos, resource_node) in berry_bush_query.iter() {
        // Check if bush has berries
        if resource_node.amount == 0 {
            continue;
        }

        // Check if bush is already fully claimed (using resource's internal claims)
        // First cleanup expired claims, then check if it's fully claimed
        let mut resource_mut = resource_node.clone();
        resource_mut.cleanup_expired_claims(current_tick);
        if resource_mut.is_fully_claimed() {
            continue;
        }

        let distance = unit_pos.distance_to(bush_pos);
        if distance < min_distance {
            min_distance = distance;
            closest = Some((entity, bush_pos.clone(), distance));
        }
    }

    closest
}

// System to assign resource targets to MoveToResourceAction
pub fn assign_resource_targets(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut MoveToResourceAction,
        &crate::components::grid_position::GridPosition,
        &crate::components::NameComponent,
    ), Without<crate::components::ClaimedResource>>,
    berry_bush_query: Query<(Entity, &crate::components::grid_position::GridPosition, &crate::components::resource::ResourceNode), With<crate::ai::BerryBushTag>>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }

    for (entity, mut action, unit_pos, name) in query.iter_mut() {
        // Skip if target already assigned
        if action.target.is_some() {
            continue;
        }

        // Use the new function to find the closest unclaimed berry bush
        if let Some((bush_entity, bush_pos, distance)) =
            find_closest_unclaimed_berry_bush(unit_pos, &berry_bush_query, sim_state.tick) {

            action.target = Some(bush_entity);
            debug.log(DebugLevel::Info, "ASSIGN_TARGET",
                &format!("{} assigned unclaimed berry bush {:?} at ({},{}) distance {} for gathering",
                    name.name, bush_entity, bush_pos.x, bush_pos.y, distance));
        } else {
            debug.log(DebugLevel::Debug, "NO_TARGET",
                &format!("{} found no unclaimed berry bushes available", name.name));
            // Remove action if no targets available
            commands.entity(entity).remove::<MoveToResourceAction>();
        }
    }
}

// PHASE 4: System to handle MoveToResourceAction with completion detection
pub fn handle_move_to_resource_action(
    sim_state: Res<crate::SimulationState>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut MoveToResourceAction,
        &crate::components::grid_position::GridPosition,
        &mut crate::components::grid_position::GridMovement,
        &mut crate::components::UnitMind,
        &mut crate::components::ClaimedResource,
        &crate::components::NameComponent,
    )>,
    mut berry_bush_queries: ParamSet<(
        Query<(Entity, &crate::components::grid_position::GridPosition, &crate::components::resource::ResourceNode), With<crate::ai::BerryBushTag>>,
        Query<(Entity, &crate::components::grid_position::GridPosition, &mut crate::components::resource::ResourceNode), With<crate::ai::BerryBushTag>>,
    )>,
    mut planner_query: Query<&mut Planner>,
    debug: Res<DebugSystem>,
) {
    if !sim_state.just_ticked {
        return;
    }

    for (entity, mut action, grid_pos, mut movement, mut mind, mut claimed_resource, name) in query.iter_mut() {
        if !action.started {
            // PHASE 1: Find and claim a berry bush atomically
            // First, find the closest unclaimed bush using the immutable query
            let closest_bush = {
                let berry_bush_query_immutable = berry_bush_queries.p0();
                find_closest_unclaimed_berry_bush(grid_pos, &berry_bush_query_immutable, sim_state.tick)
            };

            if let Some((bush_entity, bush_pos, distance)) = closest_bush {
                // Try to claim the resource directly (it tracks its own claims)
                let mut berry_bush_query = berry_bush_queries.p1();
                if let Ok((_, _, mut resource)) = berry_bush_query.get_mut(bush_entity) {
                    if resource.try_claim_with_timeout(entity, sim_state.tick) {
                        // SUCCESS: We have exclusive claim
                        debug.log(DebugLevel::Info, "EXCLUSIVE_CLAIM",
                            &format!("{} exclusively claimed berry bush {:?} at ({},{}) [distance: {}]",
                                    name.name, bush_entity, bush_pos.x, bush_pos.y, distance));

                        // Set movement target
                        movement.set_target_from(grid_pos, bush_pos.clone());
                        action.started = true;
                        action.target_position = Some((bush_pos.x, bush_pos.y));
                        action.target_entity = Some(bush_entity);
                        action.target = Some(bush_entity);

                        // Update local claimed resource
                        claimed_resource.claim_with_position(bush_entity, (bush_pos.x, bush_pos.y));

                        // Set mind state
                        *mind = crate::components::UnitMind::GoingThere {
                            destination: format!("Berry bush at ({}, {})", bush_pos.x, bush_pos.y),
                        };
                    } else {
                        // Claim failed - someone else got it
                        debug.log(DebugLevel::Debug, "CLAIM_FAILED",
                            &format!("{} failed to claim bush at ({},{}) - already taken",
                                    name.name, bush_pos.x, bush_pos.y));
                    }
                }
            } else {
                // No unclaimed berry bushes available
                debug.log(DebugLevel::Debug, "NO_BUSHES",
                    &format!("{} found no unclaimed berry bushes - removing action", name.name));
                commands.entity(entity).remove::<MoveToResourceAction>();
                if let Ok(mut planner) = planner_query.get_mut(entity) {
                    planner.current_plan.clear();
                }
            }
        } else {
            // PHASE 2: Check if we've reached the target
            if let Some(target_pos) = action.target_position {
                if grid_pos.distance_to(&crate::components::grid_position::GridPosition::new(target_pos.0, target_pos.1)) <= 1 {
                    debug.log(DebugLevel::Info, "DOGOAP_ACTION",
                        &format!("{} reached berry bush at ({},{}) - MoveToResourceAction complete!",
                                name.name, target_pos.0, target_pos.1));

                    // Remove action and let plan continue to next action (gathering)
                    commands.entity(entity).remove::<MoveToResourceAction>();
                    continue;
                }
            }

            // PHASE 3: Validate that our claim is still valid
            if let Some(target_entity) = action.target_entity {
                // Check if the resource still has our claim
                let berry_bush_query_immutable = berry_bush_queries.p0();
                if let Ok((_, _, resource)) = berry_bush_query_immutable.get(target_entity) {
                    if !resource.is_claimed_by(entity) {
                        // Our claim was lost somehow (expired?), replan
                        debug.log(DebugLevel::Warn, "CLAIM_LOST",
                            &format!("{} lost claim to berry bush {:?} - replanning", name.name, target_entity));

                        // Release local tracking
                        claimed_resource.release();

                        commands.entity(entity).remove::<MoveToResourceAction>();
                        if let Ok(mut planner) = planner_query.get_mut(entity) {
                            planner.current_plan.clear();
                        }
                    }
                }
            }
        }
    }
}

// System to debug what actions are active on entities
pub fn debug_active_actions(
    query: Query<(
        Entity,
        Option<&EatAction>,
        Option<&MoveToResourceAction>,
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
    
    for (entity, eat, move_to, gather, hunger, energy, food) in query.iter() {
        let mut active_actions = Vec::new();
        if eat.is_some() { active_actions.push("Eat"); }
        if move_to.is_some() { active_actions.push("MoveToResource"); }
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

        // Goal to stay rested and avoid exhaustion
        let goal_rested = Goal::from_reqs(&[
            Energy::is_more(15.0),  // Trigger nap when energy drops below 15 (before exhaustion)
        ]);

        // Define actions with their preconditions and effects
        let eat_action = EatAction::new()
            .add_precondition(FoodCount::is_more(0.0))
            .add_mutator(Satiety::increase(20.0))  // Increased effect to make it worthwhile
            .add_mutator(FoodCount::decrease(1.0))
            .set_cost(1);

        // NapAction - can be performed even at 0 energy (emergency recovery)
        let nap_action = NapAction::new()
            // No energy precondition - can nap even when exhausted
            .add_mutator(Energy::increase(40.0))  // Restore significant energy
            .set_cost(0);  // Highest priority (lowest cost)
        
        // Move to resource action - handled by manual systems, not GOAP directly
        // Using WanderAction as placeholder in GOAP - actual movement logic is in manual systems
        let wander_action = WanderAction::new()
            .add_precondition(NearBerryBush::is(0.0))  // Not near a bush
            .add_precondition(FoodCount::is_less(5.0)) // Need food
            .add_mutator(NearBerryBush::set(1.0))      // Will find a bush
            .set_cost(3);
        
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
                (NapAction, nap_action),  // Highest priority action
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
            goals: [goal_not_hungry, goal_rested],  // Multiple goals to maintain
        });
        
        // Configure the planner
        planner.always_plan = true;
        planner.remove_goal_on_no_plan_found = false;
        // Let the planner choose the most critical goal based on current state
        planner.current_goal = None;  // Will be selected automatically

        debug.log(DebugLevel::Info, "GOAP_SETUP",
            &format!("Created planner for entity {:?} with initial plan: {:?}",
                entity, planner.current_plan));

        // Add planner and components to the entity
        commands.entity(entity)
            .insert(planner)
            .insert(components);

        debug.log(DebugLevel::Info, "GOAP_SETUP",
            &format!("Entity {:?} now has Planner and components", entity));
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
        Option<&NapAction>,  // Check if unit is napping
    )>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (mut satiety, mut energy, movement, work, nap) in query.iter_mut() {
        // Satiety decreases over time (lower means more hungry)
        // Decrease by 0.4 per tick (4 per second at 10 ticks per second)
        satiety.0 = (satiety.0 - 0.4).max(0.0);

        // Skip energy update if napping (handled by handle_nap_action)
        if nap.is_some() {
            continue;  // Nap system handles energy recovery
        }

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
    mut peasant_query: Query<(
        Entity,
        &crate::components::grid_position::GridPosition, 
        &mut NearBerryBush,
        &mut Planner,
    ), With<crate::components::UnitTag>>,
    resource_query: Query<(&crate::components::grid_position::GridPosition, &crate::components::resource::ResourceNode)>,
    debug: Res<DebugSystem>,
) {
    // Only update on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, peasant_pos, mut near_bush, mut planner) in peasant_query.iter_mut() {
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
                        
                        // Force replanning when we reach a berry bush
                        // DON'T clear current plan - let execute_goap_plans handle it
                        // planner.current_plan.clear(); // REMOVED: This was preventing execution
                        debug.log(DebugLevel::Info, "DOGOAP_STATE",
                            &format!("Entity {:?} near berry bush - keeping existing plan", entity));
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
            // DON'T clear plan when leaving - let natural plan progression handle it
            // planner.current_plan.clear(); // REMOVED: This was preventing execution
        }
    }
}

// PHASE 1: CORE FIX - Execute GOAP plans by spawning action components
pub fn execute_goap_plans(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Planner,
        Option<&EatAction>,
        Option<&GatherFoodAction>,
        Option<&MoveToResourceAction>,
        Option<&WanderAction>,
    )>,
    debug: Res<DebugSystem>,
    sim_state: Res<crate::SimulationState>,
) {
    // Only on simulation ticks
    if !sim_state.just_ticked {
        return;
    }

    for (entity, mut planner, eat, gather, move_to, wander) in query.iter_mut() {
        // DEBUG: Log current plan contents for entity
        if !planner.current_plan.is_empty() {
            debug.log(DebugLevel::Info, "GOAP_PLAN",
                &format!("Entity {:?} current plan: {:?} (sim time: {:.1}s)",
                         entity, planner.current_plan, sim_state.accumulated_time));
        }

        // Skip if any action is already active
        if eat.is_some() || gather.is_some() || move_to.is_some() || wander.is_some() {
            debug.log(DebugLevel::Info, "GOAP_EXEC",
                &format!("Entity {:?} skipping - action already active: eat={}, gather={}, move={}, wander={}",
                    entity, eat.is_some(), gather.is_some(), move_to.is_some(), wander.is_some()));
            continue;
        }

        debug.log(DebugLevel::Info, "GOAP_EXEC",
            &format!("Entity {:?} ready to execute - plan has {} actions (sim time: {:.1}s)",
                     entity, planner.current_plan.len(), sim_state.accumulated_time));

        // Pop and execute next action from plan
        if let Some(action_type) = planner.current_plan.front() {
            let action_name = format!("{:?}", action_type);

            debug.log(DebugLevel::Info, "GOAP_EXEC",
                &format!("Entity {:?} executing action: {} (raw: {:?})", entity, action_name, action_type));

            // Spawn the appropriate action component
            // The action names from GOAP are in snake_case format
            match action_name.as_str() {
                name if name.contains("eat_action") => {
                    commands.entity(entity).insert(EatAction);
                    debug.log(DebugLevel::Info, "GOAP_EXEC",
                        &format!("Spawning EatAction for entity {:?}", entity));
                }
                name if name.contains("gather_food_action") => {
                    commands.entity(entity).insert(GatherFoodAction);
                    debug.log(DebugLevel::Info, "GOAP_EXEC",
                        &format!("Spawning GatherFoodAction for entity {:?}", entity));
                }
                name if name.contains("move_to_resource_action") => {
                    commands.entity(entity).insert(MoveToResourceAction::default());
                    debug.log(DebugLevel::Info, "GOAP_EXEC",
                        &format!("Spawning MoveToResourceAction for entity {:?}", entity));
                }
                name if name.contains("wander_action") => {
                    commands.entity(entity).insert(WanderAction);
                    debug.log(DebugLevel::Info, "GOAP_EXEC",
                        &format!("Spawning WanderAction for entity {:?}", entity));
                }
                _ => {
                    debug.log(DebugLevel::Info, "GOAP_EXEC",
                        &format!("Unknown action type: {} for entity {:?}", action_name, entity));
                }
            }

            // Remove from plan after spawning
            planner.current_plan.pop_front();
            debug.log(DebugLevel::Info, "GOAP_EXEC",
                &format!("Removed action from plan, remaining: {}", planner.current_plan.len()));
        } else {
            // Only log empty plans if we've been running for a while
            // This avoids spam during the initial 5+ seconds when GOAP is still planning
            if sim_state.accumulated_time > 10.0 {
                debug.log(DebugLevel::Debug, "GOAP_EXEC",
                    &format!("Entity {:?} has empty plan when trying to execute (sim time: {:.1}s)",
                             entity, sim_state.accumulated_time));
            }
        }
    }
}

// System to refresh resource claims for units still moving to their targets
// This prevents claims from expiring while units are in transit
pub fn refresh_resource_claims(
    mut resource_query: Query<&mut crate::components::resource::ResourceNode>,
    claimed_query: Query<(Entity, &crate::components::ClaimedResource, &crate::components::NameComponent)>,
    move_query: Query<&MoveToResourceAction>,
    sim_state: Res<crate::SimulationState>,
    debug: Res<DebugSystem>,
) {
    // Only run every 20 ticks to reduce overhead
    if sim_state.tick % 20 != 0 {
        return;
    }

    for (entity, claimed, name) in claimed_query.iter() {
        // Only refresh if entity has an active move action and a claim
        if move_query.get(entity).is_ok() && claimed.has_claim() {
            if let Some(resource_entity) = claimed.get_claimed() {
                if let Ok(mut resource) = resource_query.get_mut(resource_entity) {
                    resource.refresh_claim(entity, sim_state.tick);
                    debug.log(DebugLevel::Debug, "CLAIM_REFRESH",
                        &format!("{} refreshed claim on resource {:?} at tick {}",
                                name.name, resource_entity, sim_state.tick));
                }
            }
        }
    }
}

// System to cleanup stale claims when gathering is complete or entities die
pub fn cleanup_stale_claims(
    mut global_claims: ResMut<crate::components::GlobalResourceClaims>,
    mut claimed_query: Query<(Entity, &mut crate::components::ClaimedResource, &crate::components::NameComponent)>,
    gather_query: Query<&GatherFoodAction>,
    move_query: Query<&MoveToResourceAction>,
    debug: Res<DebugSystem>,
) {
    // Find entities that have claims but no active gathering or move actions
    for (entity, mut claimed, name) in claimed_query.iter_mut() {
        if claimed.has_claim() {
            let has_gather = gather_query.get(entity).is_ok();
            let has_move = move_query.get(entity).is_ok();

            // If no active gathering or movement actions, release claims
            if !has_gather && !has_move {
                if let Some(resource_entity) = claimed.get_claimed() {
                    global_claims.release_claim(resource_entity);
                    debug.log(DebugLevel::Info, "CLAIM_CLEANUP",
                        &format!("{} released resource claim on {:?} (no active actions)",
                                name.name, resource_entity));
                }

                if let Some(position) = claimed.get_claimed_position() {
                    global_claims.release_position_claim(position);
                    debug.log(DebugLevel::Info, "CLAIM_CLEANUP",
                        &format!("{} released position claim on ({},{}) (no active actions)",
                                name.name, position.0, position.1));
                }

                claimed.release();
            }
        }
    }
}

// System to cleanup claims when gather actions complete
pub fn cleanup_completed_gather_claims(
    mut commands: Commands,
    mut global_claims: ResMut<crate::components::GlobalResourceClaims>,
    mut query: Query<(Entity, &mut crate::components::ClaimedResource, &crate::components::NameComponent), (Without<GatherFoodAction>, Without<MoveToResourceAction>)>,
    debug: Res<DebugSystem>,
) {
    for (entity, mut claimed, name) in query.iter_mut() {
        if claimed.has_claim() {
            // Release global claims
            if let Some(resource_entity) = claimed.get_claimed() {
                global_claims.release_claim(resource_entity);
                debug.log(DebugLevel::Info, "GATHER_COMPLETE_CLEANUP",
                    &format!("{} released resource claim on {:?} (gather complete)",
                            name.name, resource_entity));
            }

            if let Some(position) = claimed.get_claimed_position() {
                global_claims.release_position_claim(position);
                debug.log(DebugLevel::Info, "GATHER_COMPLETE_CLEANUP",
                    &format!("{} released position claim on ({},{}) (gather complete)",
                            name.name, position.0, position.1));
            }

            // Clear local claim
            claimed.release();
        }
    }
}

// System to cleanup claims when entities are despawned
pub fn cleanup_despawned_entity_claims(
    mut global_claims: ResMut<crate::components::GlobalResourceClaims>,
    mut removed: RemovedComponents<crate::components::ClaimedResource>,
    debug: Res<DebugSystem>,
) {
    for entity in removed.read() {
        // Release all claims by this entity
        global_claims.release_claimant_claims(entity);
        debug.log(DebugLevel::Info, "DESPAWN_CLEANUP",
            &format!("Released all claims for despawned entity {:?}", entity));
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
            .register_type::<NapAction>()
            .register_type::<GatherFoodAction>()
            .register_type::<WanderAction>()
            .register_type::<MoveToResourceAction>()
            // Add our systems - run on simulation ticks
            // Split into two groups: setup/state updates first, then execution
            .add_systems(Update, (
                setup_dogoap_planners,
                update_needs_system,
                sync_inventory_to_food_count,  // Sync inventory berries to FoodCount
                update_near_berry_bush,         // Detect proximity to berry bushes
            )
            .in_set(GoapActionSet)
            .run_if(crate::simulation::on_simulation_tick_legacy))
            // Execution systems run after planning (in the same frame but later)
            .add_systems(Update, (
                debug_active_actions,           // Debug what actions are active
                execute_goap_plans,             // CORE FIX: Execute plans by spawning actions
                handle_eat_action,
                handle_nap_action,              // Handle nap action for energy recovery
                handle_gather_food_action,
                handle_wander_action,
                assign_resource_targets,        // Assign targets to MoveToResourceAction
                handle_move_to_resource_action,
                // Cleanup systems for global resource claims
                refresh_resource_claims,        // Refresh claims for units in transit
                cleanup_stale_claims,           // Cleanup claims when actions complete
                cleanup_completed_gather_claims, // Cleanup when gathering finishes
                cleanup_despawned_entity_claims, // Cleanup when entities are removed
                // Sync dogoap values to UnitNeedsV2 for display
                crate::ai::shared_state::sync_dogoap_to_unit_needs,
            )
            .after(GoapActionSet)  // Run after the state update systems
            .run_if(crate::simulation::on_simulation_tick_legacy));
        
        // CRITICAL: Register the DatumComponents with dogoap
        // This is required for the planner to find the components at runtime
        register_components!(
            app,
            vec![Satiety, Energy, FoodCount, NearBerryBush]
        );
    }
}