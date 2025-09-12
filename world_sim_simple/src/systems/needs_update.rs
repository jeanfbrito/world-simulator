use crate::components::{UnitInventory, UnitLocation, UnitNeeds, UnitTag};
use crate::SimulationState;
use bevy::prelude::*;
use colored::Colorize;

/// System that updates unit needs over time (hunger increases, energy decreases)
pub fn update_unit_needs_system(
    time: Res<Time>,
    sim_state: Res<SimulationState>,
    mut units: Query<(Entity, &mut UnitNeeds, &crate::components::NameComponent), With<UnitTag>>,
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
            println!(
                "{} {} is getting hungrier: {:.2} → {:.2}",
                "🍞".red(),
                name.name.yellow(),
                old_hunger,
                needs.hunger
            );
        }

        if (old_energy - needs.energy).abs() > 0.01 {
            println!(
                "{} {} is getting tired: {:.2} → {:.2}",
                "😴".blue(),
                name.name.yellow(),
                old_energy,
                needs.energy
            );
        }

        // Check critical states
        if needs.hunger > 0.8 && old_hunger <= 0.8 {
            println!(
                "{} {} is STARVING!",
                "⚠️".red().bold(),
                name.name.red().bold()
            );
        }

        if needs.energy < 0.2 && old_energy >= 0.2 {
            println!(
                "{} {} is EXHAUSTED!",
                "⚠️".yellow().bold(),
                name.name.yellow().bold()
            );
        }
    }
}

// sync_needs_to_worldstate_system removed - now handled by dogoap state sync
