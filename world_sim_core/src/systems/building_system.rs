//! Building construction and management system

use bevy_ecs::prelude::*;
use world_sim_interface::{BuildingType, Position, EntityId, EngineEvent};
use crate::components::*;

/// System for processing building construction
pub fn building_system(
    mut query: Query<(&mut BuildingComponent, &PositionComponent, Entity)>,
    workers: Query<&WorkerComponent>,
    mut events: ResMut<super::EventQueue>,
) {
    for (mut building, _position, entity) in query.iter_mut() {
        if !building.is_complete {
            // Check if workers are assigned
            let worker_count = building.assigned_workers
                .iter()
                .filter(|id| workers.iter().any(|_| true))
                .count();
            
            if worker_count > 0 {
                // Progress construction
                let progress = 0.05 * worker_count as f32;
                building.add_construction_progress(progress);
                
                if building.is_complete {
                    // Emit completion event
                    events.push(EngineEvent::BuildingCompleted {
                        building_id: entity.index() as EntityId,
                        building_type: building.building_type,
                    });
                    
                    events.push(EngineEvent::ConstructionCompleted {
                        building_id: entity.index() as EntityId,
                        building_type: building.building_type,
                        position: Position::default(), // TODO: Get actual position
                    });
                }
            }
        }
    }
}

/// System for handling build commands
pub fn handle_build_commands(
    mut commands: Commands,
    workers: Query<(&InventoryComponent, Entity), With<WorkerComponent>>,
    existing_buildings: Query<&PositionComponent, With<BuildingComponent>>,
    mut build_requests: ResMut<BuildRequests>,
    mut events: ResMut<super::EventQueue>,
) {
    for request in build_requests.drain() {
        // Check position is not occupied
        let occupied = existing_buildings.iter()
            .any(|pos| pos.position == request.position);
        
        if occupied {
            continue; // Can't build here
        }
        
        // Find builder
        if let Some((inventory, worker_entity)) = workers.iter()
            .find(|(_, e)| e.index() as EntityId == request.builder_id)
        {
            // Check resources (simplified - just check for wood)
            let wood_required = match request.building_type {
                BuildingType::House => 10,
                BuildingType::Stockpile => 5,
                BuildingType::Sawmill => 15,
                _ => 10,
            };
            
            if inventory.get_amount(world_sim_interface::ResourceType::Wood) >= wood_required {
                // Spawn building entity
                let building_entity = commands.spawn((
                    BuildingComponent::new(request.building_type)
                        .with_owner(request.builder_id),
                    PositionComponent::at(request.position),
                    StorageComponent::new(100),
                    ProductionComponent::new(1.0),
                )).id();
                
                // Emit event
                events.push(EngineEvent::ConstructionStarted {
                    building_type: request.building_type,
                    position: request.position,
                    builder_id: request.builder_id,
                });
                
                events.push(EngineEvent::BuildingConstructionStarted {
                    building_id: building_entity.index() as EntityId,
                    building_type: request.building_type,
                    position: request.position,
                    workers: vec![request.builder_id],
                    estimated_completion: 0,
                });
            }
        }
    }
}

/// Request to build a structure
#[derive(Debug, Clone)]
pub struct BuildRequest {
    pub builder_id: EntityId,
    pub building_type: BuildingType,
    pub position: Position,
}

/// Resource for build requests
#[derive(Resource, Default)]
pub struct BuildRequests {
    requests: Vec<BuildRequest>,
}

impl BuildRequests {
    pub fn add(&mut self, builder_id: EntityId, building_type: BuildingType, position: Position) {
        self.requests.push(BuildRequest { builder_id, building_type, position });
    }
    
    pub fn drain(&mut self) -> std::vec::Drain<BuildRequest> {
        self.requests.drain(..)
    }
}