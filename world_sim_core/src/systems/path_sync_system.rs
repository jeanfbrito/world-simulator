//! System to sync tilemap pathfinding results with movement components

use bevy_ecs::prelude::*;
use crate::components::MovementComponent;
use crate::tilemap::pathfinding::{PathComponent, PathReady, NoPathFound};

/// System to sync path results from tilemap pathfinding to movement components
pub fn sync_path_to_movement_system(
    mut query: Query<(
        Entity,
        &mut MovementComponent,
        Option<&PathComponent>,
        Option<&PathReady>,
        Option<&NoPathFound>,
    )>,
    mut commands: Commands,
) {
    for (entity, mut movement, path_comp, ready, no_path) in query.iter_mut() {
        // Handle successful pathfinding
        if ready.is_some() {
            if let Some(path_comp) = path_comp {
                // Copy path to movement component
                movement.set_path(path_comp.path.clone());
                
                // Remove the ready marker
                commands.entity(entity).remove::<PathReady>();
            }
        }
        
        // Handle failed pathfinding
        if no_path.is_some() {
            // Clear the target as it's unreachable
            movement.clear();
            
            // Remove the marker
            commands.entity(entity).remove::<NoPathFound>();
            
            // Could emit an event here to notify AI that path failed
        }
    }
}

/// System to update movement from path progress
pub fn update_movement_from_path_system(
    mut query: Query<(&mut MovementComponent, &PathComponent), Changed<PathComponent>>,
) {
    for (mut movement, path_comp) in query.iter_mut() {
        if !path_comp.is_complete() {
            // Sync the path progress
            let remaining_path = path_comp.path[path_comp.current_index..].to_vec();
            movement.set_path(remaining_path);
        } else {
            // Path complete, clear movement
            movement.clear();
        }
    }
}