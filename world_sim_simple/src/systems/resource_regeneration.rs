use crate::components::{
    GridPosition, GrowingResource, NameComponent, ResourceDepletedEvent, ResourceNode, 
    ResourceRegeneratedEvent, ResourceRegenerationTag,
};
use crate::SimulationState;
/// Resource regeneration system for tick-based resource respawning
///
/// Handles gradual regeneration and full respawn of depleted resources
/// using tick-based counters for deterministic behavior.
use bevy::prelude::*;
use colored::Colorize;

/// Main resource regeneration system that runs on ticks
pub fn resource_regeneration_system(
    sim_state: Res<SimulationState>,
    mut resources: Query<
        (
            Entity,
            &mut ResourceNode,
            &GridPosition,
            Option<&NameComponent>,
        ),
        With<ResourceRegenerationTag>,
    >,
    mut regen_events: EventWriter<ResourceRegeneratedEvent>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only process on ticks
    if !sim_state.just_ticked {
        return;
    }

    for (entity, mut resource, position, name) in resources.iter_mut() {
        let old_amount = resource.amount;

        // Update regeneration
        resource.tick_update();

        // Check if regeneration occurred
        if resource.amount > old_amount {
            let is_full_respawn = old_amount == 0 && resource.amount == resource.max_amount;

            // Send event
            regen_events.send(ResourceRegeneratedEvent {
                entity,
                resource_type: resource.resource_type,
                old_amount,
                new_amount: resource.amount,
                is_full_respawn,
            });

            // Log significant regeneration
            if is_full_respawn {
                let resource_name = name.map(|n| n.name.as_str()).unwrap_or("Resource");

                println!(
                    "{} {} fully respawned at ({}, {}) - {} available",
                    "🌱".green(),
                    resource_name.cyan(),
                    position.x,
                    position.y,
                    resource.amount
                );

                debug.log(
                    DebugLevel::Info,
                    "REGEN",
                    &format!(
                        "{} fully respawned at ({}, {})",
                        resource_name, position.x, position.y
                    ),
                );
            } else {
                // Log ALL regeneration for berries to see it working
                let resource_name = name.map(|n| n.name.as_str()).unwrap_or("Resource");

                // Special logging for berry bushes
                if resource_name.contains("Berry") {
                    println!(
                        "{} {} regenerated at ({}, {}): {} -> {} berries",
                        "🫐".green(),
                        resource_name.cyan(),
                        position.x,
                        position.y,
                        old_amount,
                        resource.amount
                    );
                } else if sim_state.tick % 100 == 0 {
                    // Log other resources periodically
                    debug.log(
                        DebugLevel::Debug,
                        "REGEN",
                        &format!(
                            "Resource at ({}, {}) regenerated: {} -> {}",
                            position.x, position.y, old_amount, resource.amount
                        ),
                    );
                }
            }
        }
    }
}

/// System to handle resource harvesting and depletion
pub fn resource_harvest_system(
    sim_state: Res<SimulationState>,
    resources: Query<(
        Entity,
        &mut ResourceNode,
        &GridPosition,
        Option<&NameComponent>,
    )>,
    mut depletion_events: EventWriter<ResourceDepletedEvent>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // This would be triggered by work system completing gathering
    // For now, just check for depleted resources

    if !sim_state.just_ticked {
        return;
    }

    for (entity, resource, position, name) in resources.iter() {
        // Only trigger depletion when resource goes from >0 to 0 (actual depletion)
        // ticks_since_depletion == 1 means it was just depleted last tick
        // We need to track if it had resources before
        if resource.amount == 0 && resource.ticks_since_depletion == 1 && resource.max_amount > 0 {
            // This resource was actually harvested to depletion
            let resource_name = name.map(|n| n.name.as_str()).unwrap_or("Resource");

            // Only show depletion message for resources that can respawn
            if resource.respawn_time_ticks > 0 {
                println!(
                    "{} {} depleted at ({}, {}) - respawning in {} seconds",
                    "⚠️".yellow(),
                    resource_name.yellow(),
                    position.x,
                    position.y,
                    resource.respawn_time_ticks / 10
                );

                depletion_events.send(ResourceDepletedEvent {
                    entity,
                    resource_type: resource.resource_type,
                    harvester: None, // Would be set by harvesting system
                });

                debug.log(
                    DebugLevel::Info,
                    "DEPLETION",
                    &format!(
                        "{} depleted at ({}, {})",
                        resource_name, position.x, position.y
                    ),
                );
            }
        }
    }
}

/// System to spawn initial resources with regeneration
pub fn spawn_regenerating_resources_system(
    mut commands: Commands,
    sim_state: Res<SimulationState>,
    existing: Query<Entity, With<ResourceNode>>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;
    use crate::TileEntity;

    // Only spawn once at startup
    if sim_state.tick != 2 {
        return;
    }

    // Count existing resources (trees from tree_generation.rs)
    let existing_count = existing.iter().count();

    debug.log(
        DebugLevel::Info,
        "RESOURCES",
        &format!(
            "Found {} existing resource nodes, adding berry bushes",
            existing_count
        ),
    );

    // Add more berry bushes around the map
    let berry_positions = vec![
        (30, 30),
        (35, 30),
        (40, 30), // North cluster
        (30, 35),
        (35, 35),
        (40, 35), // Center cluster
        (30, 40),
        (35, 40),
        (40, 40), // South cluster
        (25, 32),
        (45, 32), // East/West singles
        (28, 28),
        (42, 42), // Corners
        (32, 25),
        (32, 45), // North/South singles
    ];

    let berry_count = berry_positions.len();
    for (x, y) in berry_positions {
        // Create ResourceNode for compatibility
        let berry_node = ResourceNode::fruit_bush(1); // Start with only 1 berry
        
        // Create GrowingResource to handle all regeneration
        let growing = GrowingResource::fruit_bush(1, 3); // Start with 1, max 3 berries

        commands.spawn((
            NameComponent::new(format!("Berry Bush ({}, {})", x, y)),
            GridPosition { x, y },
            TileEntity {
                x: x as usize,
                y: y as usize,
            },
            berry_node,
            growing,
            ResourceRegenerationTag,
        ));
    }

    println!(
        "{} Spawned {} empty berry bushes (will regenerate over time)",
        "🫐".green(),
        berry_count
    );

    // Add some stone deposits
    let stone_positions = vec![(20, 20), (44, 20), (20, 44), (44, 44), (32, 20), (32, 44)];

    let stone_count = stone_positions.len();
    for (x, y) in stone_positions {
        let mut stone_node = ResourceNode::new(crate::resources::ResourceType::Stone, 50);
        stone_node.regeneration_rate = 2;
        stone_node.regeneration_interval = 200; // Slower regen for stone
        stone_node.respawn_time_ticks = 600; // 1 minute respawn
        stone_node.yield_amount = 8;

        commands.spawn((
            NameComponent::new(format!("Stone Deposit ({}, {})", x, y)),
            GridPosition { x, y },
            TileEntity {
                x: x as usize,
                y: y as usize,
            },
            stone_node,
            ResourceRegenerationTag,
        ));
    }

    println!(
        "{} Spawned {} regenerating stone deposits",
        "⛰️".white(),
        stone_count
    );

    debug.log(
        DebugLevel::Info,
        "RESOURCES",
        &format!(
            "Spawned {} berry bushes and {} stone deposits",
            berry_count, stone_count
        ),
    );
}

/// System to update existing trees with regeneration
pub fn add_regeneration_to_trees_system(
    mut commands: Commands,
    trees: Query<Entity, (With<ResourceNode>, Without<ResourceRegenerationTag>)>,
    sim_state: Res<SimulationState>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Only run once early in startup
    if sim_state.tick != 3 {
        return;
    }

    let mut count = 0;
    for entity in trees.iter() {
        commands.entity(entity).insert(ResourceRegenerationTag);
        count += 1;
    }

    if count > 0 {
        println!(
            "{} Added regeneration to {} existing trees",
            "🌲".green(),
            count
        );

        debug.log(
            DebugLevel::Info,
            "RESOURCES",
            &format!("Added regeneration tags to {} trees", count),
        );
    }
}

/// Display resource status periodically
pub fn resource_status_display_system(
    sim_state: Res<SimulationState>,
    resources: Query<(&ResourceNode, &GridPosition, Option<&NameComponent>)>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Display every 200 ticks (20 seconds)
    if !sim_state.just_ticked || sim_state.tick % 200 != 0 {
        return;
    }

    let total = resources.iter().count();
    let depleted = resources.iter().filter(|(r, _, _)| r.amount == 0).count();
    let regenerating = resources
        .iter()
        .filter(|(r, _, _)| r.amount > 0 && r.amount < r.max_amount)
        .count();
    let full = resources
        .iter()
        .filter(|(r, _, _)| r.amount == r.max_amount)
        .count();

    // Group by resource type
    let mut by_type: std::collections::HashMap<crate::resources::ResourceType, (u32, u32)> =
        std::collections::HashMap::new();

    for (resource, _, _) in resources.iter() {
        let entry = by_type.entry(resource.resource_type).or_insert((0, 0));
        entry.0 += resource.amount;
        entry.1 += resource.max_amount;
    }

    println!("\n{} Resource Status:", "📊".blue());
    println!(
        "  Total nodes: {} (Full: {}, Regenerating: {}, Depleted: {})",
        total.to_string().white(),
        full.to_string().green(),
        regenerating.to_string().yellow(),
        depleted.to_string().red()
    );

    for (resource_type, (current, max)) in by_type.iter() {
        let percentage = (*current as f32 / *max as f32 * 100.0) as u32;
        println!(
            "  {:?}: {}/{} ({}%)",
            resource_type,
            current.to_string().cyan(),
            max,
            percentage
        );
    }

    debug.log(
        DebugLevel::Info,
        "RESOURCES",
        &format!(
            "Resources: {} total, {} full, {} regenerating, {} depleted",
            total, full, regenerating, depleted
        ),
    );
}

/// Performance monitoring for resource systems
pub fn resource_performance_monitor_system(
    sim_state: Res<SimulationState>,
    resources: Query<&ResourceNode>,
    debug: Res<crate::debug::DebugSystem>,
) {
    use crate::debug::DebugLevel;

    // Monitor every 500 ticks
    if !sim_state.just_ticked || sim_state.tick % 500 != 0 {
        return;
    }

    let total = resources.iter().count();
    let total_amount: u32 = resources.iter().map(|r| r.amount).sum();
    let total_capacity: u32 = resources.iter().map(|r| r.max_amount).sum();

    let utilization = if total_capacity > 0 {
        total_amount as f32 / total_capacity as f32 * 100.0
    } else {
        0.0
    };

    debug.log(
        DebugLevel::Debug,
        "PERFORMANCE",
        &format!(
            "Resources: {} nodes, {}/{} available ({:.0}% utilization)",
            total, total_amount, total_capacity, utilization
        ),
    );
}
