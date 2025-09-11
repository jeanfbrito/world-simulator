use bevy::prelude::*;
use crate::components::*;
use crate::ai::{Task, TaskType, TaskStatus, ActionPlan};
use crate::debug::{DebugSystem, DebugLevel};
use crate::buildings::BuildingComponent;
use crate::{MAP_SIZE, TileEntity};
use crate::resources::ResourceType;

const TILE_SIZE: f32 = 10.0;
const TILES_PER_TICK: f32 = 0.5; // Move half a tile per tick (at 10 TPS = 5 tiles/second)

/// System to execute tasks based on GOAP actions
pub fn task_execution_system(
    mut commands: Commands,
    mut workers: Query<(
        Entity,
        &mut Transform,
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
    ), With<WorkerTag>>,
    buildings: Query<(&BuildingComponent, &PositionComponent), Without<WorkerTag>>,
    trees: Query<(Entity, &PositionComponent), (With<TreeTag>, Without<WorkerTag>)>,
    rocks: Query<(Entity, &PositionComponent), (With<RockTag>, Without<WorkerTag>)>,
    berries: Query<(Entity, &PositionComponent), (With<BerryBushTag>, Without<WorkerTag>)>,
    sim_state: Res<crate::SimulationState>,
    debug: Res<DebugSystem>,
) {
    // Only execute on simulation ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, mut transform, mut tile_entity, mut plan, mut is_working, 
         mut has_wood, mut has_stone, mut has_food, mut is_hungry, mut has_energy, mut worker_pos, mut tiles_walked) in workers.iter_mut() {
        
        // Get current action from plan
        if let Some(action) = plan.current_action() {
            is_working.0 = true;
            
            match action.name.as_str() {
                "cut_wood" => {
                    // Find nearest tree
                    if let Some((tree_entity, tree_pos)) = find_nearest_tree(&trees, &*worker_pos) {
                        // Move towards tree (tick-based)
                        if tick_move_towards(
                            &mut transform,
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
                                &format!("Worker harvested tree, now has {} wood", has_wood.0)
                            );
                        }
                    }
                }
                
                "quarry_stone" => {
                    // Find nearest rock
                    if let Some((rock_entity, rock_pos)) = find_nearest_rock(&rocks, &*worker_pos) {
                        // Move towards rock (tick-based)
                        if tick_move_towards(
                            &mut transform,
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
                                &format!("Worker mined rock, now has {} stone", has_stone.0)
                            );
                        }
                    }
                }
                
                "gather_food" => {
                    debug.log(
                        DebugLevel::Info,
                        "TASK_EXEC",
                        &format!("Executing gather_food for entity at ({}, {})", worker_pos.x, worker_pos.y)
                    );
                    // Find nearest berry bush with berries available
                    if let Some((berry_entity, berry_pos)) = find_nearest_berry(&berries, &*worker_pos) {
                        debug.log(
                            DebugLevel::Info,
                            "TASK_EXEC",
                            &format!("Found berry at ({}, {}), moving from ({}, {})", 
                                berry_pos.x, berry_pos.y, worker_pos.x, worker_pos.y)
                        );
                        // Move towards berries (tick-based)
                        if tick_move_towards(
                            &mut transform,
                            &mut tile_entity,
                            &mut worker_pos,
                            &berry_pos,
                            &mut tiles_walked,
                            &debug,
                        ) {
                            // Reached berries - start gathering work
                            let mut work_progress = WorkProgress::new();
                            work_progress.start_work(
                                WorkType::Gathering(ResourceWork {
                                    resource_type: ResourceType::Berries,
                                    amount: 3,
                                    tool_bonus: 1.0,
                                }),
                                30, // 3 seconds at 10 TPS
                                Some(berry_entity),
                            );
                            
                            commands.entity(entity).insert(work_progress);
                            
                            debug.log(
                                DebugLevel::Info,
                                "TASK_EXEC",
                                "Worker started gathering berries"
                            );
                            
                            // Update old GOAP state for compatibility
                            has_food.0 = has_food.0.saturating_add(3);
                        }
                    } else {
                        debug.log(
                            DebugLevel::Info,
                            "TASK_EXEC",
                            &format!("No berries found from position ({}, {})", worker_pos.x, worker_pos.y)
                        );
                    }
                }
                
                "move_to_storage" | "get_wood_from_stockpile" | "get_stone_from_stockpile" => {
                    // Find stockpile
                    if let Some(stockpile_pos) = find_stockpile(&buildings) {
                        // Move towards stockpile (tick-based)
                        if tick_move_towards(
                            &mut transform,
                            &mut tile_entity,
                            &mut worker_pos,
                            &stockpile_pos,
                            &mut tiles_walked,
                            &debug,
                        ) {
                            debug.log(
                                DebugLevel::Info,
                                "TASK_EXEC",
                                "Worker reached stockpile"
                            );
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
                            &format!("Worker stored {} wood, {} stone at stockpile", has_wood.0, has_stone.0)
                        );
                        has_wood.0 = 0;
                        has_stone.0 = 0;
                    }
                }
                
                "eat_food" => {
                    if has_food.0 > 0 {
                        has_food.0 = has_food.0.saturating_sub(1);
                        is_hungry.0 = (is_hungry.0 - 0.5).max(0.0);  // Eating reduces hunger by 50%
                        debug.log(
                            DebugLevel::Info,
                            "TASK_EXEC",
                            &format!("Worker ate food (hunger: {:.0}%), {} food remaining", is_hungry.0 * 100.0, has_food.0)
                        );
                    }
                    // Eating is instant, advance to next action
                    plan.advance();
                }
                
                "rest" => {
                    is_working.0 = false;
                    has_energy.0 = (has_energy.0 + 0.02).min(1.0);  // Resting restores 2% energy per tick
                    debug.log(
                        DebugLevel::Debug,
                        "TASK_EXEC",
                        &format!("Worker is resting (energy: {:.0}%)", has_energy.0 * 100.0)
                    );
                }
                
                "build_house" => {
                    // Find a good spot for house (near center for now)
                    let house_pos = PositionComponent::from_tile(32, 28);
                    if tick_move_towards(
                        &mut transform,
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
                        debug.log(
                            DebugLevel::Info,
                            "TASK_EXEC",
                            "Worker built a house!"
                        );
                    }
                }
                
                _ => {
                    debug.log(
                        DebugLevel::Debug,
                        "TASK_EXEC",
                        &format!("Unknown action: {}", action.name)
                    );
                }
            }
        } else {
            is_working.0 = false;
        }
    }
}

/// Tick-based movement towards a target position
fn tick_move_towards(
    transform: &mut Transform,
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
    
    // Check if we're close enough (within 1 tile)
    if distance < 1.0 {
        return true;
    }
    
    // Move TILES_PER_TICK tiles towards the target
    let move_distance = TILES_PER_TICK.min(distance);
    let move_x = (dx / distance) * move_distance;
    let move_y = (dy / distance) * move_distance;
    
    // Update position in tile coordinates
    position_comp.x += move_x;
    position_comp.y += move_y;
    
    // Track tiles walked
    tiles_walked.add(move_distance);
    
    // Update tile entity (integer tile position)
    tile_entity.x = position_comp.x.round() as usize;
    tile_entity.y = position_comp.y.round() as usize;
    
    // Update world transform for rendering (if we had rendering)
    let world_x = (position_comp.x - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
    let world_y = (position_comp.y - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
    transform.translation.x = world_x;
    transform.translation.y = world_y;
    
    debug.log(
        DebugLevel::Debug,
        "MOVEMENT",
        &format!("Moved to tile ({:.1}, {:.1}), distance remaining: {:.1}",
            position_comp.x, position_comp.y, distance - move_distance)
    );
    
    false
}

fn find_nearest_tree(
    trees: &Query<(Entity, &PositionComponent), (With<TreeTag>, Without<WorkerTag>)>,
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
    rocks: &Query<(Entity, &PositionComponent), (With<RockTag>, Without<WorkerTag>)>,
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
    berries: &Query<(Entity, &PositionComponent), (With<BerryBushTag>, Without<WorkerTag>)>,
    worker_pos: &PositionComponent,
) -> Option<(Entity, PositionComponent)> {
    let mut nearest = None;
    let mut min_distance = f32::MAX;
    
    for (entity, berry_pos) in berries.iter() {
        let distance = worker_pos.distance_squared(berry_pos);
        if distance < min_distance {
            min_distance = distance;
            nearest = Some((entity, berry_pos.clone()));
        }
    }
    
    nearest
}

fn find_stockpile(
    buildings: &Query<(&BuildingComponent, &PositionComponent), Without<WorkerTag>>,
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