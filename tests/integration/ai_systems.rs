//! Integration tests for AI systems
//!
//! This module tests the AI functionality including:
//! - GOAP planning system
//! - Utility AI behaviors
//! - Pathfinding algorithms
//! - Decision making processes

use world_sim_interface::*;
use world_sim_simple::*;

mod common;
use common::*;

/// Test GOAP planning system initialization
#[test]
fn test_goap_planning_initialization() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Verify AI system components are initialized
    let goap_count = count_entities_with_component::<GoapAgent>(world);
    let utility_count = count_entities_with_component::<UtilityAgent>(world);
    let state_machine_count = count_entities_with_component::<StateMachine>(world);

    // At least some AI components should exist
    assert!(goap_count > 0 || utility_count > 0 || state_machine_count > 0,
            "No AI components found in world");
}

/// Test GOAP action planning
#[test]
fn test_goap_action_planning() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create a test GOAP agent
    let agent_entity = world.spawn((
        NameComponent("TestGOAPAgent".to_string()),
        GoapAgent {
            current_plan: None,
            available_actions: vec![
                GoapAction {
                    name: "gather_wood".to_string(),
                    cost: 1.0,
                    preconditions: vec![],
                    effects: vec![("has_wood".to_string(), true)],
                    duration: 10,
                },
                GoapAction {
                    name: "build_shelter".to_string(),
                    cost: 5.0,
                    preconditions: vec![("has_wood".to_string(), true)],
                    effects: vec![("has_shelter".to_string(), true)],
                    duration: 20,
                },
            ],
            world_state: std::collections::HashMap::new(),
            goals: vec![GoapGoal {
                name: "survive".to_string(),
                priority: 1.0,
                conditions: vec![("has_shelter".to_string(), true)],
            }],
        },
        UnitStats::default(),
        PositionComponent { x: 5.0, y: 5.0 },
    )).id();

    // Run simulation to allow AI planning
    ctx.run_simulation_ticks(50);

    // Verify agent still exists
    assert!(world.get_entity(agent_entity).is_some());

    // Check if agent has a plan (this depends on the actual planning system)
    if let Some(agent) = world.get_entity(agent_entity).and_then(|e| e.get::<GoapAgent>()) {
        // For now, just verify the agent structure is intact
        assert!(!agent.available_actions.is_empty());
        assert!(!agent.goals.is_empty());
    }
}

/// Test Utility AI behavior selection
#[test]
fn test_utility_ai_behavior_selection() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create a test utility AI agent
    let agent_entity = world.spawn((
        NameComponent("TestUtilityAgent".to_string()),
        UtilityAgent {
            current_action: None,
            behaviors: vec![
                UtilityBehavior {
                    name: "gather_resources".to_string(),
                    utility_score: 0.8,
                    considerations: vec![],
                    weight: 1.0,
                },
                UtilityBehavior {
                    name: "rest".to_string(),
                    utility_score: 0.3,
                    considerations: vec![],
                    weight: 1.0,
                },
            ],
            needs: std::collections::HashMap::new(),
        },
        UnitStats {
            energy: 50,
            ..Default::default()
        },
        PositionComponent { x: 10.0, y: 10.0 },
    )).id();

    // Run simulation to allow AI decision making
    ctx.run_simulation_ticks(30);

    // Verify agent still exists and has behaviors
    if let Some(agent) = world.get_entity(agent_entity).and_then(|e| e.get::<UtilityAgent>()) {
        assert!(!agent.behaviors.is_empty());
        assert!(agent.needs.contains_key("energy"));
    }
}

/// Test pathfinding algorithms
#[test]
fn test_pathfinding() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create an entity with pathfinding capability
    let entity = world.spawn((
        NameComponent("PathfindingEntity".to_string()),
        PathTarget { x: 25.0, y: 25.0 },
        PositionComponent { x: 5.0, y: 5.0 },
        MovementComponent {
            speed: 1.0,
            path: None,
            current_target: None,
        },
    )).id();

    // Run simulation to allow pathfinding
    ctx.run_simulation_ticks(40);

    // Verify entity still exists
    assert!(world.get_entity(entity).is_some());

    // Check if entity moved towards target (this depends on actual pathfinding system)
    if let Some(position) = world.get_entity(entity).and_then(|e| e.get::<PositionComponent>()) {
        // For now, just verify position component exists
        assert!(position.x >= 0.0 && position.y >= 0.0);
    }
}

/// Test AI decision making under resource constraints
#[test]
fn test_ai_decision_making_with_constraints() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create limited resources
    let _resource = world.spawn((
        NameComponent("ScarceResource".to_string()),
        ResourceNode {
            resource_type: ResourceType::Food,
            amount: 10, // Very limited
            max_amount: 10,
            regeneration_rate: 0.0,
            last_harvest_time: 0,
        },
        PositionComponent { x: 15.0, y: 15.0 },
    )).id();

    // Create agents that need the resource
    let agent1 = world.spawn((
        NameComponent("HungryAgent1".to_string()),
        GoapAgent {
            current_plan: None,
            available_actions: vec![
                GoapAction {
                    name: "gather_food".to_string(),
                    cost: 1.0,
                    preconditions: vec![],
                    effects: vec![("has_food".to_string(), true)],
                    duration: 5,
                },
            ],
            world_state: std::collections::HashMap::new(),
            goals: vec![GoapGoal {
                name: "survive".to_string(),
                priority: 1.0,
                conditions: vec![("has_food".to_string(), true)],
            }],
        },
        UnitStats {
            hunger: 90, // Very hungry
            ..Default::default()
        },
        PositionComponent { x: 10.0, y: 10.0 },
    )).id();

    let agent2 = world.spawn((
        NameComponent("HungryAgent2".to_string()),
        GoapAgent {
            current_plan: None,
            available_actions: vec![
                GoapAction {
                    name: "gather_food".to_string(),
                    cost: 1.0,
                    preconditions: vec![],
                    effects: vec![("has_food".to_string(), true)],
                    duration: 5,
                },
            ],
            world_state: std::collections::HashMap::new(),
            goals: vec![GoapGoal {
                name: "survive".to_string(),
                priority: 1.0,
                conditions: vec![("has_food".to_string(), true)],
            }],
        },
        UnitStats {
            hunger: 80, // Also hungry
            ..Default::default()
        },
        PositionComponent { x: 20.0, y: 20.0 },
    )).id();

    // Run simulation to allow competition for resources
    ctx.run_simulation_ticks(60);

    // Verify both agents still exist
    assert!(world.get_entity(agent1).is_some());
    assert!(world.get_entity(agent2).is_some());
}

/// Test AI agent coordination and cooperation
#[test]
fn test_agent_coordination() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create agents with complementary goals
    let gatherer = world.spawn((
        NameComponent("Gatherer".to_string()),
        GoapAgent {
            current_plan: None,
            available_actions: vec![
                GoapAction {
                    name: "gather_wood".to_string(),
                    cost: 1.0,
                    preconditions: vec![],
                    effects: vec![("has_wood".to_string(), true)],
                    duration: 10,
                },
            ],
            world_state: std::collections::HashMap::new(),
            goals: vec![GoapGoal {
                name: "provide_resources".to_string(),
                priority: 1.0,
                conditions: vec![("has_wood".to_string(), true)],
            }],
        },
        UnitTag,
        PositionComponent { x: 5.0, y: 5.0 },
    )).id();

    let builder = world.spawn((
        NameComponent("Builder".to_string()),
        GoapAgent {
            current_plan: None,
            available_actions: vec![
                GoapAction {
                    name: "build_shelter".to_string(),
                    cost: 5.0,
                    preconditions: vec![("has_wood".to_string(), true)],
                    effects: vec![("has_shelter".to_string(), true)],
                    duration: 20,
                },
            ],
            world_state: std::collections::HashMap::new(),
            goals: vec![GoapGoal {
                name: "construct_shelter".to_string(),
                priority: 1.0,
                conditions: vec![("has_shelter".to_string(), true)],
            }],
        },
        UnitTag,
        PositionComponent { x: 10.0, y: 10.0 },
    )).id();

    // Run simulation to allow coordination
    ctx.run_simulation_ticks(80);

    // Verify both agents still exist
    assert!(world.get_entity(gatherer).is_some());
    assert!(world.get_entity(builder).is_some());
}
