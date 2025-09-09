use bevy::prelude::*;
use crate::components::*;
use crate::resources::{Inventory, ItemType};
use crate::buildings::{BuildingComponent, BuildingType};
use crate::debug::DebugSystem;

/// System to synchronize worker states with GOAP states
pub fn sync_goap_states_system(
    mut commands: Commands,
    mut workers: Query<(
        Entity,
        &PositionComponent,
        &EnergyComponent,
        &WorkerStats,
        Option<&Inventory>,
        Option<&mut IsHungry>,
        Option<&mut HasEnergy>,
        Option<&mut IsWorking>,
        Option<&mut IsIdle>,
        Option<&mut HasWood>,
        Option<&mut HasFood>,
        Option<&mut HasStone>,
        Option<&mut InventoryFull>,
        Option<&mut InventoryEmpty>,
    ), With<WorkerTag>>,
    buildings: Query<(&BuildingComponent, &PositionComponent)>,
    debug_system: Res<DebugSystem>,
    mut settlement_state: ResMut<SettlementState>,
) {
    // Update settlement state
    let mut total_food = 0u32;
    let mut total_wood = 0u32;
    let mut total_stone = 0u32;
    let mut population = 0i32;
    let mut building_count = 0i32;
    
    // Count buildings
    for (building, _) in buildings.iter() {
        building_count += 1;
        // Note: We'd need to count storage resources separately
        // For now just counting building types
    }
    
    // Update each worker's GOAP states
    for (entity, pos, energy, stats, inventory, 
         mut is_hungry, mut has_energy, mut is_working, mut is_idle,
         mut has_wood, mut has_food, mut has_stone,
         mut inventory_full, mut inventory_empty) in workers.iter_mut() {
        
        population += 1;
        
        // Calculate hunger based on energy (simplified for now)
        let hunger_level = 1.0 - (energy.current as f64 / 100.0);
        
        // Update or insert basic states
        if let Some(mut hungry) = is_hungry {
            hungry.update(hunger_level);
        } else {
            commands.entity(entity).insert(IsHungry(hunger_level));
        }
        
        if let Some(mut energy_state) = has_energy {
            energy_state.update(energy.current as f64 / 100.0);
        } else {
            commands.entity(entity).insert(HasEnergy(energy.current as f64 / 100.0));
        }
        
        // Update work states (simplified - check if has energy)
        let working = energy.current > 30.0;
        if let Some(mut work_state) = is_working {
            work_state.0 = working;
        } else {
            commands.entity(entity).insert(IsWorking(working));
        }
        
        if let Some(mut idle_state) = is_idle {
            idle_state.0 = !working;
        } else {
            commands.entity(entity).insert(IsIdle(!working));
        }
        
        // Update inventory states (count items by type)
        if let Some(inv) = inventory {
            let mut wood_count = 0u32;
            let mut food_count = 0u32;
            let mut stone_count = 0u32;
            let mut total_items = 0u32;
            
            for slot in &inv.slots {
                if let Some(stack) = &slot.item_stack {
                    total_items += stack.count;
                    match stack.item.item_type {
                        ItemType::Resource(ref res_type) => {
                            // Count resources based on name (simplified)
                            if stack.item.name.contains("Wood") {
                                wood_count += stack.count;
                            } else if stack.item.name.contains("Food") {
                                food_count += stack.count;
                            } else if stack.item.name.contains("Stone") {
                                stone_count += stack.count;
                            }
                        }
                        _ => {}
                    }
                }
            }
            
            if let Some(mut wood_state) = has_wood {
                wood_state.0 = wood_count;
            } else {
                commands.entity(entity).insert(HasWood(wood_count));
            }
            
            if let Some(mut food_state) = has_food {
                food_state.0 = food_count;
            } else {
                commands.entity(entity).insert(HasFood(food_count));
            }
            
            if let Some(mut stone_state) = has_stone {
                stone_state.0 = stone_count;
            } else {
                commands.entity(entity).insert(HasStone(stone_count));
            }
            
            let is_full = inv.slots.iter().all(|s| !s.is_empty());
            let is_empty = total_items == 0;
            
            if let Some(mut full_state) = inventory_full {
                full_state.0 = is_full;
            } else {
                commands.entity(entity).insert(InventoryFull(is_full));
            }
            
            if let Some(mut empty_state) = inventory_empty {
                empty_state.0 = is_empty;
            } else {
                commands.entity(entity).insert(InventoryEmpty(is_empty));
            }
            
            // Add to total resources
            total_food += food_count;
            total_wood += wood_count;
            total_stone += stone_count;
        }
        
        // Check location states
        let near_distance = 2.0;
        let mut at_resource = false;
        let mut at_storage = false;
        let mut at_home = false;
        let mut at_crafting = false;
        
        for (building, bpos) in buildings.iter() {
            let dist = ((pos.x - bpos.x).powi(2) + (pos.y - bpos.y).powi(2)).sqrt();
            if dist <= near_distance {
                match building.building_type {
                    BuildingType::Storage => at_storage = true,
                    BuildingType::House => at_home = true,
                    BuildingType::Workshop => at_crafting = true,
                    _ => {}
                }
            }
        }
        
        // Update location states
        commands.entity(entity)
            .insert(AtResource(at_resource))
            .insert(AtStorage(at_storage))
            .insert(AtHome(at_home))
            .insert(AtCraftingStation(at_crafting));
        
        // Log state changes for first worker (for debugging)
        if population == 1 {
            debug_system.log(
                crate::debug::DebugLevel::Debug,
                "GOAP",
                &format!("Worker states - Hungry: {:.1}, Energy: {:.1}, Working: {}", 
                    hunger_level, energy.current as f64 / 100.0, working)
            );
        }
    }
    
    // Update global settlement state
    settlement_state.food_supply = total_food;
    settlement_state.wood_supply = total_wood;
    settlement_state.stone_supply = total_stone;
    settlement_state.population_count = population;
    settlement_state.building_count = building_count;
    
    // Check building availability
    let has_storage = buildings.iter().any(|(b, _)| b.building_type == BuildingType::Storage);
    let has_house = buildings.iter().any(|(b, _)| b.building_type == BuildingType::House);
    let has_workshop = buildings.iter().any(|(b, _)| b.building_type == BuildingType::Workshop);
    let has_farm = buildings.iter().any(|(b, _)| b.building_type == BuildingType::Farm);
    
    // Update all workers with building availability
    for (entity, _, _, _, _, _, _, _, _, _, _, _, _, _) in workers.iter_mut() {
        commands.entity(entity)
            .insert(StorageAvailable(has_storage))
            .insert(HouseAvailable(has_house))
            .insert(WorkshopAvailable(has_workshop))
            .insert(FarmAvailable(has_farm));
    }
}