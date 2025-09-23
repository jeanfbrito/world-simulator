use crate::components::{
    GridMovement, GridPosition, MovementEffects, MovementSpeed, NameComponent, UnitTag,
    VisualPosition, UnitMind, ClaimedResource, UnitInventory,
    resource::ResourceNode,
};
use crate::{SimulationState, WorldMap, TILE_SIZE};
use crate::resources::ResourceType;
use crate::ai::BerryBushTag;
use crate::debug::{DebugSystem, DebugLevel};
use crate::systems::GridOccupationMap;
/// Tick-based movement system
///
/// This system handles all unit movement on the simulation tick,
/// updating grid positions discretely. Visual interpolation happens
/// separately for smooth presentation.
use bevy::prelude::*;
use colored::Colorize;
use rand::Rng;
use std::collections::HashSet;

fn is_position_walkable(world_map: &WorldMap, pos: &GridPosition) -> bool {
    // Check bounds first
    if pos.x >= 64 || pos.y >= 64 {
        return false;
    }
    
    // Check if the tile is walkable (not water, deep water, etc.)
    world_map.tiles[pos.y as usize][pos.x as usize].is_walkable()
}

/// System that processes movement on each simulation tick
pub fn tick_movement_system(
    sim_state: Res<SimulationState>,
    world_map: Res<WorldMap>,
    occupation_map: Res<GridOccupationMap>,
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

        // Store the target position before movement
        let next_target = movement.target.clone();

        // Update movement progress with unit-specific speed
        let completed = movement.tick_update(&mut grid_pos, effective_ticks);

        // If position changed, check if the new position is valid
        if old_pos != *grid_pos {
            // Check if the new position is blocked by a solid obstacle (not including this unit)
            if occupation_map.is_solid_obstacle(&grid_pos) {
                // Movement blocked - revert to old position and stop moving
                *grid_pos = old_pos.clone();
                movement.stop();

                debug.log(
                    DebugLevel::Debug,
                    "MOVEMENT_BLOCKED",
                    &format!(
                        "{} blocked at ({},{}) - stopping movement",
                        name.name, grid_pos.x, grid_pos.y
                    ),
                );

                // Try to find an alternative path
                if let Some(target) = next_target {
                    let obstacles = build_obstacle_map(&world_map, &occupation_map);
                    movement.set_target_from_with_pathfinding(&grid_pos, target, &obstacles);
                }
            } else {
                // Movement successful - update visual position
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
            }

            // Check if new position is valid terrain-wise
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

/// Build obstacle map from world tiles and entities
fn build_obstacle_map(world_map: &WorldMap, occupation_map: &GridOccupationMap) -> HashSet<(i32, i32)> {
    let mut obstacles = HashSet::new();

    // Add terrain obstacles
    for y in 0..crate::MAP_SIZE {
        for x in 0..crate::MAP_SIZE {
            if !world_map.tiles[y][x].is_walkable() {
                obstacles.insert((x as i32, y as i32));
            }
        }
    }

    // Add entity obstacles (resources, buildings, etc.)
    let entity_obstacles = occupation_map.get_obstacle_set();
    obstacles.extend(entity_obstacles);

    obstacles
}

/// Simple random movement system - triggers every 10-30 ticks to make units wander
/// Now includes smart food-seeking behavior when hungry!
pub fn simple_random_movement_system(
    sim_state: Res<SimulationState>,
    world_map: Res<WorldMap>,
    occupation_map: Res<GridOccupationMap>,
    mut units: Query<
        (
            Entity,
            &GridPosition,
            &mut GridMovement,
            &mut UnitMind,
            &NameComponent,
            &crate::components::UnitInventory,
            &mut crate::components::ClaimedResource,
            &crate::ai::bevy_dogoap_impl::Satiety,
            &crate::ai::bevy_dogoap_impl::Energy,
        ),
        With<UnitTag>,
    >,
    mut berry_bushes: Query<(Entity, &GridPosition, &mut ResourceNode), With<BerryBushTag>>,
    debug: Res<DebugSystem>,
) {

    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }

    // Build obstacle map for pathfinding
    let obstacles = build_obstacle_map(&world_map, &occupation_map);
    
    let mut rng = rand::thread_rng();
    
    for (entity, grid_pos, mut movement, mut mind, name, inventory, mut claimed_resource, satiety, energy) in units.iter_mut() {
        // Skip if already moving
        if movement.is_moving {
            continue;
        }
        
        // Release any existing claim if we're not moving
        if claimed_resource.has_claim() {
            if let Some(resource_entity) = claimed_resource.get_claimed() {
                if let Ok((_, _, mut resource)) = berry_bushes.get_mut(resource_entity) {
                    resource.release_claim(entity);
                }
            }
            claimed_resource.release();
        }
        
        // If hungry and no food in inventory, look for berry bushes to harvest with atomic claiming
        // Satiety: lower values = more hungry (0=starving, 100=full)
        if satiety.0 < 40.0 && inventory.get_amount(crate::resources::ResourceType::Berries) < 3 {
            const MAX_CLAIM_ATTEMPTS: usize = 3;
            let mut attempt_count = 0;
            let mut successfully_claimed = false;

            // Collect available berry bushes and sort by distance
            let mut available_bushes: Vec<(Entity, GridPosition, f32)> = Vec::new();

            for (bush_entity, bush_pos, resource) in berry_bushes.iter() {
                // Skip if no berries
                if resource.amount == 0 {
                    continue;
                }

                let dx = bush_pos.x as f32 - grid_pos.x as f32;
                let dy = bush_pos.y as f32 - grid_pos.y as f32;
                let distance = (dx * dx + dy * dy).sqrt();

                // Skip if too far
                if distance > 30.0 {
                    continue;
                }

                available_bushes.push((bush_entity, bush_pos.clone(), distance));
            }

            // Sort by distance (closest first)
            available_bushes.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

            // Try to claim bushes atomically, starting with the closest
            for (bush_entity, target_pos, _distance) in available_bushes.iter() {
                if attempt_count >= MAX_CLAIM_ATTEMPTS {
                    break;
                }
                attempt_count += 1;

                // Try to claim the resource atomically with timeout
                if let Ok((_, _, mut resource)) = berry_bushes.get_mut(*bush_entity) {
                    if !resource.try_claim_with_timeout(entity, sim_state.tick) {
                        // Claim failed, try next resource
                        debug.log(
                            DebugLevel::Debug,
                            "CLAIM_FAILED",
                            &format!(
                                "{} failed to claim berry bush at ({},{}) - already has {}/{} workers",
                                name.name, target_pos.x, target_pos.y,
                                resource.claim_count(), resource.max_workers
                            ),
                        );
                        continue;
                    }

                    // Claim succeeded! Now find a path to an adjacent position
                    let adjacent_positions = target_pos.get_adjacent();
                    let walkable_adjacent = adjacent_positions.into_iter()
                        .filter(|pos| {
                            pos.x < 64 && pos.y < 64 &&
                            world_map.tiles[pos.y as usize][pos.x as usize].is_walkable() &&
                            !obstacles.contains(&(pos.x as i32, pos.y as i32))
                        })
                        .min_by_key(|pos| pos.distance_to(&grid_pos));

                    if let Some(adjacent_target) = walkable_adjacent {
                        // Save the claimed resource
                        claimed_resource.claim(*bush_entity);
                        movement.set_target_from_with_pathfinding(&grid_pos, adjacent_target.clone(), &obstacles);
                        *mind = UnitMind::GoingThere {
                            destination: format!("Berry bush at ({}, {})", target_pos.x, target_pos.y),
                        };

                        debug.log(
                            DebugLevel::Info,
                            "RESOURCE_CLAIM",
                            &format!(
                                "{} claimed berry bush at ({},{}) for harvesting [workers: {}/{}]",
                                name.name, target_pos.x, target_pos.y,
                                resource.claim_count(), resource.max_workers
                            ),
                        );

                        println!(
                            "{} {} claimed berries at ({},{}) [workers: {}/{}]",
                            "⛏️".cyan(),
                            name.name.yellow(),
                            target_pos.x,
                            target_pos.y,
                            resource.claim_count(),
                            resource.max_workers
                        );

                        successfully_claimed = true;
                        break;
                    } else {
                        // No walkable adjacent tiles - release claim
                        resource.release_claim(entity);
                        debug.log(
                            DebugLevel::Debug,
                            "CLAIM_RELEASED_NO_PATH",
                            &format!(
                                "{} released claim on berry bush at ({},{}) - no valid adjacent tiles",
                                name.name, target_pos.x, target_pos.y
                            ),
                        );
                    }
                }
            }

            // If we successfully claimed something, continue to next unit
            if successfully_claimed {
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
        
        // Set the movement target with A* pathfinding
        let target = GridPosition::new(new_x, new_y);
        movement.set_target_from_with_pathfinding(&grid_pos, target, &obstacles);
        
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

/// System to handle units searching for food - finds and moves to berry bushes with atomic claiming
pub fn food_search_movement_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity,
        &GridPosition,
        &mut GridMovement,
        &mut UnitMind,
        &mut ClaimedResource,
        &UnitInventory,
        &NameComponent,
        &crate::ai::bevy_dogoap_impl::Satiety,
    ), With<UnitTag>>,
    mut berry_bushes: Query<(Entity, &GridPosition, &mut ResourceNode), With<BerryBushTag>>,
    world_map: Res<WorldMap>,
    occupation_map: Res<GridOccupationMap>,
    debug: Res<DebugSystem>,
) {
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }

    // Build obstacle map for pathfinding
    let obstacles = build_obstacle_map(&world_map, &occupation_map);

    for (entity, grid_pos, mut movement, mut mind, mut claimed_resource, inventory, name, satiety) in units.iter_mut() {
        // Only process units that are searching for food
        if !matches!(*mind, UnitMind::SearchingForFood) {
            continue;
        }

        // Skip if already moving
        if movement.is_moving {
            debug.log(
                DebugLevel::Debug,
                "FOOD_SEARCH",
                &format!("{} is already moving, skipping food search", name.name),
            );
            continue;
        }

        // If searching for food, find available berry bushes
        if inventory.get_amount(ResourceType::Berries) == 0 {
            // Maximum number of resources to try claiming
            const MAX_CLAIM_ATTEMPTS: usize = 5;
            let mut attempt_count = 0;
            let mut successfully_claimed = false;

            // Collect all available berry bushes and sort by distance
            let mut available_bushes: Vec<(Entity, GridPosition, f32)> = Vec::new();

            for (bush_entity, bush_pos, resource) in berry_bushes.iter() {
                // Skip if no berries
                if resource.amount == 0 {
                    continue;
                }

                // Don't skip fully claimed ones during collection - we'll try to claim atomically
                let dx = bush_pos.x as f32 - grid_pos.x as f32;
                let dy = bush_pos.y as f32 - grid_pos.y as f32;
                let distance = (dx * dx + dy * dy).sqrt();

                // Skip if too far (optimization)
                if distance > 50.0 {
                    continue;
                }

                available_bushes.push((bush_entity, bush_pos.clone(), distance));
            }

            // Sort by distance (closest first)
            available_bushes.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

            // Try to claim bushes atomically, starting with the closest
            for (bush_entity, target_pos, distance) in available_bushes.iter() {
                if attempt_count >= MAX_CLAIM_ATTEMPTS {
                    break;
                }
                attempt_count += 1;

                // Try to claim the resource atomically with timeout
                if let Ok((_, _, mut resource)) = berry_bushes.get_mut(*bush_entity) {
                    // Clean up expired claims and try to claim
                    if !resource.try_claim_with_timeout(entity, sim_state.tick) {
                        // Claim failed, try next resource
                        debug.log(
                            DebugLevel::Debug,
                            "CLAIM_FAILED",
                            &format!(
                                "{} failed to claim berry bush at ({},{}) - already has {}/{} workers",
                                name.name, target_pos.x, target_pos.y,
                                resource.claim_count(), resource.max_workers
                            ),
                        );
                        continue;
                    }

                    // Claim succeeded! Now find a path to an adjacent position
                    let adjacent_positions = target_pos.get_adjacent();
                    let mut found_valid_path = false;

                    for adj_pos in adjacent_positions {
                        // Check if adjacent position is walkable
                        if adj_pos.x >= 64 || adj_pos.y >= 64 {
                            continue;
                        }

                        if !world_map.tiles[adj_pos.y as usize][adj_pos.x as usize].is_walkable() {
                            continue;
                        }

                        if obstacles.contains(&(adj_pos.x as i32, adj_pos.y as i32)) {
                            continue;
                        }

                        // Try to find path to this adjacent position
                        movement.set_target_from_with_pathfinding(&grid_pos, adj_pos.clone(), &obstacles);

                        // Check if a path was actually found (movement is now active)
                        if movement.is_moving {
                            // Successfully claimed and found path!
                            claimed_resource.claim(*bush_entity);
                            *mind = UnitMind::GoingThere {
                                destination: format!("berry bush at ({},{})", target_pos.x, target_pos.y)
                            };

                            debug.log(
                                DebugLevel::Info,
                                "CLAIM_SUCCESS",
                                &format!(
                                    "{} successfully claimed berry bush at ({},{}) - now has {}/{} workers",
                                    name.name, target_pos.x, target_pos.y,
                                    resource.claim_count(), resource.max_workers
                                ),
                            );

                            println!(
                                "{} {} claimed food at ({},{}) [workers: {}/{}]",
                                "🎯".green(),
                                name.name.yellow(),
                                target_pos.x,
                                target_pos.y,
                                resource.claim_count(),
                                resource.max_workers
                            );

                            found_valid_path = true;
                            successfully_claimed = true;
                            break;
                        }
                    }

                    if found_valid_path {
                        break; // Exit the resource loop
                    } else {
                        // Could not find path to resource, release the claim
                        resource.release_claim(entity);
                        debug.log(
                            DebugLevel::Debug,
                            "CLAIM_RELEASED_NO_PATH",
                            &format!(
                                "{} released claim on berry bush at ({},{}) - no valid path",
                                name.name, target_pos.x, target_pos.y
                            ),
                        );
                    }
                }
            }

            // If we couldn't claim any resource after trying multiple times
            if !successfully_claimed {
                if attempt_count >= MAX_CLAIM_ATTEMPTS {
                    // Tried maximum resources but couldn't claim any
                    *mind = UnitMind::Idle;
                    debug.log(
                        DebugLevel::Debug,
                        "NO_CLAIMABLE_RESOURCES",
                        &format!(
                            "{} couldn't claim any berry bushes after {} attempts",
                            name.name, attempt_count
                        ),
                    );
                } else {
                    // No available bushes at all
                    *mind = UnitMind::Idle;
                    debug.log(
                        DebugLevel::Debug,
                        "NO_FOOD_FOUND",
                        &format!("{} couldn't find any available berry bushes", name.name),
                    );
                }
            }
        } else {
            // Has food in inventory - go back to idle
            *mind = UnitMind::Idle;
            debug.log(
                DebugLevel::Debug,
                "FOOD_SEARCH",
                &format!("{} has food, returning to idle", name.name),
            );
        }
    }
}
