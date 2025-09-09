use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    Harvest,
    Build,
    Craft,
    Move,
    Deliver,
    Guard,
    Repair,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Assigned,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub target_position: Option<Vec3>,
    pub target_entity: Option<Entity>,
    pub progress: f32,
    pub required_time: f32,
}

impl Task {
    pub fn new(id: usize, task_type: TaskType) -> Self {
        let required_time = match task_type {
            TaskType::Harvest => 5.0,
            TaskType::Build => 10.0,
            TaskType::Craft => 4.0,
            TaskType::Move => 2.0,
            TaskType::Deliver => 3.0,
            TaskType::Guard => 20.0,
            TaskType::Repair => 6.0,
        };
        
        Self {
            id,
            task_type,
            priority: TaskPriority::Normal,
            status: TaskStatus::Pending,
            target_position: None,
            target_entity: None,
            progress: 0.0,
            required_time,
        }
    }
    
    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_target_position(mut self, position: Vec3) -> Self {
        self.target_position = Some(position);
        self
    }
    
    pub fn with_target_entity(mut self, entity: Entity) -> Self {
        self.target_entity = Some(entity);
        self
    }
    
    pub fn update(&mut self, delta_time: f32) -> bool {
        if self.status != TaskStatus::InProgress {
            return false;
        }
        
        self.progress += delta_time / self.required_time;
        
        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.status = TaskStatus::Completed;
            info!("[TASK] Task {} ({:?}) completed", self.id, self.task_type);
            return true;
        }
        
        false
    }
    
    pub fn start(&mut self) {
        self.status = TaskStatus::InProgress;
        info!("[TASK] Task {} ({:?}) started", self.id, self.task_type);
    }
    
    pub fn cancel(&mut self) {
        self.status = TaskStatus::Cancelled;
        info!("[TASK] Task {} ({:?}) cancelled", self.id, self.task_type);
    }
}

#[derive(Resource)]
pub struct TaskSystem {
    next_id: usize,
    pending_tasks: VecDeque<Task>,
    active_tasks: Vec<Task>,
    completed_tasks: Vec<Task>,
}

impl Default for TaskSystem {
    fn default() -> Self {
        Self {
            next_id: 0,
            pending_tasks: VecDeque::new(),
            active_tasks: Vec::new(),
            completed_tasks: Vec::new(),
        }
    }
}

impl TaskSystem {
    pub fn new() -> Self {
        info!("[TASK] Task system initialized");
        Self::default()
    }
    
    pub fn create_task(&mut self, task_type: TaskType) -> Task {
        let task = Task::new(self.next_id, task_type);
        self.next_id += 1;
        info!("[TASK] Created task {} ({:?})", task.id, task_type);
        task
    }
    
    pub fn add_task(&mut self, task: Task) {
        let task_id = task.id;
        let task_priority = task.priority;
        
        match task.priority {
            TaskPriority::Critical => {
                // Add to front for critical tasks
                self.pending_tasks.push_front(task);
            }
            TaskPriority::High => {
                // Find position after critical tasks
                let pos = self.pending_tasks.iter()
                    .position(|t| t.priority < TaskPriority::High)
                    .unwrap_or(self.pending_tasks.len());
                self.pending_tasks.insert(pos, task);
            }
            _ => {
                // Add to back for normal and low priority
                self.pending_tasks.push_back(task);
            }
        }
        
        info!("[TASK] Added task {} to queue (priority: {:?})", 
            task_id, task_priority);
    }
    
    pub fn assign_next_task(&mut self) -> Option<Task> {
        if let Some(mut task) = self.pending_tasks.pop_front() {
            task.status = TaskStatus::Assigned;
            let task_id = task.id;
            let task_type = task.task_type;
            self.active_tasks.push(task.clone());
            info!("[TASK] Assigned task {} ({:?})", task_id, task_type);
            Some(task)
        } else {
            None
        }
    }
    
    pub fn complete_task(&mut self, task_id: usize) {
        if let Some(pos) = self.active_tasks.iter().position(|t| t.id == task_id) {
            let mut task = self.active_tasks.remove(pos);
            task.status = TaskStatus::Completed;
            self.completed_tasks.push(task);
            info!("[TASK] Task {} marked as completed", task_id);
        }
    }
    
    pub fn fail_task(&mut self, task_id: usize) {
        if let Some(pos) = self.active_tasks.iter().position(|t| t.id == task_id) {
            let mut task = self.active_tasks.remove(pos);
            task.status = TaskStatus::Failed;
            
            // Optionally re-queue failed tasks
            if task.priority >= TaskPriority::High {
                task.status = TaskStatus::Pending;
                task.progress = 0.0;
                self.add_task(task);
                info!("[TASK] High priority task {} failed and re-queued", task_id);
            } else {
                info!("[TASK] Task {} failed", task_id);
            }
        }
    }
    
    pub fn cancel_task(&mut self, task_id: usize) -> bool {
        // Check pending tasks
        if let Some(pos) = self.pending_tasks.iter().position(|t| t.id == task_id) {
            self.pending_tasks.remove(pos);
            info!("[TASK] Cancelled pending task {}", task_id);
            return true;
        }
        
        // Check active tasks
        if let Some(pos) = self.active_tasks.iter().position(|t| t.id == task_id) {
            let mut task = self.active_tasks.remove(pos);
            task.cancel();
            return true;
        }
        
        false
    }
    
    pub fn pending_count(&self) -> usize {
        self.pending_tasks.len()
    }
    
    pub fn active_count(&self) -> usize {
        self.active_tasks.len()
    }
    
    pub fn completed_count(&self) -> usize {
        self.completed_tasks.len()
    }
}