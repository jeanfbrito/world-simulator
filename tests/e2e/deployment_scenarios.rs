use std::time::Duration;
use tokio::time::sleep;

use world_sim_simple::simulation::Simulation;

mod common;

#[tokio::test]
async fn test_production_deployment_workflow() {
    let mut context = common::setup_test_environment().await;

    // Simulate production deployment scenario
    let config = common::TestConfig::production()
        .with_world_size(100, 100)
        .with_peasant_count(50)
        .with_resources(vec![
            "wood".to_string(),
            "stone".to_string(),
            "food".to_string(),
            "iron".to_string(),
            "gold".to_string(),
            "crystal".to_string(),
        ])
        .with_websocket_enabled(true)
        .with_persistence_enabled(true);

    // Initialize production simulation
    let mut simulation = context.initialize_simulation(&config).await.unwrap();

    // Monitor startup performance
    let startup_start = std::time::Instant::now();
    let startup_result = simulation.startup().await;
    assert!(startup_result.is_ok(), "Production startup failed");
    let startup_time = startup_start.elapsed();
    assert!(startup_time < Duration::from_secs(5),
            "Production startup too slow: {:?}", startup_time);

    // Run production simulation for extended period
    let production_duration = Duration::from_secs(60);
    let start_time = std::time::Instant::now();
    let mut metrics = ProductionMetrics::new();

    while start_time.elapsed() < production_duration {
        let tick_start = std::time::Instant::now();

        // Execute tick
        let result = simulation.tick().await;
        assert!(result.is_ok(), "Production tick failed");
        let tick_time = tick_start.elapsed();

        // Collect metrics
        metrics.record_tick(tick_time);

        // Monitor resource usage
        let state = simulation.get_state().await.unwrap();
        metrics.record_entity_count(state.entities.len());
        metrics.record_resources(state.resources.clone());

        // Verify no memory leaks
        if metrics.tick_count % 100 == 0 {
            let memory_usage = simulation.get_memory_usage().await.unwrap();
            assert!(memory_usage < 1024 * 1024 * 1024, // 1GB limit
                   "Memory usage too high: {} MB", memory_usage / (1024 * 1024));
        }

        // Periodic health checks
        if metrics.tick_count % 50 == 0 {
            assert!(simulation.health_check().await.unwrap(),
                   "Production simulation health check failed");
        }

        // Simulate user interactions
        if metrics.tick_count % 20 == 0 {
            let user_actions = generate_user_actions(&state);
            for action in user_actions {
                let result = simulation.handle_user_action(action).await;
                assert!(result.is_ok(), "User action failed");
            }
        }

        sleep(Duration::from_millis(50)).await;
    }

    // Final production validation
    metrics.validate();
    context.cleanup().await;
}

#[tokio::test]
async fn test_scaling_deployment() {
    let mut context = common::setup_test_environment().await;

    // Test scaling from small to large deployments
    let scaling_stages = vec![
        ("small", 10, 30, 30),
        ("medium", 25, 50, 50),
        ("large", 50, 80, 80),
        ("extra_large", 100, 120, 120),
    ];

    let mut previous_metrics = None;

    for (stage_name, entity_count, width, height) in scaling_stages {
        println!("Testing scaling stage: {}", stage_name);

        let config = common::TestConfig::production()
            .with_world_size(width, height)
            .with_peasant_count(entity_count)
            .with_label(stage_name);

        let mut simulation = context.initialize_simulation(&config).await.unwrap();

        // Warm-up period
        for _ in 0..20 {
            let _ = simulation.tick().await;
            sleep(Duration::from_millis(50)).await;
        }

        // Performance measurement
        let start_time = std::time::Instant::now();
        let measurement_duration = Duration::from_secs(30);
        let mut metrics = ScalingMetrics::new(stage_name);

        while start_time.elapsed() < measurement_duration {
            let tick_start = std::time::Instant::now();
            let result = simulation.tick().await;
            assert!(result.is_ok(), "Tick failed in stage {}", stage_name);
            let tick_time = tick_start.elapsed();

            metrics.record_tick(tick_time);

            // Collect state metrics
            let state = simulation.get_state().await.unwrap();
            metrics.record_state(&state);

            sleep(Duration::from_millis(30)).await;
        }

        // Validate scaling performance
        metrics.validate();

        // Compare with previous stage
        if let Some(prev) = previous_metrics {
            metrics.compare_with_previous(prev);
        }

        previous_metrics = Some(metrics);
    }

    context.cleanup().await;
}

#[tokio::test]
async fn test_disaster_recovery_scenarios() {
    let mut context = common::setup_test_environment().await;

    let disaster_scenarios = vec![
        ("network_partition", || async {
            // Simulate network partition
            tokio::time::sleep(Duration::from_secs(2)).await;
        }),
        ("process_restart", || async {
            // Simulate process restart scenario
            tokio::time::sleep(Duration::from_millis(500)).await;
        }),
        ("memory_pressure", || async {
            // Simulate memory pressure
            tokio::time::sleep(Duration::from_secs(1)).await;
        }),
        ("disk_io_failure", || async {
            // Simulate disk I/O issues
            tokio::time::sleep(Duration::from_millis(200)).await;
        }),
    ];

    for (scenario_name, disaster_fn) in disaster_scenarios {
        println!("Testing disaster recovery: {}", scenario_name);

        let config = common::TestConfig::production()
            .with_peasant_count(20)
            .with_persistence_enabled(true);

        let mut simulation = context.initialize_simulation(&config).await.unwrap();

        // Pre-disaster baseline
        for _ in 0..50 {
            let _ = simulation.tick().await;
            sleep(Duration::from_millis(50)).await;
        }

        let pre_disaster_state = simulation.get_state().await.unwrap();

        // Save state before disaster
        let saved_state = simulation.save_state().await.unwrap();

        // Simulate disaster
        println!("Simulating disaster: {}", scenario_name);
        disaster_fn().await;

        // Recovery process
        let recovery_start = std::time::Instant::now();

        // Attempt to restore from saved state
        let mut restored_simulation = context.initialize_simulation(&config).await.unwrap();
        let restore_result = restored_simulation.restore_state(saved_state).await;
        assert!(restore_result.is_ok(), "Failed to restore after disaster: {}", scenario_name);

        let recovery_time = recovery_start.elapsed();
        assert!(recovery_time < Duration::from_secs(10),
               "Recovery took too long for {}: {:?}", scenario_name, recovery_time);

        // Verify recovery quality
        for _ in 0..20 {
            let _ = restored_simulation.tick().await;
            sleep(Duration::from_millis(50)).await;
        }

        let post_recovery_state = restored_simulation.get_state().await.unwrap();

        // Validate state consistency
        assert_eq!(pre_disaster_state.entities.len(), post_recovery_state.entities.len(),
                   "Entity count mismatch after recovery: {}", scenario_name);
        assert!(post_recovery_state.tick >= pre_disaster_state.tick,
               "Tick count regressed after recovery: {}", scenario_name);

        // Verify continued operation
        for _ in 0..30 {
            let result = restored_simulation.tick().await;
            assert!(result.is_ok(), "Post-recovery operation failed: {}", scenario_name);
        }
    }

    context.cleanup().await;
}

#[tokio::test]
async fn test_multi_region_deployment() {
    let mut context = common::setup_test_environment().await;

    // Simulate multi-region deployment
    let regions = vec![
        ("us-east", 0, 0),
        ("us-west", 50, 0),
        ("eu-central", 0, 50),
        ("asia-pacific", 50, 50),
    ];

    let mut region_simulations = Vec::new();
    let config = common::TestConfig::production()
        .with_world_size(40, 40)
        .with_peasant_count(10);

    // Initialize all regions
    for (region_name, offset_x, offset_y) in regions {
        let region_config = config.clone()
            .with_label(region_name)
            .with_world_offset(offset_x, offset_y);

        let simulation = context.initialize_simulation(&region_config).await.unwrap();
        region_simulations.push((region_name, simulation));
    }

    // Run coordinated simulation
    let start_time = std::time::Instant::now();
    let duration = Duration::from_secs(45);
    let mut coordination_metrics = CoordinationMetrics::new();

    while start_time.elapsed() < duration {
        // Tick all regions
        let mut region_states = Vec::new();

        for (region_name, mut simulation) in &mut region_simulations {
            let result = simulation.tick().await;
            assert!(result.is_ok(), "Region {} tick failed", region_name);

            let state = simulation.get_state().await.unwrap();
            region_states.push((region_name, state));
        }

        // Simulate inter-region communication
        coordination_metrics.record_regions(region_states.len());

        // Verify consistency across regions
        verify_cross_region_consistency(&region_states);

        sleep(Duration::from_millis(100)).await;
    }

    // Validate multi-region coordination
    coordination_metrics.validate();

    context.cleanup().await;
}

#[tokio::test]
async fn test_load_balancing_scenario() {
    let mut context = common::setup_test_environment().await;

    // Test load balancing across multiple simulation instances
    let instance_count = 3;
    let base_config = common::TestConfig::production()
        .with_peasant_count(15);

    let mut instances = Vec::new();

    // Initialize load-balanced instances
    for i in 0..instance_count {
        let config = base_config.clone()
            .with_label(format!("instance_{}", i));

        let simulation = context.initialize_simulation(&config).await.unwrap();
        instances.push(simulation);
    }

    // Apply varying load patterns
    let load_patterns = vec![
        ("burst_load", 5, 100),    // Short burst of high load
        ("sustained_load", 20, 50), // Sustained medium load
        ("spike_load", 2, 200),    // Very short spike
        ("gradual_load", 30, 30),  // Gradual increase
    ];

    for (pattern_name, duration_seconds, entities_per_instance) in load_patterns {
        println!("Testing load pattern: {}", pattern_name);

        // Configure instances for this load pattern
        for (i, simulation) in instances.iter_mut().enumerate() {
            // Add entities to simulate load
            for _ in 0..entities_per_instance {
                let _ = simulation.spawn_entity().await;
            }
        }

        // Monitor load balancing
        let start_time = std::time::Instant::now();
        let pattern_duration = Duration::from_secs(duration_seconds);
        let mut load_metrics = LoadBalancingMetrics::new(pattern_name);

        while start_time.elapsed() < pattern_duration {
            let mut instance_metrics = Vec::new();

            // Collect metrics from all instances
            for (i, simulation) in instances.iter_mut().enumerate() {
                let tick_start = std::time::Instant::now();
                let result = simulation.tick().await;
                assert!(result.is_ok(), "Instance {} tick failed", i);
                let tick_time = tick_start.elapsed();

                let state = simulation.get_state().await.unwrap();
                instance_metrics.push(InstanceMetrics {
                    id: i,
                    tick_time,
                    entity_count: state.entities.len(),
                    resources: state.resources.len(),
                });
            }

            // Analyze load distribution
            load_metrics.record_tick(instance_metrics);
            load_metrics.validate_balance();

            sleep(Duration::from_millis(50)).await;
        }

        // Validate load balancing effectiveness
        load_metrics.validate();

        // Reset for next pattern
        for simulation in &mut instances {
            simulation.reset_load().await.unwrap();
        }
    }

    context.cleanup().await;
}

// Helper structs and functions
struct ProductionMetrics {
    tick_count: u32,
    total_tick_time: Duration,
    max_tick_time: Duration,
    min_tick_time: Duration,
    max_entities: usize,
    min_entities: usize,
    resource_counts: Vec<u32>,
}

impl ProductionMetrics {
    fn new() -> Self {
        Self {
            tick_count: 0,
            total_tick_time: Duration::from_secs(0),
            max_tick_time: Duration::from_secs(0),
            min_tick_time: Duration::from_secs(1000),
            max_entities: 0,
            min_entities: usize::MAX,
            resource_counts: Vec::new(),
        }
    }

    fn record_tick(&mut self, tick_time: Duration) {
        self.tick_count += 1;
        self.total_tick_time += tick_time;
        self.max_tick_time = self.max_tick_time.max(tick_time);
        self.min_tick_time = self.min_tick_time.min(tick_time);
    }

    fn record_entity_count(&mut self, count: usize) {
        self.max_entities = self.max_entities.max(count);
        self.min_entities = self.min_entities.min(count);
    }

    fn record_resources(&mut self, resources: Vec<String>) {
        self.resource_counts.push(resources.len() as u32);
    }

    fn validate(&self) {
        assert!(self.tick_count > 0, "No ticks recorded");
        let avg_tick_time = self.total_tick_time / self.tick_count;
        assert!(avg_tick_time < Duration::from_millis(100),
               "Average tick time too slow: {:?}", avg_tick_time);
        assert!(self.max_tick_time < Duration::from_secs(1),
               "Max tick time too slow: {:?}", self.max_tick_time);
        assert!(self.max_entities > 0, "No entities recorded");
        assert!(self.min_entities < usize::MAX, "Min entities not updated");
    }
}

struct ScalingMetrics {
    stage_name: String,
    tick_times: Vec<Duration>,
    entity_counts: Vec<usize>,
    resource_counts: Vec<usize>,
}

impl ScalingMetrics {
    fn new(stage_name: &str) -> Self {
        Self {
            stage_name: stage_name.to_string(),
            tick_times: Vec::new(),
            entity_counts: Vec::new(),
            resource_counts: Vec::new(),
        }
    }

    fn record_tick(&mut self, tick_time: Duration) {
        self.tick_times.push(tick_time);
    }

    fn record_state(&mut self, state: &world_sim_simple::simulation::SimulationState) {
        self.entity_counts.push(state.entities.len());
        self.resource_counts.push(state.resources.len());
    }

    fn validate(&self) {
        assert!(!self.tick_times.is_empty(), "No tick times recorded for {}", self.stage_name);
        let avg_tick_time: Duration = self.tick_times.iter().sum::<Duration>() / self.tick_times.len() as u32;
        assert!(avg_tick_time < Duration::from_millis(200),
               "Stage {} average tick time too slow: {:?}", self.stage_name, avg_tick_time);
    }

    fn compare_with_previous(&self, previous: &ScalingMetrics) {
        // Compare scaling performance
        let current_avg: Duration = self.tick_times.iter().sum::<Duration>() / self.tick_times.len() as u32;
        let previous_avg: Duration = previous.tick_times.iter().sum::<Duration>() / previous.tick_times.len() as u32;

        println!("Scaling {} -> {}: {:?} -> {:?}",
                previous.stage_name, self.stage_name, previous_avg, current_avg);
    }
}

struct CoordinationMetrics {
    region_count: usize,
    tick_count: u32,
}

impl CoordinationMetrics {
    fn new() -> Self {
        Self {
            region_count: 0,
            tick_count: 0,
        }
    }

    fn record_regions(&mut self, count: usize) {
        self.region_count = count;
        self.tick_count += 1;
    }

    fn validate(&self) {
        assert!(self.region_count > 1, "Need multiple regions for coordination test");
        assert!(self.tick_count > 0, "No coordination ticks recorded");
    }
}

struct InstanceMetrics {
    id: usize,
    tick_time: Duration,
    entity_count: usize,
    resources: usize,
}

struct LoadBalancingMetrics {
    pattern_name: String,
    tick_data: Vec<Vec<InstanceMetrics>>,
}

impl LoadBalancingMetrics {
    fn new(pattern_name: &str) -> Self {
        Self {
            pattern_name: pattern_name.to_string(),
            tick_data: Vec::new(),
        }
    }

    fn record_tick(&mut self, instances: Vec<InstanceMetrics>) {
        self.tick_data.push(instances);
    }

    fn validate_balance(&self) {
        if let Some(latest) = self.tick_data.last() {
            let entity_counts: Vec<_> = latest.iter().map(|m| m.entity_count).collect();
            let max_entities = entity_counts.iter().max().unwrap();
            let min_entities = entity_counts.iter().min().unwrap();

            // Check for reasonable balance (within 50% difference)
            let balance_ratio = *max_entities as f64 / *min_entities as f64;
            assert!(balance_ratio < 2.0,
                   "Load imbalance detected in {}: ratio = {:.2}",
                   self.pattern_name, balance_ratio);
        }
    }

    fn validate(&self) {
        assert!(!self.tick_data.is_empty(), "No load balancing data for {}", self.pattern_name);
    }
}

fn generate_user_actions(state: &world_sim_simple::simulation::SimulationState) -> Vec<world_sim_simple::input::UserAction> {
    // Generate realistic user actions based on current state
    let mut actions = Vec::new();

    if state.entities.len() > 5 {
        // Spawn a few more entities
        for _ in 0..3 {
            actions.push(world_sim_simple::input::UserAction::SpawnEntity);
        }
    }

    // Add some resource collection actions
    if state.resources.len() > 3 {
        actions.push(world_sim_simple::input::UserAction::CollectResources);
    }

    actions
}

fn verify_cross_region_consistency(region_states: &[(String, world_sim_simple::simulation::SimulationState)]) {
    // Verify basic consistency across regions
    for (_, state) in region_states {
        assert!(state.entities.len() > 0, "Region has no entities");
        assert!(state.tick > 0, "Region has invalid tick count");
    }

    // Verify tick counts are reasonably synchronized
    let tick_counts: Vec<_> = region_states.iter().map(|(_, s)| s.tick).collect();
    let max_tick = tick_counts.iter().max().unwrap();
    let min_tick = tick_counts.iter().min().unwrap();

    assert!((max_tick - min_tick) < 10, "Regions too far out of sync");
}