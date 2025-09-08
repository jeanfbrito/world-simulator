//! Priority-based AI processing queue for performance optimization

use bevy_ecs::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

/// AI processing priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AIPriority {
    /// Immediate threats, must process this frame
    Critical = 0,
    /// High priority, process within 2 frames
    High = 1,
    /// Normal thinking, process within 5 frames
    Normal = 2,
    /// Background planning, process when idle
    Low = 3,
}

/// Represents an AI task in the priority queue
#[derive(Clone)]
pub struct AITask {
    pub entity: Entity,
    pub priority: AIPriority,
    pub timestamp: f32,
    pub task_type: AITaskType,
}

#[derive(Clone, Debug)]
pub enum AITaskType {
    UpdateGoap,
    UpdateUtility,
    ProcessSocialAlert,
    PlanPath,
    EvaluateNeeds,
}

impl PartialEq for AITask {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity && self.priority == other.priority
    }
}

impl Eq for AITask {}

impl PartialOrd for AITask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AITask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower priority value = higher priority
        other.priority.cmp(&self.priority)
            .then_with(|| self.timestamp.partial_cmp(&other.timestamp).unwrap())
    }
}

/// Manages AI processing with priority queue
#[derive(Resource)]
pub struct AIProcessingQueue {
    /// Priority queue of AI tasks
    tasks: BinaryHeap<AITask>,
    
    /// Maximum tasks to process per frame
    max_per_frame: usize,
    
    /// Tasks processed this frame
    processed_this_frame: usize,
    
    /// Last update time for each entity
    last_update: HashMap<Entity, f32>,
    
    /// Frame budget in milliseconds
    frame_budget_ms: f32,
}

impl Default for AIProcessingQueue {
    fn default() -> Self {
        Self {
            tasks: BinaryHeap::new(),
            max_per_frame: 50,
            processed_this_frame: 0,
            last_update: HashMap::new(),
            frame_budget_ms: 5.0, // 5ms budget for AI
        }
    }
}

impl AIProcessingQueue {
    /// Queue an AI task with priority
    pub fn queue_task(&mut self, entity: Entity, priority: AIPriority, task_type: AITaskType, current_time: f32) {
        self.tasks.push(AITask {
            entity,
            priority,
            timestamp: current_time,
            task_type,
        });
    }
    
    /// Get next task if within budget
    pub fn get_next_task(&mut self) -> Option<AITask> {
        if self.processed_this_frame >= self.max_per_frame {
            return None;
        }
        
        self.processed_this_frame += 1;
        self.tasks.pop()
    }
    
    /// Reset frame counter
    pub fn new_frame(&mut self) {
        self.processed_this_frame = 0;
    }
    
    /// Check if entity needs update based on priority
    pub fn needs_update(&self, entity: Entity, priority: AIPriority, current_time: f32) -> bool {
        let last = self.last_update.get(&entity).copied().unwrap_or(0.0);
        let max_delay = match priority {
            AIPriority::Critical => 0.0,  // Always update
            AIPriority::High => 0.1,      // 100ms max
            AIPriority::Normal => 0.5,    // 500ms max
            AIPriority::Low => 2.0,       // 2 seconds max
        };
        
        current_time - last > max_delay
    }
    
    /// Mark entity as updated
    pub fn mark_updated(&mut self, entity: Entity, current_time: f32) {
        self.last_update.insert(entity, current_time);
    }
}

/// Component that determines AI processing priority
#[derive(Component)]
pub struct AIPriorityComponent {
    pub base_priority: AIPriority,
    pub current_priority: AIPriority,
    pub last_critical_time: f32,
}

impl Default for AIPriorityComponent {
    fn default() -> Self {
        Self {
            base_priority: AIPriority::Normal,
            current_priority: AIPriority::Normal,
            last_critical_time: 0.0,
        }
    }
}

/// System that manages AI priority based on situation
pub fn update_ai_priority_system(
    mut query: Query<(
        Entity,
        &mut AIPriorityComponent,
        &crate::components::WorkerComponent,
        &crate::components::IsHungry,
        &crate::components::HasEnergy,
        Option<&InCombat>,
        Option<&NearThreat>,
    )>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();
    
    for (entity, mut priority, worker, hunger, energy, combat, threat) in query.iter_mut() {
        // Determine priority based on situation
        priority.current_priority = if combat.is_some() {
            priority.last_critical_time = current_time;
            AIPriority::Critical
        } else if threat.is_some() {
            priority.last_critical_time = current_time;
            AIPriority::Critical
        } else if hunger.0 > 80.0 || energy.0 < 20.0 {
            AIPriority::High
        } else if worker.state == world_sim_interface::WorkerState::Working {
            AIPriority::Normal
        } else {
            AIPriority::Low
        };
        
        // Gradually reduce priority after critical events
        if priority.current_priority != AIPriority::Critical 
            && current_time - priority.last_critical_time < 2.0 {
            priority.current_priority = AIPriority::High;
        }
    }
}

/// System that processes AI tasks from the queue
pub fn process_ai_queue_system(
    mut queue: ResMut<AIProcessingQueue>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let start_time = std::time::Instant::now();
    let current_time = time.elapsed_secs();
    
    queue.new_frame();
    
    // Process tasks until budget exhausted
    while start_time.elapsed().as_secs_f32() * 1000.0 < queue.frame_budget_ms {
        if let Some(task) = queue.get_next_task() {
            // Mark as processed
            queue.mark_updated(task.entity, current_time);
            
            // Queue actual processing
            match task.task_type {
                AITaskType::UpdateGoap => {
                    commands.entity(task.entity).insert(NeedsGoapUpdate);
                }
                AITaskType::UpdateUtility => {
                    commands.entity(task.entity).insert(NeedsUtilityUpdate);
                }
                AITaskType::ProcessSocialAlert => {
                    commands.entity(task.entity).insert(ProcessAlert);
                }
                _ => {}
            }
        } else {
            break;
        }
    }
}

/// Marker components for deferred processing
#[derive(Component)]
pub struct NeedsGoapUpdate;

#[derive(Component)]
pub struct NeedsUtilityUpdate;

#[derive(Component)]
pub struct ProcessAlert;

#[derive(Component)]
pub struct InCombat;

#[derive(Component)]
pub struct NearThreat {
    pub threat_entity: Entity,
    pub distance: f32,
}