use crate::ai::ActionPlan;
use crate::components::*;
use crate::TileEntity;
use bevy::prelude::*;
use colored::Colorize;
use std::collections::HashMap;

// Store previous positions to detect movement
static mut PREV_POSITIONS: Option<HashMap<Entity, (usize, usize)>> = None;

/// Simple AI monitor that shows what each peasant is doing
pub fn simple_ai_monitor_system(
    sim_state: Res<crate::SimulationState>,
    peasants: Query<
        (
            Entity,
            &NameComponent,
            &UnitNeedsV2,  // Changed from UnitNeeds to UnitNeedsV2
            &UnitInventory,
            &UnitLocation,
            &TileEntity,
            &PositionComponent,
            &TilesWalked,
            Option<&ActionPlan>,
        ),
        With<UnitTag>,
    >,
    trees: Query<Entity, With<crate::ai::TreeTag>>,
    berries: Query<(Entity, &ResourceNode), With<crate::ai::BerryBushTag>>,
) {
    if !sim_state.just_ticked {
        return;
    }

    // Initialize previous positions if needed
    unsafe {
        if PREV_POSITIONS.is_none() {
            PREV_POSITIONS = Some(HashMap::new());
        }
    }

    // Clear screen for better readability (optional)
    // print!("\x1B[2J\x1B[1;1H");

    println!(
        "\n{}",
        format!("━━━ TICK {} ━━━", sim_state.tick)
            .bright_blue()
            .bold()
    );

    // Show resource counts by type
    let tree_count = trees.iter().count();
    let mut full_berry_count = 0;
    let mut empty_berry_count = 0;
    
    for (_entity, resource_node) in berries.iter() {
        if resource_node.amount > 0 {
            full_berry_count += 1;
        } else {
            empty_berry_count += 1;
        }
    }
    
    println!("🌍 World Resources:");
    println!("   🌲 {} trees available", tree_count);
    println!("   🫐 {} berry bushes with fruit", full_berry_count);
    println!("   🌳 {} depleted berry bushes", empty_berry_count);

    // Show each peasant's status
    for (entity, name, needs, inventory, location, tile, position, tiles_walked, plan) in
        peasants.iter()
    {
        // Check if peasant moved
        let current_pos = (tile.x, tile.y);
        let moved = unsafe {
            let prev_map = PREV_POSITIONS.as_mut().unwrap();
            let prev_pos = prev_map.get(&entity).copied();
            prev_map.insert(entity, current_pos);

            if let Some(prev) = prev_pos {
                prev != current_pos
            } else {
                false
            }
        };

        let movement_indicator = if moved { "🚶" } else { "🧍" };
        let status = if plan.is_some() {
            "📋 Has Plan"
        } else {
            "❓ No Plan"
        };
        let hunger_bar = create_bar(needs.hunger(), true);  // Use method instead of field
        let energy_bar = create_bar(needs.energy(), false);  // Use method instead of field

        // Get inventory summary
        let wood = inventory.get_amount(crate::resources::ResourceType::Wood);
        let food = inventory.get_amount(crate::resources::ResourceType::Berries);
        let stone = inventory.get_amount(crate::resources::ResourceType::Stone);

        // Get location
        let loc = match location.location_type {
            LocationType::Wilderness => "🏞️ Wilderness",
            LocationType::Storage => "📦 Storage",
            LocationType::Home => "🏠 Home",
            LocationType::Workshop => "🔨 Workshop",
            LocationType::Farm => "🌾 Farm",
            LocationType::Resource(_) => "🌲 Resource",
        };

        println!(
            "👤 {} {} @ ({},{}) {} | {}",
            name.name.cyan(),
            movement_indicator,
            tile.x,
            tile.y,
            format!("({:?})", entity).bright_black(),
            tiles_walked.display().bright_magenta()
        );
        println!(
            "   {} | Hunger {} ({:.2}) | Energy {} ({:.2})",
            status, hunger_bar, needs.hunger(), energy_bar, needs.energy()
        );
        println!(
            "   📍 {} | Inventory: {}🪵 {}🍖 {}⛏️ (weight: {:.1}/{:.1})",
            loc, wood, food, stone, inventory.current_weight, inventory.max_weight
        );

        if let Some(plan) = plan {
            if !plan.actions.is_empty() && plan.current_index < plan.actions.len() {
                let current = &plan.actions[plan.current_index];
                println!("   ➡️ Current: {}", current.name.green());
                if plan.current_index + 1 < plan.actions.len() {
                    let remaining: Vec<&str> = plan.actions[plan.current_index + 1..]
                        .iter()
                        .map(|a| a.name.as_str())
                        .collect();
                    println!("   📝 Next: {}", remaining.join(" → ").bright_black());
                }
            } else if plan.is_complete() {
                // Don't show "Plan completed" if there's still a current action
                // This happens when action doesn't auto-advance (like gather_food)
                if plan.current_action().is_none() {
                    println!("   ✅ {}", "Plan completed".green());
                }
            }
        } else {
            println!("   ⚠️ {}", "Trying to create plan...".yellow());
        }
    }

    println!("{}", "━".repeat(50).bright_black());
}

fn create_bar(value: f32, reverse: bool) -> String {
    let width = 10;
    let filled = (value * width as f32) as usize;
    let empty = width - filled;

    let (filled_char, empty_char, color) = if reverse {
        // For hunger: more filled = bad
        if value > 0.7 {
            ("●", "○", "red")
        } else if value > 0.3 {
            ("●", "○", "yellow")
        } else {
            ("●", "○", "green")
        }
    } else {
        // For energy: more filled = good
        if value > 0.7 {
            ("●", "○", "green")
        } else if value > 0.3 {
            ("●", "○", "yellow")
        } else {
            ("●", "○", "red")
        }
    };

    let bar = format!(
        "[{}{}]",
        filled_char.repeat(filled),
        empty_char.repeat(empty)
    );

    match color {
        "red" => bar.red().to_string(),
        "yellow" => bar.yellow().to_string(),
        "green" => bar.green().to_string(),
        _ => bar,
    }
}
