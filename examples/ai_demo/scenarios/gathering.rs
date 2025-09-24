//! Resource Gathering AI Scenario
//!
//! This scenario demonstrates AI agents gathering resources efficiently,
//! dealing with competition, and adapting to changing conditions.

use bevy::prelude::*;
use world_sim_interface::*;
use world_sim_simple::*;

/// Setup a resource gathering scenario with multiple AI agents
pub fn setup_gathering_scenario(
    mut commands: Commands,
    mut pack_system: Option<Res<packs::PackSystem>>,
) {
    println!("🌾 Setting up Resource Gathering Scenario...");

    // Create abundant resources
    let resource_positions = vec![
        (8, 8), (24, 8), (8, 24), (24, 24),  // Corners
        (16, 8), (8, 16), (24, 16), (16, 24),  // Edges
        (12, 12), (20, 12), (12, 20), (20, 20), // Inner ring
    ];

    for (x, y) in resource_positions {
        commands.spawn((
            NameComponent("GatheringResource".to_string()),
            ResourceNode {
                resource_type: ResourceType::Wood,
                amount: 150,
                max_amount: 150,
                regeneration_rate: 0.02,
                last_harvest_time: 0,
            },
            PositionComponent { x: x as f32, y: y as f32 },
        ));
    }

    // Create specialized gathering agents
    create_specialized_gatherers(&mut commands);

    println!("✅ Resource Gathering Scenario ready!");
    println!("🎯 Agents will demonstrate:");
    println!("   • Efficient resource gathering");
    println!("   • Competition and cooperation");
    println!("   • Adaptive behavior based on availability");
}

/// Create specialized gathering agents with different strategies
fn create_specialized_gatherers(commands: &mut Commands) {
    // Efficient gatherers - focus on closest resources
    for i in 0..2 {
        commands.spawn((
            NameComponent(format!("EfficientGatherer_{}", i)),
            UnitTag,
            UnitStats {
                energy: 100,
                max_energy: 100,
                movement_speed: 1.2, // Faster movement
                ..Default::default()
            },
            PositionComponent {
                x: 10.0 + i as f32 * 2.0,
                y: 10.0 + i as f32 * 2.0,
            },
            GoapAgent {
                current_plan: None,
                available_actions: vec![
                    GoapAction {
                        name: "gather_wood_efficient".to_string(),
                        cost: 0.8, // Lower cost for efficiency
                        preconditions: vec![],
                        effects: vec![("has_wood".to_string(), true)],
                        duration: 8, // Faster gathering
                    },
                ],
                world_state: std::collections::HashMap::new(),
                goals: vec![GoapGoal {
                    name: "efficient_gathering".to_string(),
                    priority: 1.0,
                    conditions: vec![("has_wood".to_string(), true)],
                }],
            },
            Inventory::new(),
        ));
    }

    // Persistent gatherers - work longer but slower
    for i in 0..2 {
        commands.spawn((
            NameComponent(format!("PersistentGatherer_{}", i)),
            UnitTag,
            UnitStats {
                energy: 150, // More energy
                max_energy: 150,
                movement_speed: 0.8, // Slower but steady
                ..Default::default()
            },
            PositionComponent {
                x: 20.0 + i as f32 * 2.0,
                y: 20.0 + i as f32 * 2.0,
            },
            UtilityAgent {
                current_action: None,
                behaviors: vec![
                    UtilityBehavior {
                        name: "persistent_gathering".to_string(),
                        utility_score: 0.9,
                        considerations: vec![],
                        weight: 1.0,
                    },
                ],
                needs: std::collections::HashMap::from([
                    ("energy".to_string(), 0.8), // High energy need
                    ("wood".to_string(), 0.7),   // High resource need
                ]),
            },
            Inventory::new(),
        ));
    }
}

/// Monitor the gathering scenario and provide insights
pub fn monitor_gathering_scenario(
    sim_state: Res<SimulationState>,
    resource_query: Query<&ResourceNode>,
    unit_query: Query<(&NameComponent, &Inventory, Option<&GoapAgent>, Option<&UtilityAgent>)>,
) {
    if sim_state.tick % 150 == 0 && sim_state.tick > 0 {
        let total_resources = resource_query.iter().map(|r| r.amount).sum::<u32>();
        let max_resources = resource_query.iter().map(|r| r.max_amount).sum::<u32>();
        let resource_percentage = (total_resources as f32 / max_resources as f32) * 100.0;

        println!("🌾 Gathering Analysis (tick {}): {:.1}% resources remaining",
            sim_state.tick, resource_percentage);

        // Analyze different gatherer types
        let mut efficient_wood = 0;
        let mut persistent_wood = 0;

        for (name, inventory, goap_agent, utility_agent) in unit_query.iter() {
            let wood_amount = inventory.get_item_amount("wood");

            if name.0.starts_with("EfficientGatherer") {
                efficient_wood += wood_amount;
            } else if name.0.starts_with("PersistentGatherer") {
                persistent_wood += wood_amount;
            }
        }

        println!("   • Efficient gatherers: {} wood", efficient_wood);
        println!("   • Persistent gatherers: {} wood", persistent_wood);

        if efficient_wood > 0 || persistent_wood > 0 {
            let efficiency_ratio = efficient_wood as f32 / (efficient_wood + persistent_wood) as f32;
            println!("   • Efficiency ratio: {:.1}%", efficiency_ratio * 100.0);
        }
    }
}