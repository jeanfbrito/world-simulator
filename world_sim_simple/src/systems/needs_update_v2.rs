use crate::ai::BehaviorStateNew as BehaviorState;
use crate::components::{NameComponent, UnitInventory, UnitLocation, UnitNeedsV2, UnitTag, UnitMind};
use crate::SimulationState;
/// Tick-based needs update system
///
/// This system updates unit needs ONLY on simulation ticks, not every frame.
/// It uses the new counter-based UnitNeedsV2 component for better performance
/// with large numbers of units.
use bevy::prelude::*;
use colored::Colorize;

/// System that updates unit needs on each simulation tick
pub fn update_unit_needs_tick_system(
    sim_state: Res<SimulationState>,
    mut units: Query<
        (
            Entity,
            &mut UnitNeedsV2,
            &NameComponent,
            Option<&BehaviorState>,
            Option<&UnitMind>,
        ),
        With<UnitTag>,
    >,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // CRITICAL: Only update on ticks, not every frame!
    if !sim_state.just_ticked {
        return;
    }

    for (entity, mut needs, name, behavior, mind) in units.iter_mut() {
        // Store old values for change detection
        let old_hunger = needs.hunger();
        let old_energy = needs.energy();
        let old_starving = needs.is_starving();
        let old_exhausted = needs.is_exhausted();

        // Check if unit is resting (either through BehaviorState or UnitMind)
        let is_resting = matches!(behavior, Some(BehaviorState::Resting)) || 
                        matches!(mind, Some(UnitMind::Resting));

        // Update based on current state
        if is_resting {
            // Special update when resting - recovers energy
            needs.tick_rest();
            debug.log(
                DebugLevel::Debug,
                "NEEDS",
                &format!("{} is resting, energy recovering", name.name),
            );
        } else if matches!(behavior, Some(BehaviorState::Eating)) {
            // Eating happens as discrete action, not tick update
            needs.tick_update();
        } else {
            // Normal tick update
            needs.tick_update();
        }

        // Log significant state changes
        if needs.is_starving() && !old_starving {
            println!(
                "{} {} is now {}!",
                "⚠️".red().bold(),
                name.name.red().bold(),
                "STARVING".red().bold()
            );
            debug.log(
                DebugLevel::Info,
                "NEEDS",
                &format!("{} entered STARVING state", name.name),
            );
        }

        if needs.is_exhausted() && !old_exhausted {
            println!(
                "{} {} is now {}!",
                "⚠️".yellow().bold(),
                name.name.yellow().bold(),
                "EXHAUSTED".yellow().bold()
            );
            debug.log(
                DebugLevel::Info,
                "NEEDS",
                &format!("{} entered EXHAUSTED state", name.name),
            );
        }

        // Log periodic status (every 10 ticks)
        if sim_state.tick % 10 == 0 {
            debug.log(
                DebugLevel::Debug,
                "NEEDS",
                &format!("{}: {}", name.name, needs.debug_string()),
            );
        }
    }
}

// sync_needs_v2_to_worldstate_system removed - now handled by dogoap state sync

/// System to handle eating action (immediate effect, not tick-based)
pub fn eating_action_system(
    sim_state: Res<SimulationState>,
    mut query: Query<
        (
            &mut UnitNeedsV2,
            &mut UnitInventory,
            &BehaviorState,
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

    for (mut needs, mut inventory, behavior, name) in query.iter_mut() {
        if *behavior == BehaviorState::Eating {
            // Check for berries first (our main food source)
            let berries_amount = inventory.get_amount(crate::resources::ResourceType::Berries);
            if berries_amount > 0 && needs.is_hungry() {
                // Consume one berry
                if inventory.remove_item(crate::resources::ResourceType::Berries, 1) {
                    needs.eat_food(1);

                    println!(
                        "{} {} ate blueberries (hunger: {:.0}% → {:.0}%)",
                        "🫐".green(),
                        name.name.cyan(),
                        needs.hunger() * 100.0 + 20.0, // Before (approximate)
                        needs.hunger() * 100.0         // After
                    );

                    debug.log(
                        DebugLevel::Info,
                        "ACTION",
                        &format!(
                            "{} consumed blueberries, hunger now at {:.0}%",
                            name.name,
                            needs.hunger() * 100.0
                        ),
                    );
                    continue; // Successfully ate
                }
            }

            // For now, only berries are supported as food
            // Can add more food types here later (Wheat, Bread, Fish, Meat)
        }
    }
}

/// Sync HasEnergy component back to UnitNeedsV2 when modified by actions
pub fn sync_has_energy_to_needs_system(
    sim_state: Res<SimulationState>,
    mut query: Query<
        (
            &mut UnitNeedsV2,
            &crate::components::HasEnergy,
        ),
        With<UnitTag>,
    >,
) {
    // Only sync on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (mut needs, has_energy) in query.iter_mut() {
        // Convert HasEnergy (0.0-1.0) to energy_counter (0-100,000)
        let new_energy_counter = (has_energy.0 * 100_000.0) as u32;
        
        // Get current values
        let current_energy = needs.energy();
        let current_energy_counter = (current_energy * 100_000.0) as u32;
        
        // Only update if significantly different (avoid tiny floating point differences)
        if (current_energy_counter as i32 - new_energy_counter as i32).abs() > 100 {
            // Keep hunger and morale the same, only update energy
            let hunger_counter = (needs.hunger() * 100_000.0) as u32;
            let morale_counter = (needs.morale() * 100_000.0) as u32;
            
            needs.set_counters(hunger_counter, new_energy_counter, morale_counter);
        }
    }
}

/// Performance monitoring for the needs system
pub fn needs_performance_monitor_system(
    sim_state: Res<SimulationState>,
    units: Query<&UnitNeedsV2, With<UnitTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only check every 100 ticks
    if !sim_state.just_ticked || sim_state.tick % 100 != 0 {
        return;
    }

    let unit_count = units.iter().count();
    let updates_per_second = unit_count * 10; // At 10 TPS

    debug.log(
        DebugLevel::Debug,
        "PERFORMANCE",
        &format!(
            "Needs system: {} units, {} updates/sec (tick-based)",
            unit_count, updates_per_second
        ),
    );

    // Compare to old system
    let old_updates_per_second = unit_count * 60; // At 60 FPS
    let improvement = old_updates_per_second as f32 / updates_per_second as f32;

    debug.log(
        DebugLevel::Debug,
        "PERFORMANCE",
        &format!(
            "Performance improvement: {:.1}x fewer updates than frame-based",
            improvement
        ),
    );
}
