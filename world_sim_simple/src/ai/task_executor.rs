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
    mut berries: Query<
        (Entity, &PositionComponent, &mut ResourceNode, Option<&GrowingResource>),
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
                        // Use grid movement system
                        if let (Some(ref mut grid_move), Some(ref grid_pos)) = (grid_movement.as_mut(), grid_pos.as_ref()) {
                            // Check if at target
                            if is_at_target(grid_pos, &tree_pos, &debug) {
                                // Reached tree, harvest it
                                has_wood.0 = has_wood.0.saturating_add(10);
                                commands.entity(tree_entity).despawn();
                                debug.log(
                                    DebugLevel::Info,
                                    "TASK_EXEC",
                                    &format!("Worker harvested tree, now has {} wood", has_wood.0),
                                );
                            } else if !grid_move.is_moving {
                                // Start movement if not already moving
                                start_grid_movement(grid_move, grid_pos, &tree_pos, &debug);
                            }
                        }
                    }
                }

                "quarry_stone" => {
                    // Find nearest rock
                    if let Some((rock_entity, rock_pos)) = find_nearest_rock(&rocks, &worker_pos) {
                        // Use grid movement system
                        if let (Some(ref mut grid_move), Some(ref grid_pos)) = (grid_movement.as_mut(), grid_pos.as_ref()) {
                            // Check if at target
                            if is_at_target(grid_pos, &rock_pos, &debug) {
                                // Reached rock, mine it
                                has_stone.0 = has_stone.0.saturating_add(10);
                                commands.entity(rock_entity).despawn();
                                debug.log(
                                    DebugLevel::Info,
                                    "TASK_EXEC",
                                    &format!("Worker mined rock, now has {} stone", has_stone.0),
                                );
                            } else if !grid_move.is_moving {
                                // Start movement if not already moving
                                start_grid_movement(grid_move, grid_pos, &rock_pos, &debug);
                            }
                        }
                    }
                }

                "move_to_resource" => {
                    // Move to nearest berry bush for harvesting
                    if let Some((berry_entity, berry_pos)) = find_nearest_berry(&berries, &worker_pos, entity) {
                        // Use grid movement system
                        if let (Some(ref mut grid_move), Some(ref grid_pos)) = (grid_movement.as_mut(), grid_pos.as_ref()) {
                            // Check if at target
                            if is_at_target(grid_pos, &berry_pos, &debug) {
                                debug.log(
                                    DebugLevel::Info,
                                    "TASK_EXEC",
                                    "Worker reached resource (berry bush)",
                                );
                                // Mark as complete so next action can harvest
                                plan.advance();
                            } else if !grid_move.is_moving {
                                // Start movement if not already moving
                                start_grid_movement(grid_move, grid_pos, &berry_pos, &debug);
                            }
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
                    if let Some((berry_entity, berry_pos)) = find_nearest_berry(&berries, &worker_pos, entity) {
                        if let (Some(ref mut grid_move), Some(ref grid_pos)) = (grid_movement.as_mut(), grid_pos.as_ref()) {
                            if is_at_target(grid_pos, &berry_pos, &debug) {
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
                            } else if !grid_move.is_moving {
                                // Not close enough, move closer
                                start_grid_movement(grid_move, grid_pos, &berry_pos, &debug);
                            }
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
                        find_nearest_berry(&berries, &worker_pos, entity)
                    {
                        println!(
                            "🫐 Found berry bush at ({}, {}) for peasant at ({}, {})",
                            berry_pos.x, berry_pos.y, worker_pos.x, worker_pos.y
                        );
                        
                        // Use grid movement system for pathfinding
                        let reached = if let (Some(ref mut grid_move), Some(ref grid_pos)) = (grid_movement.as_mut(), grid_pos.as_ref()) {
                            if is_at_target(grid_pos, &berry_pos, &debug) {
                                true
                            } else {
                                if !grid_move.is_moving {
                                    start_grid_movement(grid_move, grid_pos, &berry_pos, &debug);
                                }
                                false
                            }
                        } else {
                            false
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
                                
                                // Claim the resource so others don't try to harvest it
                                if let Ok((_, _, mut resource_node, _)) = berries.get_mut(berry_entity) {
                                    resource_node.claimed_by.insert(entity);
                                    println!("   🔒 Claimed berry bush (claims: {}/{})", 
                                        resource_node.claimed_by.len(), resource_node.max_workers);
                                }
                                
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
                        // Use grid movement system
                        if let (Some(ref mut grid_move), Some(ref grid_pos)) = (grid_movement.as_mut(), grid_pos.as_ref()) {
                            if is_at_target(grid_pos, &stockpile_pos, &debug) {
                                debug.log(DebugLevel::Info, "TASK_EXEC", "Worker reached stockpile");
                            } else if !grid_move.is_moving {
                                start_grid_movement(grid_move, grid_pos, &stockpile_pos, &debug);
                            }
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
                    
                    // Check if fully rested (energy >= 90%)
                    if has_energy.0 >= 0.9 {
                        debug.log(
                            DebugLevel::Info,
                            "TASK_EXEC",
                            &format!("{} fully rested (energy: {:.0}%)", name.name, has_energy.0 * 100.0),
                        );
                        // Advance to next action when sufficiently rested
                        plan.advance();
                    } else {
                        debug.log(
                            DebugLevel::Debug,
                            "TASK_EXEC",
                            &format!("{} is resting (energy: {:.0}%)", name.name, has_energy.0 * 100.0),
                        );
                    }
                }

                "build_house" => {
                    // Find a good spot for house (near center for now)
                    let house_pos = PositionComponent::from_tile(32, 28);
                    
                    if let (Some(ref mut grid_move), Some(ref grid_pos)) = (grid_movement.as_mut(), grid_pos.as_ref()) {
                        if is_at_target(grid_pos, &house_pos, &debug) {
                            // Build house (consume resources)
                            has_wood.0 = has_wood.0.saturating_sub(15);
                            has_stone.0 = has_stone.0.saturating_sub(10);

                            // TODO: Actually spawn house entity
                            debug.log(DebugLevel::Info, "TASK_EXEC", "Worker built a house!");
                        } else if !grid_move.is_moving {
                            start_grid_movement(grid_move, grid_pos, &house_pos, &debug);
                        }
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



/// Check if we're at target using GridMovement
fn is_at_target(
    grid_pos: &GridPosition,
    target_pos: &PositionComponent,
    debug: &DebugSystem,
) -> bool {
    // Convert target position to grid coordinates
    let target_grid_x = target_pos.x.round() as u32;
    let target_grid_y = target_pos.y.round() as u32;
    
    // Check if we're close enough (within 1 tile)
    let distance = grid_pos.distance_to(&GridPosition::new(target_grid_x, target_grid_y));
    
    if distance <= 1 {
        debug.log(
            DebugLevel::Debug,
            "MOVE_COMPLETE",
            &format!("Reached target at ({}, {})", grid_pos.x, grid_pos.y),
        );
        return true;
    }
    false
}

/// Start grid-based movement towards target
fn start_grid_movement(
    grid_movement: &mut GridMovement,
    grid_pos: &GridPosition,
    target_pos: &PositionComponent,
    debug: &DebugSystem,
) {
    // Convert target to grid coordinates
    let target_grid = GridPosition::new(
        target_pos.x.round() as u32,
        target_pos.y.round() as u32,
    );
    
    // Simple pathfinding for now - just move towards target
    let path = simple_pathfind(grid_pos, &target_grid);
    
    if !path.is_empty() {
        grid_movement.set_path(path.clone());
        debug.log(
            DebugLevel::Debug,
            "GRID_MOVE",
            &format!("Started movement to ({}, {}) with {} steps",
                target_grid.x, target_grid.y, path.len()),
        );
    }
}

/// Simple pathfinding (straight line for now)
fn simple_pathfind(from: &GridPosition, to: &GridPosition) -> Vec<GridPosition> {
    let mut path = Vec::new();
    let mut current = from.clone();
    
    while current != *to {
        current = current.step_toward(to);
        path.push(current.clone());
    }
    
    path
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
        (Entity, &PositionComponent, &mut ResourceNode, Option<&GrowingResource>),
        (With<BerryBushTag>, Without<UnitTag>),
    >,
    worker_pos: &PositionComponent,
    worker_entity: Entity,
) -> Option<(Entity, PositionComponent)> {
    use rand::Rng;
    
    // Collect all available berry bushes with their distances
    let mut available_bushes = Vec::new();
    
    for (entity, berry_pos, resource_node, growing_resource) in berries.iter() {
        // Check if bush has berries available
        let has_berries = if let Some(growing) = growing_resource {
            // Use GrowingResource if available (more accurate)
            growing.harvestable_amount > 0
        } else {
            // Fall back to ResourceNode
            resource_node.can_harvest()
        };
        
        // Check if bush is not fully claimed
        let can_claim = resource_node.claimed_by.len() < resource_node.max_workers
            || resource_node.claimed_by.contains(&worker_entity); // Or we already have a claim
        
        if has_berries && can_claim {
            let distance = worker_pos.distance_squared(berry_pos);
            available_bushes.push((entity, berry_pos.clone(), distance));
        }
    }
    
    if available_bushes.is_empty() {
        println!("   ❌ No berry bushes with fruit on entire map!");
        return None;
    }
    
    // Sort by distance
    available_bushes.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    
    // Take the top N nearest bushes (max 5)
    let candidates_count = available_bushes.len().min(5);
    let candidates = &available_bushes[0..candidates_count];
    
    // Randomly select one from the candidates with weighted probability
    // Closer bushes have higher chance of being selected
    let mut rng = rand::thread_rng();
    
    // Simple weighted random: closer bushes get more weight
    // Weight = 1/distance for simple inverse relationship
    let weights: Vec<f32> = candidates.iter()
        .map(|(_, _, dist)| {
            if *dist < 1.0 { 10.0 } // Very close gets high weight
            else { 1.0 / dist.sqrt() } // Further away gets less weight
        })
        .collect();
    
    let total_weight: f32 = weights.iter().sum();
    let mut random_value = rng.gen::<f32>() * total_weight;
    
    for (i, weight) in weights.iter().enumerate() {
        random_value -= weight;
        if random_value <= 0.0 {
            let (entity, pos, dist) = &candidates[i];
            println!("🎲 Selected berry bush {} of {} candidates at ({:.0}, {:.0}), distance: {:.1}", 
                i + 1, candidates_count, pos.x, pos.y, dist.sqrt());
            return Some((*entity, pos.clone()));
        }
    }
    
    // Fallback to first (nearest) if somehow we didn't select
    let (entity, pos, _) = &candidates[0];
    Some((*entity, pos.clone()))
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
