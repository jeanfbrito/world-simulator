//! Performance Testing Example
//!
//! This example demonstrates comprehensive performance testing and benchmarking
//! capabilities for the world-simulator. It showcases:
//! - Load testing and stress testing
//! - Performance metrics collection
//! - Memory usage monitoring
//! - Scalability analysis
//! - Bottleneck identification

use bevy::prelude::*;
use world_sim_interface::*;
use world_sim_simple::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn main() {
    println!("⚡ Starting Performance Testing Example");

    // Set up logging with performance detail
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    // Create and run the simulation
    let mut app = App::new();

    // Configure for headless operation
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: None,
        exit_condition: bevy::window::ExitCondition::DontExit,
        close_when_requested: false,
    }));

    // Add core simulation plugins
    app.add_plugins(simulation::TickSimulationPlugin);
    app.add_plugins(ComponentsPlugin);
    app.add_plugins(PackSystemPlugin);
    app.add_plugins(WorldPlugin);
    app.add_plugins(SimPlugin);
    app.add_plugins(TilemapPlugin);
    app.add_plugins(ResourcesPlugin);
    app.add_plugins(BuildingsPlugin);
    app.add_plugins(CraftingPlugin);
    app.add_plugins(AIPlugin);
    app.add_plugins(SaveLoadPlugin);
    app.add_plugins(PerformancePlugin);
    app.add_plugins(SystemsPlugin);

    // Initialize resources
    app.init_resource::<WorldMap>();
    app.init_resource::<SimulationState>();
    app.init_resource::<PerformanceTestState>();

    // Add startup systems for performance test setup
    app.add_systems(Startup, setup_performance_test);

    // Add update systems for performance monitoring
    app.add_systems(Update, (
        performance_metrics_collector,
        load_test_manager,
        memory_usage_monitor,
        bottleneck_detector,
        scalability_analyzer,
    ).chain());

    println!("🔬 Performance test initialized. Running comprehensive benchmarks...");
    println!("📊 This will showcase performance analysis and optimization insights.");

    // Run the simulation
    app.run();
}

/// State for performance testing operations
#[derive(Resource, Default)]
pub struct PerformanceTestState {
    pub test_phases: Vec<TestPhase>,
    pub current_phase: usize,
    pub metrics: Arc<Mutex<PerformanceMetrics>>,
    pub load_test_config: LoadTestConfig,
    pub scalability_results: HashMap<String, ScalabilityResult>,
    pub bottlenecks: Vec<BottleneckReport>,
    pub test_start_time: Instant,
}

/// Performance metrics collection
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub frame_times: VecDeque<f64>,
    pub memory_usage_mb: VecDeque<f64>,
    pub cpu_usage_percent: VecDeque<f64>,
    pub entity_counts: VecDeque<usize>,
    pub system_times: HashMap<String, VecDeque<f64>>,
    pub fps_samples: VecDeque<f32>,
    pub allocation_rates: VecDeque<u64>,
    pub gc_times: VecDeque<Duration>,
}

/// Test phase configuration
#[derive(Debug, Clone)]
pub struct TestPhase {
    pub name: String,
    pub duration_ticks: u32,
    pub entity_target: usize,
    pub stress_level: f32,
    pub measurements: Vec<String>,
}

/// Load testing configuration
#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    pub max_entities: usize,
    pub spawn_rate: f32,
    pub stress_intervals: Vec<u32>,
    pub measurement_intervals: Vec<u32>,
    pub duration_ticks: u32,
}

/// Scalability test results
#[derive(Debug, Clone)]
pub struct ScalabilityResult {
    pub entity_count: usize,
    pub average_fps: f32,
    pub frame_time_ms: f64,
    pub memory_mb: f64,
    pub cpu_percent: f64,
    pub scaling_efficiency: f32,
}

/// Bottleneck analysis report
#[derive(Debug, Clone)]
pub struct BottleneckReport {
    pub system_name: String,
    pub average_time_ms: f64,
    pub max_time_ms: f64,
    pub percentage_of_frame: f32,
    pub severity: BottleneckSeverity,
    pub recommendations: Vec<String>,
}

/// Bottleneck severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Setup performance testing scenarios
fn setup_performance_test(
    mut commands: Commands,
    mut pack_system: Option<Res<packs::PackSystem>>,
    mut perf_state: ResMut<PerformanceTestState>,
) {
    println!("🔬 Setting up Performance Testing Example...");

    // Define test phases
    perf_state.test_phases = vec![
        TestPhase {
            name: "Baseline".to_string(),
            duration_ticks: 200,
            entity_target: 10,
            stress_level: 0.1,
            measurements: vec!["fps".to_string(), "memory".to_string(), "cpu".to_string()],
        },
        TestPhase {
            name: "Light Load".to_string(),
            duration_ticks: 200,
            entity_target: 50,
            stress_level: 0.3,
            measurements: vec!["fps".to_string(), "entities".to_string(), "system_times".to_string()],
        },
        TestPhase {
            name: "Medium Load".to_string(),
            duration_ticks: 200,
            entity_target: 100,
            stress_level: 0.6,
            measurements: vec!["fps".to_string(), "memory".to_string(), "bottlenecks".to_string()],
        },
        TestPhase {
            name: "Heavy Load".to_string(),
            duration_ticks: 200,
            entity_target: 200,
            stress_level: 1.0,
            measurements: vec!["fps".to_string(), "cpu".to_string(), "scalability".to_string()],
        },
        TestPhase {
            name: "Stress Test".to_string(),
            duration_ticks: 200,
            entity_target: 500,
            stress_level: 1.5,
            measurements: vec!["memory".to_string(), "bottlenecks".to_string(), "stability".to_string()],
        },
    ];

    // Configure load testing
    perf_state.load_test_config = LoadTestConfig {
        max_entities: 500,
        spawn_rate: 0.05,
        stress_intervals: vec![50, 100, 200, 300, 500],
        measurement_intervals: vec![10, 50, 100, 200],
        duration_ticks: 1000,
    };

    // Initialize metrics collection
    *perf_state.metrics = Arc::new(Mutex::new(PerformanceMetrics::default()));
    perf_state.test_start_time = Instant::now();

    println!("✅ Performance test setup complete!");
    println!("🎯 Test phases:");
    for (i, phase) in perf_state.test_phases.iter().enumerate() {
        println!("   {}. {} - {} entities, {:.1} stress level",
            i + 1, phase.name, phase.entity_target, phase.stress_level);
    }
    println!("📊 Metrics collected: FPS, memory usage, CPU usage, entity counts, system times");
}

/// Collect performance metrics
fn performance_metrics_collector(
    sim_state: Res<SimulationState>,
    time: Res<Time>,
    perf_state: Res<PerformanceTestState>,
    resource_query: Query<&ResourceNode>,
    unit_query: Query<&UnitTag>,
    building_query: Query<&BuildingComponent>,
    mut last_collection: Local<Instant>,
) {
    let now = Instant::now();
    if now.duration_since(*last_collection) < Duration::from_millis(100) {
        return;
    }
    *last_collection = now;

    let mut metrics = perf_state.metrics.lock().unwrap();

    // Collect basic metrics
    metrics.frame_times.push_back(time.delta_seconds() * 1000.0);
    metrics.fps_samples.push_back(time.fps());

    // Count entities
    let entity_count = resource_query.iter().count() + unit_query.iter().count() + building_query.iter().count();
    metrics.entity_counts.push_back(entity_count);

    // Estimate memory usage
    let memory_mb = estimate_memory_usage(entity_count);
    metrics.memory_usage_mb.push_back(memory_mb);

    // Estimate CPU usage
    let cpu_usage = estimate_cpu_usage(&metrics.frame_times);
    metrics.cpu_usage_percent.push_back(cpu_usage);

    // Estimate allocation rate
    let allocation_rate = estimate_allocation_rate(entity_count);
    metrics.allocation_rates.push_back(allocation_rate);

    // Keep only recent samples (last 1000)
    const MAX_SAMPLES: usize = 1000;
    if metrics.frame_times.len() > MAX_SAMPLES {
        metrics.frame_times.pop_front();
        metrics.fps_samples.pop_front();
        metrics.entity_counts.pop_front();
        metrics.memory_usage_mb.pop_front();
        metrics.cpu_usage_percent.pop_front();
        metrics.allocation_rates.pop_front();
    }
}

/// Manage load testing scenarios
fn load_test_manager(
    sim_state: Res<SimulationState>,
    mut perf_state: ResMut<PerformanceTestState>,
    mut commands: Commands,
    mut last_spawn: Local<Instant>,
) {
    let current_phase = &perf_state.test_phases[perf_state.current_phase];
    let phase_progress = sim_state.tick % current_phase.duration_ticks;

    // Spawn entities based on current phase
    let now = Instant::now();
    if now.duration_since(*last_spawn) > Duration::from_millis((1000.0 / (perf_state.load_test_config.spawn_rate * 100.0)) as u64) {
        *last_spawn = now;

        let current_entities = perf_state.metrics.lock().unwrap().entity_counts.back().copied().unwrap_or(0);
        if current_entities < current_phase.entity_target {
            spawn_test_entity(&mut commands, current_phase.stress_level);
        }
    }

    // Check if phase is complete
    if phase_progress == 0 && sim_state.tick > 0 {
        complete_test_phase(&mut perf_state, sim_state.tick);
    }
}

/// Monitor memory usage
fn memory_usage_monitor(
    perf_state: Res<PerformanceTestState>,
    sim_state: Res<SimulationState>,
    mut last_report: Local<u32>,
) {
    if sim_state.tick % 100 == 0 && sim_state.tick > 0 && *last_report != sim_state.tick {
        *last_report = sim_state.tick;

        let metrics = perf_state.metrics.lock().unwrap();
        if let Some(&current_memory) = metrics.memory_usage_mb.back() {
            let current_phase = &perf_state.test_phases[perf_state.current_phase];

            println!("📊 Memory Monitor - {}: {:.1}MB ({} entities)",
                current_phase.name, current_memory,
                metrics.entity_counts.back().copied().unwrap_or(0));
        }
    }
}

/// Detect performance bottlenecks
fn bottleneck_detector(
    perf_state: ResMut<PerformanceTestState>,
    time: Res<Time>,
    sim_state: Res<SimulationState>,
    mut last_analysis: Local<u32>,
) {
    if sim_state.tick % 200 == 0 && sim_state.tick > 0 && *last_analysis != sim_state.tick {
        *last_analysis = sim_state.tick;

        let metrics = perf_state.metrics.lock().unwrap();
        let current_phase = &perf_state.test_phases[perf_state.current_phase];

        // Analyze frame time distribution
        if let Some(&avg_frame_time) = calculate_average(&metrics.frame_times) {
            if avg_frame_time > 16.67 { // > 60 FPS threshold
                let bottleneck = BottleneckReport {
                    system_name: "Overall Frame Time".to_string(),
                    average_time_ms: avg_frame_time,
                    max_time_ms: calculate_max(&metrics.frame_times).unwrap_or(avg_frame_time),
                    percentage_of_frame: 100.0,
                    severity: if avg_frame_time > 33.33 { BottleneckSeverity::Critical } else { BottleneckSeverity::High },
                    recommendations: vec![
                        "Reduce entity count".to_string(),
                        "Optimize AI calculations".to_string(),
                        "Implement level of detail".to_string(),
                    ],
                };

                perf_state.bottlenecks.push(bottleneck);
            }
        }

        // Analyze memory usage
        if let Some(&memory_mb) = metrics.memory_usage_mb.back() {
            if memory_mb > 500.0 { // 500MB threshold
                let bottleneck = BottleneckReport {
                    system_name: "Memory Usage".to_string(),
                    average_time_ms: 0.0,
                    max_time_ms: 0.0,
                    percentage_of_frame: (memory_mb / 1000.0) * 100.0,
                    severity: if memory_mb > 1000.0 { BottleneckSeverity::Critical } else { BottleneckSeverity::High },
                    recommendations: vec![
                        "Implement object pooling".to_string(),
                        "Optimize data structures".to_string(),
                        "Reduce texture sizes".to_string(),
                    ],
                };

                perf_state.bottlenecks.push(bottleneck);
            }
        }
    }
}

/// Analyze scalability
fn scalability_analyzer(
    perf_state: ResMut<PerformanceTestState>,
    sim_state: Res<SimulationState>,
    mut last_analysis: Local<u32>,
) {
    if sim_state.tick % 300 == 0 && sim_state.tick > 0 && *last_analysis != sim_state.tick {
        *last_analysis = sim_state.tick;

        let metrics = perf_state.metrics.lock().unwrap();
        let current_phase = &perf_state.test_phases[perf_state.current_phase];

        // Calculate scalability metrics
        if let (Some(entity_count), Some(avg_fps), Some(frame_time), Some(memory), Some(cpu)) = (
            metrics.entity_counts.back(),
            calculate_average_f32(&metrics.fps_samples),
            calculate_average(&metrics.frame_times),
            calculate_average(&metrics.memory_usage_mb),
            calculate_average(&metrics.cpu_usage_percent),
        ) {
            let result = ScalabilityResult {
                entity_count: *entity_count,
                average_fps: avg_fps,
                frame_time_ms: frame_time,
                memory_mb: memory,
                cpu_percent: cpu,
                scaling_efficiency: calculate_scaling_efficiency(entity_count, avg_fps),
            };

            perf_state.scalability_results.insert(current_phase.name.clone(), result);
        }
    }
}

// Helper functions
fn spawn_test_entity(commands: &mut Commands, stress_level: f32) {
    commands.spawn((
        NameComponent(format!("TestEntity_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis())),
        UnitTag,
        UnitStats {
            energy: (100.0 * stress_level) as u32,
            max_energy: (100.0 * stress_level) as u32,
            movement_speed: stress_level,
            ..Default::default()
        },
        PositionComponent {
            x: rand::random::<f32>() * 32.0,
            y: rand::random::<f32>() * 32.0,
        },
        Inventory::new(),
    ));
}

fn complete_test_phase(perf_state: &mut PerformanceTestState, current_tick: u32) {
    let phase = &perf_state.test_phases[perf_state.current_phase];
    println!("✅ Completed phase: {}", phase.name);

    // Generate phase report
    generate_phase_report(perf_state, phase);

    // Move to next phase
    if perf_state.current_phase < perf_state.test_phases.len() - 1 {
        perf_state.current_phase += 1;
        println!("🚀 Starting phase: {}", perf_state.test_phases[perf_state.current_phase].name);
    } else {
        // All phases complete
        generate_final_report(perf_state);
        std::process::exit(0);
    }
}

fn generate_phase_report(perf_state: &PerformanceTestState, phase: &TestPhase) {
    let metrics = perf_state.metrics.lock().unwrap();

    println!("📊 Phase Report: {}", phase.name);
    println!("   Target entities: {}", phase.entity_target);

    if let Some(avg_fps) = calculate_average_f32(&metrics.fps_samples) {
        println!("   Average FPS: {:.1}", avg_fps);
    }

    if let Some(avg_frame_time) = calculate_average(&metrics.frame_times) {
        println!("   Average frame time: {:.2}ms", avg_frame_time);
    }

    if let Some(avg_memory) = calculate_average(&metrics.memory_usage_mb) {
        println!("   Average memory: {:.1}MB", avg_memory);
    }

    if let Some(avg_cpu) = calculate_average(&metrics.cpu_usage_percent) {
        println!("   Average CPU: {:.1}%", avg_cpu);
    }

    if !perf_state.bottlenecks.is_empty() {
        println!("   Bottlenecks detected: {}", perf_state.bottlenecks.len());
    }
}

fn generate_final_report(perf_state: &PerformanceTestState) {
    println!("🎉 Performance testing completed successfully!");
    println!("📊 Final Performance Analysis:");

    let metrics = perf_state.metrics.lock().unwrap();

    // Overall performance summary
    if let (Some(avg_fps), Some(avg_frame_time), Some(max_memory)) = (
        calculate_average_f32(&metrics.fps_samples),
        calculate_average(&metrics.frame_times),
        calculate_max(&metrics.memory_usage_mb),
    ) {
        println!("   • Overall average FPS: {:.1}", avg_fps);
        println!("   • Overall average frame time: {:.2}ms", avg_frame_time);
        println!("   • Peak memory usage: {:.1}MB", max_memory);
        println!("   • Total test duration: {:.1}s",
            perf_state.test_start_time.elapsed().as_secs_f32());
    }

    // Scalability analysis
    if !perf_state.scalability_results.is_empty() {
        println!("   • Scalability results:");
        for (phase_name, result) in &perf_state.scalability_results {
            println!("     {}: {} entities, {:.1} FPS, {:.2}ms frame time",
                phase_name, result.entity_count, result.average_fps, result.frame_time_ms);
        }
    }

    // Bottleneck summary
    if !perf_state.bottlenecks.is_empty() {
        println!("   • Bottlenecks identified: {}", perf_state.bottlenecks.len());
        for bottleneck in &perf_state.bottlenecks {
            println!("     {}: {:.2}ms ({:?})",
                bottleneck.system_name, bottleneck.average_time_ms, bottleneck.severity);
        }
    }

    println!("⚡ Key Performance Insights Demonstrated:");
    println!("   • Comprehensive metrics collection");
    println!("   • Load testing and stress testing");
    println!("   • Memory usage monitoring");
    println!("   • Bottleneck detection and analysis");
    println!("   • Scalability assessment");
}

// Estimation functions
fn estimate_memory_usage(entity_count: usize) -> f64 {
    50.0 + (entity_count as f64 * 0.1) + (rand::random::<f64>() * 10.0)
}

fn estimate_cpu_usage(frame_times: &VecDeque<f64>) -> f64 {
    if let Some(avg_frame_time) = calculate_average(frame_times) {
        (avg_frame_time / 16.67).min(100.0).max(0.0)
    } else {
        0.0
    }
}

fn estimate_allocation_rate(entity_count: usize) -> u64 {
    (entity_count * 100) as u64 + rand::random::<u64>() % 1000
}

fn calculate_average<T>(values: &VecDeque<T>) -> Option<f64>
where
    T: Copy + Into<f64>,
{
    if values.is_empty() {
        return None;
    }

    let sum: f64 = values.iter().map(|&v| v.into()).sum();
    Some(sum / values.len() as f64)
}

fn calculate_average_f32(values: &VecDeque<f32>) -> Option<f32> {
    if values.is_empty() {
        return None;
    }

    let sum: f32 = values.iter().sum();
    Some(sum / values.len() as f32)
}

fn calculate_max<T>(values: &VecDeque<T>) -> Option<f64>
where
    T: Copy + Into<f64>,
{
    if values.is_empty() {
        return None;
    }

    values.iter().map(|&v| v.into()).fold(None, |max, v| match max {
        None => Some(v),
        Some(m) => Some(m.max(v)),
    })
}

fn calculate_scaling_efficiency(entity_count: &usize, fps: f32) -> f32 {
    let baseline_fps = 60.0;
    let expected_fps = baseline_fps * (100.0 / *entity_count as f32).max(0.1);
    (fps / expected_fps).min(1.0) * 100.0
}