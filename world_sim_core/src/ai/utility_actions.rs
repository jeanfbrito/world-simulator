//! Utility AI actions for immediate reactive behaviors

use bevy_ecs::prelude::*;
use bevy::prelude::Time;
use big_brain::prelude::*;
use crate::components::*;
use world_sim_interface::{EngineEvent, EntityId};

// Emergency survival actions

/// Emergency eating action when critically hungry
#[derive(Debug, Component, ActionBuilder, Clone)]
pub struct EmergencyEatAction;

/// Emergency rest action when exhausted
#[derive(Debug, Component, ActionBuilder, Clone)]
pub struct EmergencyRestAction;

/// Emergency sleep action when completely exhausted
#[derive(Debug, Component, ActionBuilder, Clone)]
pub struct EmergencySleepAction;

// Defensive actions

/// Flee from immediate danger
#[derive(Debug, Component, ActionBuilder, Clone)]
pub struct FleeAction;

/// Defensive stance against threats
#[derive(Debug, Component, ActionBuilder, Clone)]
pub struct DefensiveAction;

// Opportunistic actions

/// Grab nearby valuable resource
#[derive(Debug, Component, ActionBuilder, Clone)]
pub struct GrabResourceAction;

/// Trade resources for profit
#[derive(Debug, Component, ActionBuilder, Clone)]
pub struct TradeAction;

/// Help a nearby ally in need
#[derive(Debug, Component, ActionBuilder, Clone)]
pub struct HelpAllyAction;

/// System for emergency eating
pub fn emergency_eat_action_system(
    mut commands: Commands,
    mut query: Query<(&Actor, &mut ActionState, &EmergencyEatAction)>,
    mut workers: Query<(&mut WorkerComponent, &mut IsHungry, &mut HasFood)>,
    mut events: ResMut<crate::systems::EventQueue>,
) {
    for (Actor(actor), mut state, _) in query.iter_mut() {
        if let Ok((mut worker, mut hunger, mut food)) = workers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    // Check if we have food
                    if food.0 > 0 {
                        *state = ActionState::Executing;
                    } else {
                        // No food available, fail
                        *state = ActionState::Failure;
                    }
                }
                ActionState::Executing => {
                    // Consume food immediately
                    food.0 -= 1;
                    hunger.0 = (hunger.0 - 40.0).max(0.0);
                    worker.consume_food(0.4);
                    
                    events.push(EngineEvent::ResourceConsumed {
                        entity_id: actor.index() as EntityId,
                        resource_type: world_sim_interface::ResourceType::Food,
                        amount: 1,
                    });
                    
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

/// System for emergency rest
pub fn emergency_rest_action_system(
    mut query: Query<(&Actor, &mut ActionState, &EmergencyRestAction)>,
    mut workers: Query<(&mut WorkerComponent, &mut HasEnergy)>,
    time: Res<Time>,
) {
    for (Actor(actor), mut state, _) in query.iter_mut() {
        if let Ok((mut worker, mut energy)) = workers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    *state = ActionState::Executing;
                    worker.state = world_sim_interface::WorkerState::Idle;
                }
                ActionState::Executing => {
                    // Rest quickly
                    let restore_rate = 40.0 * time.delta_secs() as f64;
                    energy.0 = (energy.0 + restore_rate).min(100.0);
                    worker.energy = (worker.energy + restore_rate as f32 / 100.0).min(1.0);
                    
                    // Complete when energy is restored enough
                    if energy.0 > 50.0 {
                        *state = ActionState::Success;
                        worker.state = world_sim_interface::WorkerState::Idle;
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

/// System for fleeing from danger
pub fn flee_action_system(
    mut query: Query<(&Actor, &mut ActionState, &FleeAction)>,
    mut workers: Query<(&mut PositionComponent, &mut MovementComponent), With<WorkerComponent>>,
    threats: Query<&PositionComponent, With<super::scorers::Threat>>,
    time: Res<Time>,
) {
    for (Actor(actor), mut state, _) in query.iter_mut() {
        if let Ok((mut pos, mut movement)) = workers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    // Find nearest threat
                    let mut flee_direction = (0.0, 0.0);
                    let mut threat_found = false;
                    
                    for threat_pos in threats.iter() {
                        let distance = pos.distance_to(threat_pos);
                        if distance < 20.0 {
                            // Calculate flee direction (opposite of threat)
                            let dir = pos.direction_to(threat_pos);
                            flee_direction.0 -= dir.0;
                            flee_direction.1 -= dir.1;
                            threat_found = true;
                        }
                    }
                    
                    if threat_found {
                        // Normalize and flee
                        let magnitude = (flee_direction.0 * flee_direction.0 + 
                                       flee_direction.1 * flee_direction.1).sqrt();
                        if magnitude > 0.0 {
                            flee_direction.0 /= magnitude;
                            flee_direction.1 /= magnitude;
                        }
                        
                        // Move away quickly
                        let speed = movement.speed * 1.5 * time.delta_secs();
                        pos.position.x += (flee_direction.0 * speed) as i32;
                        pos.position.y += (flee_direction.1 * speed) as i32;
                    } else {
                        // No threats nearby, success
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

/// System for grabbing nearby valuable resources
pub fn grab_resource_action_system(
    mut commands: Commands,
    mut query: Query<(&Actor, &mut ActionState, &GrabResourceAction)>,
    mut workers: Query<(&PositionComponent, &mut InventoryComponent), With<WorkerComponent>>,
    mut resources: Query<(Entity, &mut ResourceNodeComponent, &PositionComponent), Without<WorkerComponent>>,
    mut events: ResMut<crate::systems::EventQueue>,
) {
    for (Actor(actor), mut state, _) in query.iter_mut() {
        if let Ok((worker_pos, mut inventory)) = workers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    // Check if inventory has space
                    if !inventory.is_full() {
                        *state = ActionState::Executing;
                    } else {
                        *state = ActionState::Failure;
                    }
                }
                ActionState::Executing => {
                    // Find nearest valuable resource
                    let mut grabbed = false;
                    
                    for (resource_entity, mut resource, resource_pos) in resources.iter_mut() {
                        let distance = worker_pos.distance_to(resource_pos);
                        if distance < 2.0 {
                            // Grab some of the resource
                            let amount = 3.min(resource.amount);
                            let harvested = resource.harvest(amount);
                            
                            inventory.add_resource(resource.resource_type, harvested);
                            
                            events.push(EngineEvent::ResourceCollected {
                                worker_id: actor.index() as EntityId,
                                resource_type: resource.resource_type,
                                amount: harvested,
                            });
                            
                            if resource.is_depleted() {
                                commands.entity(resource_entity).despawn();
                            }
                            
                            grabbed = true;
                            break;
                        }
                    }
                    
                    *state = if grabbed { 
                        ActionState::Success 
                    } else { 
                        ActionState::Failure 
                    };
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

/// System for helping nearby allies
pub fn help_ally_action_system(
    mut query: Query<(&Actor, &mut ActionState, &HelpAllyAction)>,
    mut workers: Query<(&PositionComponent, &mut InventoryComponent, &mut HasFood)>,
    mut needy_workers: Query<(Entity, &PositionComponent, &mut IsHungry, &mut HasFood), With<WorkerComponent>>,
    mut events: ResMut<crate::systems::EventQueue>,
) {
    for (Actor(actor), mut state, _) in query.iter_mut() {
        if let Ok((my_pos, mut my_inventory, mut my_food)) = workers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    // Check if we have resources to share
                    if my_food.0 > 1 {
                        *state = ActionState::Executing;
                    } else {
                        *state = ActionState::Failure;
                    }
                }
                ActionState::Executing => {
                    // Find nearby ally in need
                    let mut helped = false;
                    
                    for (other_entity, other_pos, mut other_hunger, mut other_food) in needy_workers.iter_mut() {
                        if other_entity == *actor {
                            continue;
                        }
                        
                        let distance = my_pos.distance_to(other_pos);
                        if distance < 5.0 && other_hunger.0 > 60.0 && my_food.0 > 1 {
                            // Share food
                            my_food.0 -= 1;
                            other_food.0 += 1;
                            other_hunger.0 = (other_hunger.0 - 20.0).max(0.0);
                            
                            events.push(EngineEvent::ResourceTransferred {
                                from_entity: actor.index() as EntityId,
                                to_entity: other_entity.index() as EntityId,
                                resource_type: world_sim_interface::ResourceType::Food,
                                amount: 1,
                            });
                            
                            helped = true;
                            break;
                        }
                    }
                    
                    *state = if helped { 
                        ActionState::Success 
                    } else { 
                        ActionState::Failure 
                    };
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}