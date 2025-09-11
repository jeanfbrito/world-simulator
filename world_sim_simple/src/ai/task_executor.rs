use crate::ai::{ActionPlan, pathfinding::find_path};
use crate::buildings::BuildingComponent;
use crate::components::work_progress::{ResourceWork, WorkProgress, WorkType};
use crate::components::*;
use crate::components::{GridPosition, GridMovement};
use crate::debug::{DebugLevel, DebugSystem};
use crate::resources::ResourceType;
use crate::{TileEntity, WorldMap, MAP_SIZE};
use bevy::prelude::*;
use std::collections::HashSet;

const TILE_SIZE: f32 = 10.0;
const TILES_PER_TICK: f32 = 0.5; // Move half a tile per tick (at 10 TPS = 5 tiles/second)

/// System to execute tasks based on GOAP actions
pub fn task_execution_system(
    mut commands: Commands,
    mut workers: Query<
        (
            Entity,
            &mut TileEntity,
            &mut ActionPlan,
            &mut IsWorking,
            &mut HasWood,
            &mut HasStone,
            &mut HasFood,
            &mut IsHungry,
            &mut HasEnergy,
            &mut PositionComponent,
            &mut TilesWalked,
            &crate::components::NameComponent,
            Option<&mut GridPosition>,
            Option<&mut GridMovement>,
        ),
        With<UnitTag>,
    >,
    mut work_progress_query: Query<&mut WorkProgress>,
    buildings: Query<
        (&BuildingComponent, &PositionComponent),
        (
            Without<UnitTag>,
            Without<TreeTag>,
            Without<RockTag>,
            Without<BerryBushTag>,
        ),
    >,
    trees: Query<(Entity, &PositionComponent), (With<TreeTag>, Without<UnitTag>)>,
    rocks: Query<(Entity, &PositionComponent), (With<RockTag>, Without<UnitTag>)>,
    berries: Query<
        (Entity, &PositionComponent, &mut ResourceNode),
        (With<BerryBushTag>, Without<UnitTag>),
    >,
    world_map: Res<WorldMap>,
    sim_state: Res<crate::SimulationState>,
    debug: Res<DebugSystem>,
) {
    // Only execute on simulation ticks
    if !sim_state.just_ticked {
        return;
    }

    // Debug: Count workers with plans
    let worker_count = workers.iter().count();
    if worker_count > 0 && sim_state.tick % 10 == 0 {
        println!(
            "📋 Task executor: {} workers, tick {}",
            worker_count, sim_state.tick
        );
    }

    for (
        entity,
        mut tile_entity,
        mut plan,
        mut is_working,
        mut has_wood,
        mut has_stone,
        mut has_food,
        mut is_hungry,
        mut has_energy,
        mut worker_pos,
        mut tiles_walked,
        name,
        grid_pos,
        mut grid_movement,
    ) in workers.iter_mut()
    {
        // Get current action from plan
        if let Some(action) = plan.current_action() {
            is_working.0 = true;

            // Debug specific actions
            if action.name == "gather_food" {
                println!("🎯 {} executing action: {}", name.name, action.name);
            }

            match action.name.as_str() {
                "cut_wood" => {
                    // Find nearest tree
                    if let Some((tree_entity, tree_pos)) = find_nearest_tree(&trees, &worker_pos) {
                        // Set movement target if we have grid components
                        if let (Some(mut grid_movement), Some(grid_pos)) = (grid_movement.as_mut(), grid_pos.as_ref()) {
                            let target = GridPosition::new(tree_pos.x.round() as u32, tree_pos.y.round() as u32);
                            if !grid_movement.is_moving || grid_movement.target != Some(target.clone()) {
                                grid_movement.set_target(target.clone());
                                debug.log(
                                    DebugLevel::Info,
                                    "TASK_EXEC",
                                    &format!("Worker moving to tree at ({}, {})", target.x, target.y),
                                );
                            }
                            // Check if reached
                            if grid_pos.distance_to(&target) <= 1 {
                                // Reached tree, harvest it
                                has_wood.0 = has_wood.0.saturating_add(10);
                                commands.entity(tree_entity).despawn();
                                debug.log(
                                    DebugLevel::Info,
                                    "TASK_EXEC",
                                    &format!("Worker harvested tree, now has {} wood", has_wood.0),
                                );
                                plan.advance();
                            }
                        } else {
                            // Fallback to old movement
                            if tick_move_towards(
                                &mut tile_entity,
                                &mut worker_pos,
                                &tree_pos,
                                &mut tiles_walked,
                                &debug,
                            ) {
                                // Reached tree, harvest it
                                has_wood.0 = has_wood.0.saturating_add(10);
                                commands.entity(tree_entity).despawn();
                                debug.log(
                                    DebugLevel::Info,
                                    "TASK_EXEC",
                                    &format!("Worker harvested tree, now has {} wood", has_wood.0),
                                );
                            }
                        }
                    }
                }

                "quarry_stone" => {
                    // Find nearest rock
                    if let Some((rock_entity, rock_pos)) = find_nearest_rock(&rocks, &worker_pos) {
                        // Move towards rock (tick-based)
                        if tick_move_towards(
                            &mut tile_entity,
                            &mut worker_pos,
                            &rock_pos,
                            &mut tiles_walked,
                            &debug,
                        ) {
                            // Reached rock, mine it
                            has_stone.0 = has_stone.0.saturating_add(10);
                            commands.entity(rock_entity).despawn();
                            debug.log(
                                DebugLevel::Info,
                                "TASK_EXEC",
                                &format!("Worker mined rock, now has {} stone", has_stone.0),
                            );
                        }
                    }
                }

                "move_to_resource" => {
                    // Move to nearest berry bush for harvesting
                    if let Some((berry_entity, berry_pos)) = find_nearest_berry(&berries, &worker_pos) {
                        // Move towards berry bush (tick-based)
                        if tick_move_towards(
                            &mut tile_entity,
                            &mut worker_pos,
                            &berry_pos,
                            &mut tiles_walked,
                            &debug,
                        ) {
                            debug.log(
                                DebugLevel::Info,
                                "TASK_EXEC",
                                "Worker reached resource (berry bush)",
                            );
                            // Mark as complete so next action can harvest
                            plan.advance();
                        }
                    } else {
                        debug.log(
                            DebugLevel::Info,
                            "TASK_EXEC",
                            "No berry bushes found for move_to_resource",
                        );
                        // Complete anyway to avoid getting stuck
                        plan.advance();
                    }
                }

                "harvest_resource" => {
                    // We should be at a berry bush now, harvest it
                    if let Some((berry_entity, berry_pos)) = find_nearest_berry(&berries, &worker_pos) {
                        let distance = worker_pos.distance_squared(&berry_pos).sqrt();
                        if distance <= 2.0 {
                            // Close enough to harvest
                            debug.log(
                                DebugLevel::Info,
                                "TASK_EXEC",
                                "Harvesting berries from bush",
                            );
                            
                            // Add wood to simulate harvesting (should be berries but using wood for now)
                            has_wood.0 += 5;
                            
                            // Mark action complete
                            plan.advance();
                        } else {
                            // Not close enough, move closer
                            tick_move_towards(
                                    &mut tile_entity,
                                &mut worker_pos,
                                &berry_pos,
                                &mut tiles_walked,
                                &debug,
                            );
                        }
                    } else {
                        debug.log(
                            DebugLevel::Info,
                            "TASK_EXEC",
                            "No berry bushes found for harvesting",
                        );
                        plan.advance();
                    }
                }

                "gather_food" => {
                    // Check if work is already in progress
                    if let Ok(work_progress) = work_progress_query.get(entity) {
                        if work_progress.is_working {
                            // Work is already happening, let it continue
                            continue;
                        }
                    }

                    debug.log(
                        DebugLevel::Info,
                        "TASK_EXEC",
                        &format!(
                            "Executing gather_food for entity at ({}, {})",
                            worker_pos.x, worker_pos.y
                        ),
                    );
                    // Find nearest berry bush with berries available
                    if let Some((berry_entity, berry_pos)) =
                        find_nearest_berry(&berries, &worker_pos)
                    {
                        println!(
                            "🫐 Found berry bush at ({}, {}) for peasant at ({}, {})",
                            berry_pos.x, berry_pos.y, worker_pos.x, worker_pos.y
                        );
                        
                        // Use grid movement if available
                        let reached = if let (Some(grid_movement), Some(grid_pos)) = (grid_movement.as_mut(), grid_pos.as_ref()) {
                            let target = GridPosition::new(berry_pos.x.round() as u32, berry_pos.y.round() as u32);
                            
                            // Set movement target if not already moving there
                            if !grid_movement.is_moving || grid_movement.target != Some(target.clone()) {
                                // Calculate simple path
                                let mut path = Vec::new();
                                let mut current = (**grid_pos).clone();
                                while current != target {
                                    current = current.step_toward(&target);
                                    path.push(current.clone());
                                }
                                
                                if !path.is_empty() {
                                    grid_movement.set_path(path);
                                    println!("   Set grid movement path to berry bush");
                                }
                            }
                            
                            // Check if reached
                            grid_pos.distance_to(&target) <= 1
                        } else {
                            // Fallback to old movement
                            tick_move_with_pathfinding(
                                &mut commands,
                                entity,
                                &mut tile_entity,
                                &mut worker_pos,
                                &berry_pos,
                                &mut tiles_walked,
                                &world_map,
                                &debug,
                            )
                        };
                        
                        println!("   Movement to berry: worker at ({:.1}, {:.1}), target ({:.1}, {:.1}), reached: {}", 
                            worker_pos.x, worker_pos.y, berry_pos.x, berry_pos.y, reached);
                        
                        if reached {
                            // Reached berries - start gathering work
                            println!("🍓 {} reached berry bush! Starting work...", name.name);

                            // Get the existing WorkProgress component and update it
                            let work_progress_result = work_progress_query.get_mut(entity);
                            println!("   WorkProgress query result: {:?}", work_progress_result.is_ok());
                            
                            if let Ok(mut work_progress) = work_progress_result {
                                println!("   Current work state - is_working: {}, progress: {}/{}", 
                                    work_progress.is_working, work_progress.progress_counter, work_progress.required_ticks);
                                    
                                work_progress.start_work(
                                    WorkType::Gathering(ResourceWork {
                                        resource_type: ResourceType::Berries,
                                        amount: 3,
                                        tool_bonus: 1.0,
                                    }),
                                    30, // 3 seconds at 10 TPS
                                    Some(berry_entity),
                                );
                                println!("   ✅ Work started! Now is_working: {}, required: {} ticks", 
                                    work_progress.is_working, work_progress.required_ticks);
                            } else {
                                // Create new if somehow missing
                                println!("   ⚠️ No WorkProgress component, creating new one");
                                let mut new_work_progress = WorkProgress::new();
                                new_work_progress.start_work(
                                    WorkType::Gathering(ResourceWork {
                                        resource_type: ResourceType::Berries,
                                        amount: 3,
                                        tool_bonus: 1.0,
                                    }),
                                    30, // 3 seconds at 10 TPS
                                    Some(berry_entity),
                                );
                                commands.entity(entity).insert(new_work_progress);
                            }

                            debug.log(
                                DebugLevel::Info,
                                "TASK_EXEC",
                                "Worker started gathering berries",
                            );

                            // Don't add food yet - let the work system handle it when complete
                        }
                    } else {
                        println!(
                            "❌ No berry bushes with berries found for peasant at ({}, {})",
                            worker_pos.x, worker_pos.y
                        );
                        debug.log(
                            DebugLevel::Info,
                            "TASK_EXEC",
                            &format!(
                                "No berries found from position ({}, {})",
                                worker_pos.x, worker_pos.y
                            ),
                        );
                    }
                }

                "move_to_storage" | "get_wood_from_stockpile" | "get_stone_from_stockpile" => {
                    // Find stockpile
                    if let Some(stockpile_pos) = find_stockpile(&buildings) {
                        // Move towards stockpile (tick-based)
                        if tick_move_towards(
                            &mut tile_entity,
                            &mut worker_pos,
                            &stockpile_pos,
                            &mut tiles_walked,
                            &debug,
                        ) {
                            debug.log(DebugLevel::Info, "TASK_EXEC", "Worker reached stockpile");
                        }
                    }
                }

                "store_resources" => {
                    // At stockpile, store resources
                    if let Some(_stockpile_pos) = find_stockpile(&buildings) {
                        // TODO: Actually add to stockpile inventory
                        debug.log(
                            DebugLevel::Info,
                            "TASK_EXEC",
                            &format!(
                                "Worker stored {} wood, {} stone at stockpile",
                                has_wood.0, has_stone.0
                            ),
                        );
                        has_wood.0 = 0;
                        has_stone.0 = 0;
                    }
                }

                "eat_food" => {
                    if has_food.0 > 0 {
                        has_food.0 = has_food.0.saturating_sub(1);
                        is_hungry.0 = (is_hungry.0 - 0.5).max(0.0); // Eating reduces hunger by 50%
                        debug.log(
                            DebugLevel::Info,
                            "TASK_EXEC",
                            &format!(
                                "Worker ate food (hunger: {:.0}%), {} food remaining",
                                is_hungry.0 * 100.0,
                                has_food.0
                            ),
                        );
                    }
                    // Eating is instant, advance to next action
                    plan.advance();
                }

                "rest" => {
                    is_working.0 = false;
                    has_energy.0 = (has_energy.0 + 0.02).min(1.0); // Resting restores 2% energy per tick
                    debug.log(
                        DebugLevel::Debug,
                        "TASK_EXEC",
                        &format!("Worker is resting (energy: {:.0}%)", has_energy.0 * 100.0),
                    );
                }

                "build_house" => {
                    // Find a good spot for house (near center for now)
                    let house_pos = PositionComponent::from_tile(32, 28);
                    if tick_move_towards(
                        &mut tile_entity,
                        &mut worker_pos,
                        &house_pos,
                        &mut tiles_walked,
                        &debug,
                    ) {
                        // Build house (consume resources)
                        has_wood.0 = has_wood.0.saturating_sub(15);
                        has_stone.0 = has_stone.0.saturating_sub(10);

                        // TODO: Actually spawn house entity
                        debug.log(DebugLevel::Info, "TASK_EXEC", "Worker built a house!");
                    }
                }

                _ => {
                    debug.log(
                        DebugLevel::Debug,
                        "TASK_EXEC",
                        &format!("Unknown action: {}", action.name),
                    );
                }
            }
        } else {
            is_working.0 = false;
        }
    }
}


/// Tick-based movement with A* pathfinding around obstacles
fn tick_move_with_pathfinding(
    commands: &mut Commands,
    entity: Entity,
    tile_entity: &mut TileEntity,
    position_comp: &mut PositionComponent,
    target_pos: &PositionComponent,
    tiles_walked: &mut TilesWalked,
    world_map: &WorldMap,
    debug: &DebugSystem,
) -> bool {
    // Check if already at target (within 1 tile)
    let dx = target_pos.x - position_comp.x;
    let dy = target_pos.y - position_comp.y;
    let distance = (dx * dx + dy * dy).sqrt();
    
    if distance < 1.0 {
        return true;
    }
    
    // For now, we'll handle path following differently since we can't query components from commands
    // We'll need to pass in the path from the system that calls this function
    // For immediate fix, let's just calculate the path each time (will optimize later)
    
    // Check if we're already close enough
    if distance < 1.0 {
        return true;
    }
    
    // Calculate path if we don't have one stored (temporary solution)
    let start_pos = Vec3::new(position_comp.x * 10.0, position_comp.y * 10.0, 0.0);
    let goal_pos = Vec3::new(target_pos.x * 10.0, target_pos.y * 10.0, 0.0);
    
    // Get obstacles from map
    let mut obstacles = HashSet::new();
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            if !world_map.tiles[y][x].is_walkable() {
                obstacles.insert((x as i32, y as i32));
            }
        }
    }
    
    // Find path
    if let Some(path) = find_path(start_pos, goal_pos, &obstacles) {
        let waypoints = path.get_waypoints();
        
        if waypoints.len() > 2 {
            debug.log(
                DebugLevel::Info,
                "PATHFINDING",
                &format!("Path has {} waypoints - navigating around obstacles!", waypoints.len()),
            );
        }
        
        if !waypoints.is_empty() {
            // Get next waypoint (skip first as it's current position)
            let next_waypoint = if waypoints.len() > 1 { waypoints[1] } else { waypoints[0] };
            let waypoint_pos = PositionComponent::new(
                next_waypoint.0 as f32,
                next_waypoint.1 as f32
            );
            
            // Move toward waypoint using simple movement for now
            debug.log(
                DebugLevel::Debug,
                "PATHFINDING",
                &format!("Following waypoint ({}, {}) of {} total", next_waypoint.0, next_waypoint.1, waypoints.len()),
            );
            return tick_move_towards(tile_entity, position_comp, &waypoint_pos, tiles_walked, debug);
        }
    } else {
        debug.log(
            DebugLevel::Info,
            "PATHFINDING",
            "No path found, using direct movement",
        );
    }
    
    // Fallback to direct movement if no path found
    return tick_move_towards(tile_entity, position_comp, target_pos, tiles_walked, debug);
}

/// Tick-based movement towards a target position (simple direct movement)
/// NOTE: This function now only checks if we're close enough - actual movement
/// is handled by the grid movement system to avoid conflicts
fn tick_move_towards(
    tile_entity: &mut TileEntity,
    position_comp: &mut PositionComponent,
    target_pos: &PositionComponent,
    tiles_walked: &mut TilesWalked,
    debug: &DebugSystem,
) -> bool {
    // Work in tile coordinates for simplicity
    let current_tile_x = position_comp.x;
    let current_tile_y = position_comp.y;
    let target_tile_x = target_pos.x;
    let target_tile_y = target_pos.y;

    let dx = target_tile_x - current_tile_x;
    let dy = target_tile_y - current_tile_y;
    let distance = (dx * dx + dy * dy).sqrt();

    // Debug: Log check
    debug.log(
        DebugLevel::Debug,
        "MOVE_CHECK",
        &format!(
            "Checking distance from ({:.2}, {:.2}) to ({:.2}, {:.2}): {:.2} tiles",
            current_tile_x, current_tile_y, target_tile_x, target_tile_y, distance
        ),
    );

    // Check if we're close enough (within 1 tile)
    if distance < 1.0 {
        debug.log(
            DebugLevel::Debug,
            "MOVE_COMPLETE",
            &format!("At target ({:.2}, {:.2})", position_comp.x, position_comp.y),
        );
        return true;
    }

    // Movement is handled by grid_movement_system, not here
    // This avoids conflicts between the two systems
    debug.log(
        DebugLevel::Debug,
        "MOVE_WAITING",
        &format!("Waiting for grid movement system to move unit (distance: {:.2})", distance),
    );

    false
}

fn find_nearest_tree(
    trees: &Query<(Entity, &PositionComponent), (With<TreeTag>, Without<UnitTag>)>,
    worker_pos: &PositionComponent,
) -> Option<(Entity, PositionComponent)> {
    let mut nearest = None;
    let mut min_distance = f32::MAX;

    for (entity, tree_pos) in trees.iter() {
        let distance = worker_pos.distance_squared(tree_pos);
        if distance < min_distance {
            min_distance = distance;
            nearest = Some((entity, tree_pos.clone()));
        }
    }

    nearest
}

fn find_nearest_rock(
    rocks: &Query<(Entity, &PositionComponent), (With<RockTag>, Without<UnitTag>)>,
    worker_pos: &PositionComponent,
) -> Option<(Entity, PositionComponent)> {
    let mut nearest = None;
    let mut min_distance = f32::MAX;

    for (entity, rock_pos) in rocks.iter() {
        let distance = worker_pos.distance_squared(rock_pos);
        if distance < min_distance {
            min_distance = distance;
            nearest = Some((entity, rock_pos.clone()));
        }
    }

    nearest
}

fn find_nearest_berry(
    berries: &Query<
        (Entity, &PositionComponent, &mut ResourceNode),
        (With<BerryBushTag>, Without<UnitTag>),
    >,
    worker_pos: &PositionComponent,
) -> Option<(Entity, PositionComponent)> {
    let mut nearest = None;
    let mut min_distance = f32::MAX;

    for (entity, berry_pos, resource_node) in berries.iter() {
        // Only consider bushes that have berries available
        if resource_node.can_harvest() {
            let distance = worker_pos.distance_squared(berry_pos);
            if distance < min_distance {
                min_distance = distance;
                nearest = Some((entity, berry_pos.clone()));
            }
        }
    }

    nearest
}

fn find_stockpile(
    buildings: &Query<
        (&BuildingComponent, &PositionComponent),
        (
            Without<UnitTag>,
            Without<TreeTag>,
            Without<RockTag>,
            Without<BerryBushTag>,
        ),
    >,
) -> Option<PositionComponent> {
    for (building, pos) in buildings.iter() {
        if building.building_type == crate::buildings::BuildingType::Stockpile {
            return Some(pos.clone());
        }
    }
    None
}

// Marker components for resources
#[derive(Component)]
pub struct TreeTag;

#[derive(Component)]
pub struct RockTag;

#[derive(Component)]
pub struct BerryBushTag;
