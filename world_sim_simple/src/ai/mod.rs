mod behaviors;
mod task_system;
mod pathfinding;

pub use behaviors::{AIBehavior, BehaviorState, WorkerAI};
pub use task_system::{TaskSystem, Task, TaskType, TaskPriority, TaskStatus};
pub use pathfinding::{find_path, Path};

use bevy::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TaskSystem>()
            .add_systems(Startup, ai_init_system)
            .add_systems(Update, (
                task_assignment_system,
                worker_ai_update_system,
                pathfinding_update_system,
            ).chain());
    }
}

fn ai_init_system(debug: Res<DebugSystem>) {
    debug.log(
        DebugLevel::Info,
        "AI",
        "AI systems initialized"
    );
}

fn task_assignment_system(
    mut task_system: ResMut<TaskSystem>,
    mut query: Query<(Entity, &mut WorkerAI), Without<Task>>,
    debug: Res<DebugSystem>,
) {
    for (entity, mut ai) in query.iter_mut() {
        if let Some(task) = task_system.assign_next_task() {
            debug.log(
                DebugLevel::Debug,
                "AI",
                &format!("Assigned {:?} to worker", task.task_type)
            );
            ai.current_task = Some(task);
        }
    }
}

fn worker_ai_update_system(
    mut query: Query<(&mut WorkerAI, &mut Transform)>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    for (mut ai, mut transform) in query.iter_mut() {
        ai.update(time.delta_seconds(), &mut transform, &debug);
    }
}

fn pathfinding_update_system(
    mut query: Query<(&mut Path, &mut Transform)>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    for (mut path, mut transform) in query.iter_mut() {
        if path.follow(time.delta_seconds(), &mut transform) {
            debug.log(
                DebugLevel::Debug,
                "PATHFINDING",
                "Reached destination"
            );
        }
    }
}