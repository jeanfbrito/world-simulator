use bevy::prelude::*;
use colored::Colorize;
use crate::components::*;
use crate::ai::{WorldState, ActionPlan};

/// Simple AI monitor that shows what each peasant is doing
pub fn simple_ai_monitor_system(
    sim_state: Res<crate::SimulationState>,
    peasants: Query<(
        Entity,
        &NameComponent,
        &UnitNeeds,
        &UnitInventory,
        &UnitLocation,
        Option<&ActionPlan>,
    ), With<PeasantTag>>,
    resources: Query<Entity, Or<(With<crate::ai::TreeTag>, With<crate::ai::RockTag>, With<crate::ai::BerryBushTag>)>>,
) {
    if !sim_state.just_ticked {
        return;
    }
    
    // Clear screen for better readability (optional)
    // print!("\x1B[2J\x1B[1;1H");
    
    println!("\n{}", format!("━━━ TICK {} ━━━", sim_state.tick).bright_blue().bold());
    
    // Show resource counts
    let trees = resources.iter().filter(|_| true).count(); // TODO: properly filter by type
    println!("🌍 World Resources: {} trees/rocks/berries available", trees);
    
    // Show each peasant's status
    for (entity, name, needs, inventory, location, plan) in peasants.iter() {
        let status = if plan.is_some() { "📋 Has Plan" } else { "❓ No Plan" };
        let hunger_bar = create_bar(needs.hunger, true);
        let energy_bar = create_bar(needs.energy, false);
        
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
        
        println!("👤 {} ({:?})", name.name.cyan(), entity);
        println!("   {} | Hunger {} | Energy {} | {}", 
            status,
            hunger_bar,
            energy_bar,
            loc
        );
        println!("   Inventory: {}🪵 {}🍖 {}⛏️", 
            wood, food, stone
        );
        
        if let Some(plan) = plan {
            if !plan.actions.is_empty() {
                let current = &plan.actions[plan.current_index];
                println!("   ➡️ Current: {}", current.name.green());
                if plan.actions.len() > 1 {
                    let remaining: Vec<&str> = plan.actions[plan.current_index + 1..]
                        .iter()
                        .map(|a| a.name.as_str())
                        .collect();
                    println!("   📝 Next: {}", remaining.join(" → ").bright_black());
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
    
    let bar = format!("[{}{}]", 
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