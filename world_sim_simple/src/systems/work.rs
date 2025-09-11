/// Tick-based work execution system
/// 
/// This system handles all work progress on simulation ticks,
/// including resource gathering, building, crafting, etc.

use bevy::prelude::*;
use crate::components::{
    WorkProgress, WorkSpeed, WorkQueue, WorkType, QueuedWork,
    ResourceWork, BuildingWork, CraftingWork,
    GridPosition, UnitInventory, UnitNeedsV2,
    PeasantTag, NameComponent
};
use crate::SimulationState;
use crate::resources::ResourceType;
use colored::Colorize;

/// Main work execution system that runs on ticks
pub fn tick_work_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity,
        &mut WorkProgress,
        &WorkSpeed,
        &mut UnitInventory,
        &mut UnitNeedsV2,
        &GridPosition,
        &NameComponent,
    ), With<PeasantTag>>,
    mut resources: Query<&mut crate::components::ResourceNode>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (_entity, mut work, _speed, mut inventory, mut needs, position, name) in units.iter_mut() {
        if !work.is_working {
            continue;
        }
        
        // Consume energy while working
        needs.tick_update(); // Additional energy loss while working
        
        // Update work progress
        let completed = work.tick_update();
        
        if completed {
            // Handle work completion based on type
            if let Some(work_type) = &work.work_type {
                handle_work_completion(
                    work_type,
                    &mut inventory,
                    &mut needs,
                    position,
                    name,
                    work.target_entity,
                    &mut resources,
                    &debug,
                );
            }
            
            println!("{} {} completed {}",
                "✅".green(),
                name.name.cyan(),
                work.work_type.as_ref()
                    .map(|w| format!("{:?}", w))
                    .unwrap_or_else(|| "work".to_string())
            );
            
            debug.log(
                DebugLevel::Info,
                "WORK",
                &format!("{} completed work at ({},{})",
                    name.name, position.x, position.y)
            );
        }
    }
}

/// Handle work completion effects
fn handle_work_completion(
    work_type: &WorkType,
    inventory: &mut UnitInventory,
    _needs: &mut UnitNeedsV2,
    _position: &GridPosition,
    name: &NameComponent,
    target_entity: Option<Entity>,
    resources: &mut Query<&mut crate::components::ResourceNode>,
    debug: &crate::debug::DebugSystem,
) {
    use crate::debug::DebugLevel;
    
    match work_type {
        WorkType::Gathering(resource_work) => {
            // First, deplete the resource if we have a target entity
            let mut resource_depleted = false;
            if let Some(resource_entity) = target_entity {
                if let Ok(mut resource) = resources.get_mut(resource_entity) {
                    // Check if resource has enough
                    if resource.amount >= resource_work.amount {
                        resource.amount -= resource_work.amount;
                        resource_depleted = true;
                        
                        println!("{} Resource depleted to {} remaining",
                            "⛏️".yellow(),
                            resource.amount
                        );
                        
                        debug.log(
                            DebugLevel::Debug,
                            "RESOURCE",
                            &format!("Resource depleted by {}, {} remaining",
                                resource_work.amount, resource.amount)
                        );
                    } else if resource.amount > 0 {
                        // Take what's available
                        let actual_amount = resource.amount;
                        resource.amount = 0;
                        resource_depleted = true;
                        
                        println!("{} Resource fully depleted (took {})",
                            "⛏️".red(),
                            actual_amount
                        );
                    }
                }
            }
            
            // Only add to inventory if we actually depleted the resource
            if resource_depleted {
                let added = inventory.add_item(resource_work.resource_type, resource_work.amount);
                
                if added {
                    println!("{} {} gathered {} {}",
                        "🌲".green(),
                        name.name.cyan(),
                        resource_work.amount,
                        format!("{:?}", resource_work.resource_type).yellow()
                    );
                    
                    debug.log(
                        DebugLevel::Info,
                        "GATHER",
                        &format!("{} gathered {} {:?}",
                            name.name, resource_work.amount, resource_work.resource_type)
                    );
                } else {
                    println!("{} {} inventory full!",
                        "⚠️".yellow(),
                        name.name.yellow()
                    );
                }
            }
        }
        
        WorkType::Building(_building_work) => {
            // TODO: Update building construction progress when buildings module is ready
            println!("{} {} building work completed!",
                "🏗️".green(),
                name.name.cyan()
            );
            
            debug.log(
                DebugLevel::Info,
                "BUILD",
                &format!("{} completed building work", name.name)
            );
        }
        
        WorkType::Crafting(crafting_work) => {
            // Create crafted items
            // TODO: Implement recipe system
            println!("{} {} crafted {}",
                "🔨".cyan(),
                name.name.cyan(),
                crafting_work.recipe_id.green()
            );
            
            debug.log(
                DebugLevel::Info,
                "CRAFT",
                &format!("{} crafted {} (x{})",
                    name.name, crafting_work.recipe_id, crafting_work.output_count)
            );
        }
        
        _ => {
            // Generic work completion
            debug.log(
                DebugLevel::Debug,
                "WORK",
                &format!("{} completed {:?}", name.name, work_type)
            );
        }
    }
}

/// System to assign work from queues
pub fn work_assignment_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity,
        &mut WorkProgress,
        &mut WorkQueue,
        &WorkSpeed,
        &NameComponent,
    ), With<PeasantTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (_entity, mut work, mut queue, speed, name) in units.iter_mut() {
        // Skip if already working
        if work.is_working {
            continue;
        }
        
        // Get next work from queue
        if let Some(queued) = queue.dequeue() {
            let ticks = speed.get_ticks_for(&queued.work_type);
            work.start_work(queued.work_type.clone(), ticks, queued.target_entity);
            
            debug.log(
                DebugLevel::Info,
                "WORK",
                &format!("{} started {:?} ({}t)",
                    name.name, queued.work_type, ticks)
            );
        }
    }
}

/// System to queue gathering work when near resources
pub fn auto_gather_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity,
        &GridPosition,
        &mut WorkQueue,
        &UnitInventory,
        &NameComponent,
    ), With<PeasantTag>>,
    resources: Query<(Entity, &GridPosition, &crate::components::ResourceNode)>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only process every 10 ticks
    if !sim_state.just_ticked || sim_state.tick % 10 != 0 {
        return;
    }
    
    for (_unit_entity, unit_pos, mut queue, inventory, name) in units.iter_mut() {
        // Skip if inventory is full
        if inventory.is_full() {
            continue;
        }
        
        // Skip if queue is full
        if !queue.is_empty() {
            continue;
        }
        
        // Check for nearby resources
        for (resource_entity, resource_pos, resource_node) in resources.iter() {
            if unit_pos.distance_to(resource_pos) <= 1 {
                // Adjacent to resource, queue gathering work
                let work = QueuedWork {
                    work_type: WorkType::Gathering(ResourceWork {
                        resource_type: resource_node.resource_type,
                        amount: resource_node.yield_amount,
                        tool_bonus: 1.0,
                    }),
                    required_ticks: 10,
                    target_entity: Some(resource_entity),
                    priority: 5,
                };
                
                if queue.enqueue(work) {
                    debug.log(
                        DebugLevel::Debug,
                        "WORK",
                        &format!("{} queued gathering {:?}",
                            name.name, resource_node.resource_type)
                    );
                }
                
                break; // Only queue one resource at a time
            }
        }
    }
}

/// System to update work effects (exhaustion, skill gain, etc.)
pub fn work_effects_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        &WorkProgress,
        &mut UnitNeedsV2,
        &mut WorkSpeed,
        &NameComponent,
    ), With<PeasantTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (work, _needs, mut speed, name) in units.iter_mut() {
        if !work.is_working {
            continue;
        }
        
        // Working increases hunger faster
        // This is in addition to normal tick_update
        // (handled in main work system)
        
        // Gain experience (simplified)
        if sim_state.tick % 100 == 0 && work.progress() > 0.5 {
            speed.global_modifier = (speed.global_modifier + 0.01).min(2.0);
            
            debug.log(
                DebugLevel::Debug,
                "SKILL",
                &format!("{} work speed improved to {:.0}%",
                    name.name, speed.global_modifier * 100.0)
            );
        }
    }
}

/// System to add work components to units
pub fn add_work_components_system(
    mut commands: Commands,
    query: Query<Entity, (With<PeasantTag>, Without<WorkProgress>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            WorkProgress::new(),
            WorkSpeed::default(),
            WorkQueue::new(10),
        ));
        
        println!("Added work components to entity {:?}", entity);
    }
}

/// Performance monitoring for work systems
pub fn work_performance_monitor_system(
    sim_state: Res<SimulationState>,
    working_units: Query<&WorkProgress, With<PeasantTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only check every 100 ticks
    if !sim_state.just_ticked || sim_state.tick % 100 != 0 {
        return;
    }
    
    let total = working_units.iter().count();
    let working = working_units.iter().filter(|w| w.is_working).count();
    let idle = total - working;
    
    debug.log(
        DebugLevel::Debug,
        "PERFORMANCE",
        &format!("Work system: {} working, {} idle ({}% utilization)",
            working, idle, (working * 100) / total.max(1))
    );
}