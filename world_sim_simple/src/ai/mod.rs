pub mod behavior_state;
mod behaviors;
pub mod pathfinding;
mod task_executor;
mod task_system;
// New AI modules
pub mod bevy_dogoap_impl;  // GOAP planning using bevy_dogoap - make public for websocket access
mod shared_state;       // For hybrid AI
mod big_brain_impl;     // Reactive AI - ENABLED for execution layer

pub use behavior_state::{
    BehaviorCycle, BehaviorState as BehaviorStateNew,
};
pub use behaviors::WorkerAI;
pub use pathfinding::Path;
pub use task_executor::{BerryBushTag, TreeTag};
pub use task_system::{Task, TaskPriority, TaskStatus, TaskSystem, TaskType};
// New AI exports
pub use bevy_dogoap_impl::{
    BevyDogoapPlugin,
    Satiety, Energy, FoodCount, NearBerryBush,  // Export GOAP state components
    EatAction, WanderAction, GatherFoodAction,  // Export GOAP actions
};
pub use big_brain_impl::BigBrainAIPlugin;  // ENABLED for reactive execution
pub use shared_state::ai_mode_selection_system;

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
        // Add AI plugins - enable BOTH for hybrid architecture
        app.add_plugins((
            BevyDogoapPlugin,      // GOAP planning (strategic layer)
            BigBrainAIPlugin,      // Reactive behaviors (execution layer)
        ));
        app.init_resource::<TaskSystem>()
            .insert_resource(crate::components::SettlementState::default())
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
    debug: Res<DebugSystem>,
) {
    const TILES_PER_TICK: f32 = 0.15; // Move 0.15 tiles per tick for smooth, consistent movement
    
    for (mut path, mut transform) in query.iter_mut() {
        if path.follow_tick(&mut transform, TILES_PER_TICK) {
            debug.log(DebugLevel::Debug, "PATHFINDING", "Reached destination");
        }
    }
}
