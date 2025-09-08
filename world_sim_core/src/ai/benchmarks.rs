//! AI performance benchmarking and metrics

use bevy_ecs::prelude::*;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Resource for tracking AI performance metrics
#[derive(Resource)]
pub struct AIMetrics {
    /// Total entities processed this frame
    pub entities_processed: AtomicUsize,
    
    /// Time spent on AI this frame (microseconds)
    pub ai_time_us: AtomicU64,
    
    /// Time spent on pathfinding (microseconds)
    pub pathfinding_time_us: AtomicU64,
    
    /// Time spent on GOAP planning (microseconds)
    pub goap_time_us: AtomicU64,
    
    /// Time spent on utility scoring (microseconds)
    pub utility_time_us: AtomicU64,
    
    /// Number of parallel tasks spawned
    pub parallel_tasks: AtomicUsize,
    
    /// Frame time history for averaging
    pub frame_times: Vec<Duration>,
    
    /// Maximum recorded frame time
    pub max_frame_time: Duration,
    
    /// CPU cores utilized
    pub cores_used: usize,
    
    /// Memory used by AI (bytes)
    pub memory_used: AtomicU64,
}

impl Default for AIMetrics {
    fn default() -> Self {
        Self {
            entities_processed: AtomicUsize::new(0),
            ai_time_us: AtomicU64::new(0),
            pathfinding_time_us: AtomicU64::new(0),
            goap_time_us: AtomicU64::new(0),
            utility_time_us: AtomicU64::new(0),
            parallel_tasks: AtomicUsize::new(0),
            frame_times: Vec::with_capacity(120), // Store 2 seconds at 60fps
            max_frame_time: Duration::ZERO,
            cores_used: num_cpus::get(),
            memory_used: AtomicU64::new(0),
        }
    }
}

impl AIMetrics {
    /// Reset per-frame metrics
    pub fn new_frame(&mut self) {
        self.entities_processed.store(0, Ordering::Relaxed);
        self.ai_time_us.store(0, Ordering::Relaxed);
        self.pathfinding_time_us.store(0, Ordering::Relaxed);
        self.goap_time_us.store(0, Ordering::Relaxed);
        self.utility_time_us.store(0, Ordering::Relaxed);
        self.parallel_tasks.store(0, Ordering::Relaxed);
    }
    
    /// Record a frame time
    pub fn record_frame(&mut self, duration: Duration) {
        self.frame_times.push(duration);
        if self.frame_times.len() > 120 {
            self.frame_times.remove(0);
        }
        
        if duration > self.max_frame_time {
            self.max_frame_time = duration;
        }
    }
    
    /// Get average frame time
    pub fn average_frame_time(&self) -> Duration {
        if self.frame_times.is_empty() {
            return Duration::ZERO;
        }
        
        let sum: Duration = self.frame_times.iter().sum();
        sum / self.frame_times.len() as u32
    }
    
    /// Get current FPS based on average frame time
    pub fn fps(&self) -> f32 {
        let avg = self.average_frame_time();
        if avg.as_secs_f32() > 0.0 {
            1.0 / avg.as_secs_f32()
        } else {
            0.0
        }
    }
    
    /// Get AI time as percentage of frame
    pub fn ai_percentage(&self) -> f32 {
        let frame_us = self.average_frame_time().as_micros() as f32;
        let ai_us = self.ai_time_us.load(Ordering::Relaxed) as f32;
        
        if frame_us > 0.0 {
            (ai_us / frame_us) * 100.0
        } else {
            0.0
        }
    }
}

/// Macro for timing a block of code
#[macro_export]
macro_rules! time_ai_operation {
    ($metrics:expr, $field:ident, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let elapsed = start.elapsed().as_micros() as u64;
        $metrics.$field.fetch_add(elapsed, std::sync::atomic::Ordering::Relaxed);
        result
    }};
}

/// System to update metrics
pub fn update_metrics_system(
    mut metrics: ResMut<AIMetrics>,
    time: Res<Time>,
    entities: Query<Entity, Or<(
        With<crate::components::WorkerComponent>,
        With<super::scorers::Threat>,
    )>>,
) {
    let frame_time = time.delta();
    metrics.record_frame(frame_time);
    
    let entity_count = entities.iter().count();
    metrics.entities_processed.store(entity_count, Ordering::Relaxed);
    
    // Estimate memory usage (rough)
    let memory_per_entity = 2048; // 2KB per entity estimate
    let total_memory = entity_count * memory_per_entity;
    metrics.memory_used.store(total_memory as u64, Ordering::Relaxed);
}

/// System to display metrics
pub fn display_metrics_system(
    metrics: Res<AIMetrics>,
    time: Res<Time>,
) {
    static mut LAST_DISPLAY: f32 = 0.0;
    
    unsafe {
        LAST_DISPLAY += time.delta_secs();
        if LAST_DISPLAY < 1.0 {
            return;
        }
        LAST_DISPLAY = 0.0;
    }
    
    println!("\n=== AI Performance Metrics ===");
    println!("FPS: {:.1}", metrics.fps());
    println!("Entities: {}", metrics.entities_processed.load(Ordering::Relaxed));
    println!("AI Time: {:.2}ms ({:.1}% of frame)", 
        metrics.ai_time_us.load(Ordering::Relaxed) as f32 / 1000.0,
        metrics.ai_percentage()
    );
    println!("  - GOAP: {:.2}ms", 
        metrics.goap_time_us.load(Ordering::Relaxed) as f32 / 1000.0
    );
    println!("  - Utility: {:.2}ms", 
        metrics.utility_time_us.load(Ordering::Relaxed) as f32 / 1000.0
    );
    println!("  - Pathfinding: {:.2}ms", 
        metrics.pathfinding_time_us.load(Ordering::Relaxed) as f32 / 1000.0
    );
    println!("Parallel Tasks: {}", metrics.parallel_tasks.load(Ordering::Relaxed));
    println!("CPU Cores Used: {}/{}", metrics.cores_used, num_cpus::get());
    println!("Memory (est): {:.1} MB", 
        metrics.memory_used.load(Ordering::Relaxed) as f32 / 1_048_576.0
    );
    println!("==============================");
}

/// Benchmark spawn system for stress testing
pub fn spawn_benchmark_entities(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    entities: Query<Entity, With<crate::components::WorkerComponent>>,
) {
    // Press B to spawn 100 workers
    if keyboard.just_pressed(KeyCode::KeyB) {
        println!("Spawning 100 benchmark workers...");
        
        for i in 0..100 {
            let x = (i % 10) * 10;
            let y = (i / 10) * 10;
            
            super::spawn_hybrid_worker(
                &mut commands,
                crate::Position { x, y },
                format!("BenchWorker_{}", i),
            );
        }
        
        println!("Total workers: {}", entities.iter().count() + 100);
    }
    
    // Press N to spawn 1000 workers
    if keyboard.just_pressed(KeyCode::KeyN) {
        println!("STRESS TEST: Spawning 1000 workers!");
        
        for i in 0..1000 {
            let x = (i % 32) * 10;
            let y = (i / 32) * 10;
            
            // Use simpler AI for mass spawning
            commands.spawn((
                crate::components::WorkerComponent::new(format!("MassWorker_{}", i)),
                crate::components::PositionComponent::new(x, y),
                super::lod_system::LODComponent {
                    complexity: super::lod_system::AIComplexity::Simple,
                    ..default()
                },
            ));
        }
        
        println!("Total workers: {}", entities.iter().count() + 1000);
    }
}

/// Performance comparison system
pub fn compare_performance_system(
    metrics: Res<AIMetrics>,
    mut comparison: Local<PerformanceComparison>,
) {
    comparison.update(&metrics);
}

#[derive(Default)]
struct PerformanceComparison {
    baseline_fps: Option<f32>,
    baseline_entities: Option<usize>,
    current_fps: f32,
    current_entities: usize,
}

impl PerformanceComparison {
    fn update(&mut self, metrics: &AIMetrics) {
        self.current_fps = metrics.fps();
        self.current_entities = metrics.entities_processed.load(Ordering::Relaxed);
        
        // Set baseline on first run
        if self.baseline_fps.is_none() && self.current_fps > 0.0 {
            self.baseline_fps = Some(self.current_fps);
            self.baseline_entities = Some(self.current_entities);
            println!("Performance baseline set: {:.1} FPS with {} entities", 
                self.current_fps, self.current_entities);
        }
        
        // Compare if we have significantly more entities
        if let (Some(base_fps), Some(base_entities)) = (self.baseline_fps, self.baseline_entities) {
            if self.current_entities > base_entities * 2 {
                let scaling_factor = self.current_entities as f32 / base_entities as f32;
                let fps_ratio = self.current_fps / base_fps;
                let efficiency = fps_ratio / (1.0 / scaling_factor);
                
                println!("Performance Scaling: {}x entities, {:.1}% FPS retained, {:.1}% efficiency",
                    scaling_factor, fps_ratio * 100.0, efficiency * 100.0);
            }
        }
    }
}

/// Component for marking benchmark entities
#[derive(Component)]
pub struct BenchmarkEntity;

use bevy_input::keyboard::KeyCode;
use bevy_input::ButtonInput;