//! Movement system for entity pathfinding and movement

use bevy_ecs::prelude::*;
use world_sim_interface::{Position, EntityId};
use crate::components::*;
use crate::tilemap::{PathRequestEvent, PathCallback, pathfinding::PathComponent};

/// System for processing movement
pub fn movement_system(
    mut query: Query<(&mut PositionComponent, &mut MovementComponent, Entity)>,
) {
    for (mut position, mut movement, _entity) in query.iter_mut() {
        if let Some(target) = movement.target {
            // Simple movement towards target
            if position.position != target {
                // Get next position in path or move directly
                if let Some(next_pos) = movement.get_next_position() {
                    position.position = next_pos;
                } else {
                    // Direct movement (simplified)
                    position.move_towards(&target, 1);
                }
                
                // Check if reached target
                if position.position == target {
                    movement.clear();
                }
            }
        }
    }
}

/// System for pathfinding using tilemap
pub fn pathfinding_system(
    mut query: Query<(Entity, &PositionComponent, &mut MovementComponent), Changed<MovementComponent>>,
    mut path_events: EventWriter<PathRequestEvent>,
) {
    for (entity, position, movement) in query.iter() {
        if let Some(target) = movement.target {
            if movement.path.is_empty() {
                // Request path from tilemap pathfinding system
                path_events.send(PathRequestEvent {
                    entity,
                    start: position.position,
                    target,
                    callback: PathCallback::Movement,
                });
            }
        }
    }
}

/// Calculate a simple path between two positions
fn calculate_path(from: Position, to: Position) -> Vec<Position> {
    let mut path = Vec::new();
    let mut current = from;
    
    while current != to {
        // Move one step at a time towards target
        let dx = (to.x - current.x).signum();
        let dy = (to.y - current.y).signum();
        
        current.x += dx;
        current.y += dy;
        
        path.push(current);
        
        // Safety limit
        if path.len() > 1000 {
            break;
        }
    }
    
    path
}

/// System for handling move commands
pub fn handle_move_commands(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut MovementComponent)>,
    mut move_requests: ResMut<MoveRequests>,
) {
    for request in move_requests.drain() {
        // Find entity
        if let Some((entity, mut movement)) = entities.iter_mut()
            .find(|(e, _)| e.index() as EntityId == request.entity_id)
        {
            movement.set_target(request.target);
        }
    }
}

/// Request to move an entity
#[derive(Debug, Clone)]
pub struct MoveRequest {
    pub entity_id: EntityId,
    pub target: Position,
}

/// Resource for move requests
#[derive(Resource, Default)]
pub struct MoveRequests {
    requests: Vec<MoveRequest>,
}

impl MoveRequests {
    pub fn add(&mut self, entity_id: EntityId, target: Position) {
        self.requests.push(MoveRequest { entity_id, target });
    }
    
    pub fn drain(&mut self) -> std::vec::Drain<MoveRequest> {
        self.requests.drain(..)
    }
}