/// Tick-based simulation core module
///
/// This module implements the core tick-based simulation architecture,
/// separating game logic (runs at fixed ticks) from presentation (runs at framerate).
/// Based on architecture used by Factorio, Dwarf Fortress, and RimWorld.
pub mod tick_config;
pub mod events;
pub mod tick_manager;
pub mod tick_debugger;

pub use tick_config::*;
pub use events::*;
pub use tick_manager::*;
pub use tick_debugger::*;

use bevy::prelude::*;

/// Event that fires when a simulation tick occurs
#[derive(Event, Debug, Clone)]
pub struct SimulationTickEvent {
    pub tick: u64,
}

/// Plugin that sets up the tick-based simulation system
pub struct TickSimulationPlugin;

impl Plugin for TickSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add event plugin
            .add_plugins(TickEventPlugin)
            // Resources
            .init_resource::<TickAccumulator>()
            .init_resource::<SimulationTickState>()
            .init_resource::<TickManager>()
            // Core tick system that runs before everything else
            .add_systems(PreUpdate, tick_accumulator_system)
            // Separate schedules for simulation and presentation
            .add_systems(
                Update,
                (
                    // Handle speed changes
                    tick_manager_system,
                    // This will trigger all tick-based systems
                    run_simulation_ticks,
                ).chain()
            )
            // Input handling
            .add_systems(Update, handle_speed_input)
            // Debug systems (can be removed in production)
            .add_systems(Update, (debug_tick_events, example_tick_driven_system));
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
    sim_state: Res<crate::SimulationState>,
) {
    // Only accumulate time if simulation is running
    if sim_state.running {
        accumulator.update(time.delta_secs() * sim_state.speed);
    } else {
        // Clear pending ticks if paused
        accumulator.pending_ticks = 0;
    }
}

/// System that runs the appropriate number of simulation ticks
pub fn run_simulation_ticks(
    time: Res<Time>,
    mut accumulator: ResMut<TickAccumulator>,
    mut tick_state: ResMut<SimulationTickState>,
    mut sim_state: ResMut<crate::SimulationState>,
    mut pre_tick_events: EventWriter<PreTickEvent>,
    mut tick_events: EventWriter<TickEvent>,
    mut post_tick_events: EventWriter<PostTickEvent>,
) {
    let ticks_to_run = accumulator.pending_ticks;
    tick_state.ticks_this_frame = ticks_to_run;

    if ticks_to_run > 0 {
        // Emit pre-tick event before processing any ticks
        pre_tick_events.send(PreTickEvent {
            tick: tick_state.current_tick,
            delta_time: time.delta_secs(),
        });

        // Run each tick
        for sub_tick in 0..ticks_to_run {
            tick_state.is_ticking = true;
            tick_state.current_tick += 1;

            // Update old SimulationState for compatibility
            sim_state.tick += 1;
            sim_state.just_ticked = true;

            // Emit main tick event
            tick_events.send(TickEvent {
                tick: tick_state.current_tick,
                sub_tick,
            });

            // All tick-based systems will check sim_state.just_ticked
            // and run their logic

            // Note: In future, we'll trigger a custom schedule here
            // app.world.run_schedule(SimulationSchedule);
        }

        // Emit post-tick event after all ticks processed
        post_tick_events.send(PostTickEvent {
            tick: tick_state.current_tick,
            ticks_processed: ticks_to_run,
        });
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
pub fn on_simulation_tick(tick_state: Res<SimulationTickState>) -> bool {
    tick_state.is_ticking
}

/// Run condition using the legacy SimulationState (for compatibility)
pub fn on_simulation_tick_legacy(sim_state: Res<crate::SimulationState>) -> bool {
    let result = sim_state.just_ticked;
    // Debug every 100 ticks
    if sim_state.tick % 100 == 0 && result {
        println!("🔧 on_simulation_tick_legacy returning true for tick {}", sim_state.tick);
    }
    result
}
