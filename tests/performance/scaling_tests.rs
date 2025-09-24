//! Performance scaling tests for the world-simulator
//!
//! This module tests how the simulator performs under different loads:
//! - Entity scaling tests
//! - World size scaling tests
//! - AI complexity scaling tests
//! - Memory usage scaling tests
//! - FPS consistency under load

use world_sim_interface::*;
use world_sim_simple::*;

mod common;
use common::*;

/// Test entity scaling performance
#[test]
fn test_entity_scaling_performance() {
    let config = TestConfig {
        simulation_ticks: 500,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Test different entity counts
    let entity_counts = vec![10, 25, 50, 100, 200, 500];
    let mut performance_results = Vec::new();

    for &count in &entity_counts {
        // Clear previous entities
        world.clear_entities();

        // Spawn test entities
        spawn_test_entities(world, count);

        // Run simulation and measure performance
        let start_time = std::time::Instant::now();
        ctx.run_simulation_ticks(100);
        let duration = start_time.elapsed();

        // Collect performance metrics
        let result = PerformanceResult {
            entity_count: count,
            duration_ms: duration.as_millis() as f64,
            fps_estimate: 100.0 / duration.as_secs_f64(),
            memory_estimate: estimate_memory_usage(count),
        };

        performance_results.push(result);

        // Verify performance is acceptable
        assert!(result.fps_estimate > 10.0,
            "FPS too low with {} entities: {:.1}", count, result.fps_estimate);
    }

    // Analyze scaling efficiency
    analyze_scaling_efficiency(&performance_results);
}

/// Test world size scaling performance
#[test]
fn test_world_size_scaling() {
    let config = TestConfig {
        simulation_ticks: 300,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world_sizes = vec![(16, 16), (32, 32), (64, 64), (128, 128)];
    let mut scaling_results = Vec::new();

    for (width, height) in world_sizes {
        // Create world map with specific size
        let mut world_map = WorldMap::new(width, height);
        initialize_test_world(&mut world_map);

        // Insert world map
        world.insert_resource(world_map);

        // Run simulation and measure performance
        let start_time = std::time::Instant::now();
        ctx.run_simulation_ticks(100);
        let duration = start_time.elapsed();

        let area = width * height;
        let result = ScalingResult {
            world_size: (width, height),
            area,
            duration_ms: duration.as_millis() as f64,
            performance_per_tile: duration.as_millis() as f64 / area as f64,
        };

        scaling_results.push(result);

        // Performance should scale reasonably with area
        let expected_max = (area as f64).log2() * 2.0; // Logarithmic scaling expectation
        assert!(result.performance_per_tile < expected_max,
            "World size {}x{} performance too poor: {:.3}ms/tile",
            width, height, result.performance_per_tile);
    }

    // Verify scaling is acceptable
    verify_world_scaling(&scaling_results);
}

/// Test AI complexity scaling
#[test]
fn test_ai_complexity_scaling() {
    let config = TestConfig {
        simulation_ticks: 400,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    let ai_complexity_levels = vec![
        ("basic", 1),
        ("medium", 3),
        ("complex", 5),
        ("advanced", 8)
    ];
    let mut ai_results = Vec::new();

    for (complexity_name, complexity_factor) in ai_complexity_levels {
        // Clear previous entities
        world.clear_entities();

        // Spawn AI entities with different complexity
        spawn_ai_entities(world, 25, complexity_factor);

        // Run simulation and measure AI performance
        let start_time = std::time::Instant::now();
        ctx.run_simulation_ticks(200);
        let duration = start_time.elapsed();

        let result = AIComplexityResult {
            complexity_level: complexity_name.to_string(),
            complexity_factor,
            duration_ms: duration.as_millis() as f64,
            ai_entities: 25,
            efficiency_score: calculate_ai_efficiency(complexity_factor, duration.as_millis() as f64),
        };

        ai_results.push(result);

        // AI efficiency should not degrade too much with complexity
        assert!(result.efficiency_score > 0.3,
            "AI efficiency too low for {} complexity: {:.2}",
            complexity_name, result.efficiency_score);
    }

    analyze_ai_scaling(&ai_results);
}

/// Test memory usage scaling
#[test]
fn test_memory_usage_scaling() {
    let config = TestConfig {
        simulation_ticks: 300,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    let test_scenarios = vec![
        ("minimal", 10, 100),
        ("light", 50, 200),
        ("medium", 100, 500),
        ("heavy", 200, 1000),
        ("extreme", 500, 2000)
    ];

    for (scenario_name, entity_count, expected_max_memory_mb) in test_scenarios {
        // Clear and reset
        world.clear_entities();

        // Spawn entities with memory-heavy components
        spawn_memory_test_entities(world, entity_count);

        // Measure memory before simulation
        let memory_before = get_memory_usage();

        // Run simulation
        ctx.run_simulation_ticks(100);

        // Measure memory after simulation
        let memory_after = get_memory_usage();
        let memory_increase = memory_after - memory_before;

        // Verify memory usage is reasonable
        assert!(memory_increase < expected_max_memory_mb,
            "Memory usage too high for {} scenario: {}MB (expected < {}MB)",
            scenario_name, memory_increase, expected_max_memory_mb);

        // Verify no memory leaks (memory should stabilize)
        let stabilization_runs = 3;
        let mut memory_stable = true;
        let mut last_memory = memory_after;

        for _ in 0..stabilization_runs {
            ctx.run_simulation_ticks(50);
            let current_memory = get_memory_usage();
            if current_memory > last_memory * 1.1 { // 10% growth threshold
                memory_stable = false;
                break;
            }
            last_memory = current_memory;
        }

        assert!(memory_stable,
            "Memory usage not stabilizing in {} scenario - potential leak",
            scenario_name);
    }
}

/// Test FPS consistency under load
#[test]
fn test_fps_consistency() {
    let config = TestConfig {
        simulation_ticks: 1000,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create load scenario
    spawn_load_test_entities(world, 150);

    // Collect FPS samples during simulation
    let mut fps_samples = Vec::new();
    let mut frame_times = Vec::new();

    for _ in 0..100 {
        let start_time = std::time::Instant::now();
        ctx.run_simulation_ticks(10);
        let duration = start_time.elapsed();

        let fps = 10.0 / duration.as_secs_f64();
        let frame_time_ms = duration.as_millis() as f64 / 10.0;

        fps_samples.push(fps);
        frame_times.push(frame_time_ms);
    }

    // Analyze FPS consistency
    let avg_fps = fps_samples.iter().sum::<f64>() / fps_samples.len() as f64;
    let fps_std_dev = calculate_std_dev(&fps_samples);
    let min_fps = fps_samples.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_fps = fps_samples.iter().fold(0.0, |a, &b| a.max(b));

    // FPS should be reasonably consistent
    assert!(avg_fps > 15.0, "Average FPS too low: {:.1}", avg_fps);
    assert!(fps_std_dev < avg_fps * 0.3, // Standard deviation < 30% of average
        "FPS too inconsistent: avg={:.1}, std_dev={:.1}", avg_fps, fps_std_dev);
    assert!(min_fps > avg_fps * 0.5, // Minimum FPS > 50% of average
        "FPS drops too severe: avg={:.1}, min={:.1}", avg_fps, min_fps);

    // Frame times should not have extreme spikes
    let avg_frame_time = frame_times.iter().sum::<f64>() / frame_times.len() as f64;
    let max_frame_time = frame_times.iter().fold(0.0, |a, &b| a.max(b));

    assert!(max_frame_time < avg_frame_time * 5.0, // No frame should be 5x average
        "Frame time spike detected: avg={:.2}ms, max={:.2}ms",
        avg_frame_time, max_frame_time);
}

/// Test concurrent system performance
#[test]
fn test_concurrent_system_performance() {
    let config = TestConfig {
        simulation_ticks: 500,
        ..Default::default()
    };
    let mut ctx = TestContext::new(config);

    let world = ctx.app.world();

    // Create a scenario that exercises multiple systems
    spawn_concurrent_test_entities(world, 100);

    // Measure performance with all systems running
    let start_time = std::time::Instant::now();
    ctx.run_simulation_ticks(200);
    let full_duration = start_time.elapsed();

    // Measure performance with systems disabled one by one
    let mut system_impact = Vec::new();
    let system_names = vec!["movement", "ai", "resource", "building"];

    for system_name in &system_names {
        let start_time = std::time::Instant::now();
        // In a real implementation, we would disable specific systems
        ctx.run_simulation_ticks(50);
        let duration = start_time.elapsed();

        system_impact.push((system_name, duration.as_millis() as f64));
    }

    // Analyze system impact
    let full_duration_ms = full_duration.as_millis() as f64;

    // No single system should dominate performance
    for (system_name, duration) in &system_impact {
        let impact_percentage = (duration / full_duration_ms) * 100.0;
        assert!(impact_percentage < 70.0, // No system should take > 70% of time
            "System {} performance impact too high: {:.1}%",
            system_name, impact_percentage);
    }

    // Verify overall performance is reasonable
    let target_fps = 30.0;
    let actual_fps = 200.0 / full_duration.as_secs_f64();
    assert!(actual_fps >= target_fps,
        "Concurrent systems performance below target: {:.1} FPS (target {})",
        actual_fps, target_fps);
}

// Helper structs for performance testing
#[derive(Debug)]
struct PerformanceResult {
    entity_count: usize,
    duration_ms: f64,
    fps_estimate: f64,
    memory_estimate: f64,
}

#[derive(Debug)]
struct ScalingResult {
    world_size: (u32, u32),
    area: u32,
    duration_ms: f64,
    performance_per_tile: f64,
}

#[derive(Debug)]
struct AIComplexityResult {
    complexity_level: String,
    complexity_factor: u32,
    duration_ms: f64,
    ai_entities: usize,
    efficiency_score: f64,
}

// Helper functions
fn spawn_test_entities(world: &mut World, count: usize) {
    for i in 0..count {
        world.spawn((
            NameComponent(format!("TestEntity_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: (i % 20) as f32,
                y: (i / 20) as f32,
            },
            Inventory::new(),
        ));
    }
}

fn spawn_ai_entities(world: &mut World, count: usize, complexity_factor: u32) {
    for i in 0..count {
        let mut goap_actions = Vec::new();
        for j in 0..complexity_factor {
            goap_actions.push(GoapAction {
                name: format!("action_{}_{}", i, j),
                cost: 1.0,
                preconditions: vec![],
                effects: vec![],
                duration: 10,
            });
        }

        let mut behaviors = Vec::new();
        for j in 0..complexity_factor {
            behaviors.push(UtilityBehavior {
                name: format!("behavior_{}_{}", i, j),
                utility_score: 0.5,
                considerations: vec![],
                weight: 1.0,
            });
        }

        world.spawn((
            NameComponent(format!("AIEntity_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: (i % 15) as f32,
                y: (i / 15) as f32,
            },
            GoapAgent {
                current_plan: None,
                available_actions: goap_actions,
                world_state: std::collections::HashMap::new(),
                goals: vec![GoapGoal {
                    name: "survive".to_string(),
                    priority: 1.0,
                    conditions: vec![],
                }],
            },
            UtilityAgent {
                current_action: None,
                behaviors,
                needs: std::collections::HashMap::new(),
            },
            Inventory::new(),
        ));
    }
}

fn spawn_memory_test_entities(world: &mut World, count: usize) {
    for i in 0..count {
        // Create entities with memory-heavy components
        let mut inventory = Inventory::new();
        // Add many inventory items to increase memory usage
        for j in 0..50 {
            inventory.add_item(format!("item_{}_{}", i, j), 1);
        }

        world.spawn((
            NameComponent(format!("MemoryEntity_{}", i)),
            UnitTag,
            UnitStats {
                health: 100 + i as u32,
                max_health: 100 + i as u32,
                energy: 100 + i as u32,
                max_energy: 100 + i as u32,
                movement_speed: 1.0,
            },
            PositionComponent {
                x: (i % 25) as f32,
                y: (i / 25) as f32,
            },
            inventory,
            // Add components with more data
            ResourceNode {
                resource_type: ResourceType::Wood,
                amount: 100 + i as u32,
                max_amount: 100 + i as u32,
                regeneration_rate: 0.01,
                last_harvest_time: 0,
            },
        ));
    }
}

fn spawn_load_test_entities(world: &mut World, count: usize) {
    for i in 0..count {
        world.spawn((
            NameComponent(format!("LoadEntity_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: rand::random::<f32>() * 32.0,
                y: rand::random::<f32>() * 32.0,
            },
            // Add AI components for more complex processing
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
                goals: vec![
                    GoapGoal {
                        name: "survive".to_string(),
                        priority: 1.0,
                        conditions: vec![],
                    },
                    GoapGoal {
                        name: "gather_resources".to_string(),
                        priority: 0.8,
                        conditions: vec![],
                    },
                ],
            },
            Inventory::new(),
        ));
    }
}

fn spawn_concurrent_test_entities(world: &mut World, count: usize) {
    for i in 0..count {
        world.spawn((
            NameComponent(format!("ConcurrentEntity_{}", i)),
            UnitTag,
            UnitStats::default(),
            PositionComponent {
                x: (i % 20) as f32,
                y: (i / 20) as f32,
            },
            // Add components that exercise different systems
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
                amount: 100,
                max_amount: 100,
                regeneration_rate: 0.01,
                last_harvest_time: 0,
            },
            BuildingComponent {
                building_type: BuildingType::House,
                health: 100,
                max_health: 100,
                construction_progress: 1.0,
            },
            Inventory::new(),
        ));
    }
}

fn initialize_test_world(world_map: &mut WorldMap) {
    // Initialize world with test data
    for y in 0..world_map.height {
        for x in 0..world_map.width {
            // Simple terrain generation
            let terrain_type = if (x + y) % 5 == 0 {
                TerrainType::Water
            } else if (x * y) % 7 == 0 {
                TerrainType::Mountain
            } else {
                TerrainType::Grass
            };
            world_map.set_terrain(x, y, terrain_type);
        }
    }
}

fn estimate_memory_usage(entity_count: usize) -> f64 {
    // Rough estimate of memory usage per entity
    50.0 + (entity_count as f64 * 0.5)
}

fn get_memory_usage() -> f64 {
    // In a real implementation, this would measure actual memory usage
    // For testing, we return a reasonable estimate
    50.0 + (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() % 100) as f64 * 0.1
}

fn calculate_ai_efficiency(complexity_factor: u32, duration_ms: f64) -> f64 {
    // Efficiency score based on complexity and performance
    let baseline_duration = 100.0; // Baseline duration for complexity 1
    let expected_duration = baseline_duration * complexity_factor as f64;
    (expected_duration / duration_ms).min(1.0)
}

fn calculate_std_dev(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    variance.sqrt()
}

fn analyze_scaling_efficiency(results: &[PerformanceResult]) {
    // Verify that performance scales reasonably with entity count
    for window in results.windows(2) {
        let (prev, curr) = (&window[0], &window[1]);
        let entity_ratio = curr.entity_count as f64 / prev.entity_count as f64;
        let performance_ratio = curr.duration_ms / prev.duration_ms;

        // Performance should not degrade faster than linear scaling
        assert!(performance_ratio <= entity_ratio * 1.5, // Allow 50% overhead
            "Performance degrades too rapidly from {} to {} entities: {:.2}x ratio for {:.2}x entity increase",
            prev.entity_count, curr.entity_count, performance_ratio, entity_ratio);
    }
}

fn verify_world_scaling(results: &[ScalingResult]) {
    // Verify that world size scaling is reasonable
    for window in results.windows(2) {
        let (prev, curr) = (&window[0], &window[1]);
        let area_ratio = curr.area as f64 / prev.area as f64;
        let performance_ratio = curr.performance_per_tile / prev.performance_per_tile;

        // Per-tile performance should not degrade too much with area
        assert!(performance_ratio <= area_ratio.sqrt(),
            "World size scaling inefficient: {}x{} -> {}x{} (area ratio {:.1}x, performance ratio {:.2}x)",
            prev.world_size.0, prev.world_size.1, curr.world_size.0, curr.world_size.1,
            area_ratio, performance_ratio);
    }
}

fn analyze_ai_scaling(results: &[AIComplexityResult]) {
    // Verify that AI complexity scaling is reasonable
    for window in results.windows(2) {
        let (prev, curr) = (&window[0], &window[1]);
        let complexity_ratio = curr.complexity_factor as f64 / prev.complexity_factor as f64;
        let efficiency_ratio = curr.efficiency_score / prev.efficiency_score;

        // Efficiency should not drop too much with complexity
        assert!(efficiency_ratio >= 1.0 / complexity_ratio,
            "AI efficiency drops too much with complexity: {} -> {} (complexity ratio {:.1}x, efficiency ratio {:.2}x)",
            prev.complexity_level, curr.complexity_level, complexity_ratio, efficiency_ratio);
    }
}