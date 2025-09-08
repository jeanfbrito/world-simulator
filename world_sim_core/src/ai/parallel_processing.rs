//! Parallel AI processing using all available CPU cores

use bevy_ecs::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, ComputeTaskPool, TaskPool};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use crate::components::*;

/// Resource for managing parallel AI computation
#[derive(Resource)]
pub struct ParallelAIProcessor {
    /// Number of worker threads for AI
    pub ai_thread_count: usize,
    /// Batch size for parallel processing
    pub batch_size: usize,
    /// Enable SIMD optimizations
    pub use_simd: bool,
    /// Cache line size for alignment
    pub cache_line_size: usize,
}

impl Default for ParallelAIProcessor {
    fn default() -> Self {
        // Use 75% of available cores for AI, leave some for rendering/OS
        let cpu_count = num_cpus::get();
        let ai_threads = (cpu_count as f32 * 0.75).max(2.0) as usize;
        
        Self {
            ai_thread_count: ai_threads,
            batch_size: 64, // Process 64 entities per batch
            use_simd: true,
            cache_line_size: 64, // Standard cache line
        }
    }
}

/// Parallel scorer evaluation using SIMD
pub fn parallel_score_evaluation_system(
    processor: Res<ParallelAIProcessor>,
    pool: Res<ComputeTaskPool>,
    mut scorers: Query<(&mut big_brain::prelude::Score, &crate::components::IsHungry, &crate::components::HasEnergy)>,
) {
    // Collect entities into batches for parallel processing
    let mut batches = Vec::new();
    let mut current_batch = Vec::new();
    
    for (score, hunger, energy) in scorers.iter_mut() {
        current_batch.push((score, hunger.0, energy.0));
        
        if current_batch.len() >= processor.batch_size {
            batches.push(std::mem::take(&mut current_batch));
        }
    }
    if !current_batch.is_empty() {
        batches.push(current_batch);
    }
    
    // Process batches in parallel
    pool.scope(|scope| {
        for mut batch in batches {
            scope.spawn(async move {
                process_score_batch_simd(&mut batch);
            });
        }
    });
}

/// SIMD-optimized batch score processing
#[cfg(target_arch = "x86_64")]
fn process_score_batch_simd(batch: &mut Vec<(Mut<big_brain::prelude::Score>, f64, f64)>) {
    use std::arch::x86_64::*;
    
    // Process 4 scores at a time using AVX
    let chunks = batch.chunks_exact_mut(4);
    
    unsafe {
        for chunk in chunks {
            // Load hunger values into SIMD register
            let hunger = _mm256_set_pd(
                chunk[3].1,
                chunk[2].1,
                chunk[1].1,
                chunk[0].1,
            );
            
            // Load energy values
            let energy = _mm256_set_pd(
                chunk[3].2,
                chunk[2].2,
                chunk[1].2,
                chunk[0].2,
            );
            
            // Calculate combined score: (hunger / 100) * 0.5 + ((100 - energy) / 100) * 0.5
            let hundred = _mm256_set1_pd(100.0);
            let half = _mm256_set1_pd(0.5);
            
            let hunger_score = _mm256_div_pd(hunger, hundred);
            let fatigue = _mm256_sub_pd(hundred, energy);
            let fatigue_score = _mm256_div_pd(fatigue, hundred);
            
            let weighted_hunger = _mm256_mul_pd(hunger_score, half);
            let weighted_fatigue = _mm256_mul_pd(fatigue_score, half);
            
            let final_scores = _mm256_add_pd(weighted_hunger, weighted_fatigue);
            
            // Extract results
            let mut results = [0.0f64; 4];
            _mm256_storeu_pd(results.as_mut_ptr(), final_scores);
            
            // Update scores
            for (i, score) in results.iter().enumerate() {
                chunk[i].0.set(*score as f32);
            }
        }
    }
    
    // Handle remaining entities
    for (mut score, hunger, energy) in batch.chunks_exact_mut(4).remainder() {
        let hunger_score = (*hunger / 100.0) as f32;
        let fatigue_score = ((100.0 - *energy) / 100.0) as f32;
        score.set(hunger_score * 0.5 + fatigue_score * 0.5);
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn process_score_batch_simd(batch: &mut Vec<(Mut<big_brain::prelude::Score>, f64, f64)>) {
    // Fallback for non-x86_64 architectures
    for (mut score, hunger, energy) in batch {
        let hunger_score = (*hunger / 100.0) as f32;
        let fatigue_score = ((100.0 - *energy) / 100.0) as f32;
        score.set(hunger_score * 0.5 + fatigue_score * 0.5);
    }
}

/// Parallel GOAP planning using worker threads
pub fn parallel_goap_planning_system(
    processor: Res<ParallelAIProcessor>,
    pool: Res<AsyncComputeTaskPool>,
    planners: Query<(Entity, &bevy_dogoap::prelude::Planner), Without<PlanningInProgress>>,
    mut commands: Commands,
) {
    // Collect entities that need planning
    let entities_to_plan: Vec<_> = planners.iter()
        .filter(|(_, planner)| planner.needs_planning())
        .map(|(entity, planner)| (entity, planner.clone()))
        .collect();
    
    if entities_to_plan.is_empty() {
        return;
    }
    
    // Distribute planning across worker threads
    let chunk_size = (entities_to_plan.len() / processor.ai_thread_count).max(1);
    
    for chunk in entities_to_plan.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        
        // Spawn async planning task
        let task = pool.spawn(async move {
            let mut results = Vec::new();
            
            for (entity, mut planner) in chunk {
                // Perform planning (expensive operation)
                planner.plan();
                results.push((entity, planner));
            }
            
            results
        });
        
        // Store task for later retrieval
        commands.spawn(PlanningTask { task });
    }
}

/// Component for tracking planning tasks
#[derive(Component)]
pub struct PlanningTask {
    task: bevy_tasks::Task<Vec<(Entity, bevy_dogoap::prelude::Planner)>>,
}

/// Component marking entities with planning in progress
#[derive(Component)]
pub struct PlanningInProgress;

/// System to collect completed planning tasks
pub fn collect_planning_results_system(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut PlanningTask)>,
) {
    for (task_entity, mut task) in tasks.iter_mut() {
        if let Some(results) = futures_lite::future::block_on(
            futures_lite::future::poll_once(&mut task.task)
        ) {
            // Apply planning results
            for (entity, planner) in results {
                commands.entity(entity)
                    .insert(planner)
                    .remove::<PlanningInProgress>();
            }
            
            // Remove completed task
            commands.entity(task_entity).despawn();
        }
    }
}

/// Parallel spatial queries using rayon
pub fn parallel_spatial_query_system(
    processor: Res<ParallelAIProcessor>,
    positions: Query<(Entity, &PositionComponent)>,
    threats: Query<(&PositionComponent, &super::scorers::Threat)>,
    mut alerts: Query<&mut super::social_alerts::ReceivedAlert>,
) {
    // Collect all positions for parallel processing
    let all_positions: Vec<_> = positions.iter().collect();
    let all_threats: Vec<_> = threats.iter().collect();
    
    // Process in parallel using rayon
    let alert_updates: Vec<_> = all_positions
        .par_chunks(processor.batch_size)
        .flat_map(|chunk| {
            let mut updates = Vec::new();
            
            for (entity, pos) in chunk {
                for (threat_pos, threat) in &all_threats {
                    let distance = pos.distance_to(threat_pos);
                    
                    if distance < 30.0 {
                        updates.push((*entity, super::social_alerts::AlertType::Danger(threat.danger_level), distance));
                    }
                }
            }
            
            updates
        })
        .collect();
    
    // Apply updates (must be done sequentially due to ECS constraints)
    for (entity, alert_type, distance) in alert_updates {
        if let Ok(mut alert) = alerts.get_mut(entity) {
            alert.alert_type = alert_type;
            alert.intensity = 1.0 - (distance / 30.0);
        }
    }
}

/// Cache-friendly component layout for better performance
#[repr(C, align(64))] // Align to cache line
#[derive(Component)]
pub struct CacheFriendlyWorkerData {
    // Hot data (accessed frequently)
    pub position_x: f32,
    pub position_y: f32,
    pub hunger: f32,
    pub energy: f32,
    pub health: f32,
    pub state: u8,
    _padding1: [u8; 40], // Pad to 64 bytes
    
    // Cold data (accessed rarely)
    pub name: [u8; 32],
    pub settlement_id: u32,
    pub happiness: f32,
    _padding2: [u8; 24], // Pad to 64 bytes
}

/// Batch update system using cache-friendly layout
pub fn batch_update_worker_needs_system(
    pool: Res<ComputeTaskPool>,
    mut workers: Query<&mut CacheFriendlyWorkerData>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    
    // Process workers in parallel batches
    workers.par_iter_mut().for_each_mut(|mut worker| {
        // Update needs (all data in same cache line)
        worker.hunger = (worker.hunger + delta * 0.01).min(1.0);
        worker.energy = (worker.energy - delta * 0.005).max(0.0);
        
        // Simple state update
        if worker.hunger > 0.8 || worker.energy < 0.2 {
            worker.state = 2; // Critical
        } else if worker.hunger > 0.5 || worker.energy < 0.5 {
            worker.state = 1; // Needs attention
        } else {
            worker.state = 0; // Normal
        }
    });
}

/// Parallel pathfinding using jump point search
pub fn parallel_pathfinding_system(
    processor: Res<ParallelAIProcessor>,
    pool: Res<AsyncComputeTaskPool>,
    pathfinders: Query<(Entity, &PositionComponent, &MovementComponent), Without<PathfindingInProgress>>,
    mut commands: Commands,
) {
    // Collect entities needing paths
    let path_requests: Vec<_> = pathfinders.iter()
        .filter(|(_, _, movement)| movement.target.is_some() && movement.path.is_empty())
        .map(|(entity, pos, movement)| {
            (entity, pos.clone(), movement.target.unwrap())
        })
        .collect();
    
    if path_requests.is_empty() {
        return;
    }
    
    // Process pathfinding in parallel
    let chunk_size = (path_requests.len() / processor.ai_thread_count).max(1);
    
    for chunk in path_requests.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        
        let task = pool.spawn(async move {
            let mut results = Vec::new();
            
            for (entity, start, target) in chunk {
                // Perform pathfinding (expensive)
                let path = calculate_path(&start, &target);
                results.push((entity, path));
            }
            
            results
        });
        
        commands.spawn(PathfindingTask { task });
    }
}

/// Simple pathfinding placeholder
fn calculate_path(start: &PositionComponent, target: &Position) -> Vec<Position> {
    // This would be A* or JPS in real implementation
    vec![
        Position { x: start.x, y: start.y },
        Position { x: target.x, y: target.y },
    ]
}

#[derive(Component)]
pub struct PathfindingTask {
    task: bevy_tasks::Task<Vec<(Entity, Vec<Position>)>>,
}

#[derive(Component)]
pub struct PathfindingInProgress;

/// Memory pool for reducing allocations
#[derive(Resource)]
pub struct MemoryPool {
    position_buffers: Vec<Vec<Position>>,
    score_buffers: Vec<Vec<f32>>,
}

impl Default for MemoryPool {
    fn default() -> Self {
        // Pre-allocate buffers
        Self {
            position_buffers: (0..16).map(|_| Vec::with_capacity(1000)).collect(),
            score_buffers: (0..16).map(|_| Vec::with_capacity(1000)).collect(),
        }
    }
}

/// Extension trait for Planner
trait PlannerExt {
    fn needs_planning(&self) -> bool;
    fn plan(&mut self);
}

impl PlannerExt for bevy_dogoap::prelude::Planner {
    fn needs_planning(&self) -> bool {
        self.current_goal.is_some() && self.current_plan.is_none()
    }
    
    fn plan(&mut self) {
        // Placeholder for actual planning
        self.always_plan = true;
    }
}