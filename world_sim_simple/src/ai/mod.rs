mod behaviors;
mod task_system;
mod pathfinding;
mod goap_state_sync;
pub mod goap_actions;
pub mod goap_planner;
mod goap_bridge;
mod task_executor;
// New AI modules (temporarily disabled due to library compatibility issues)
// mod bevy_dogoap_impl;  // GOAP planning using bevy_dogoap
// mod shared_state;       // For hybrid AI
// mod big_brain_impl;     // Reactive AI

pub use behaviors::{AIBehavior, BehaviorState, WorkerAI};
pub use task_system::{TaskSystem, Task, TaskType, TaskPriority, TaskStatus};
pub use pathfinding::{find_path, Path};
pub use goap_state_sync::sync_goap_states_system;
pub use goap_actions::{GoapAction, WorldState, StateValue, ActionSet, ActionPlan};
pub use goap_planner::{GoapPlanner, goap_planning_system, goap_execution_system};
pub use goap_bridge::{goap_to_task_bridge_system, update_needs_system};
pub use task_executor::{task_execution_system, TreeTag, RockTag, BerryBushTag};
// New AI exports (temporarily disabled due to library compatibility issues)
// pub use bevy_dogoap_impl::BevyDogoapPlugin;
// pub use big_brain_impl::BigBrainAIPlugin;
// pub use shared_state::{update_worker_stats_system, ai_mode_selection_system};

use bevy::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};
use crate::SimulationState;

/// Run condition to check if simulation is running
fn simulation_running(sim_state: Res<SimulationState>) -> bool {
    sim_state.running
}

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        // Add AI plugins - temporarily disabled due to library compatibility issues
        // app.add_plugins((
        //     BevyDogoapPlugin,      // GOAP planning
        //     BigBrainAIPlugin,      // Reactive behaviors  
        // ));
        app.init_resource::<TaskSystem>()
            .insert_resource(crate::components::SettlementState::default())
            .insert_resource(ActionSet::default())
            .add_systems(Startup, ai_init_system)
            .add_systems(Update, (
                // Task assignment must run first
                task_assignment_system,
            ).run_if(simulation_running))
            .add_systems(Update, (
                // These can run in parallel as they work on different components
                worker_ai_update_system,
                pathfinding_update_system,
                // sync_goap_states_system,    // DISABLED - GOAP is broken
                // update_needs_system,        // DISABLED - GOAP is broken
                // goap_planning_system,       // DISABLED - GOAP is broken
                // goap_execution_system,      // DISABLED - GOAP is broken
                // goap_to_task_bridge_system, // DISABLED - GOAP is broken
                task_execution_system          // Execute tasks with actual movement
            ).run_if(simulation_running));
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
    for (_entity, mut ai) in query.iter_mut() {
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
        ai.update(time.delta_secs(), &mut transform, &debug);
    }
}

fn pathfinding_update_system(
    mut query: Query<(&mut Path, &mut Transform)>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    for (mut path, mut transform) in query.iter_mut() {
        if path.follow(time.delta_secs(), &mut transform) {
            debug.log(
                DebugLevel::Debug,
                "PATHFINDING",
                "Reached destination"
            );
        }
    }
}