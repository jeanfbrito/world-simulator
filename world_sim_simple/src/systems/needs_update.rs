use bevy::prelude::*;
use crate::components::{UnitNeeds, UnitInventory, UnitLocation, PeasantTag};
use crate::SimulationState;
use colored::Colorize;

/// System that updates unit needs over time (hunger increases, energy decreases)
pub fn update_unit_needs_system(
    time: Res<Time>,
    sim_state: Res<SimulationState>,
    mut units: Query<(Entity, &mut UnitNeeds, &crate::components::NameComponent), With<PeasantTag>>,
) {
    // Only update on simulation ticks for consistency
    if !sim_state.running {
        return;
    }
    
    let delta = time.delta().as_secs_f32();
    
    for (entity, mut needs, name) in units.iter_mut() {
        let old_hunger = needs.hunger;
        let old_energy = needs.energy;
        
        // Update needs based on time
        needs.update(delta);
        
        // Log significant changes
        if (needs.hunger - old_hunger).abs() > 0.01 {
            println!("{} {} is getting hungrier: {:.2} → {:.2}",
                "🍞".red(),
                name.name.yellow(),
                old_hunger,
                needs.hunger
            );
        }
        
        if (old_energy - needs.energy).abs() > 0.01 {
            println!("{} {} is getting tired: {:.2} → {:.2}",
                "😴".blue(),
                name.name.yellow(),
                old_energy,
                needs.energy
            );
        }
        
        // Check critical states
        if needs.hunger > 0.8 && old_hunger <= 0.8 {
            println!("{} {} is STARVING!",
                "⚠️".red().bold(),
                name.name.red().bold()
            );
        }
        
        if needs.energy < 0.2 && old_energy >= 0.2 {
            println!("{} {} is EXHAUSTED!",
                "⚠️".yellow().bold(),
                name.name.yellow().bold()
            );
        }
    }
}

/// System to sync UnitNeeds with the GOAP WorldState
pub fn sync_needs_to_worldstate_system(
    mut query: Query<(&UnitNeeds, &UnitInventory, &UnitLocation, &mut crate::ai::WorldState), With<PeasantTag>>,
) {
    for (needs, inventory, location, mut world_state) in query.iter_mut() {
        // Sync needs
        world_state.set("is_hungry", crate::ai::StateValue::Float(needs.hunger as f64));
        world_state.set("has_energy", crate::ai::StateValue::Float(needs.energy as f64));
        world_state.set("morale", crate::ai::StateValue::Float(needs.morale as f64));
        world_state.set("has_shelter", crate::ai::StateValue::Bool(needs.shelter));
        
        // Sync inventory
        let wood = inventory.get_amount(crate::resources::ResourceType::Wood);
        let food = inventory.get_amount(crate::resources::ResourceType::Berries);
        let stone = inventory.get_amount(crate::resources::ResourceType::Stone);
        
        world_state.set("has_wood", crate::ai::StateValue::Int(wood));
        world_state.set("has_food", crate::ai::StateValue::Int(food));
        world_state.set("has_stone", crate::ai::StateValue::Int(stone));
        world_state.set("inventory_full", crate::ai::StateValue::Bool(inventory.is_full()));
        
        // Sync location
        world_state.set("at_storage", crate::ai::StateValue::Bool(location.is_at_storage()));
        world_state.set("at_home", crate::ai::StateValue::Bool(location.is_at_home()));
        world_state.set("at_resource", crate::ai::StateValue::Bool(location.is_at_resource()));
    }
}