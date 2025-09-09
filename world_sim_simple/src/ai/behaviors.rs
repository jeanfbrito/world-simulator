use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::debug::{DebugSystem, DebugLevel};
use super::{Task, TaskType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AIBehavior {
    Idle,
    Moving,
    Working,
    Harvesting,
    Building,
    Crafting,
    Resting,
    Fleeing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BehaviorState {
    Starting,
    InProgress,
    Completed,
    Failed,
}

#[derive(Component, Debug, Clone)]
pub struct WorkerAI {
    pub behavior: AIBehavior,
    pub state: BehaviorState,
    pub current_task: Option<Task>,
    pub target_position: Option<Vec3>,
    pub work_progress: f32,
    pub rest_timer: f32,
    pub decision_timer: f32,
}

impl Default for WorkerAI {
    fn default() -> Self {
        Self {
            behavior: AIBehavior::Idle,
            state: BehaviorState::Completed,
            current_task: None,
            target_position: None,
            work_progress: 0.0,
            rest_timer: 0.0,
            decision_timer: 0.0,
        }
    }
}

impl WorkerAI {
    pub fn new() -> Self {
        info!("[AI] Created new worker AI");
        Self::default()
    }
    
    pub fn update(&mut self, delta_time: f32, transform: &mut Transform, debug: &DebugSystem) {
        self.decision_timer -= delta_time;
        
        if self.decision_timer <= 0.0 {
            self.make_decision(debug);
            self.decision_timer = 1.0; // Make decisions every second
        }
        
        match self.behavior {
            AIBehavior::Idle => self.idle_behavior(delta_time, debug),
            AIBehavior::Moving => self.move_behavior(delta_time, transform, debug),
            AIBehavior::Working => self.work_behavior(delta_time, debug),
            AIBehavior::Harvesting => self.harvest_behavior(delta_time, debug),
            AIBehavior::Building => self.build_behavior(delta_time, debug),
            AIBehavior::Crafting => self.craft_behavior(delta_time, debug),
            AIBehavior::Resting => self.rest_behavior(delta_time, debug),
            AIBehavior::Fleeing => self.flee_behavior(delta_time, transform, debug),
        }
    }
    
    fn make_decision(&mut self, debug: &DebugSystem) {
        // Check if we have a task
        if let Some(task) = &self.current_task {
            if self.state == BehaviorState::Completed || self.state == BehaviorState::Failed {
                // Task complete, start new behavior based on task type
                self.behavior = match task.task_type {
                    TaskType::Harvest => AIBehavior::Harvesting,
                    TaskType::Build => AIBehavior::Building,
                    TaskType::Craft => AIBehavior::Crafting,
                    TaskType::Move => AIBehavior::Moving,
                    TaskType::Deliver => AIBehavior::Moving,
                    TaskType::Guard => AIBehavior::Idle,
                    TaskType::Repair => AIBehavior::Working,
                };
                self.state = BehaviorState::Starting;
                
                debug.log(
                    DebugLevel::Debug,
                    "AI",
                    &format!("Starting behavior: {:?}", self.behavior)
                );
            }
        } else if self.rest_timer > 0.0 {
            self.behavior = AIBehavior::Resting;
        } else {
            self.behavior = AIBehavior::Idle;
        }
    }
    
    fn idle_behavior(&mut self, delta_time: f32, debug: &DebugSystem) {
        if self.state == BehaviorState::Starting {
            debug.log(DebugLevel::Debug, "AI", "Worker is idle");
            self.state = BehaviorState::InProgress;
        }
        
        // Occasionally move to random position when idle
        if rand::random::<f32>() < 0.01 * delta_time {
            self.target_position = Some(Vec3::new(
                rand::random::<f32>() * 100.0 - 50.0,
                rand::random::<f32>() * 100.0 - 50.0,
                0.0,
            ));
            self.behavior = AIBehavior::Moving;
            self.state = BehaviorState::Starting;
        }
    }
    
    fn move_behavior(&mut self, delta_time: f32, transform: &mut Transform, debug: &DebugSystem) {
        if self.state == BehaviorState::Starting {
            if self.target_position.is_none() {
                self.state = BehaviorState::Failed;
                return;
            }
            self.state = BehaviorState::InProgress;
            debug.log(DebugLevel::Debug, "AI", "Worker moving to target");
        }
        
        if let Some(target) = self.target_position {
            let direction = (target - transform.translation).normalize();
            let speed = 50.0; // Units per second
            transform.translation += direction * speed * delta_time;
            
            // Check if reached target
            if transform.translation.distance(target) < 5.0 {
                self.state = BehaviorState::Completed;
                self.target_position = None;
                debug.log(DebugLevel::Debug, "AI", "Worker reached destination");
            }
        }
    }
    
    fn work_behavior(&mut self, delta_time: f32, debug: &DebugSystem) {
        if self.state == BehaviorState::Starting {
            self.work_progress = 0.0;
            self.state = BehaviorState::InProgress;
            debug.log(DebugLevel::Debug, "AI", "Worker started working");
        }
        
        self.work_progress += delta_time * 0.2; // 20% per second
        
        if self.work_progress >= 1.0 {
            self.state = BehaviorState::Completed;
            self.work_progress = 0.0;
            debug.log(DebugLevel::Info, "AI", "Worker completed work");
        }
    }
    
    fn harvest_behavior(&mut self, delta_time: f32, debug: &DebugSystem) {
        if self.state == BehaviorState::Starting {
            self.work_progress = 0.0;
            self.state = BehaviorState::InProgress;
            debug.log(DebugLevel::Debug, "AI", "Worker started harvesting");
        }
        
        self.work_progress += delta_time * 0.15; // 15% per second
        
        if self.work_progress >= 1.0 {
            self.state = BehaviorState::Completed;
            self.work_progress = 0.0;
            debug.log(DebugLevel::Info, "AI", "Worker completed harvesting");
        }
    }
    
    fn build_behavior(&mut self, delta_time: f32, debug: &DebugSystem) {
        if self.state == BehaviorState::Starting {
            self.work_progress = 0.0;
            self.state = BehaviorState::InProgress;
            debug.log(DebugLevel::Debug, "AI", "Worker started building");
        }
        
        self.work_progress += delta_time * 0.1; // 10% per second
        
        if self.work_progress >= 1.0 {
            self.state = BehaviorState::Completed;
            self.work_progress = 0.0;
            debug.log(DebugLevel::Info, "AI", "Worker completed building");
        }
    }
    
    fn craft_behavior(&mut self, delta_time: f32, debug: &DebugSystem) {
        if self.state == BehaviorState::Starting {
            self.work_progress = 0.0;
            self.state = BehaviorState::InProgress;
            debug.log(DebugLevel::Debug, "AI", "Worker started crafting");
        }
        
        self.work_progress += delta_time * 0.25; // 25% per second
        
        if self.work_progress >= 1.0 {
            self.state = BehaviorState::Completed;
            self.work_progress = 0.0;
            debug.log(DebugLevel::Info, "AI", "Worker completed crafting");
        }
    }
    
    fn rest_behavior(&mut self, delta_time: f32, debug: &DebugSystem) {
        if self.state == BehaviorState::Starting {
            self.state = BehaviorState::InProgress;
            debug.log(DebugLevel::Debug, "AI", "Worker is resting");
        }
        
        self.rest_timer -= delta_time;
        
        if self.rest_timer <= 0.0 {
            self.state = BehaviorState::Completed;
            self.rest_timer = 0.0;
            debug.log(DebugLevel::Debug, "AI", "Worker finished resting");
        }
    }
    
    fn flee_behavior(&mut self, delta_time: f32, transform: &mut Transform, debug: &DebugSystem) {
        if self.state == BehaviorState::Starting {
            // Set flee target away from danger
            self.target_position = Some(Vec3::new(
                transform.translation.x + rand::random::<f32>() * 100.0 - 50.0,
                transform.translation.y + rand::random::<f32>() * 100.0 - 50.0,
                0.0,
            ));
            self.state = BehaviorState::InProgress;
            debug.log(DebugLevel::Info, "AI", "Worker fleeing from danger!");
        }
        
        // Move faster when fleeing
        if let Some(target) = self.target_position {
            let direction = (target - transform.translation).normalize();
            let speed = 100.0; // Double speed when fleeing
            transform.translation += direction * speed * delta_time;
            
            if transform.translation.distance(target) < 5.0 {
                self.state = BehaviorState::Completed;
                self.target_position = None;
                debug.log(DebugLevel::Info, "AI", "Worker reached safety");
            }
        }
    }
}