use crate::components::{
    GridMovement, GridPosition, MovementEffects, MovementSpeed, NameComponent, UnitTag,
    VisualPosition, UnitMind,
};
use crate::{SimulationState, WorldMap, TILE_SIZE};
/// Tick-based movement system
///
/// This system handles all unit movement on the simulation tick,
/// updating grid positions discretely. Visual interpolation happens
/// separately for smooth presentation.
use bevy::prelude::*;
use colored::Colorize;
use rand::Rng;

fn is_position_walkable(_world_map: &WorldMap, pos: &GridPosition) -> bool {
    // Simple bounds check for now
    // TODO: Implement proper collision detection with WorldMap
    pos.x < 64 && pos.y < 64
}

/// System that processes movement on each simulation tick
pub fn tick_movement_system(
    sim_state: Res<SimulationState>,
    world_map: Res<WorldMap>,
    mut units: Query<
        (
            Entity,
            &mut GridPosition,
            &mut GridMovement,
            &mut VisualPosition,
            &NameComponent,
            Option<&MovementSpeed>,
            Option<&MovementEffects>,
        ),
        With<UnitTag>,
    >,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }

    // Debug: count units being processed
    let mut moving_count = 0;
    let total_count = units.iter().count();
    
    // Check if any units exist
    if total_count == 0 {
        println!("WARNING: No units found with UnitTag, GridPosition, GridMovement, and VisualPosition!");
        return;
    }

    for (entity, mut grid_pos, mut movement, mut visual_pos, name, speed, effects) in
        units.iter_mut()
    {
        
        // Skip if not moving
        if !movement.is_moving {
            continue;
        }
        
        moving_count += 1;

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
                &format!(
                    "{} moved from ({},{}) to ({},{})",
                    name.name, old_pos.x, old_pos.y, grid_pos.x, grid_pos.y
                ),
            );

            // Check if new position is valid
            if !is_position_walkable(&world_map, &grid_pos) {
                // Revert movement if blocked
                *grid_pos = old_pos;
                movement.stop();

                debug.log(
                    DebugLevel::Info,
                    "MOVEMENT",
                    &format!(
                        "{} movement blocked at ({},{})",
                        name.name, grid_pos.x, grid_pos.y
                    ),
                );
            }
        }

        if completed {
            println!(
                "{} {} reached destination at ({},{})",
                "📍".green(),
                name.name.cyan(),
                grid_pos.x,
                grid_pos.y
            );

            debug.log(
                DebugLevel::Info,
                "MOVEMENT",
                &format!(
                    "{} completed movement to ({},{})",
                    name.name, grid_pos.x, grid_pos.y
                ),
            );
        }
    }
    
    // Debug: log how many units were processed
    if moving_count > 0 || total_count > 0 {
        println!("Movement system: {}/{} units moving", moving_count, total_count);
    }
}

/// System that interpolates visual positions every frame for smooth movement
pub fn visual_interpolation_system(
    time: Res<Time>,
    mut units: Query<(&mut Transform, &mut VisualPosition), With<UnitTag>>,
) {
    let delta = time.delta_secs();

    for (mut transform, mut visual_pos) in units.iter_mut() {
        // Interpolate visual position
        visual_pos.interpolate(delta);

        // Update transform for rendering
        transform.translation = visual_pos.current;
    }
}

/// System to sync TileEntity with GridPosition changes
pub fn sync_tile_entity_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(&GridPosition, &mut crate::TileEntity), (With<UnitTag>, Changed<GridPosition>)>,
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

/*
/// System to handle movement requests from AI plans
pub fn movement_request_system(
    sim_state: Res<SimulationState>,
    mut units: Query<
        (
            Entity,
            &GridPosition,
            &mut GridMovement,
            &ActionPlan,
            &NameComponent,
        ),
        With<UnitTag>,
    >,
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
                        &format!(
                            "{} starting movement to ({},{}) with {} steps",
                            name.name,
                            target.x,
                            target.y,
                            path.len()
                        ),
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
    mut units: Query<
        (&GridPosition, &mut crate::components::PositionComponent),
        Changed<GridPosition>,
    >,
) {
    // Only sync on ticks
    if !sim_state.just_ticked {
        return;
    }

    for (grid_pos, mut position) in units.iter_mut() {
        // Keep positions in tile coordinates, not world coordinates
        position.x = grid_pos.x as f32;
        position.y = grid_pos.y as f32;
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
    moving_units: Query<&GridMovement, (With<UnitTag>, With<GridMovement>)>,
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
        &format!(
            "Movement system: {}/{} units moving",
            moving_count, total_units
        ),
    );
}
*/

/// Simple random movement system - triggers every 10-30 ticks to make units wander
/// Now includes smart food-seeking behavior when hungry!
pub fn simple_random_movement_system(
    sim_state: Res<SimulationState>,
    mut units: Query<
        (
            Entity,
            &GridPosition,
            &mut GridMovement,
            &mut UnitMind,
            &NameComponent,
            &crate::components::UnitNeedsV2,
        ),
        With<UnitTag>,
    >,
    berry_bushes: Query<(&GridPosition, &crate::components::resource::ResourceNode), With<crate::ai::BerryBushTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    let mut rng = rand::thread_rng();
    
    for (entity, grid_pos, mut movement, mut mind, name, needs) in units.iter_mut() {
        // Skip if already moving
        if movement.is_moving {
            continue;
        }
        
        // If hungry, look for nearby berry bushes!
        if needs.is_hungry() {  // Getting hungry
            // Find nearest berry bush with berries
            let mut nearest_bush = None;
            let mut nearest_distance = f32::MAX;
            
            for (bush_pos, resource) in berry_bushes.iter() {
                if resource.amount > 0 {
                    let dx = (bush_pos.x as f32 - grid_pos.x as f32);
                    let dy = (bush_pos.y as f32 - grid_pos.y as f32);
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    if distance < nearest_distance {
                        nearest_distance = distance;
                        nearest_bush = Some(bush_pos.clone());
                    }
                }
            }
            
            // If found a berry bush, go there!
            if let Some(target_pos) = nearest_bush {
                movement.set_target_from(&grid_pos, target_pos.clone());
                *mind = UnitMind::GoingThere {
                    destination: format!("Berry bush at ({}, {})", target_pos.x, target_pos.y),
                };
                
                debug.log(
                    DebugLevel::Info,
                    "FOOD_SEEKING",
                    &format!(
                        "{} is hungry ({}% full) and moving to berry bush at ({},{})",
                        name.name, ((1.0 - needs.hunger()) * 100.0) as i32, target_pos.x, target_pos.y
                    ),
                );
                
                println!(
                    "{} {} is hungry and heading to berries at ({},{})",
                    "🍓".red(),
                    name.name.yellow(),
                    target_pos.x,
                    target_pos.y
                );
                continue;
            }
        }
        
        // Otherwise, random movement as before
        // Random chance to start moving (roughly every 10-30 ticks)
        if rng.gen_range(0..20) != 0 {
            continue;
        }
        
        // Pick a random destination 5-15 tiles away
        let range = rng.gen_range(5..15);
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        
        let new_x = (grid_pos.x as i32 + (angle.cos() * range as f32) as i32)
            .max(0)
            .min(63) as u32;
        let new_y = (grid_pos.y as i32 + (angle.sin() * range as f32) as i32)
            .max(0)
            .min(63) as u32;
        
        // Don't move to same position
        if new_x == grid_pos.x && new_y == grid_pos.y {
            continue;
        }
        
        // Set the movement target with path
        let target = GridPosition::new(new_x, new_y);
        movement.set_target_from(&grid_pos, target);
        
        // Update mind state
        *mind = UnitMind::Wandering;
        
        debug.log(
            DebugLevel::Info,
            "RANDOM_MOVEMENT",
            &format!(
                "{} randomly wandering from ({},{}) to ({},{})",
                name.name, grid_pos.x, grid_pos.y, new_x, new_y
            ),
        );
        
        println!(
            "{} {} starts wandering to ({},{})",
            "🚶".cyan(),
            name.name.green(),
            new_x,
            new_y
        );
    }
}
