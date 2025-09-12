use bevy::prelude::*;
use crate::components::{
    GridPosition, UnitNeedsV2, UnitInventory, NameComponent, UnitTag, UnitMind,
    resource::ResourceNode,
};
use crate::resources::{ItemType, ResourceType};
use crate::{SimulationState, ai::BerryBushTag};
use crate::debug::{DebugSystem, DebugLevel};
use colored::Colorize;

/// System that handles food gathering when units are at berry bushes
pub fn food_gathering_system(
    sim_state: Res<SimulationState>,
    mut units: Query<(
        Entity,
        &GridPosition,
        &mut UnitNeedsV2,
        &mut UnitInventory,
        &mut UnitMind,
        &NameComponent,
    ), With<UnitTag>>,
    mut berry_bushes: Query<(
        Entity,
        &GridPosition,
        &mut ResourceNode,
    ), With<BerryBushTag>>,
    debug: Res<DebugSystem>,
) {
    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }
    
    for (unit_entity, unit_pos, mut needs, mut inventory, mut mind, name) in units.iter_mut() {
        // Check if unit is at a berry bush
        for (bush_entity, bush_pos, mut resource) in berry_bushes.iter_mut() {
            // Check if positions match (at same tile)
            if unit_pos.x == bush_pos.x && unit_pos.y == bush_pos.y {
                // Check if bush has berries and unit needs food
                if resource.amount > 0 && needs.is_hungry() {
                    // Gather berries!
                    let berries_to_gather = resource.amount.min(3);
                    resource.amount -= berries_to_gather;
                    
                    // Add berries to inventory (simplified for now)
                    // Just track that we gathered
                    
                    // Update mind state
                    *mind = UnitMind::Gathering {
                        resource: "berries".to_string(),
                    };
                    
                    debug.log(
                        DebugLevel::Info,
                        "GATHERING",
                        &format!(
                            "{} gathered {} berries at ({},{})",
                            name.name, berries_to_gather, bush_pos.x, bush_pos.y
                        ),
                    );
                    
                    println!(
                        "{} {} gathered {} berries!",
                        "🫐".purple(),
                        name.name.green(),
                        berries_to_gather
                    );
                }
                
                // Now eat if hungry
                if needs.is_hungry() {
                    // Eat the berries we just gathered (1 food item)
                    needs.eat_food(1);
                    
                    *mind = UnitMind::Eating;
                    
                    debug.log(
                        DebugLevel::Info,
                        "EATING",
                        &format!(
                            "{} ate berries, hunger now at {}%",
                            name.name, ((1.0 - needs.hunger()) * 100.0) as i32
                        ),
                    );
                    
                    println!(
                        "{} {} ate some berries! Hunger: {}%",
                        "🍽️".yellow(),
                        name.name.green(),
                        ((1.0 - needs.hunger()) * 100.0) as i32
                    );
                }
            }
        }
    }
}