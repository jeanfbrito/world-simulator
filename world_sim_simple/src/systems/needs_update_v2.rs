use crate::ai::BehaviorStateNew as BehaviorState;
use crate::components::{NameComponent, UnitInventory, UnitLocation, UnitTag, UnitMind};
use crate::SimulationState;
/// Tick-based needs update system
///
/// This system updates unit needs ONLY on simulation ticks, not every frame.
/// It uses the new counter-based UnitNeedsV2 component for better performance
/// with large numbers of units.
use bevy::prelude::*;
use colored::Colorize;


// sync_needs_v2_to_worldstate_system removed - now handled by dogoap state sync

/// System to handle eating action (immediate effect, not tick-based)
pub fn eating_action_system(
    sim_state: Res<SimulationState>,
    mut query: Query<
        (
            &mut crate::components::IsHungry,
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

    for (mut is_hungry, mut inventory, behavior, name) in query.iter_mut() {
        if *behavior == BehaviorState::Eating {
            // Check for berries first (our main food source)
            let berries_amount = inventory.get_amount(crate::resources::ResourceType::Berries);
            if berries_amount > 0 && is_hungry.0 > 0.3 {
                // Consume one berry
                if inventory.remove_item(crate::resources::ResourceType::Berries, 1) {
                    // Reduce hunger (0.0 = not hungry, 1.0 = very hungry)
                    is_hungry.0 = (is_hungry.0 - 0.2).max(0.0);

                    println!(
                        "{} {} ate blueberries (hunger: {:.0}% → {:.0}%)",
                        "🫐".green(),
                        name.name.cyan(),
                        (is_hungry.0 + 0.2) * 100.0, // Before
                        is_hungry.0 * 100.0         // After
                    );

                    debug.log(
                        DebugLevel::Info,
                        "ACTION",
                        &format!(
                            "{} consumed blueberries, hunger now at {:.0}%",
                            name.name,
                            is_hungry.0 * 100.0
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

/// Update GOAP HasEnergy component based on actions and time
pub fn update_goap_energy_system(
    sim_state: Res<SimulationState>,
    mut query: Query<
        (
            &mut crate::components::HasEnergy,
            &crate::components::NameComponent,
        ),
        With<UnitTag>,
    >,
) {
    // Only update on ticks
    if !sim_state.just_ticked {
        return;
    }

    for (mut has_energy, name) in query.iter_mut() {
        // Energy slowly decreases over time (0.001 per tick = 0.01 per second at 10 TPS)
        has_energy.0 -= 0.001;
        has_energy.0 = has_energy.0.max(0.0); // Don't go below 0

        // Log when energy gets low
        if has_energy.0 < 0.1 && has_energy.0 > 0.099 {
            println!(
                "{} {} is getting tired (energy: {:.1}%)",
                "⚡".yellow(),
                name.name.yellow(),
                has_energy.0 * 100.0
            );
        }
    }
}

