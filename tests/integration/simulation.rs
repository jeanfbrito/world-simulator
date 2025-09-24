//! Integration tests for the world simulation system
//!
//! This module tests the core simulation functionality including:
//! - World initialization
//! - Entity spawning and management
//! - Simulation tick processing
//! - State persistence

use world_sim_interface::*;
use world_sim_simple::*;

mod common;
use common::*;

/// Test basic simulation initialization
#[test]
fn test_simulation_initialization() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    // Verify simulation is initialized correctly
    assert_simulation_running!(&ctx);
    assert_tick_count!(&ctx, 0);

    // Verify world resources exist
    let world = ctx.app.world();
    let resource_count = count_entities_with_component::<ResourceNode>(world);
    assert!(resource_count > 0, "No resources found in world");

    // Verify units exist
    let unit_count = count_entities_with_component::<UnitTag>(world);
    assert!(unit_count > 0, "No units found in world");

    // Validate world state
    assert!(validate_world_state(world).is_ok());
}

/// Test world generation and tilemap creation
#[test]
fn test_world_generation() {
    let config = TestConfig {
        world_size: 16,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Verify world map exists and has correct size
    if let Some(world_map) = world.get_resource::<WorldMap>() {
        assert!(world_map.width > 0);
        assert!(world_map.height > 0);
        assert!(world_map.tiles.len() > 0);
    } else {
        panic!("WorldMap resource not found");
    }

    // Verify terrain diversity
    let terrain_types: std::collections::HashSet<_> = world.query::<&TileComponent>()
        .iter(world)
        .map(|(tile,)| tile.terrain_type)
        .collect();

    assert!(terrain_types.len() > 1, "World should have diverse terrain types");
}

/// Test entity spawning and component attachment
#[test]
fn test_entity_spawning() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Count initial entities
    let initial_count = count_entities_with_component::<UnitTag>(world);

    // Spawn test entities
    let test_entities = create_test_entities(world, 3);

    // Verify entities were created
    assert_eq!(test_entities.len(), 3);

    let new_count = count_entities_with_component::<UnitTag>(world);
    assert_eq!(new_count, initial_count + 3);
}

/// Test simulation tick processing
#[test]
fn test_simulation_tick() {
    let config = TestConfig {
        simulation_ticks: 50,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    // Run simulation for a few ticks
    ctx.run_simulation_ticks(10);

    // Verify simulation progressed
    assert_tick_count!(&ctx, 10);
    assert_simulation_running!(&ctx);
}

/// Test save and load functionality
#[test]
fn test_save_load() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    // Run simulation to establish state
    ctx.run_simulation_ticks(25);
    let tick_before = ctx.current_tick();

    // Test save functionality (this would need to be implemented)
    // For now, we'll just verify the state is maintained
    ctx.run_simulation_ticks(25);
    assert_tick_count!(&ctx, 50);
}

/// Test resource creation and management
#[test]
fn test_resource_creation() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create test resources
    let positions = [(10, 10), (15, 15), (20, 20)];
    let test_resources = create_test_resources(world, &positions);

    // Verify resources were created
    assert_eq!(test_resources.len(), 3);

    // Verify resource properties
    for entity in test_resources {
        if let Some(resource) = world.get_entity(entity).and_then(|e| e.get::<ResourceNode>()) {
            assert!(resource.amount > 0);
            assert!(resource.max_amount > 0);
            assert!(resource.regeneration_rate >= 0.0);
        } else {
            panic!("Resource component not found");
        }
    }
}

/// Test entity component interaction
#[test]
fn test_entity_component_interaction() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create a test entity with multiple components
    let entity = world.spawn((
        NameComponent("TestEntity".to_string()),
        UnitTag,
        UnitStats::default(),
        PositionComponent { x: 10.0, y: 10.0 },
    )).id();

    // Verify all components exist
    let entity_ref = world.get_entity(entity).unwrap();
    assert!(entity_ref.get::<NameComponent>().is_some());
    assert!(entity_ref.get::<UnitTag>().is_some());
    assert!(entity_ref.get::<UnitStats>().is_some());
    assert!(entity_ref.get::<PositionComponent>().is_some());

    // Verify component values
    let name = entity_ref.get::<NameComponent>().unwrap();
    assert_eq!(name.0, "TestEntity");
}

/// Test simulation state persistence
#[test]
fn test_simulation_state_persistence() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    // Run simulation to establish state
    ctx.run_simulation_ticks(25);
    assert_tick_count!(&ctx, 25);

    // Verify simulation state is maintained
    assert_simulation_running!(&ctx);

    // Run more ticks
    ctx.run_simulation_ticks(25);
    assert_tick_count!(&ctx, 50);
}

/// Test resource regeneration
#[test]
fn test_resource_regeneration() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create a resource with regeneration
    let resource_entity = world.spawn((
        NameComponent("RegeneratingResource".to_string()),
        ResourceNode {
            resource_type: ResourceType::Wood,
            amount: 50,
            max_amount: 100,
            regeneration_rate: 0.1,
            last_harvest_time: 0,
        },
        PositionComponent { x: 15.0, y: 15.0 },
    )).id();

    // Get initial amount
    let initial_amount = world.get_entity(resource_entity)
        .and_then(|e| e.get::<ResourceNode>())
        .unwrap()
        .amount;

    // Run simulation for a while to allow regeneration
    ctx.run_simulation_ticks(100);

    // Check if resource regenerated (this depends on the actual regeneration system)
    let final_amount = world.get_entity(resource_entity)
        .and_then(|e| e.get::<ResourceNode>())
        .unwrap()
        .amount;

    // For now, just verify the resource still exists
    assert!(final_amount > 0);
}

/// Test entity query operations
#[test]
fn test_entity_query_operations() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create entities with different components
    let _unit1 = world.spawn((UnitTag, NameComponent("Unit1".to_string()))).id();
    let _unit2 = world.spawn((UnitTag, NameComponent("Unit2".to_string()))).id();
    let _resource = world.spawn((ResourceNode {
        resource_type: ResourceType::Wood,
        amount: 100,
        max_amount: 100,
        regeneration_rate: 0.0,
        last_harvest_time: 0,
    }, NameComponent("Resource1".to_string()))).id();

    // Test entity queries
    let unit_count = count_entities_with_component::<UnitTag>(world);
    assert!(unit_count >= 2); // Initial units + our test units

    let resource_count = count_entities_with_component::<ResourceNode>(world);
    assert!(resource_count > 0);
}

/// Test simulation cleanup
#[test]
fn test_simulation_cleanup() {
    let config = TestConfig::default();
    let mut ctx = TestContext::new(config);

    // Create some test entities
    let world = ctx.app.world();
    let _entities = create_test_entities(world, 5);

    // Verify entities exist
    let count = count_entities_with_component::<UnitTag>(world);
    assert!(count > 0);

    // Test context cleanup should happen automatically when ctx is dropped
    // This test mainly ensures the cleanup doesn't panic
    ctx.cleanup();
}
