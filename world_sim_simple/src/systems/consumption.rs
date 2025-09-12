use bevy::prelude::*;
use crate::components::{
    UnitNeedsV2, UnitInventory, NameComponent, UnitTag, UnitMind,
};
use crate::resources::ResourceType;
use crate::SimulationState;
use crate::debug::{DebugSystem, DebugLevel};
use colored::Colorize;

/// System that handles consumption of resources from inventory
/// This includes eating food when hungry, but could be extended for other consumption
/// This is triggered by AI decisions, not by being at a resource location
pub fn consumption_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity,
        &mut UnitNeedsV2,
        &mut UnitInventory,
        &mut UnitMind,
        &NameComponent,
    ), With<UnitTag>>,
    debug: Res<DebugSystem>,
) {
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    // Check every 10 ticks (1 second at 10 TPS)
    if sim_state.tick % 10 != 0 {
        return;
    }
    
    for (entity, mut needs, mut inventory, mut mind, name) in units.iter_mut() {
        // Check if unit is hungry and has food in inventory
        if needs.is_hungry() {
            // Check for berries in inventory
            let berry_count = inventory.get_amount(ResourceType::Berries);
            
            if berry_count > 0 {
                // Eat one unit of berries
                if inventory.remove_item(ResourceType::Berries, 1) {
                    needs.eat_food(1);
                    *mind = UnitMind::Eating;
                    
                    debug.log(
                        DebugLevel::Info,
                        "CONSUMPTION",
                        &format!(
                            "{} ate 1 berry from inventory ({}→{}), hunger now {}%",
                            name.name,
                            berry_count,
                            berry_count - 1,
                            ((1.0 - needs.hunger()) * 100.0) as i32
                        ),
                    );
                    
                    println!(
                        "{} {} ate berries from inventory! Hunger: {}%, Berries left: {}",
                        "🍽️".yellow(),
                        name.name.green(),
                        ((1.0 - needs.hunger()) * 100.0) as i32,
                        berry_count - 1
                    );
                    
                    // After eating, go back to idle (AI will decide next action)
                    // Don't set to idle immediately - let it show eating for one tick
                }
            } else {
                // Hungry but no food in inventory
                if sim_state.tick % 50 == 0 {  // Log less frequently
                    debug.log(
                        DebugLevel::Debug,
                        "HUNGRY_NO_FOOD",
                        &format!(
                            "{} is hungry ({}% full) but has no food in inventory",
                            name.name,
                            ((1.0 - needs.hunger()) * 100.0) as i32
                        ),
                    );
                }
            }
        }
        
        // Reset mind state if it was eating
        if matches!(*mind, UnitMind::Eating) {
            *mind = UnitMind::Idle;
        }
    }
}

/// System to automatically trigger gathering when units are hungry and have no food
/// This is a temporary system until Big Brain AI is properly integrated
pub fn hunger_response_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity,
        &UnitNeedsV2,
        &UnitInventory,
        &mut UnitMind,
        &NameComponent,
    ), With<UnitTag>>,
    debug: Res<DebugSystem>,
) {
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    // Check every 20 ticks (2 seconds at 10 TPS)
    if sim_state.tick % 20 != 0 {
        return;
    }
    
    for (entity, needs, inventory, mut mind, name) in units.iter_mut() {
        // If hungry and no food in inventory, signal need to gather
        if needs.is_hungry() && inventory.get_amount(ResourceType::Berries) == 0 {
            // Only update if currently idle
            if matches!(*mind, UnitMind::Idle) {
                *mind = UnitMind::SearchingForFood;
                
                debug.log(
                    DebugLevel::Info,
                    "HUNGER_TRIGGER",
                    &format!(
                        "{} needs food (hunger: {}%)",
                        name.name,
                        (needs.hunger() * 100.0) as i32
                    ),
                );
            }
        }
    }
}