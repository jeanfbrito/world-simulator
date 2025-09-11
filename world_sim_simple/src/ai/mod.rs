pub mod behavior_state;
mod behaviors;
mod debug_logger;
pub mod goap_actions;
mod goap_bridge;
pub mod goap_planner;
mod goap_state_sync;
mod pathfinding;
mod task_executor;
mod task_system;
// New AI modules (temporarily disabled due to library compatibility issues)
// mod bevy_dogoap_impl;  // GOAP planning using bevy_dogoap
// mod shared_state;       // For hybrid AI
// mod big_brain_impl;     // Reactive AI

pub use behavior_state::{
    behavior_state_machine_system, BehaviorCycle, BehaviorState as BehaviorStateNew,
};
pub use behaviors::WorkerAI;
pub use debug_logger::enhanced_debug_system;
pub use goap_actions::{ActionPlan, ActionSet, GoapAction, StateValue, WorldState};
pub use goap_bridge::{goap_to_task_bridge_system, update_needs_system};
pub use goap_planner::{goap_execution_system, goap_planning_system};
pub use goap_state_sync::sync_goap_states_system;
pub use pathfinding::Path;
pub use task_executor::{task_execution_system, BerryBushTag, TreeTag};
pub use task_system::{Task, TaskPriority, TaskStatus, TaskSystem, TaskType};
// New AI exports (temporarily disabled due to library compatibility issues)
// pub use bevy_dogoap_impl::BevyDogoapPlugin;
// pub use big_brain_impl::BigBrainAIPlugin;
// pub use shared_state::{update_worker_stats_system, ai_mode_selection_system};

use crate::debug::{DebugLevel, DebugSystem};
use crate::SimulationState;
use bevy::prelude::*;

/// Run condition to check if simulation is running
fn simulation_running(sim_state: Res<SimulationState>) -> bool {
    sim_state.running
}

/// Run condition to check if a simulation tick just occurred
fn simulation_just_ticked(sim_state: Res<SimulationState>) -> bool {
    sim_state.just_ticked
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
            .add_systems(
                Update,
                (
                    // Task assignment must run first
                    task_assignment_system,
                )
                    .run_if(simulation_running),
            )
            .add_systems(
                Update,
                (
                    // These can run in parallel as they work on different components
                    worker_ai_update_system.run_if(simulation_running),
                    pathfinding_update_system.run_if(simulation_running),
                    sync_goap_states_system.run_if(simulation_just_ticked),
                    update_needs_system.run_if(simulation_just_ticked),
                    behavior_state_machine_system.run_if(simulation_just_ticked), // Run before planning
                    goap_planning_system.run_if(simulation_just_ticked),
                    goap_execution_system.run_if(simulation_just_ticked),
                    goap_to_task_bridge_system.run_if(simulation_just_ticked),
                    task_execution_system.run_if(simulation_running),
                    enhanced_debug_system.run_if(simulation_just_ticked),
                ),
            );
    }
}

fn ai_init_system(debug: Res<DebugSystem>) {
    debug.log(DebugLevel::Info, "AI", "AI systems initialized");
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
                &format!("Assigned {:?} to worker", task.task_type),
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
            debug.log(DebugLevel::Debug, "PATHFINDING", "Reached destination");
        }
    }
}
