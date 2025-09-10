/// Tick-based simulation core module
/// 
/// This module implements the core tick-based simulation architecture,
/// separating game logic (runs at fixed ticks) from presentation (runs at framerate).
/// Based on architecture used by Factorio, Dwarf Fortress, and RimWorld.

pub mod tick_config;

pub use tick_config::*;

use bevy::prelude::*;

/// Plugin that sets up the tick-based simulation system
pub struct TickSimulationPlugin;

impl Plugin for TickSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<TickAccumulator>()
            .init_resource::<SimulationTickState>()
            
            // Core tick system that runs before everything else
            .add_systems(
                PreUpdate,
                tick_accumulator_system
            )
            
            // Separate schedules for simulation and presentation
            .add_systems(
                Update,
                (
                    // This will trigger all tick-based systems
                    run_simulation_ticks,
                ).chain()
            );
    }
}

/// Tracks the current state of simulation ticks
#[derive(Resource, Default, Debug)]
pub struct SimulationTickState {
    /// Current simulation tick number
    pub current_tick: u64,
    /// Whether we're currently processing a tick
    pub is_ticking: bool,
    /// Number of ticks processed this frame
    pub ticks_this_frame: u32,
}

/// System that updates the tick accumulator each frame
fn tick_accumulator_system(
    time: Res<Time>,
    mut accumulator: ResMut<TickAccumulator>,
) {
    accumulator.update(time.delta_secs());
}

/// System that runs the appropriate number of simulation ticks
fn run_simulation_ticks(
    mut accumulator: ResMut<TickAccumulator>,
    mut tick_state: ResMut<SimulationTickState>,
    mut sim_state: ResMut<crate::SimulationState>,
) {
    let ticks_to_run = accumulator.pending_ticks;
    tick_state.ticks_this_frame = ticks_to_run;
    
    // Run each tick
    for _ in 0..ticks_to_run {
        tick_state.is_ticking = true;
        tick_state.current_tick += 1;
        
        // Update old SimulationState for compatibility
        sim_state.tick += 1;
        sim_state.just_ticked = true;
        
        // All tick-based systems will check sim_state.just_ticked
        // and run their logic
        
        // Note: In future, we'll trigger a custom schedule here
        // app.world.run_schedule(SimulationSchedule);
    }
    
    // Clear the pending ticks
    accumulator.pending_ticks = 0;
    tick_state.is_ticking = false;
    
    // Reset just_ticked if no ticks ran
    if ticks_to_run == 0 {
        sim_state.just_ticked = false;
    }
}

/// Run condition for systems that should only run during simulation ticks
pub fn on_simulation_tick(
    tick_state: Res<SimulationTickState>,
) -> bool {
    tick_state.is_ticking
}

/// Run condition using the legacy SimulationState (for compatibility)
pub fn on_simulation_tick_legacy(
    sim_state: Res<crate::SimulationState>,
) -> bool {
    sim_state.just_ticked
}