use crate::components::{
    GridPosition, NameComponent, Stockpile, StorageBuilding, StorageChangeType,
    StorageChangedEvent, StorageTask, StorageTaskState, StorageUpdateTag, UnitInventory, UnitTag,
    Warehouse, WorkProgress,
};
use crate::resources::ResourceType;
use crate::{SimulationState, TileEntity};
/// Storage management systems for resource buildings
///
/// Handles resource deposits, withdrawals, and transfers
/// between units and storage buildings on simulation ticks.
use bevy::prelude::*;
use colored::Colorize;

/// System to handle units depositing resources into storage
pub fn storage_deposit_system(
    sim_state: Res<SimulationState>,
    mut units: Query<
        (
            Entity,
            &GridPosition,
            &mut UnitInventory,
            &mut StorageTask,
            &NameComponent,
        ),
        With<UnitTag>,
    >,
    mut storages: Query<(
        Entity,
        &GridPosition,
        &mut StorageBuilding,
        Option<&Stockpile>,
        Option<&Warehouse>,
    )>,
    mut events: EventWriter<StorageChangedEvent>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }

    for (unit_entity, unit_pos, mut inventory, mut task, name) in units.iter_mut() {
        // Skip if not doing storage task
        if task.state != StorageTaskState::Depositing {
            continue;
        }

        // Find nearby storage
        let mut deposited = false;
        for (storage_entity, storage_pos, mut storage, stockpile, warehouse) in storages.iter_mut()
        {
            // Check if adjacent to storage
            if unit_pos.distance_to(storage_pos) > 1 {
                continue;
            }

            // Try to deposit each resource type in inventory
            for (resource_type, amount) in inventory.get_all_resources() {
                if amount == 0 {
                    continue;
                }

                let old_amount = storage.get_amount(&resource_type);
                let deposited_amount = storage.deposit(resource_type.clone(), amount);

                if deposited_amount > 0 {
                    inventory.remove_item(resource_type.clone(), deposited_amount);

                    // Send event
                    events.write(StorageChangedEvent {
                        storage_entity,
                        resource_type: resource_type.clone(),
                        old_amount,
                        new_amount: old_amount + deposited_amount,
                        change_type: StorageChangeType::Deposit,
                    });

                    println!(
                        "{} {} deposited {} {} into storage",
                        "📦".green(),
                        name.name.cyan(),
                        deposited_amount,
                        format!("{:?}", resource_type).yellow()
                    );

                    debug.log(
                        DebugLevel::Info,
                        "STORAGE",
                        &format!(
                            "{} deposited {} {:?} into storage at ({},{})",
                            name.name,
                            deposited_amount,
                            resource_type,
                            storage_pos.x,
                            storage_pos.y
                        ),
                    );

                    deposited = true;
                }
            }
        }

        if deposited {
            task.state = StorageTaskState::Complete;
        }
    }
}

/// System to handle units withdrawing resources from storage
pub fn storage_withdrawal_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(Entity, &GridPosition, &mut UnitInventory, &NameComponent), With<UnitTag>>,
    mut storages: Query<(Entity, &GridPosition, &mut StorageBuilding)>,
    mut events: EventWriter<StorageChangedEvent>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only process on ticks
    if !sim_state.just_ticked || sim_state.tick % 10 != 0 {
        return;
    }

    for (unit_entity, unit_pos, mut inventory, name) in units.iter_mut() {
        // Skip if inventory is not empty (has resources to deposit first)
        if !inventory.is_empty() {
            continue;
        }

        // Find nearby storage to withdraw from
        for (storage_entity, storage_pos, mut storage) in storages.iter_mut() {
            // Check if adjacent to storage
            if unit_pos.distance_to(storage_pos) > 1 {
                continue;
            }

            // Look for needed resources (e.g., food when hungry)
            // For now, just withdraw any available resource
            // Collect available resources first to avoid borrow issues
            let available_resources: Vec<(ResourceType, u32)> = storage
                .stored
                .iter()
                .filter(|(_, &amount)| amount > 0)
                .map(|(k, v)| (k.clone(), *v))
                .collect();

            for (resource_type, stored_amount) in available_resources {
                // Try to withdraw up to inventory capacity
                let space = inventory.remaining_capacity();
                let withdraw_amount = space.min(stored_amount).min(10); // Max 10 at a time

                if withdraw_amount > 0 {
                    let old_amount = stored_amount;
                    let withdrawn = storage.withdraw(resource_type.clone(), withdraw_amount);

                    if withdrawn > 0 {
                        inventory.add_item(resource_type.clone(), withdrawn);

                        // Send event
                        events.write(StorageChangedEvent {
                            storage_entity,
                            resource_type: resource_type.clone(),
                            old_amount,
                            new_amount: old_amount - withdrawn,
                            change_type: StorageChangeType::Withdrawal,
                        });

                        debug.log(
                            DebugLevel::Debug,
                            "STORAGE",
                            &format!(
                                "{} withdrew {} {:?} from storage",
                                name.name, withdrawn, resource_type
                            ),
                        );

                        break; // Only withdraw one type per tick
                    }
                }
            }
        }
    }
}

/// System to assign storage tasks to units with full inventories
pub fn storage_task_assignment_system(
    sim_state: Res<SimulationState>,
    mut units: Query<
        (
            Entity,
            &GridPosition,
            &UnitInventory,
            &mut StorageTask,
            &WorkProgress,
            &NameComponent,
        ),
        With<UnitTag>,
    >,
    storages: Query<(Entity, &GridPosition, &StorageBuilding)>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only process every 5 ticks
    if !sim_state.just_ticked || sim_state.tick % 5 != 0 {
        return;
    }

    for (unit_entity, unit_pos, inventory, mut task, work, name) in units.iter_mut() {
        // Skip if already has a task or is working
        if task.state != StorageTaskState::Idle || work.is_working {
            continue;
        }

        // Skip if inventory is not nearly full
        if inventory.current_weight < inventory.max_weight * 0.8 {
            continue;
        }

        // Find nearest storage with space
        let mut best_storage: Option<(Entity, u32)> = None;

        for (storage_entity, storage_pos, storage) in storages.iter() {
            // Check if storage can accept deposits
            if !storage.accepting_deposits || storage.is_full() {
                continue;
            }

            // Check if storage accepts the resources we have
            let can_store_any = inventory
                .get_all_resources()
                .iter()
                .any(|(res_type, amount)| *amount > 0 && storage.can_store(res_type));

            if !can_store_any {
                continue;
            }

            let distance = unit_pos.distance_to(storage_pos);

            if best_storage.is_none() || distance < best_storage.unwrap().1 {
                best_storage = Some((storage_entity, distance));
            }
        }

        // Assign storage task to nearest valid storage
        if let Some((target_entity, distance)) = best_storage {
            // Get primary resource type to transport
            if let Some((resource_type, amount)) = inventory
                .get_all_resources()
                .iter()
                .max_by_key(|(_, amt)| *amt)
                .map(|(t, a)| (t.clone(), *a))
            {
                *task = StorageTask::new(target_entity, resource_type.clone(), amount);

                debug.log(
                    DebugLevel::Info,
                    "STORAGE",
                    &format!(
                        "{} assigned to deposit {} {:?} to storage ({}m away)",
                        name.name, amount, resource_type, distance
                    ),
                );
            }
        }
    }
}

/// System to update storage tasks
pub fn storage_task_update_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(&mut StorageTask, &GridPosition, &NameComponent), With<UnitTag>>,
    storages: Query<&GridPosition, With<StorageBuilding>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }

    for (mut task, unit_pos, name) in units.iter_mut() {
        if task.state == StorageTaskState::Idle || task.state == StorageTaskState::Complete {
            continue;
        }

        // Update task progress
        if task.tick_update() {
            debug.log(
                DebugLevel::Debug,
                "STORAGE",
                &format!("{} completed storage task", name.name),
            );

            // Reset task
            *task = StorageTask::default();
        } else {
            // Check state transitions based on position
            if task.state == StorageTaskState::MovingToStorage {
                if let Some(target) = task.target_storage {
                    if let Ok(storage_pos) = storages.get(target) {
                        if unit_pos.distance_to(storage_pos) <= 1 {
                            task.state = StorageTaskState::Depositing;
                            task.progress_counter = 5; // 5 ticks to deposit
                        }
                    }
                }
            }
        }
    }
}

/// System to spawn initial storage buildings
pub fn spawn_storage_buildings_system(
    mut commands: Commands,
    sim_state: Res<SimulationState>,
    existing: Query<Entity, With<StorageBuilding>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only spawn once at startup
    if sim_state.tick != 1 || !existing.is_empty() {
        return;
    }

    // Spawn a stockpile at center of map
    let center_x = 32u32;
    let center_y = 32u32;

    // Main stockpile (3x3)
    commands.spawn((
        NameComponent::new("Central Stockpile".to_string()),
        GridPosition {
            x: center_x,
            y: center_y,
        },
        TileEntity {
            x: center_x as usize,
            y: center_y as usize,
        },
        StorageBuilding::new(900), // 3x3 * 100 capacity
        Stockpile::new(3, 3),
        StorageUpdateTag,
    ));

    println!(
        "{} Central Stockpile created at ({}, {})",
        "🏗️".green(),
        center_x,
        center_y
    );

    // Spawn a warehouse nearby
    commands.spawn((
        NameComponent::new("Main Warehouse".to_string()),
        GridPosition {
            x: center_x + 5,
            y: center_y,
        },
        TileEntity {
            x: (center_x + 5) as usize,
            y: center_y as usize,
        },
        StorageBuilding::new(2000), // Larger capacity
        Warehouse::new(),
        StorageUpdateTag,
    ));

    println!(
        "{} Main Warehouse created at ({}, {})",
        "🏢".cyan(),
        center_x + 5,
        center_y
    );

    debug.log(
        DebugLevel::Info,
        "STORAGE",
        "Initial storage buildings spawned",
    );
}

/// System to update warehouse efficiency based on workers
pub fn warehouse_efficiency_system(
    sim_state: Res<SimulationState>,
    mut warehouses: Query<(&mut StorageBuilding, &Warehouse), Changed<Warehouse>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }

    for (mut storage, warehouse) in warehouses.iter_mut() {
        let efficiency = warehouse.get_efficiency();

        // Adjust capacity based on efficiency
        let base_capacity = 2000u32;
        storage.total_capacity = (base_capacity as f32 * efficiency) as u32;

        debug.log(
            DebugLevel::Debug,
            "STORAGE",
            &format!(
                "Warehouse efficiency: {:.0}%, capacity: {}",
                efficiency * 100.0,
                storage.total_capacity
            ),
        );
    }
}

/// System to display storage contents (debug)
pub fn storage_display_system(
    sim_state: Res<SimulationState>,
    storages: Query<(&NameComponent, &StorageBuilding)>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only display every 100 ticks
    if !sim_state.just_ticked || sim_state.tick % 100 != 0 {
        return;
    }

    for (name, storage) in storages.iter() {
        if storage.is_empty() {
            continue;
        }

        let mut contents = String::new();
        for (resource_type, amount) in &storage.stored {
            if *amount > 0 {
                contents.push_str(&format!("{:?}:{} ", resource_type, amount));
            }
        }

        println!(
            "{} {} [{}/{}]: {}",
            "📊".blue(),
            name.name.white(),
            storage.current_total,
            storage.total_capacity,
            contents.yellow()
        );

        debug.log(
            DebugLevel::Info,
            "STORAGE",
            &format!(
                "{} contains: {} ({:.0}% full)",
                name.name,
                contents,
                storage.fill_percentage() * 100.0
            ),
        );
    }
}

/// Performance monitoring for storage systems
pub fn storage_performance_monitor_system(
    sim_state: Res<SimulationState>,
    storages: Query<&StorageBuilding>,
    tasks: Query<&StorageTask, With<UnitTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only check every 200 ticks
    if !sim_state.just_ticked || sim_state.tick % 200 != 0 {
        return;
    }

    let total_storages = storages.iter().count();
    let total_capacity: u32 = storages.iter().map(|s| s.total_capacity).sum();
    let total_stored: u32 = storages.iter().map(|s| s.current_total).sum();
    let active_tasks = tasks
        .iter()
        .filter(|t| t.state != StorageTaskState::Idle && t.state != StorageTaskState::Complete)
        .count();

    let utilization = if total_capacity > 0 {
        (total_stored as f32 / total_capacity as f32) * 100.0
    } else {
        0.0
    };

    debug.log(
        DebugLevel::Debug,
        "PERFORMANCE",
        &format!(
            "Storage: {} buildings, {}/{} capacity ({:.0}% util), {} active tasks",
            total_storages, total_stored, total_capacity, utilization, active_tasks
        ),
    );
}
