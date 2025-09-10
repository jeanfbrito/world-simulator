use bevy::prelude::*;
use crate::components::*;
use crate::ai::{Task, TaskType, TaskStatus, ActionPlan};
use crate::debug::{DebugSystem, DebugLevel};
use crate::buildings::BuildingComponent;
use crate::{MAP_SIZE, TileEntity};

const TILE_SIZE: f32 = 10.0;
const MOVE_SPEED: f32 = 20.0; // World units per second

/// System to execute tasks based on GOAP actions
pub fn task_execution_system(
    mut commands: Commands,
    mut workers: Query<(
        Entity,
        &mut Transform,
        &mut TileEntity,
        &ActionPlan,
        &mut IsWorking,
        &mut HasWood,
        &mut HasStone,
        &mut HasFood,
        &mut IsHungry,
        &mut HasEnergy,
        &PositionComponent,
    ), With<WorkerTag>>,
    buildings: Query<(&BuildingComponent, &PositionComponent)>,
    trees: Query<(Entity, &PositionComponent), With<TreeTag>>,
    rocks: Query<(Entity, &PositionComponent), With<RockTag>>,
    berries: Query<(Entity, &PositionComponent), With<BerryBushTag>>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    for (entity, mut transform, mut tile_entity, plan, mut is_working, 
         mut has_wood, mut has_stone, mut has_food, mut is_hungry, mut has_energy, worker_pos) in workers.iter_mut() {
        
        // Get current action from plan
        if let Some(action) = plan.current_action() {
            is_working.0 = true;
            
            match action.name.as_str() {
                "cut_wood" => {
                    // Find nearest tree
                    if let Some((tree_entity, tree_pos)) = find_nearest_tree(&trees, worker_pos) {
                        // Move towards tree
                        if move_towards(
                            &mut transform,
                            &mut tile_entity,
                            &tree_pos,
                            time.delta_secs(),
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
                    if let Some((rock_entity, rock_pos)) = find_nearest_rock(&rocks, worker_pos) {
                        // Move towards rock
                        if move_towards(
                            &mut transform,
                            &mut tile_entity,
                            &rock_pos,
                            time.delta_secs(),
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
                    // Find nearest berry bush
                    if let Some((berry_entity, berry_pos)) = find_nearest_berry(&berries, worker_pos) {
                        // Move towards berries
                        if move_towards(
                            &mut transform,
                            &mut tile_entity,
                            &berry_pos,
                            time.delta_secs(),
                            &debug,
                        ) {
                            // Reached berries, gather them
                            has_food.0 = has_food.0.saturating_add(3);
                            commands.entity(berry_entity).despawn();
                            debug.log(
                                DebugLevel::Info,
                                "TASK_EXEC",
                                &format!("Worker gathered berries, now has {} food", has_food.0)
                            );
                        }
                    }
                }
                
                "move_to_storage" | "get_wood_from_stockpile" | "get_stone_from_stockpile" => {
                    // Find stockpile
                    if let Some(stockpile_pos) = find_stockpile(&buildings) {
                        // Move towards stockpile
                        if move_towards(
                            &mut transform,
                            &mut tile_entity,
                            &stockpile_pos,
                            time.delta_secs(),
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
                    if move_towards(
                        &mut transform,
                        &mut tile_entity,
                        &house_pos,
                        time.delta_secs(),
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

fn move_towards(
    transform: &mut Transform,
    tile_entity: &mut TileEntity,
    target_pos: &PositionComponent,
    delta_time: f32,
    debug: &DebugSystem,
) -> bool {
    let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
    let target = Vec2::new(target_pos.x, target_pos.y);
    let direction = target - current_pos;
    let distance = direction.length();
    
    if distance < 5.0 {
        // Close enough
        return true;
    }
    
    // Move towards target
    let move_distance = MOVE_SPEED * delta_time;
    let movement = direction.normalize() * move_distance.min(distance);
    
    transform.translation.x += movement.x;
    transform.translation.y += movement.y;
    
    // Update tile position
    tile_entity.x = ((transform.translation.x / TILE_SIZE) + (MAP_SIZE as f32 / 2.0)) as usize;
    tile_entity.y = ((transform.translation.y / TILE_SIZE) + (MAP_SIZE as f32 / 2.0)) as usize;
    
    false
}

fn find_nearest_tree(
    trees: &Query<(Entity, &PositionComponent), With<TreeTag>>,
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
    rocks: &Query<(Entity, &PositionComponent), With<RockTag>>,
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
    berries: &Query<(Entity, &PositionComponent), With<BerryBushTag>>,
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
    buildings: &Query<(&BuildingComponent, &PositionComponent)>,
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