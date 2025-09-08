//! Harvest system for resource collection

use bevy_ecs::prelude::*;
use world_sim_interface::{EntityId, EngineEvent, ResourceType};
use crate::components::*;

/// System for processing harvest tasks
pub fn harvest_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut WorkerComponent, &mut InventoryComponent, &PositionComponent, Option<&mut HarvestingComponent>)>,
    mut resources: Query<(Entity, &mut ResourceNodeComponent, &PositionComponent), Without<WorkerComponent>>,
    mut events: ResMut<super::EventQueue>,
) {
    for (worker_entity, mut worker, mut inventory, worker_pos, harvesting) in query.iter_mut() {
        // Check if worker is harvesting
        if let Some(mut harvest) = harvesting {
            // Find the target resource
            if let Some((resource_entity, mut resource, resource_pos)) = resources.iter_mut()
                .find(|(e, _, _)| e.index() as EntityId == harvest.target_resource) 
            {
                // Check distance
                if worker_pos.is_adjacent_to(resource_pos) {
                    // Update harvest progress
                    if harvest.update(0.1) {
                        // Harvest complete
                        let amount = 5.min(resource.amount);
                        let harvested = resource.harvest(amount);
                        
                        // Add to inventory
                        inventory.add_resource(resource.resource_type, harvested);
                        
                        // Emit event
                        events.push(EngineEvent::HarvestCompleted {
                            worker_id: worker_entity.index() as EntityId,
                            resource_id: resource_entity.index() as EntityId,
                            amount: harvested,
                        });
                        
                        events.push(EngineEvent::ResourceCollected {
                            worker_id: worker_entity.index() as EntityId,
                            resource_type: resource.resource_type,
                            amount: harvested,
                        });
                        
                        // Reset harvest
                        harvest.reset();
                        
                        // Remove component if resource depleted
                        if resource.is_depleted() {
                            commands.entity(resource_entity).despawn();
                            events.push(EngineEvent::ResourceDepleted {
                                resource_id: resource_entity.index() as EntityId,
                                will_regenerate: false,
                                regeneration_time: None,
                            });
                        }
                    }
                }
            }
        }
    }
}

/// System for starting harvest tasks
pub fn start_harvest_system(
    mut commands: Commands,
    workers: Query<(Entity, &PositionComponent), With<WorkerComponent>>,
    resources: Query<(Entity, &ResourceNodeComponent, &PositionComponent), Without<WorkerComponent>>,
    mut harvest_requests: ResMut<super::HarvestRequests>,
    mut events: ResMut<super::EventQueue>,
) {
    for request in harvest_requests.drain() {
        // Find worker
        if let Some((worker_entity, _worker_pos)) = workers.iter()
            .find(|(e, _)| e.index() as EntityId == request.worker_id)
        {
            // Find resource
            if let Some((resource_entity, _resource, _resource_pos)) = resources.iter()
                .find(|(e, _, _)| e.index() as EntityId == request.resource_id)
            {
                // Add harvesting component
                commands.entity(worker_entity).insert(HarvestingComponent::new(request.resource_id));
                
                // Emit event
                events.push(EngineEvent::HarvestStarted {
                    worker_id: request.worker_id,
                    resource_id: request.resource_id,
                });
            }
        }
    }
}

/// Request to start harvesting
#[derive(Debug, Clone)]
pub struct HarvestRequest {
    pub worker_id: EntityId,
    pub resource_id: EntityId,
}

/// Resource for harvest requests
#[derive(Resource, Default)]
pub struct HarvestRequests {
    requests: Vec<HarvestRequest>,
}

impl HarvestRequests {
    pub fn add(&mut self, worker_id: EntityId, resource_id: EntityId) {
        self.requests.push(HarvestRequest { worker_id, resource_id });
    }
    
    pub fn drain(&mut self) -> std::vec::Drain<HarvestRequest> {
        self.requests.drain(..)
    }
}