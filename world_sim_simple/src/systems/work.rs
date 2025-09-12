use crate::components::{
    GridPosition, NameComponent, QueuedWork, ResourceWork, UnitInventory, UnitNeedsV2, UnitTag,
    WorkProgress, WorkQueue, WorkSpeed, WorkType,
};
use crate::simulation::TickEvent;
use crate::SimulationState;
/// Tick-based work execution system
///
/// This system handles all work progress on simulation ticks,
/// including resource gathering, building, crafting, etc.
use bevy::prelude::*;
use colored::Colorize;

/// Main work execution system that runs on ticks
pub fn tick_work_system(
    mut tick_reader: EventReader<TickEvent>,
    mut units: Query<
        (
            Entity,
            &mut WorkProgress,
            Option<&WorkSpeed>,
            &mut UnitInventory,
            &mut UnitNeedsV2,
            &GridPosition,
            &NameComponent,
            Option<&mut crate::ai::ActionPlan>,
        ),
        With<UnitTag>,
    >,
    mut resources: Query<(&mut crate::components::ResourceNode, Option<&mut crate::components::growth::GrowingResource>)>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Process each tick event
    for tick_event in tick_reader.read() {
        println!("✅ Work system ACTIVE on tick {}", tick_event.tick);

        // Debug: Count working units
        let total_units = units.iter().count();
    let working_count = units
        .iter()
        .filter(|(_, work, _, _, _, _, _, _)| work.is_working)
        .count();
    
        // Always log to see if system is running
        println!("⚙️ Tick work system: {}/{} units working, tick {}", 
            working_count, total_units, tick_event.tick);
        
        if working_count > 0 {
            println!("  Found working units - processing...");
        }

        for (_entity, mut work, _speed, mut inventory, mut needs, position, name, plan) in
            units.iter_mut()
        {
            // Debug each unit's work state - only show if actually working
            if work.is_working {
                println!("  {} - is_working: {}, work_type: {:?}, progress: {}/{}", 
                    name.name, work.is_working, work.work_type.is_some(), 
                    work.progress_counter, work.required_ticks);
            }
            
            // Skip if not working
            if !work.is_working {
                continue;
            }

            // Consume energy while working
            needs.tick_update(); // Additional energy loss while working

            // Update work progress
            let completed = work.tick_update();

            // Debug log work progress
            if work.progress_counter % 10 == 0 && work.progress_counter > 0 {
                debug.log(
                    DebugLevel::Debug,
                    "WORK",
                    &format!(
                        "{} working: {}/{} ticks",
                        name.name, work.progress_counter, work.required_ticks
                    ),
                );
            }

            if completed {
                println!(
                    "🔨 {} work completed! Type: {:?}, Target: {:?}",
                    name.name.cyan(),
                    work.work_type,
                    work.target_entity
                );

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
                        _entity,
                        &work,
                    );
                } else {
                    println!(
                        "⚠️ {} work completed but NO WORK TYPE SET!",
                        name.name.yellow()
                    );
                }

                let work_type_str = work
                    .work_type
                    .as_ref()
                    .map(|w| format!("{:?}", w))
                    .unwrap_or_else(|| "work (no type)".to_string());

                println!(
                    "{} {} completed {}",
                    "✅".green(),
                    name.name.cyan(),
                    work_type_str
                );

                debug.log(
                    DebugLevel::Info,
                    "WORK_DEBUG",
                    &format!(
                        "Work completed - Type: {}, Target: {:?}",
                        work_type_str, work.target_entity
                    ),
                );

                debug.log(
                    DebugLevel::Info,
                    "WORK",
                    &format!(
                        "{} completed work at ({},{})",
                        name.name, position.x, position.y
                    ),
                );

                // NOW clear the work after we've handled it
                work.complete_work();

                // Advance GOAP plan when work completes
                if let Some(mut action_plan) = plan {
                    action_plan.advance();
                    debug.log(
                        DebugLevel::Info,
                        "WORK",
                        "Advanced GOAP plan to next action",
                    );
                }
            }
        }
    } // End of tick event loop
}

/// Handle work completion effects
fn handle_work_completion(
    work_type: &WorkType,
    inventory: &mut UnitInventory,
    _needs: &mut UnitNeedsV2,
    _position: &GridPosition,
    name: &NameComponent,
    target_entity: Option<Entity>,
    resources: &mut Query<(&mut crate::components::ResourceNode, Option<&mut crate::components::growth::GrowingResource>)>,
    debug: &crate::debug::DebugSystem,
    worker_entity: Entity,
    work_progress: &WorkProgress,
) {
    use crate::debug::DebugLevel;

    match work_type {
        WorkType::Gathering(resource_work) => {
            // First, deplete the resource if we have a target entity
            let mut resource_harvested = false;
            let mut actual_harvest_amount = resource_work.amount;
            
            if let Some(resource_entity) = target_entity {
                if let Ok((mut resource, growing)) = resources.get_mut(resource_entity) {
                    // If this resource has a GrowingResource component, use its harvest method
                    if let Some(mut growing_resource) = growing {
                        // Use GrowingResource's harvest method which handles depletion properly
                        actual_harvest_amount = growing_resource.harvest(resource_work.amount);
                        
                        // Sync the ResourceNode amount with GrowingResource
                        resource.amount = growing_resource.current_amount;
                        
                        resource_harvested = actual_harvest_amount > 0;
                        
                        if resource.amount == 0 {
                            println!(
                                "{} Resource fully harvested (took {})",
                                "⛏️".red(),
                                actual_harvest_amount
                            );
                        } else {
                            println!(
                                "{} Resource harvested {}, {} remaining",
                                "⛏️".yellow(),
                                actual_harvest_amount,
                                resource.amount
                            );
                        }
                    } else {
                        // Fallback to old behavior for resources without GrowingResource
                        if resource.amount >= resource_work.amount {
                            resource.amount -= resource_work.amount;
                            resource_harvested = true;

                            println!(
                                "{} Resource harvested, {} remaining",
                                "⛏️".yellow(),
                                resource.amount
                            );

                            debug.log(
                                DebugLevel::Debug,
                                "RESOURCE",
                                &format!(
                                    "Resource harvested {}, {} remaining",
                                    resource_work.amount, resource.amount
                                ),
                            );
                        } else if resource.amount > 0 {
                            // Take what's available
                            actual_harvest_amount = resource.amount;
                            resource.amount = 0;
                            resource_harvested = true;

                            println!(
                                "{} Resource fully harvested (took {})",
                                "⛏️".red(),
                                actual_harvest_amount
                            );
                        }
                    }
                }
            } else {
                // No target entity means we're harvesting from terrain (berries, etc)
                resource_harvested = true;
            }

            // Release claim on the resource when done
            if let Some(target_entity) = work_progress.target_entity {
                if let Ok((mut resource, _)) = resources.get_mut(target_entity) {
                    resource.claimed_by.remove(&worker_entity);
                    println!("   🔓 Released claim on resource (claims remaining: {}/{})", 
                        resource.claimed_by.len(), resource.max_workers);
                }
            }
            
            // Add to inventory if we successfully harvested
            if resource_harvested {
                // Debug: Check inventory state before adding
                println!(
                    "📦 {} inventory before: weight={}/{}, items={:?}",
                    name.name.cyan(),
                    inventory.current_weight,
                    inventory.max_weight,
                    inventory.items
                );

                let added = inventory.add_item(resource_work.resource_type, actual_harvest_amount);

                if added {
                    println!(
                        "{} {} gathered {} {}",
                        "🌲".green(),
                        name.name.cyan(),
                        actual_harvest_amount,
                        format!("{:?}", resource_work.resource_type).yellow()
                    );

                    // Debug: Check inventory state after adding
                    println!(
                        "📦 {} inventory after: weight={}/{}, items={:?}",
                        name.name.cyan(),
                        inventory.current_weight,
                        inventory.max_weight,
                        inventory.items
                    );

                    debug.log(
                        DebugLevel::Info,
                        "GATHER",
                        &format!(
                            "{} gathered {} {:?}",
                            name.name, actual_harvest_amount, resource_work.resource_type
                        ),
                    );
                } else {
                    println!(
                        "{} {} inventory full! weight={}/{}, trying to add {} of {:?} (weight={})",
                        "⚠️".yellow(),
                        name.name.yellow(),
                        inventory.current_weight,
                        inventory.max_weight,
                        resource_work.amount,
                        resource_work.resource_type,
                        resource_work.resource_type.weight() * resource_work.amount as f32
                    );
                }
            } else {
                println!(
                    "⚠️ {} NOT adding to inventory: resource_harvested={}, type={:?}",
                    name.name.yellow(),
                    resource_harvested,
                    resource_work.resource_type
                );
            }
        }

        WorkType::Building(_building_work) => {
            // TODO: Update building construction progress when buildings module is ready
            println!(
                "{} {} building work completed!",
                "🏗️".green(),
                name.name.cyan()
            );

            debug.log(
                DebugLevel::Info,
                "BUILD",
                &format!("{} completed building work", name.name),
            );
        }

        WorkType::Crafting(crafting_work) => {
            // Create crafted items
            // TODO: Implement recipe system
            println!(
                "{} {} crafted {}",
                "🔨".cyan(),
                name.name.cyan(),
                crafting_work.recipe_id.green()
            );

            debug.log(
                DebugLevel::Info,
                "CRAFT",
                &format!(
                    "{} crafted {} (x{})",
                    name.name, crafting_work.recipe_id, crafting_work.output_count
                ),
            );
        }

        _ => {
            // Generic work completion
            debug.log(
                DebugLevel::Debug,
                "WORK",
                &format!("{} completed {:?}", name.name, work_type),
            );
        }
    }
}

/// System to assign work from queues
pub fn work_assignment_system(
    mut tick_reader: EventReader<TickEvent>,
    mut units: Query<
        (
            Entity,
            &mut WorkProgress,
            &mut WorkQueue,
            &WorkSpeed,
            &NameComponent,
        ),
        With<UnitTag>,
    >,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Process each tick event
    for _tick_event in tick_reader.read() {
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
                    &format!("{} started {:?} ({}t)", name.name, queued.work_type, ticks),
                );
            }
        }
    }
}

/// System to queue gathering work when near resources
pub fn auto_gather_system(
    mut tick_reader: EventReader<TickEvent>,
    mut units: Query<
        (
            Entity,
            &GridPosition,
            &mut WorkQueue,
            &UnitInventory,
            &NameComponent,
        ),
        With<UnitTag>,
    >,
    resources: Query<(Entity, &GridPosition, &crate::components::ResourceNode)>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Process each tick event
    for tick_event in tick_reader.read() {
        // Only process every 10 ticks
        if tick_event.tick % 10 != 0 {
            continue;
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
                            &format!(
                                "{} queued gathering {:?}",
                                name.name, resource_node.resource_type
                            ),
                        );
                    }

                    break; // Only queue one resource at a time
                }
            }
        }
    }
}

/// System to update work effects (exhaustion, skill gain, etc.)
pub fn work_effects_system(
    mut tick_reader: EventReader<TickEvent>,
    mut units: Query<
        (
            &WorkProgress,
            &mut UnitNeedsV2,
            &mut WorkSpeed,
            &NameComponent,
        ),
        With<UnitTag>,
    >,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Process each tick event
    for tick_event in tick_reader.read() {
        for (work, _needs, mut speed, name) in units.iter_mut() {
            if !work.is_working {
                continue;
            }

            // Working increases hunger faster
            // This is in addition to normal tick_update
            // (handled in main work system)

            // Gain experience (simplified)
            if tick_event.tick % 100 == 0 && work.progress() > 0.5 {
                speed.global_modifier = (speed.global_modifier + 0.01).min(2.0);

                debug.log(
                    DebugLevel::Debug,
                    "SKILL",
                    &format!(
                        "{} work speed improved to {:.0}%",
                        name.name,
                        speed.global_modifier * 100.0
                    ),
                );
            }
        }
    }
}

/// System to add work components to units
pub fn add_work_components_system(
    mut commands: Commands,
    query: Query<Entity, (With<UnitTag>, Without<WorkProgress>)>,
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
    mut tick_reader: EventReader<TickEvent>,
    working_units: Query<&WorkProgress, With<UnitTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Process each tick event
    for tick_event in tick_reader.read() {
        // Only check every 100 ticks
        if tick_event.tick % 100 != 0 {
            continue;
        }

        let total = working_units.iter().count();
        let working = working_units.iter().filter(|w| w.is_working).count();
        let idle = total - working;

        debug.log(
            DebugLevel::Debug,
            "PERFORMANCE",
            &format!(
                "Work system: {} working, {} idle ({}% utilization)",
                working,
                idle,
                (working * 100) / total.max(1)
            ),
        );
    }
}
