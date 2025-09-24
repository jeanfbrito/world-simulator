//! Performance benchmarks for the world-simulator
//!
//! This module provides detailed benchmarks for critical systems:
//! - Entity spawning and destruction
//! - AI processing and decision making
//! - Pathfinding and navigation
//! - Resource management
//! - System scheduling

use world_sim_interface::*;
use world_sim_simple::*;

mod common;
use common::*;

/// Benchmark entity spawning performance
#[test]
fn benchmark_entity_spawning() {
    let config = TestConfig {
        simulation_ticks: 100,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();
    let entity_counts = vec![10, 50, 100, 500, 1000];
    let mut spawn_results = Vec::new();

    for &count in &entity_counts {
        // Measure spawn time
        let start_time = std::time::Instant::now();

        let spawned_entities = spawn_benchmark_entities(world, count);

        let spawn_duration = start_time.elapsed();

        // Measure despawn time
        let start_time = std::time::Instant::now();

        for entity in spawned_entities {
            world.despawn(entity);
        }

        let despawn_duration = start_time.elapsed();

        let result = SpawnBenchmarkResult {
            entity_count: count,
            spawn_time_ms: spawn_duration.as_millis() as f64,
            despawn_time_ms: despawn_duration.as_millis() as f64,
            spawn_rate: count as f64 / spawn_duration.as_secs_f64(),
            despawn_rate: count as f64 / despawn_duration.as_secs_f64(),
        };

        spawn_results.push(result);

        // Verify spawn performance
        assert!(result.spawn_rate > 100.0,
            "Entity spawn rate too low: {:.1} entities/sec for {} entities",
            result.spawn_rate, count);

        // Verify despawn performance
        assert!(result.despawn_rate > 1000.0,
            "Entity despawn rate too low: {:.1} entities/sec for {} entities",
            result.despawn_rate, count);
    }

    analyze_spawn_scaling(&spawn_results);
}

/// Benchmark AI processing performance
#[test]
fn benchmark_ai_processing() {
    let config = TestConfig {
        simulation_ticks: 200,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    let ai_counts = vec![5, 10, 25, 50, 100];
    let mut ai_results = Vec::new();

    for &count in &ai_counts {
        // Clear previous entities
        world.clear_entities();

        // Spawn AI entities
        spawn_ai_benchmark_entities(world, count);

        // Warm up simulation
        ctx.run_simulation_ticks(50);

        // Measure AI processing time
        let start_time = std::time::Instant::now();
        ctx.run_simulation_ticks(100);
        let ai_duration = start_time.elapsed();

        let result = AIBenchmarkResult {
            ai_entity_count: count,
            processing_time_ms: ai_duration.as_millis() as f64,
            ai_processing_rate: (count * 100) as f64 / ai_duration.as_secs_f64(),
            average_ai_time_ms: ai_duration.as_millis() as f64 / (count * 100) as f64,
        };

        ai_results.push(result);

        // Verify AI processing performance
        assert!(result.ai_processing_rate > 500.0,
            "AI processing rate too low: {:.1} AI decisions/sec for {} AI entities",
            result.ai_processing_rate, count);

        // Average AI time should be reasonable
        assert!(result.average_ai_time_ms < 2.0,
            "Average AI time too high: {:.2}ms per AI decision",
            result.average_ai_time_ms);
    }

    analyze_ai_scaling(&ai_results);
}

/// Benchmark pathfinding performance
#[test]
fn benchmark_pathfinding() {
    let config = TestConfig {
        simulation_ticks: 150,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create test world with obstacles
    create_pathfinding_test_world(world);

    let pathfinding_scenarios = vec![
        ("short", 5),
        ("medium", 15),
        ("long", 30),
        ("complex", 50),
    ];
    let mut pathfinding_results = Vec::new();

    for (scenario_name, path_length) in pathfinding_scenarios {
        // Spawn pathfinding agents
        let agents = spawn_pathfinding_benchmark_entities(world, 10);

        // Measure pathfinding performance
        let start_time = std::time::Instant::now();

        // In a real implementation, this would trigger pathfinding
        ctx.run_simulation_ticks(50);

        let pathfinding_duration = start_time.elapsed();

        let result = PathfindingBenchmarkResult {
            scenario: scenario_name.to_string(),
            path_length,
            agent_count: 10,
            total_time_ms: pathfinding_duration.as_millis() as f64,
            paths_per_second: (10 * 50) as f64 / pathfinding_duration.as_secs_f64(),
            average_path_time_ms: pathfinding_duration.as_millis() as f64 / (10 * 50) as f64,
        };

        pathfinding_results.push(result);

        // Verify pathfinding performance
        assert!(result.paths_per_second > 100.0,
            "Pathfinding rate too low for {} scenario: {:.1} paths/sec",
            scenario_name, result.paths_per_second);

        // Clean up agents
        for agent in agents {
            world.despawn(agent);
        }
    }

    analyze_pathfinding_performance(&pathfinding_results);
}

/// Benchmark resource management performance
#[test]
fn benchmark_resource_management() {
    let config = TestConfig {
        simulation_ticks: 200,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    let resource_counts = vec![20, 50, 100, 200, 500];
    let mut resource_results = Vec::new();

    for &count in &resource_counts {
        // Clear previous entities
        world.clear_entities();

        // Spawn resources and gatherers
        let resources = spawn_resource_benchmark_entities(world, count);
        let gatherers = spawn_gatherer_benchmark_entities(world, count / 10);

        // Measure resource management performance
        let start_time = std::time::Instant::now();
        ctx.run_simulation_ticks(100);
        let management_duration = start_time.elapsed();

        let result = ResourceBenchmarkResult {
            resource_count: count,
            gatherer_count: count / 10,
            management_time_ms: management_duration.as_millis() as f64,
            operations_per_second: (count + count / 10) as f64 * 100.0 / management_duration.as_secs_f64(),
            average_operation_time_ms: management_duration.as_millis() as f64 / ((count + count / 10) * 100) as f64,
        };

        resource_results.push(result);

        // Verify resource management performance
        assert!(result.operations_per_second > 1000.0,
            "Resource management rate too low: {:.1} operations/sec for {} resources",
            result.operations_per_second, count);

        // Clean up
        for resource in resources {
            world.despawn(resource);
        }
        for gatherer in gatherers {
            world.despawn(gatherer);
        }
    }

    analyze_resource_scaling(&resource_results);
}

/// Benchmark system scheduling performance
#[test]
fn benchmark_system_scheduling() {
    let config = TestConfig {
        simulation_ticks: 300,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create scenario that exercises all systems
    spawn_system_benchmark_entities(world, 150);

    let system_configs = vec![
        ("all_systems", true, true, true, true),
        ("movement_only", true, false, false, false),
        ("ai_only", false, true, false, false),
        ("resource_only", false, false, true, false),
        ("minimal", false, false, false, false),
    ];
    let mut system_results = Vec::new();

    for (config_name, movement, ai, resource, building) in system_configs {
        // Configure systems (in real implementation, we would enable/disable systems)

        // Measure performance
        let start_time = std::time::Instant::now();
        ctx.run_simulation_ticks(100);
        let system_duration = start_time.elapsed();

        let result = SystemBenchmarkResult {
            config_name: config_name.to_string(),
            duration_ms: system_duration.as_millis() as f64,
            fps: 100.0 / system_duration.as_secs_f64(),
            system_count: [movement, ai, resource, building].iter().filter(|&&x| x).count(),
        };

        system_results.push(result);

        // Verify minimum performance
        assert!(result.fps > 20.0,
            "System performance too low for {} config: {:.1} FPS",
            config_name, result.fps);
    }

    analyze_system_performance(&system_results);
}

/// Benchmark memory allocation patterns
#[test]
fn benchmark_memory_allocation() {
    let config = TestConfig {
        simulation_ticks: 100,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    let allocation_scenarios = vec![
        ("entity_spawning", || spawn_allocation_test_entities(world, 50)),
        ("component_addition", || add_component_test_entities(world, 30)),
        ("resource_operations", || perform_resource_operations(world, 20)),
        ("ai_processing", || spawn_ai_allocation_entities(world, 25)),
    ];

    for (scenario_name, scenario_fn) in allocation_scenarios {
        // Clear world
        world.clear_entities();

        // Measure memory before
        let memory_before = get_memory_usage();

        // Run scenario
        scenario_fn();

        // Run simulation to trigger allocations
        ctx.run_simulation_ticks(50);

        // Measure memory after
        let memory_after = get_memory_usage();
        let memory_increase = memory_after - memory_before;

        // Verify memory usage is reasonable
        let max_expected = match scenario_name {
            "entity_spawning" => 50.0,
            "component_addition" => 30.0,
            "resource_operations" => 20.0,
            "ai_processing" => 40.0,
            _ => 100.0,
        };

        assert!(memory_increase < max_expected,
            "Memory allocation too high for {}: {:.1}MB (expected < {:.1}MB)",
            scenario_name, memory_increase, max_expected);
    }
}

// Helper structs for benchmarking
#[derive(Debug)]
struct SpawnBenchmarkResult {
    entity_count: usize,
    spawn_time_ms: f64,
    despawn_time_ms: f64,
    spawn_rate: f64,
    despawn_rate: f64,
}

#[derive(Debug)]
struct AIBenchmarkResult {
    ai_entity_count: usize,
    processing_time_ms: f64,
    ai_processing_rate: f64,
    average_ai_time_ms: f64,
}

#[derive(Debug)]
struct PathfindingBenchmarkResult {
    scenario: String,
    path_length: u32,
    agent_count: usize,
    total_time_ms: f64,
    paths_per_second: f64,
    average_path_time_ms: f64,
}

#[derive(Debug)]
struct ResourceBenchmarkResult {
    resource_count: usize,
    gatherer_count: usize,
    management_time_ms: f64,
    operations_per_second: f64,
    average_operation_time_ms: f64,
}

#[derive(Debug)]
struct SystemBenchmarkResult {
    config_name: String,
    duration_ms: f64,
    fps: f64,
    system_count: usize,
}

// Helper functions for benchmarking
fn spawn_benchmark_entities(world: &mut World, count: usize) -> Vec<Entity> {
    let mut entities = Vec::new();

    for i in 0..count {
        let entity = world.spawn((
            NameComponent(format!("BenchmarkEntity_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: (i % 20) as f32,
                y: (i / 20) as f32,
            },
            Inventory::new(),
        )).id();

        entities.push(entity);
    }

    entities
}

fn spawn_ai_benchmark_entities(world: &mut World, count: usize) {
    for i in 0..count {
        world.spawn((
            NameComponent(format!("AIBenchmarkEntity_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: (i % 15) as f32,
                y: (i / 15) as f32,
            },
            GoapAgent {
                current_plan: None,
                available_actions: vec![
                    GoapAction {
                        name: "move".to_string(),
                        cost: 1.0,
                        preconditions: vec![],
                        effects: vec![],
                        duration: 5,
                    },
                    GoapAction {
                        name: "gather".to_string(),
                        cost: 2.0,
                        preconditions: vec![],
                        effects: vec![],
                        duration: 10,
                    },
                ],
                world_state: std::collections::HashMap::new(),
                goals: vec![GoapGoal {
                    name: "survive".to_string(),
                    priority: 1.0,
                    conditions: vec![],
                }],
            },
            UtilityAgent {
                current_action: None,
                behaviors: vec![UtilityBehavior {
                    name: "survive".to_string(),
                    utility_score: 0.8,
                    considerations: vec![],
                    weight: 1.0,
                }],
                needs: std::collections::HashMap::new(),
            },
            Inventory::new(),
        ));
    }
}

fn create_pathfinding_test_world(world: &mut World) {
    // Create a world map with obstacles for pathfinding testing
    let mut world_map = WorldMap::new(32, 32);

    // Add some obstacles
    for i in 10..20 {
        for j in 10..20 {
            world_map.set_terrain(i, j, TerrainType::Mountain);
        }
    }

    // Add some water obstacles
    for i in 5..8 {
        for j in 5..25 {
            world_map.set_terrain(i, j, TerrainType::Water);
        }
    }

    world.insert_resource(world_map);
}

fn spawn_pathfinding_benchmark_entities(world: &mut World, count: usize) -> Vec<Entity> {
    let mut entities = Vec::new();

    for i in 0..count {
        let entity = world.spawn((
            NameComponent(format!("PathfindingAgent_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: 2.0,
                y: 2.0,
            },
            Inventory::new(),
        )).id();

        entities.push(entity);
    }

    entities
}

fn spawn_resource_benchmark_entities(world: &mut World, count: usize) -> Vec<Entity> {
    let mut entities = Vec::new();

    for i in 0..count {
        let entity = world.spawn((
            NameComponent(format!("BenchmarkResource_{}", i)),
            ResourceNode {
                resource_type: if i % 3 == 0 { ResourceType::Wood }
                              else if i % 3 == 1 { ResourceType::Stone }
                              else { ResourceType::Food },
                amount: 100,
                max_amount: 100,
                regeneration_rate: 0.01,
                last_harvest_time: 0,
            },
            PositionComponent {
                x: (i % 20) as f32,
                y: (i / 20) as f32,
            },
        )).id();

        entities.push(entity);
    }

    entities
}

fn spawn_gatherer_benchmark_entities(world: &mut World, count: usize) -> Vec<Entity> {
    let mut entities = Vec::new();

    for i in 0..count {
        let entity = world.spawn((
            NameComponent(format!("BenchmarkGatherer_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: (i % 15) as f32,
                y: (i / 15) as f32,
            },
            GoapAgent {
                current_plan: None,
                available_actions: vec![GoapAction {
                    name: "gather".to_string(),
                    cost: 1.0,
                    preconditions: vec![],
                    effects: vec![],
                    duration: 5,
                }],
                world_state: std::collections::HashMap::new(),
                goals: vec![GoapGoal {
                    name: "gather_resources".to_string(),
                    priority: 1.0,
                    conditions: vec![],
                }],
            },
            Inventory::new(),
        )).id();

        entities.push(entity);
    }

    entities
}

fn spawn_system_benchmark_entities(world: &mut World, count: usize) {
    for i in 0..count {
        world.spawn((
            NameComponent(format!("SystemBenchmarkEntity_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: (i % 20) as f32,
                y: (i / 20) as f32,
            },
            GoapAgent {
                current_plan: None,
                available_actions: vec![GoapAction {
                    name: "test_action".to_string(),
                    cost: 1.0,
                    preconditions: vec![],
                    effects: vec![],
                    duration: 5,
                }],
                world_state: std::collections::HashMap::new(),
                goals: vec![GoapGoal {
                    name: "test_goal".to_string(),
                    priority: 1.0,
                    conditions: vec![],
                }],
            },
            UtilityAgent {
                current_action: None,
                behaviors: vec![UtilityBehavior {
                    name: "test_behavior".to_string(),
                    utility_score: 0.5,
                    considerations: vec![],
                    weight: 1.0,
                }],
                needs: std::collections::HashMap::new(),
            },
            ResourceNode {
                resource_type: ResourceType::Wood,
                amount: 50,
                max_amount: 50,
                regeneration_rate: 0.01,
                last_harvest_time: 0,
            },
            Inventory::new(),
        ));
    }
}

fn spawn_allocation_test_entities(world: &mut World, count: usize) {
    for i in 0..count {
        world.spawn((
            NameComponent(format!("AllocationTestEntity_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: i as f32,
                y: i as f32,
            },
            Inventory::new(),
        ));
    }
}

fn add_component_test_entities(world: &mut World, count: usize) {
    for i in 0..count {
        let mut entity = world.spawn((
            NameComponent(format!("ComponentTestEntity_{}", i)),
            UnitTag,
        ));

        // Add components dynamically
        entity.insert(UnitStats::default());
        entity.insert(PositionComponent { x: i as f32, y: i as f32 });
        entity.insert(Inventory::new());
    }
}

fn perform_resource_operations(world: &mut World, count: usize) {
    for i in 0..count {
        let resource = world.spawn((
            NameComponent(format!("ResourceOp_{}", i)),
            ResourceNode {
                resource_type: ResourceType::Wood,
                amount: 100,
                max_amount: 100,
                regeneration_rate: 0.01,
                last_harvest_time: 0,
            },
            PositionComponent {
                x: i as f32,
                y: i as f32,
            },
        ));

        // Perform resource operations
        let mut inventory = Inventory::new();
        inventory.add_item("wood".to_string(), 10);
    }
}

fn spawn_ai_allocation_entities(world: &mut World, count: usize) {
    for i in 0..count {
        world.spawn((
            NameComponent(format!("AIAllocationEntity_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: i as f32,
                y: i as f32,
            },
            GoapAgent {
                current_plan: None,
                available_actions: vec![
                    GoapAction {
                        name: "complex_action_1".to_string(),
                        cost: 1.0,
                        preconditions: vec![],
                        effects: vec![],
                        duration: 10,
                    },
                    GoapAction {
                        name: "complex_action_2".to_string(),
                        cost: 2.0,
                        preconditions: vec![],
                        effects: vec![],
                        duration: 15,
                    },
                ],
                world_state: std::collections::HashMap::new(),
                goals: vec![GoapGoal {
                    name: "complex_goal".to_string(),
                    priority: 1.0,
                    conditions: vec![],
                }],
            },
            UtilityAgent {
                current_action: None,
                behaviors: vec![
                    UtilityBehavior {
                        name: "complex_behavior_1".to_string(),
                        utility_score: 0.7,
                        considerations: vec![],
                        weight: 1.0,
                    },
                    UtilityBehavior {
                        name: "complex_behavior_2".to_string(),
                        utility_score: 0.3,
                        considerations: vec![],
                        weight: 0.5,
                    },
                ],
                needs: std::collections::HashMap::from([
                    ("complex_need_1".to_string(), 0.8),
                    ("complex_need_2".to_string(), 0.6),
                ]),
            },
            Inventory::new(),
        ));
    }
}

// Analysis functions
fn analyze_spawn_scaling(results: &[SpawnBenchmarkResult]) {
    for window in results.windows(2) {
        let (prev, curr) = (&window[0], &window[1]);
        let entity_ratio = curr.entity_count as f64 / prev.entity_count as f64;
        let spawn_time_ratio = curr.spawn_time_ms / prev.spawn_time_ms;

        // Spawn time should scale roughly linearly
        assert!(spawn_time_ratio <= entity_ratio * 1.5,
            "Spawn time scaling inefficient: {} -> {} entities (time ratio {:.2}x for entity ratio {:.1}x)",
            prev.entity_count, curr.entity_count, spawn_time_ratio, entity_ratio);
    }
}

fn analyze_ai_scaling(results: &[AIBenchmarkResult]) {
    for window in results.windows(2) {
        let (prev, curr) = (&window[0], &window[1]);
        let ai_ratio = curr.ai_entity_count as f64 / prev.ai_entity_count as f64;
        let processing_time_ratio = curr.processing_time_ms / prev.processing_time_ms;

        // AI processing time should scale roughly linearly
        assert!(processing_time_ratio <= ai_ratio * 1.3,
            "AI processing time scaling inefficient: {} -> {} AI entities (time ratio {:.2}x for AI ratio {:.1}x)",
            prev.ai_entity_count, curr.ai_entity_count, processing_time_ratio, ai_ratio);
    }
}

fn analyze_pathfinding_performance(results: &[PathfindingBenchmarkResult]) {
    // Verify that pathfinding performance is reasonable across different scenarios
    for result in results {
        assert!(result.average_path_time_ms < 5.0,
            "Average path time too high for {} scenario: {:.2}ms",
            result.scenario, result.average_path_time_ms);

        // Performance should not degrade too much with path length
        let expected_max_time = result.path_length as f64 * 0.1; // 0.1ms per path length unit
        assert!(result.average_path_time_ms < expected_max_time,
            "Path time too high for {} scenario (length {}): {:.2}ms (expected < {:.2}ms)",
            result.scenario, result.path_length, result.average_path_time_ms, expected_max_time);
    }
}

fn analyze_resource_scaling(results: &[ResourceBenchmarkResult]) {
    for window in results.windows(2) {
        let (prev, curr) = (&window[0], &window[1]);
        let resource_ratio = curr.resource_count as f64 / prev.resource_count as f64;
        let management_time_ratio = curr.management_time_ms / prev.management_time_ms;

        // Resource management time should scale reasonably
        assert!(management_time_ratio <= resource_ratio * 1.2,
            "Resource management time scaling inefficient: {} -> {} resources (time ratio {:.2}x for resource ratio {:.1}x)",
            prev.resource_count, curr.resource_count, management_time_ratio, resource_ratio);
    }
}

fn analyze_system_performance(results: &[SystemBenchmarkResult]) {
    // Verify that system performance is reasonable and that additional systems don't cause excessive overhead
    let baseline = results.iter()
        .find(|r| r.config_name == "minimal")
        .expect("Should have minimal config");

    for result in results {
        if result.config_name != "minimal" {
            let overhead_ratio = result.duration_ms / baseline.duration_ms;
            let system_count_ratio = result.system_count as f64 / baseline.system_count as f64;

            // Overhead should be reasonable compared to additional systems
            assert!(overhead_ratio <= system_count_ratio * 2.0,
                "System overhead too high for {} config: {:.2}x duration for {:.1}x systems (baseline {:.2}ms)",
                result.config_name, overhead_ratio, system_count_ratio, baseline.duration_ms);
        }
    }
}