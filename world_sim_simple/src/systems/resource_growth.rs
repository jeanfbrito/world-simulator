use crate::components::{GrowingResource, GrowthUpdate, ResourceGrowthEvent, ResourceNode};
use crate::debug::{DebugLevel, DebugSystem};
use crate::SimulationState;
use bevy::prelude::*;

/// System that handles growth and regeneration of resources
pub fn resource_growth_system(
    sim_state: Res<SimulationState>,
    mut resources: Query<(Entity, &mut GrowingResource, Option<&mut ResourceNode>)>,
    mut events: EventWriter<ResourceGrowthEvent>,
    debug: Res<DebugSystem>,
) {
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }

    for (entity, mut growing, resource_node) in resources.iter_mut() {
        // First sync FROM ResourceNode if it has been modified (e.g., by gathering)
        if let Some(node) = &resource_node {
            if node.amount < growing.harvestable_amount {
                // ResourceNode was depleted by external system (e.g., food_gathering)
                let harvested = growing.harvestable_amount - node.amount;
                growing.harvest(harvested);
                debug.log(
                    DebugLevel::Debug,
                    "GROWTH",
                    &format!(
                        "Detected external harvest of {} from {:?}, updating GrowingResource",
                        harvested, growing.resource_type
                    ),
                );
            }
        }
        
        let update = growing.tick_update();

        // Then sync TO ResourceNode if growth happened
        if let Some(mut node) = resource_node {
            if node.amount != growing.harvestable_amount {
                node.amount = growing.harvestable_amount;
                debug.log(
                    DebugLevel::Debug,
                    "GROWTH",
                    &format!(
                        "Synced ResourceNode amount to {} for {:?} after growth",
                        node.amount, growing.resource_type
                    ),
                );
            }
        }

        // Log and emit events for significant changes
        match &update {
            GrowthUpdate::NoChange => {}
            GrowthUpdate::Regenerated(amount) => {
                debug.log(
                    DebugLevel::Info,
                    "GROWTH",
                    &format!("{:?} regenerated {} units", growing.resource_type, amount),
                );
                events.send(ResourceGrowthEvent {
                    entity,
                    resource_type: growing.resource_type,
                    update: update.clone(),
                    new_amount: growing.current_amount,
                });
            }
            GrowthUpdate::Ripened(amount) => {
                debug.log(
                    DebugLevel::Info,
                    "GROWTH",
                    &format!("{} berries ripened", amount),
                );
                events.send(ResourceGrowthEvent {
                    entity,
                    resource_type: growing.resource_type,
                    update: update.clone(),
                    new_amount: growing.current_amount,
                });
            }
            GrowthUpdate::Replenished(amount) => {
                debug.log(
                    DebugLevel::Debug,
                    "GROWTH",
                    &format!("Resource replenished to {}", amount),
                );
                events.send(ResourceGrowthEvent {
                    entity,
                    resource_type: growing.resource_type,
                    update: update.clone(),
                    new_amount: growing.current_amount,
                });
            }
            GrowthUpdate::StageChanged(stage) => {
                debug.log(
                    DebugLevel::Info,
                    "GROWTH",
                    &format!("Tree changed to stage: {}", stage),
                );
                events.send(ResourceGrowthEvent {
                    entity,
                    resource_type: growing.resource_type,
                    update: update.clone(),
                    new_amount: growing.current_amount,
                });
            }
            GrowthUpdate::Regrown => {
                debug.log(
                    DebugLevel::Info,
                    "GROWTH",
                    "Tree regrown from stump",
                );
                events.send(ResourceGrowthEvent {
                    entity,
                    resource_type: growing.resource_type,
                    update: update.clone(),
                    new_amount: growing.current_amount,
                });
            }
        }
    }
}

/// System that handles harvesting from GrowingResource
pub fn harvest_growing_resource_system(
    mut resources: Query<(&mut GrowingResource, &mut ResourceNode)>,
    debug: Res<DebugSystem>,
) {
    for (mut growing, mut node) in resources.iter_mut() {
        // If ResourceNode was depleted by work system, update GrowingResource
        if node.amount < growing.harvestable_amount {
            let harvested = growing.harvestable_amount - node.amount;
            growing.harvest(harvested);
            
            debug.log(
                DebugLevel::Debug,
                "HARVEST",
                &format!(
                    "Harvested {} from {:?}, {} remaining",
                    harvested, growing.resource_type, growing.harvestable_amount
                ),
            );
        }
    }
}