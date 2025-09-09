//! GOAP action handler systems

use bevy_ecs::prelude::*;
use bevy::prelude::{Time, Timer, TimerMode};
use bevy_dogoap::prelude::*;
use crate::components::*;
use world_sim_interface::{EngineEvent, EntityId, Position, ResourceType};
use rand::Rng;
use std::collections::HashMap;

/// Handle eat action - worker consumes food to reduce hunger
pub fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &EatAction,
        &mut IsHungry,
        &mut HasFood,
        &mut WorkerComponent,
    )>,
    mut events: ResMut<super::EventQueue>,
) {
    for (entity, _action, mut hunger, mut has_food, mut worker) in query.iter_mut() {
        // Check if we have food
        if has_food.0 > 0 {
            // Consume food
            has_food.0 -= 1;
            hunger.0 = (hunger.0 - 25.0).max(0.0);
            worker.consume_food(0.25);
            
            // Emit event
            events.push(EngineEvent::ResourceConsumed {
                entity_id: entity.index() as EntityId,
                resource_type: ResourceType::Food,
                amount: 1,
            });
            
            // Remove action when done
            commands.entity(entity).remove::<EatAction>();
        } else {
            // No food available, cancel action
            commands.entity(entity).remove::<EatAction>();
        }
    }
}

/// Handle rest action - worker rests to restore energy
pub fn handle_rest_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &RestAction,
        &mut HasEnergy,
        &mut WorkerComponent,
    )>,
    mut rest_timers: Local<HashMap<Entity, Timer>>,
) {
    for (entity, _action, mut energy, mut worker) in query.iter_mut() {
        let timer = rest_timers.entry(entity).or_insert_with(|| {
            Timer::from_seconds(3.0, TimerMode::Once)
        });
        
        if timer.tick(time.delta()).just_finished() {
            // Rest complete
            energy.0 = (energy.0 + 30.0).min(100.0);
            worker.energy = (worker.energy + 0.3).min(1.0);
            
            commands.entity(entity).remove::<RestAction>();
            rest_timers.remove(&entity);
        } else {
            // Gradually restore energy while resting
            let restore_rate = 10.0 * time.delta_secs() as f64;
            energy.0 = (energy.0 + restore_rate).min(100.0);
            worker.energy = (worker.energy + restore_rate as f32 / 100.0).min(1.0);
        }
    }
}

/// Handle sleep action - worker sleeps at home for full energy restoration
pub fn handle_sleep_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &SleepAction,
        &mut HasEnergy,
        &mut AtHome,
        &mut WorkerComponent,
        &mut Planner,
    )>,
    mut sleep_timers: Local<HashMap<Entity, Timer>>,
) {
    for (entity, _action, mut energy, at_home, mut worker, mut planner) in query.iter_mut() {
        if !at_home.0 {
            // Not at home, can't sleep
            commands.entity(entity).remove::<SleepAction>();
            continue;
        }
        
        // Stop planning while sleeping
        planner.always_plan = false;
        
        let timer = sleep_timers.entry(entity).or_insert_with(|| {
            Timer::from_seconds(5.0, TimerMode::Once)
        });
        
        if timer.tick(time.delta()).just_finished() {
            // Sleep complete
            energy.0 = 100.0;
            worker.energy = 1.0;
            
            commands.entity(entity).remove::<SleepAction>();
            sleep_timers.remove(&entity);
            
            // Resume planning
            planner.always_plan = true;
        } else {
            // Restore energy while sleeping
            let restore_rate = 20.0 * time.delta_secs() as f64;
            energy.0 = (energy.0 + restore_rate).min(100.0);
            worker.energy = (worker.energy + restore_rate as f32 / 100.0).min(1.0);
        }
    }
}

/// Handle harvest wood action
pub fn handle_harvest_wood_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &HarvestWoodAction,
        &PositionComponent,
        &mut HasWood,
        &mut InventoryComponent,
        &mut HasEnergy,
    )>,
    mut resources: Query<(Entity, &mut ResourceNodeComponent, &PositionComponent), Without<WorkerComponent>>,
    mut events: ResMut<super::EventQueue>,
    mut harvest_timers: Local<HashMap<Entity, Timer>>,
) {
    for (worker_entity, _action, worker_pos, mut has_wood, mut inventory, mut energy) in query.iter_mut() {
        // Find nearest wood resource
        let mut closest_resource = None;
        let mut closest_distance = f32::MAX;
        
        for (resource_entity, resource, resource_pos) in resources.iter() {
            if resource.resource_type == ResourceType::Wood {
                let distance = worker_pos.distance_to(resource_pos);
                if distance < closest_distance && distance <= 1.5 {
                    closest_resource = Some((resource_entity, resource_pos));
                    closest_distance = distance;
                }
            }
        }
        
        if let Some((resource_entity, _resource_pos)) = closest_resource {
            let timer = harvest_timers.entry(worker_entity).or_insert_with(|| {
                Timer::from_seconds(2.0, TimerMode::Once)
            });
            
            if timer.tick(time.delta()).just_finished() {
                // Harvest complete
                if let Ok((_, mut resource, _)) = resources.get_mut(resource_entity) {
                    let amount = 5.min(resource.amount);
                    let harvested = resource.harvest(amount);
                    
                    has_wood.0 += harvested;
                    inventory.add_resource(ResourceType::Wood, harvested);
                    
                    // Consume energy
                    energy.0 = (energy.0 - 10.0).max(0.0);
                    
                    // Emit event
                    events.push(EngineEvent::ResourceCollected {
                        worker_id: worker_entity.index() as EntityId,
                        resource_type: ResourceType::Wood,
                        amount: harvested,
                    });
                    
                    if resource.is_depleted() {
                        commands.entity(resource_entity).despawn();
                    }
                }
                
                commands.entity(worker_entity).remove::<HarvestWoodAction>();
                harvest_timers.remove(&worker_entity);
            }
        } else {
            // No resource nearby, cancel action
            commands.entity(worker_entity).remove::<HarvestWoodAction>();
        }
    }
}

/// Handle gather food action
pub fn handle_gather_food_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &GatherFoodAction,
        &PositionComponent,
        &mut HasFood,
        &mut InventoryComponent,
        &mut HasEnergy,
    )>,
    mut resources: Query<(Entity, &mut ResourceNodeComponent, &PositionComponent), Without<WorkerComponent>>,
    mut events: ResMut<super::EventQueue>,
    mut gather_timers: Local<HashMap<Entity, Timer>>,
) {
    for (worker_entity, _action, worker_pos, mut has_food, mut inventory, mut energy) in query.iter_mut() {
        // Find nearest food resource
        let mut closest_resource = None;
        let mut closest_distance = f32::MAX;
        
        for (resource_entity, resource, resource_pos) in resources.iter() {
            if resource.resource_type == ResourceType::Food {
                let distance = worker_pos.distance_to(resource_pos);
                if distance < closest_distance && distance <= 1.5 {
                    closest_resource = Some((resource_entity, resource_pos));
                    closest_distance = distance;
                }
            }
        }
        
        if let Some((resource_entity, _resource_pos)) = closest_resource {
            let timer = gather_timers.entry(worker_entity).or_insert_with(|| {
                Timer::from_seconds(1.5, TimerMode::Once)
            });
            
            if timer.tick(time.delta()).just_finished() {
                // Gather complete
                if let Ok((_, mut resource, _)) = resources.get_mut(resource_entity) {
                    let amount = 3.min(resource.amount);
                    let gathered = resource.harvest(amount);
                    
                    has_food.0 += gathered;
                    inventory.add_resource(ResourceType::Food, gathered);
                    
                    // Consume energy
                    energy.0 = (energy.0 - 5.0).max(0.0);
                    
                    // Emit event
                    events.push(EngineEvent::ResourceCollected {
                        worker_id: worker_entity.index() as EntityId,
                        resource_type: ResourceType::Food,
                        amount: gathered,
                    });
                    
                    if resource.is_depleted() {
                        commands.entity(resource_entity).despawn();
                    }
                }
                
                commands.entity(worker_entity).remove::<GatherFoodAction>();
                gather_timers.remove(&worker_entity);
            }
        } else {
            // No resource nearby, cancel action
            commands.entity(worker_entity).remove::<GatherFoodAction>();
        }
    }
}

/// Handle go to resource action
pub fn handle_go_to_resource_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &GoToResourceAction,
        &mut PositionComponent,
        &mut AtResource,
        &mut MovementComponent,
    )>,
    resources: Query<(Entity, &ResourceNodeComponent, &PositionComponent), Without<WorkerComponent>>,
) {
    for (entity, _action, mut worker_pos, mut at_resource, mut movement) in query.iter_mut() {
        // Find nearest resource
        let mut closest_resource = None;
        let mut closest_distance = f32::MAX;
        
        for (_resource_entity, _resource, resource_pos) in resources.iter() {
            let distance = worker_pos.distance_to(resource_pos);
            if distance < closest_distance {
                closest_resource = Some(resource_pos.clone());
                closest_distance = distance;
            }
        }
        
        if let Some(target_pos) = closest_resource {
            if closest_distance > 1.5 {
                // Move towards resource
                movement.set_target(Position { 
                    x: target_pos.x, 
                    y: target_pos.y 
                });
                
                // Simple movement for now
                let direction = target_pos.direction_to(&worker_pos);
                let speed = movement.speed * time.delta_secs();
                worker_pos.x += direction.0 * speed;
                worker_pos.y += direction.1 * speed;
            } else {
                // Reached resource
                at_resource.0 = true;
                movement.clear();
                commands.entity(entity).remove::<GoToResourceAction>();
            }
        } else {
            // No resources found
            commands.entity(entity).remove::<GoToResourceAction>();
        }
    }
}

/// Handle go to storage action
pub fn handle_go_to_storage_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &GoToStorageAction,
        &mut PositionComponent,
        &mut AtStorage,
        &mut MovementComponent,
    )>,
    storages: Query<(Entity, &StorageComponent, &PositionComponent), Without<WorkerComponent>>,
) {
    for (entity, _action, mut worker_pos, mut at_storage, mut movement) in query.iter_mut() {
        // Find nearest storage
        let mut closest_storage = None;
        let mut closest_distance = f32::MAX;
        
        for (_storage_entity, _storage, storage_pos) in storages.iter() {
            let distance = worker_pos.distance_to(storage_pos);
            if distance < closest_distance {
                closest_storage = Some(storage_pos.clone());
                closest_distance = distance;
            }
        }
        
        if let Some(target_pos) = closest_storage {
            if closest_distance > 1.5 {
                // Move towards storage
                movement.set_target(Position { 
                    x: target_pos.x, 
                    y: target_pos.y 
                });
                
                // Simple movement
                let direction = target_pos.direction_to(&worker_pos);
                let speed = movement.speed * time.delta_secs();
                worker_pos.x += direction.0 * speed;
                worker_pos.y += direction.1 * speed;
            } else {
                // Reached storage
                at_storage.0 = true;
                movement.clear();
                commands.entity(entity).remove::<GoToStorageAction>();
            }
        } else {
            // No storage found
            commands.entity(entity).remove::<GoToStorageAction>();
        }
    }
}

/// Handle store resources action
pub fn handle_store_resources_action(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &StoreResourcesAction,
        &mut HasWood,
        &mut HasFood,
        &mut HasStone,
        &mut InventoryComponent,
        &AtStorage,
    )>,
    mut storages: Query<(Entity, &mut StorageComponent), Without<WorkerComponent>>,
    mut events: ResMut<super::EventQueue>,
) {
    for (worker_entity, _action, mut has_wood, mut has_food, mut has_stone, mut inventory, at_storage) in query.iter_mut() {
        if !at_storage.0 {
            // Not at storage, cancel action
            commands.entity(worker_entity).remove::<StoreResourcesAction>();
            continue;
        }
        
        // Find storage building
        if let Some((storage_entity, mut storage)) = storages.iter_mut().next() {
            // Store wood
            if has_wood.0 > 0 {
                let stored = storage.store_resource(ResourceType::Wood, has_wood.0);
                has_wood.0 -= stored;
                inventory.remove_resource(ResourceType::Wood, stored);
                
                events.push(EngineEvent::ResourceStored {
                    worker_id: worker_entity.index() as EntityId,
                    building_id: storage_entity.index() as EntityId,
                    resource_type: ResourceType::Wood,
                    amount: stored,
                });
            }
            
            // Store food
            if has_food.0 > 0 {
                let stored = storage.store_resource(ResourceType::Food, has_food.0);
                has_food.0 -= stored;
                inventory.remove_resource(ResourceType::Food, stored);
                
                events.push(EngineEvent::ResourceStored {
                    worker_id: worker_entity.index() as EntityId,
                    building_id: storage_entity.index() as EntityId,
                    resource_type: ResourceType::Food,
                    amount: stored,
                });
            }
            
            // Store stone
            if has_stone.0 > 0 {
                let stored = storage.store_resource(ResourceType::Stone, has_stone.0);
                has_stone.0 -= stored;
                inventory.remove_resource(ResourceType::Stone, stored);
                
                events.push(EngineEvent::ResourceStored {
                    worker_id: worker_entity.index() as EntityId,
                    building_id: storage_entity.index() as EntityId,
                    resource_type: ResourceType::Stone,
                    amount: stored,
                });
            }
        }
        
        commands.entity(worker_entity).remove::<StoreResourcesAction>();
    }
}

/// Update worker needs over time
pub fn update_worker_needs(
    time: Res<Time>,
    mut query: Query<(&mut IsHungry, &mut HasEnergy, &mut WorkerComponent)>,
) {
    let mut rng = rand::thread_rng();
    
    for (mut hunger, mut energy, mut worker) in query.iter_mut() {
        // Increase hunger over time
        let hunger_rate = rng.gen_range(5.0..10.0) * time.delta_secs() as f64;
        hunger.0 = (hunger.0 + hunger_rate).min(100.0);
        worker.hunger = (hunger.0 as f32 / 100.0).clamp(0.0, 1.0);
        
        // Decrease energy over time (more when working)
        let energy_rate = if worker.state == world_sim_interface::WorkerState::Working {
            rng.gen_range(8.0..15.0)
        } else {
            rng.gen_range(2.0..5.0)
        };
        let energy_loss = energy_rate * time.delta_secs() as f64;
        energy.0 = (energy.0 - energy_loss).max(0.0);
        worker.energy = (energy.0 as f32 / 100.0).clamp(0.0, 1.0);
    }
}

/// System to handle idle action
pub fn handle_idle_action(
    mut query: Query<(Entity, &IdleAction, &mut IsIdle, &mut WorkerComponent)>,
) {
    for (_entity, _action, mut is_idle, mut worker) in query.iter_mut() {
        is_idle.0 = true;
        worker.state = world_sim_interface::WorkerState::Idle;
    }
}