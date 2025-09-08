//! Task components for managing entity actions

use bevy_ecs::prelude::*;
use world_sim_interface::{EntityId, Position, ResourceType, BuildingType};
use std::time::Duration;

/// Types of tasks that can be assigned
#[derive(Debug, Clone, PartialEq)]
pub enum TaskType {
    Move(Position),
    Harvest(EntityId),
    Build(BuildingType, Position),
    Store(EntityId),
    Retrieve(EntityId, ResourceType, u32),
    Repair(EntityId),
    Guard(Position),
    Patrol(Vec<Position>),
    Deliver(EntityId, HashMap<ResourceType, u32>),
    Wait(Duration),
}

use std::collections::HashMap;

/// Component for entities with assigned tasks
#[derive(Component, Debug, Clone)]
pub struct TaskComponent {
    pub task_type: TaskType,
    pub priority: u8,
    pub progress: f32,
    pub started_at: Option<u64>,
    pub estimated_duration: Duration,
    pub prerequisites: Vec<TaskType>,
    pub is_interruptible: bool,
}

impl TaskComponent {
    pub fn new(task_type: TaskType) -> Self {
        let (estimated_duration, is_interruptible) = match &task_type {
            TaskType::Move(_) => (Duration::from_secs(5), true),
            TaskType::Harvest(_) => (Duration::from_secs(10), true),
            TaskType::Build(_, _) => (Duration::from_secs(30), false),
            TaskType::Store(_) => (Duration::from_secs(3), true),
            TaskType::Retrieve(_, _, _) => (Duration::from_secs(3), true),
            TaskType::Repair(_) => (Duration::from_secs(15), false),
            TaskType::Guard(_) => (Duration::from_secs(60), true),
            TaskType::Patrol(_) => (Duration::from_secs(120), true),
            TaskType::Deliver(_, _) => (Duration::from_secs(8), true),
            TaskType::Wait(duration) => (*duration, true),
        };
        
        Self {
            task_type,
            priority: 5,
            progress: 0.0,
            started_at: None,
            estimated_duration,
            prerequisites: Vec::new(),
            is_interruptible,
        }
    }
    
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_prerequisites(mut self, prerequisites: Vec<TaskType>) -> Self {
        self.prerequisites = prerequisites;
        self
    }
    
    pub fn start(&mut self, current_tick: u64) {
        self.started_at = Some(current_tick);
        self.progress = 0.0;
    }
    
    pub fn update_progress(&mut self, delta: f32) {
        if self.started_at.is_some() {
            let duration_secs = self.estimated_duration.as_secs_f32();
            if duration_secs > 0.0 {
                self.progress = (self.progress + delta / duration_secs).min(1.0);
            }
        }
    }
    
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }
    
    pub fn can_interrupt(&self) -> bool {
        self.is_interruptible
    }
    
    pub fn reset(&mut self) {
        self.progress = 0.0;
        self.started_at = None;
    }
}

/// Component for task queue management
#[derive(Component, Debug, Clone)]
pub struct TaskQueueComponent {
    pub tasks: Vec<TaskComponent>,
    pub current_task_index: Option<usize>,
    pub max_queue_size: usize,
}

impl TaskQueueComponent {
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            tasks: Vec::new(),
            current_task_index: None,
            max_queue_size,
        }
    }
    
    pub fn queue_task(&mut self, task: TaskComponent) -> bool {
        if self.tasks.len() < self.max_queue_size {
            self.tasks.push(task);
            if self.current_task_index.is_none() && !self.tasks.is_empty() {
                self.current_task_index = Some(0);
            }
            true
        } else {
            false
        }
    }
    
    pub fn insert_priority_task(&mut self, task: TaskComponent) {
        // Find insertion point based on priority
        let insert_pos = self.tasks.iter()
            .position(|t| t.priority < task.priority)
            .unwrap_or(self.tasks.len());
        
        self.tasks.insert(insert_pos, task);
        
        // Adjust current task index if needed
        if let Some(index) = self.current_task_index {
            if insert_pos <= index {
                self.current_task_index = Some(index + 1);
            }
        } else if !self.tasks.is_empty() {
            self.current_task_index = Some(0);
        }
        
        // Remove oldest low-priority task if over capacity
        if self.tasks.len() > self.max_queue_size {
            self.tasks.pop();
        }
    }
    
    pub fn get_current_task(&self) -> Option<&TaskComponent> {
        self.current_task_index.and_then(|i| self.tasks.get(i))
    }
    
    pub fn get_current_task_mut(&mut self) -> Option<&mut TaskComponent> {
        self.current_task_index.and_then(|i| self.tasks.get_mut(i))
    }
    
    pub fn complete_current_task(&mut self) -> Option<TaskComponent> {
        if let Some(index) = self.current_task_index {
            if index < self.tasks.len() {
                let completed = self.tasks.remove(index);
                
                // Update current task index
                if self.tasks.is_empty() {
                    self.current_task_index = None;
                } else if index >= self.tasks.len() {
                    self.current_task_index = Some(0);
                }
                
                return Some(completed);
            }
        }
        None
    }
    
    pub fn cancel_current_task(&mut self) -> bool {
        if let Some(task) = self.get_current_task() {
            if task.can_interrupt() {
                self.complete_current_task();
                return true;
            }
        }
        false
    }
    
    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
        self.current_task_index = None;
    }
    
    pub fn has_tasks(&self) -> bool {
        !self.tasks.is_empty()
    }
    
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
}

/// Component for tracking task execution state
#[derive(Component, Debug, Clone)]
pub struct TaskExecutorComponent {
    pub execution_speed: f32,
    pub skill_multipliers: HashMap<String, f32>,
    pub failures: u32,
    pub completions: u32,
}

impl TaskExecutorComponent {
    pub fn new() -> Self {
        Self {
            execution_speed: 1.0,
            skill_multipliers: HashMap::new(),
            failures: 0,
            completions: 0,
        }
    }
    
    pub fn with_skill(mut self, skill: String, multiplier: f32) -> Self {
        self.skill_multipliers.insert(skill, multiplier);
        self
    }
    
    pub fn get_speed_for_task(&self, task: &TaskType) -> f32 {
        let base_speed = self.execution_speed;
        
        let skill_bonus = match task {
            TaskType::Harvest(_) => self.skill_multipliers.get("harvesting").copied().unwrap_or(1.0),
            TaskType::Build(_, _) => self.skill_multipliers.get("building").copied().unwrap_or(1.0),
            TaskType::Repair(_) => self.skill_multipliers.get("repair").copied().unwrap_or(1.0),
            _ => 1.0,
        };
        
        base_speed * skill_bonus
    }
    
    pub fn record_completion(&mut self) {
        self.completions += 1;
    }
    
    pub fn record_failure(&mut self) {
        self.failures += 1;
    }
    
    pub fn success_rate(&self) -> f32 {
        let total = self.completions + self.failures;
        if total > 0 {
            self.completions as f32 / total as f32
        } else {
            1.0
        }
    }
}