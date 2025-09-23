use bevy::prelude::*;
use crate::components::{
    GridPosition, GridMovement, UnitInventory, NameComponent, UnitTag, UnitMind,
    resource::ResourceNode, WorkProgress, WorkType, ResourceWork, ClaimedResource,
};
use crate::resources::ResourceType;
use crate::{SimulationState, ai::BerryBushTag};
use crate::debug::{DebugSystem, DebugLevel};
use colored::Colorize;

/// System that handles resource harvesting when units are at resource nodes
/// This system ONLY handles the work of transferring resources from nodes to inventory
/// Eating is handled separately by the consumption system
pub fn resource_harvesting_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity,
        &GridPosition,
        &mut UnitInventory,
        &mut WorkProgress,
        &mut UnitMind,
        &mut ClaimedResource,
        &NameComponent,
    ), With<UnitTag>>,
    mut resources: Query<(
        Entity,
        &GridPosition,
        &mut ResourceNode,
    )>,
    debug: Res<DebugSystem>,
) {
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (unit_entity, unit_pos, mut inventory, mut work_progress, mut mind, mut claimed_resource, name) in units.iter_mut() {
        // Check if unit has a claimed resource
        if let Some(resource_entity) = claimed_resource.get_claimed() {
            // Get the resource
            if let Ok((_, resource_pos, mut resource)) = resources.get_mut(resource_entity) {
                // Check if unit is adjacent to the resource (not on the same tile)
                if unit_pos.is_adjacent_to(&resource_pos) {
                    // Check if resource still has items and inventory has space
                    if resource.amount > 0 && !inventory.is_full() {
                        // Start or continue harvesting work
                        if !work_progress.is_working {
                            // Start new harvesting work
                            let work_type = WorkType::Gathering(ResourceWork {
                                resource_type: resource.resource_type.clone(),
                                amount: resource.yield_amount.min(resource.amount),
                                tool_bonus: 1.0, // TODO: Check for tools
                            });
                            
                            // 10 ticks to harvest (1 second at 10 TPS)
                            work_progress.start_work(work_type, 10, Some(resource_entity));
                            *mind = UnitMind::Gathering {
                                resource: format!("{:?}", resource.resource_type),
                            };
                            
                            debug.log(
                                DebugLevel::Info,
                                "HARVESTING",
                                &format!(
                                    "{} started harvesting {:?} at ({},{})",
                                    name.name, resource.resource_type, resource_pos.x, resource_pos.y
                                ),
                            );
                        }
                        
                        // Update work progress
                        if work_progress.tick_update() {
                            // Work completed! Transfer resources to inventory
                            if let Some(WorkType::Gathering(ref work)) = work_progress.work_type {
                                let amount_to_harvest = work.amount.min(resource.amount);
                                
                                // Try to add to inventory
                                if inventory.add_item(work.resource_type.clone(), amount_to_harvest) {
                                    // Remove from resource node
                                    resource.amount -= amount_to_harvest;
                                    
                                    debug.log(
                                        DebugLevel::Info,
                                        "HARVEST_COMPLETE",
                                        &format!(
                                            "{} harvested {} {:?}, inventory now has {}",
                                            name.name, 
                                            amount_to_harvest,
                                            work.resource_type,
                                            inventory.get_amount(work.resource_type.clone())
                                        ),
                                    );
                                    
                                    println!(
                                        "{} {} harvested {} {:?}! Inventory: {}",
                                        "⛏️".cyan(),
                                        name.name.green(),
                                        amount_to_harvest,
                                        work.resource_type,
                                        inventory.get_amount(work.resource_type.clone())
                                    );
                                    
                                    // Check if should continue harvesting or stop
                                    if resource.amount == 0 || inventory.is_full() {
                                        // Resource depleted or inventory full - release claim
                                        resource.release_claim(unit_entity);
                                        claimed_resource.release();
                                        work_progress.complete_work();
                                        *mind = UnitMind::Idle;
                                        
                                        debug.log(
                                            DebugLevel::Info,
                                            "HARVEST_DONE",
                                            &format!(
                                                "{} finished harvesting (resource: {}, inventory full: {})",
                                                name.name,
                                                resource.amount,
                                                inventory.is_full()
                                            ),
                                        );
                                    } else {
                                        // More to harvest - start another work cycle
                                        work_progress.complete_work();
                                    }
                                } else {
                                    // Inventory full - stop harvesting
                                    resource.release_claim(unit_entity);
                                    claimed_resource.release();
                                    work_progress.complete_work();
                                    *mind = UnitMind::Idle;
                                    
                                    debug.log(
                                        DebugLevel::Info,
                                        "INVENTORY_FULL",
                                        &format!("{} inventory full, stopping harvest", name.name),
                                    );
                                }
                            }
                        }
                    } else {
                        // Can't harvest (resource empty or inventory full) - release claim
                        resource.release_claim(unit_entity);
                        claimed_resource.release();
                        work_progress.cancel_work();
                        *mind = UnitMind::Idle;
                        
                        debug.log(
                            DebugLevel::Debug,
                            "HARVEST_BLOCKED",
                            &format!(
                                "{} can't harvest (resource: {}, inv full: {})",
                                name.name, resource.amount, inventory.is_full()
                            ),
                        );
                    }
                } else {
                    // Unit not at resource location anymore - this shouldn't happen normally
                    // but we should handle it by releasing the claim
                    if resource.is_claimed_by(unit_entity) {
                        resource.release_claim(unit_entity);
                    }
                    claimed_resource.release();
                    work_progress.cancel_work();
                    
                    debug.log(
                        DebugLevel::Debug,
                        "CLAIM_ABANDONED",
                        &format!(
                            "{} abandoned claim (moved away from resource)",
                            name.name
                        ),
                    );
                }
            } else {
                // Resource entity doesn't exist anymore - clear claim
                claimed_resource.release();
                work_progress.cancel_work();
            }
        }
    }
}

/// System to periodically clean up expired claims on all resources
/// This runs every 50 ticks to remove claims from disconnected or dead units
pub fn cleanup_expired_resource_claims(
    sim_state: Res<SimulationState>,
    mut resources: Query<&mut ResourceNode>,
    debug: Res<DebugSystem>,
) {
    // Only run every 50 ticks to reduce overhead
    if sim_state.tick % 50 != 0 {
        return;
    }

    let mut total_expired = 0;
    for mut resource in resources.iter_mut() {
        let before_count = resource.claim_count();
        resource.cleanup_expired_claims(sim_state.tick);
        let after_count = resource.claim_count();

        if before_count > after_count {
            total_expired += before_count - after_count;
            debug.log(
                DebugLevel::Debug,
                "CLAIM_EXPIRY",
                &format!(
                    "Cleaned up {} expired claims on resource (was {}, now {})",
                    before_count - after_count, before_count, after_count
                ),
            );
        }
    }

    if total_expired > 0 {
        debug.log(
            DebugLevel::Info,
            "CLAIM_CLEANUP",
            &format!("Cleaned up {} total expired resource claims", total_expired),
        );
    }
}

/// System to handle releasing claims when units change targets
pub fn claim_cleanup_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity,
        &GridMovement,
        &mut ClaimedResource,
    ), (With<UnitTag>, Changed<GridMovement>)>,
    mut resources: Query<&mut ResourceNode>,
    debug: Res<DebugSystem>,
) {
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (unit_entity, movement, mut claimed_resource) in units.iter_mut() {
        // If unit started moving to a new location, release any existing claim
        if movement.is_moving && claimed_resource.has_claim() {
            if let Some(resource_entity) = claimed_resource.get_claimed() {
                if let Ok(mut resource) = resources.get_mut(resource_entity) {
                    if resource.is_claimed_by(unit_entity) {
                        resource.release_claim(unit_entity);
                        
                        debug.log(
                            DebugLevel::Debug,
                            "CLAIM_RELEASED",
                            &format!("Unit released claim (started moving)"),
                        );
                    }
                }
            }
            claimed_resource.release();
        }
    }
}