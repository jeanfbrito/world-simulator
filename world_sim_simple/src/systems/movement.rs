/// Tick-based movement system
/// 
/// This system handles all unit movement on the simulation tick,
/// updating grid positions discretely. Visual interpolation happens
/// separately for smooth presentation.

use bevy::prelude::*;
use crate::components::{
    GridPosition, VisualPosition, GridMovement,
    NameComponent, PeasantTag, MovementSpeed, MovementEffects
};
use crate::{SimulationState, WorldMap, TILE_SIZE};
use crate::ai::ActionPlan;
use colored::Colorize;

/// System that processes movement on each simulation tick
pub fn tick_movement_system(
    sim_state: Res<SimulationState>,
    world_map: Res<WorldMap>,
    mut units: Query<(
        Entity,
        &mut GridPosition,
        &mut GridMovement,
        &mut VisualPosition,
        &NameComponent,
        Option<&MovementSpeed>,
        Option<&MovementEffects>,
        Option<&ActionPlan>,
    ), With<PeasantTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, mut grid_pos, mut movement, mut visual_pos, name, speed, effects, plan) in units.iter_mut() {
        // Skip if not moving
        if !movement.is_moving {
            continue;
        }
        
        let old_pos = grid_pos.clone();
        
        // Calculate effective movement speed
        let base_ticks = speed.map(|s| s.get_ticks_per_tile()).unwrap_or(3);
        let modifier = effects.map(|e| e.get_total_modifier()).unwrap_or(1.0);
        let effective_ticks = ((base_ticks as f32 / modifier).max(1.0)) as u32;
        
        // Update movement progress with unit-specific speed
        let completed = movement.tick_update(&mut grid_pos, effective_ticks);
        
        // If position changed, update visual target
        if old_pos != *grid_pos {
            visual_pos.set_target(&grid_pos, TILE_SIZE);
            
            // Log movement
            debug.log(
                DebugLevel::Debug,
                "MOVEMENT",
                &format!("{} moved from ({},{}) to ({},{})",
                    name.name, old_pos.x, old_pos.y, grid_pos.x, grid_pos.y)
            );
            
            // Check if new position is valid
            if !is_position_walkable(&world_map, &grid_pos) {
                // Revert movement if blocked
                *grid_pos = old_pos;
                movement.stop();
                
                debug.log(
                    DebugLevel::Info,
                    "MOVEMENT",
                    &format!("{} movement blocked at ({},{})",
                        name.name, grid_pos.x, grid_pos.y)
                );
            }
        }
        
        if completed {
            println!("{} {} reached destination at ({},{})",
                "📍".green(),
                name.name.cyan(),
                grid_pos.x, grid_pos.y
            );
            
            debug.log(
                DebugLevel::Info,
                "MOVEMENT",
                &format!("{} completed movement to ({},{})",
                    name.name, grid_pos.x, grid_pos.y)
            );
        }
    }
}

/// System that interpolates visual positions every frame for smooth movement
pub fn visual_interpolation_system(
    time: Res<Time>,
    mut units: Query<(&mut Transform, &mut VisualPosition), With<PeasantTag>>,
) {
    let delta = time.delta_secs();
    
    for (mut transform, mut visual_pos) in units.iter_mut() {
        // Interpolate visual position
        visual_pos.interpolate(delta);
        
        // Update transform for rendering
        transform.translation = visual_pos.current;
    }
}

/// System to handle movement requests from AI plans
pub fn movement_request_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity,
        &GridPosition,
        &mut GridMovement,
        &ActionPlan,
        &NameComponent,
    ), With<PeasantTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, grid_pos, mut movement, plan, name) in units.iter_mut() {
        // Skip if already moving
        if movement.is_moving {
            continue;
        }
        
        // Check if current action requires movement
        if let Some(action) = plan.current_action() {
            // Parse movement from action effects
            if let Some(target) = extract_movement_target(action) {
                // Simple pathfinding (for now just move directly)
                let path = simple_pathfind(grid_pos, &target);
                
                if !path.is_empty() {
                    movement.set_path(path.clone());
                    
                    debug.log(
                        DebugLevel::Info,
                        "MOVEMENT",
                        &format!("{} starting movement to ({},{}) with {} steps",
                            name.name, target.x, target.y, path.len())
                    );
                }
            }
        }
    }
}

/// System to update legacy TileEntity positions from GridPosition
pub fn sync_tile_entity_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(&GridPosition, &mut crate::TileEntity), Changed<GridPosition>>,
) {
    // Only sync on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (grid_pos, mut tile_entity) in units.iter_mut() {
        tile_entity.x = grid_pos.x as usize;
        tile_entity.y = grid_pos.y as usize;
    }
}

/// System to update legacy PositionComponent from GridPosition
pub fn sync_position_component_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(&GridPosition, &mut crate::components::PositionComponent), Changed<GridPosition>>,
) {
    // Only sync on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (grid_pos, mut position) in units.iter_mut() {
        position.x = grid_pos.x as f32 * TILE_SIZE;
        position.y = grid_pos.y as f32 * TILE_SIZE;
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Check if a position is walkable on the map
fn is_position_walkable(world_map: &WorldMap, pos: &GridPosition) -> bool {
    if pos.x >= crate::MAP_SIZE as u32 || pos.y >= crate::MAP_SIZE as u32 {
        return false;
    }
    
    world_map.tiles[pos.y as usize][pos.x as usize].is_walkable()
}

/// Extract movement target from action (temporary implementation)
fn extract_movement_target(action: &crate::ai::GoapAction) -> Option<GridPosition> {
    // This is a simplified implementation
    // In reality, you'd parse the action's effects or have specific movement actions
    
    match action.name.as_str() {
        "move_to_resource" => {
            // TODO: Get actual resource position
            Some(GridPosition::new(30, 30))
        }
        "move_to_storage" => {
            // Central storage position
            Some(GridPosition::new(32, 32))
        }
        "move_to_home" => {
            // TODO: Get actual home position
            Some(GridPosition::new(35, 35))
        }
        _ => None,
    }
}

/// Simple pathfinding (straight line for now)
fn simple_pathfind(from: &GridPosition, to: &GridPosition) -> Vec<GridPosition> {
    let mut path = Vec::new();
    let mut current = from.clone();
    
    // Simple straight-line pathfinding
    // TODO: Replace with A* pathfinding
    while current != *to {
        current = current.step_toward(to);
        path.push(current.clone());
    }
    
    path
}

/// Performance monitoring for movement system
pub fn movement_performance_monitor_system(
    sim_state: Res<SimulationState>,
    moving_units: Query<&GridMovement, (With<PeasantTag>, With<GridMovement>)>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only check every 100 ticks
    if !sim_state.just_ticked || sim_state.tick % 100 != 0 {
        return;
    }
    
    let total_units = moving_units.iter().count();
    let moving_count = moving_units.iter().filter(|m| m.is_moving).count();
    
    debug.log(
        DebugLevel::Debug,
        "PERFORMANCE",
        &format!("Movement system: {}/{} units moving", moving_count, total_units)
    );
}