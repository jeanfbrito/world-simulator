/// Tick-based needs update system
/// 
/// This system updates unit needs ONLY on simulation ticks, not every frame.
/// It uses the new counter-based UnitNeedsV2 component for better performance
/// with large numbers of units.

use bevy::prelude::*;
use crate::components::{UnitNeedsV2, UnitInventory, UnitLocation, PeasantTag, NameComponent};
use crate::SimulationState;
use crate::ai::BehaviorStateNew as BehaviorState;
use colored::Colorize;

/// System that updates unit needs on each simulation tick
pub fn update_unit_needs_tick_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity, 
        &mut UnitNeedsV2,
        &NameComponent,
        Option<&BehaviorState>
    ), With<PeasantTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // CRITICAL: Only update on ticks, not every frame!
    if !sim_state.just_ticked {
        return;
    }
    
    for (entity, mut needs, name, behavior) in units.iter_mut() {
        // Store old values for change detection
        let old_hunger = needs.hunger();
        let old_energy = needs.energy();
        let old_starving = needs.is_starving();
        let old_exhausted = needs.is_exhausted();
        
        // Update based on current behavior
        match behavior {
            Some(BehaviorState::Resting) => {
                // Special update when resting
                needs.tick_rest();
                debug.log(
                    DebugLevel::Debug,
                    "NEEDS",
                    &format!("{} is resting, energy recovering", name.name)
                );
            }
            Some(BehaviorState::Eating) => {
                // Eating happens as discrete action, not tick update
                needs.tick_update();
            }
            _ => {
                // Normal tick update
                needs.tick_update();
            }
        }
        
        // Log significant state changes
        if needs.is_starving() && !old_starving {
            println!("{} {} is now {}!",
                "⚠️".red().bold(),
                name.name.red().bold(),
                "STARVING".red().bold()
            );
            debug.log(
                DebugLevel::Info,
                "NEEDS",
                &format!("{} entered STARVING state", name.name)
            );
        }
        
        if needs.is_exhausted() && !old_exhausted {
            println!("{} {} is now {}!",
                "⚠️".yellow().bold(),
                name.name.yellow().bold(),
                "EXHAUSTED".yellow().bold()
            );
            debug.log(
                DebugLevel::Info,
                "NEEDS",
                &format!("{} entered EXHAUSTED state", name.name)
            );
        }
        
        // Log periodic status (every 10 ticks)
        if sim_state.tick % 10 == 0 {
            debug.log(
                DebugLevel::Debug,
                "NEEDS",
                &format!("{}: {}", name.name, needs.debug_string())
            );
        }
    }
}

/// System to sync the new tick-based needs with GOAP WorldState
pub fn sync_needs_v2_to_worldstate_system(
    sim_state: Res<SimulationState>,
    mut query: Query<(
        &UnitNeedsV2,
        &UnitInventory,
        &UnitLocation,
        &mut crate::ai::WorldState
    ), With<PeasantTag>>,
) {
    // Only sync on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (needs, inventory, location, mut world_state) in query.iter_mut() {
        // Sync needs as floats for GOAP compatibility
        world_state.set("is_hungry", crate::ai::StateValue::Float(needs.hunger() as f64));
        world_state.set("has_energy", crate::ai::StateValue::Float(needs.energy() as f64));
        world_state.set("morale", crate::ai::StateValue::Float(needs.morale() as f64));
        world_state.set("has_shelter", crate::ai::StateValue::Bool(needs.has_shelter));
        
        // Also set boolean flags for easier GOAP conditions
        world_state.set("needs_food", crate::ai::StateValue::Bool(needs.is_hungry()));
        world_state.set("needs_rest", crate::ai::StateValue::Bool(needs.is_tired()));
        world_state.set("critical_hunger", crate::ai::StateValue::Bool(needs.is_starving()));
        world_state.set("critical_energy", crate::ai::StateValue::Bool(needs.is_exhausted()));
        
        // Sync inventory (unchanged)
        let wood = inventory.get_amount(crate::resources::ResourceType::Wood);
        let food = inventory.get_amount(crate::resources::ResourceType::Berries);
        let stone = inventory.get_amount(crate::resources::ResourceType::Stone);
        
        world_state.set("has_wood", crate::ai::StateValue::Int(wood));
        world_state.set("has_food", crate::ai::StateValue::Int(food));
        world_state.set("has_stone", crate::ai::StateValue::Int(stone));
        world_state.set("inventory_full", crate::ai::StateValue::Bool(inventory.is_full()));
        
        // Sync location (unchanged)
        world_state.set("at_storage", crate::ai::StateValue::Bool(location.is_at_storage()));
        world_state.set("at_home", crate::ai::StateValue::Bool(location.is_at_home()));
        world_state.set("at_resource", crate::ai::StateValue::Bool(location.is_at_resource()));
    }
}

/// System to handle eating action (immediate effect, not tick-based)
pub fn eating_action_system(
    sim_state: Res<SimulationState>,
    mut query: Query<(
        &mut UnitNeedsV2,
        &mut UnitInventory,
        &BehaviorState,
        &NameComponent
    ), With<PeasantTag>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (mut needs, mut inventory, behavior, name) in query.iter_mut() {
        if *behavior == BehaviorState::Eating {
            // Check if we have food
            let food_amount = inventory.get_amount(crate::resources::ResourceType::Berries);
            if food_amount > 0 && needs.is_hungry() {
                // Consume one food item
                if inventory.remove_item(crate::resources::ResourceType::Berries, 1) {
                    needs.eat_food(1);
                    
                    println!("{} {} ate food (hunger: {:.0}% → {:.0}%)",
                        "🍖".green(),
                        name.name.cyan(),
                        needs.hunger() * 100.0 + 20.0, // Before (approximate)
                        needs.hunger() * 100.0  // After
                    );
                    
                    debug.log(
                        DebugLevel::Info,
                        "ACTION",
                        &format!("{} consumed 1 food, hunger now at {:.0}%", 
                            name.name, needs.hunger() * 100.0)
                    );
                }
            }
        }
    }
}

/// Performance monitoring for the needs system
pub fn needs_performance_monitor_system(
    sim_state: Res<SimulationState>,
    units: Query<&UnitNeedsV2, With<PeasantTag>>,
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
        &format!("Needs system: {} units, {} updates/sec (tick-based)", 
            unit_count, updates_per_second)
    );
    
    // Compare to old system
    let old_updates_per_second = unit_count * 60; // At 60 FPS
    let improvement = old_updates_per_second as f32 / updates_per_second as f32;
    
    debug.log(
        DebugLevel::Debug,
        "PERFORMANCE",
        &format!("Performance improvement: {:.1}x fewer updates than frame-based", 
            improvement)
    );
}