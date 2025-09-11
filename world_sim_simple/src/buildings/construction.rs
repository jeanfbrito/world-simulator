use super::BuildingType;
use crate::debug::{DebugLevel, DebugSystem};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstructionStatus {
    Queued,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionTask {
    pub building_type: BuildingType,
    pub position: (i32, i32),
    pub progress: f32,
    pub required_time: f32,
    pub status: ConstructionStatus,
    pub workers_assigned: usize,
}

impl ConstructionTask {
    pub fn new(building_type: BuildingType, position: (i32, i32)) -> Self {
        let requirements = building_type.requirements();

        Self {
            building_type,
            position,
            progress: 0.0,
            required_time: requirements.build_time,
            status: ConstructionStatus::Queued,
            workers_assigned: 0,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        if self.status != ConstructionStatus::InProgress {
            return false;
        }

        // Progress based on number of workers
        let work_rate = self.workers_assigned as f32 * 0.1; // 10% per worker per second
        self.progress += work_rate * delta_time / self.required_time;

        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.status = ConstructionStatus::Completed;
            info!(
                "[CONSTRUCTION] Completed {:?} at {:?}",
                self.building_type, self.position
            );
            return true;
        }

        false
    }

    pub fn assign_worker(&mut self) {
        self.workers_assigned += 1;
        if self.status == ConstructionStatus::Queued {
            self.status = ConstructionStatus::InProgress;
            info!(
                "[CONSTRUCTION] Started {:?} at {:?}",
                self.building_type, self.position
            );
        }
    }

    pub fn unassign_worker(&mut self) {
        if self.workers_assigned > 0 {
            self.workers_assigned -= 1;
        }
    }

    pub fn cancel(&mut self) {
        self.status = ConstructionStatus::Cancelled;
        info!(
            "[CONSTRUCTION] Cancelled {:?} at {:?}",
            self.building_type, self.position
        );
    }
}

#[derive(Resource, Default)]
pub struct ConstructionQueue {
    tasks: VecDeque<ConstructionTask>,
    active_tasks: Vec<ConstructionTask>,
    max_active: usize,
}

impl ConstructionQueue {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
            active_tasks: Vec::new(),
            max_active: 3,
        }
    }

    pub fn add_task(&mut self, building_type: BuildingType, position: (i32, i32)) {
        let task = ConstructionTask::new(building_type, position);
        self.tasks.push_back(task);

        info!(
            "[CONSTRUCTION] Added {:?} to queue at position {:?}",
            building_type, position
        );
    }

    pub fn update(&mut self, delta_time: f32, debug: &DebugSystem) {
        // Move queued tasks to active if space available
        while self.active_tasks.len() < self.max_active && !self.tasks.is_empty() {
            if let Some(mut task) = self.tasks.pop_front() {
                task.status = ConstructionStatus::InProgress;
                debug.log(
                    DebugLevel::Info,
                    "CONSTRUCTION",
                    &format!("Activating construction of {:?}", task.building_type),
                );
                self.active_tasks.push(task);
            }
        }

        // Update active tasks
        self.active_tasks.retain_mut(|task| {
            let completed = task.update(delta_time);
            if completed {
                debug.log(
                    DebugLevel::Info,
                    "CONSTRUCTION",
                    &format!("Completed {:?} at {:?}", task.building_type, task.position),
                );
            }
            !completed
        });
    }

    pub fn cancel_task(&mut self, position: (i32, i32)) -> bool {
        // Check active tasks
        if let Some(index) = self
            .active_tasks
            .iter()
            .position(|t| t.position == position)
        {
            self.active_tasks[index].cancel();
            self.active_tasks.remove(index);
            return true;
        }

        // Check queued tasks
        if let Some(index) = self.tasks.iter().position(|t| t.position == position) {
            self.tasks.remove(index);
            return true;
        }

        false
    }

    pub fn get_task_at(&self, position: (i32, i32)) -> Option<&ConstructionTask> {
        self.active_tasks
            .iter()
            .find(|t| t.position == position)
            .or_else(|| self.tasks.iter().find(|t| t.position == position))
    }

    pub fn active_count(&self) -> usize {
        self.active_tasks.len()
    }

    pub fn queued_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn total_count(&self) -> usize {
        self.active_count() + self.queued_count()
    }
}
