use crate::ai::goap_actions::{StateValue, WorldState};
use crate::buildings::{BuildingComponent, BuildingType};
use crate::components::*;
use crate::debug::DebugSystem;
use crate::resources::{Inventory, ItemType};
use bevy::prelude::*;

/// System to synchronize worker states with GOAP states
pub fn sync_goap_states_system(
    mut commands: Commands,
    mut workers: Query<
        (
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
            Option<&mut HasHouse>,
        ),
        With<UnitTag>,
    >,
    unit_inventories: Query<&UnitInventory>,
    mut world_states: Query<&mut WorldState>,
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
    for (_, _) in buildings.iter() {
        building_count += 1;
        // Note: We'd need to count storage resources separately
        // For now just counting building types
    }

    // Update each worker's GOAP states
    for (
        entity,
        pos,
        energy,
        stats,
        inventory,
        is_hungry,
        has_energy,
        is_working,
        is_idle,
        has_wood,
        has_food,
        has_stone,
        inventory_full,
        inventory_empty,
        has_house,
    ) in workers.iter_mut()
    {
        population += 1;

        // Don't overwrite GOAP states - they're managed by update_needs_system
        // Just ensure they exist with initial values if missing
        if is_hungry.is_none() {
            commands.entity(entity).insert(IsHungry(0.0));
        }

        if has_energy.is_none() {
            commands.entity(entity).insert(HasEnergy(1.0));
        }

        // Read current hunger/energy for logging
        let hunger_level = if let Some(ref hungry) = is_hungry {
            hungry.0
        } else {
            0.0
        };

        let energy_level = if let Some(ref energy_state) = has_energy {
            energy_state.0
        } else {
            1.0
        };

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
        let mut wood_count = 0u32;
        let mut food_count = 0u32;
        let mut stone_count = 0u32;
        let mut is_full = false;
        let mut is_empty = true;

        // Check UnitInventory first (most common for peasants)
        if let Ok(unit_inv) = unit_inventories.get(entity) {
            // Count items from UnitInventory
            wood_count = unit_inv.get_amount(crate::resources::ResourceType::Wood);
            food_count = unit_inv.get_amount(crate::resources::ResourceType::Berries); // Berries are food!
            stone_count = unit_inv.get_amount(crate::resources::ResourceType::Stone);

            is_full = unit_inv.is_full();
            is_empty = unit_inv.is_empty();

            // Update component states with inventory counts
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
        } else if let Some(inv) = inventory {
            // Fallback to slot-based inventory system
            let mut total_items = 0u32;

            for slot in &inv.slots {
                if let Some(stack) = &slot.item_stack {
                    total_items += stack.count;
                    if let ItemType::Resource(ref res_type) = stack.item.item_type {
                        // Count resources based on name (simplified)
                        if stack.item.name.contains("Wood") {
                            wood_count += stack.count;
                        } else if stack.item.name.contains("Food")
                            || stack.item.name.contains("Berries")
                        {
                            food_count += stack.count; // Berries count as food
                        } else if stack.item.name.contains("Stone") {
                            stone_count += stack.count;
                        }
                    }
                }
            }

            // Update component states with inventory counts
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

            is_full = inv.slots.iter().all(|s| !s.is_empty());
            is_empty = total_items == 0;
        } else {
            // No inventory system - use the simple component values directly
            // Don't overwrite existing values, they're managed by task_executor
            wood_count = has_wood.as_ref().map_or(0, |w| w.0);
            food_count = has_food.as_ref().map_or(0, |f| f.0);
            stone_count = has_stone.as_ref().map_or(0, |s| s.0);

            // Ensure components exist with current values
            if has_wood.is_none() {
                commands.entity(entity).insert(HasWood(wood_count));
            }
            if has_food.is_none() {
                commands.entity(entity).insert(HasFood(food_count));
            }
            if has_stone.is_none() {
                commands.entity(entity).insert(HasStone(stone_count));
            }

            // Simple heuristic for full/empty
            let total = wood_count + food_count + stone_count;
            is_full = total >= 30; // Arbitrary max capacity
            is_empty = total == 0;
        }

        // Update inventory full/empty states
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

        // Check location states
        let near_distance = 2.0;
        let at_resource = false;
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
        commands
            .entity(entity)
            .insert(AtResource(at_resource))
            .insert(AtStorage(at_storage))
            .insert(AtHome(at_home))
            .insert(AtCraftingStation(at_crafting));

        // Update WorldState component with current states
        if let Ok(mut ws) = world_states.get_mut(entity) {
            ws.set("is_hungry", StateValue::Float(hunger_level));
            ws.set("has_energy", StateValue::Float(energy_level));
            ws.set("is_working", StateValue::Bool(working));
            ws.set("has_wood", StateValue::Int(wood_count));
            ws.set("has_food", StateValue::Int(food_count));
            ws.set("has_stone", StateValue::Int(stone_count));
            ws.set("at_resource", StateValue::Bool(at_resource));
            ws.set("at_storage", StateValue::Bool(at_storage));
            ws.set("inventory_full", StateValue::Bool(is_full));
            ws.set(
                "has_house",
                StateValue::Bool(has_house.as_ref().is_some_and(|h| h.0)),
            );
        } else {
            // Create new WorldState if missing
            let mut ws = WorldState::new();
            ws.set("is_hungry", StateValue::Float(hunger_level));
            ws.set("has_energy", StateValue::Float(energy_level));
            ws.set("is_working", StateValue::Bool(working));
            ws.set("has_wood", StateValue::Int(wood_count));
            ws.set("has_food", StateValue::Int(food_count));
            ws.set("has_stone", StateValue::Int(stone_count));
            ws.set("at_resource", StateValue::Bool(at_resource));
            ws.set("at_storage", StateValue::Bool(at_storage));
            ws.set("inventory_full", StateValue::Bool(is_full));
            ws.set(
                "has_house",
                StateValue::Bool(has_house.as_ref().is_some_and(|h| h.0)),
            );
            commands.entity(entity).insert(ws);
        }

        // Log state changes for first worker (for debugging)
        if population == 1 {
            debug_system.log(
                crate::debug::DebugLevel::Info,
                "WORKER_STATE",
                &format!(
                    "Worker 1 - Hunger: {:.0}%, Energy: {:.0}%, Wood: {}, Food: {}, Working: {}",
                    hunger_level * 100.0,
                    energy_level * 100.0,
                    wood_count,
                    food_count,
                    if working { "✓" } else { "✗" }
                ),
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
    let has_storage = buildings
        .iter()
        .any(|(b, _)| b.building_type == BuildingType::Storage);
    let has_house = buildings
        .iter()
        .any(|(b, _)| b.building_type == BuildingType::House);
    let has_workshop = buildings
        .iter()
        .any(|(b, _)| b.building_type == BuildingType::Workshop);
    let has_farm = buildings
        .iter()
        .any(|(b, _)| b.building_type == BuildingType::Farm);

    // Update all workers with building availability
    for (entity, _, _, _, _, _, _, _, _, _, _, _, _, _, _) in workers.iter_mut() {
        commands
            .entity(entity)
            .insert(StorageAvailable(has_storage))
            .insert(HouseAvailable(has_house))
            .insert(WorkshopAvailable(has_workshop))
            .insert(FarmAvailable(has_farm));
    }
}
