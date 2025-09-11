use bevy::prelude::*;
use super::{PreTickEvent, TickEvent, PostTickEvent, SimulationSpeedChangedEvent, SimulationPausedEvent, SimulationResumedEvent};

/// Debug system that logs tick events
pub fn debug_tick_events(
    mut pre_tick_reader: EventReader<PreTickEvent>,
    mut tick_reader: EventReader<TickEvent>,
    mut post_tick_reader: EventReader<PostTickEvent>,
    mut speed_reader: EventReader<SimulationSpeedChangedEvent>,
    mut pause_reader: EventReader<SimulationPausedEvent>,
    mut resume_reader: EventReader<SimulationResumedEvent>,
) {
    for event in pre_tick_reader.read() {
        debug!("PreTick: tick={}, delta={:.3}s", event.tick, event.delta_time);
    }

    for event in tick_reader.read() {
        if event.tick % 10 == 0 {
            info!("Tick #{} (sub_tick: {})", event.tick, event.sub_tick);
        }
    }

    for event in post_tick_reader.read() {
        if event.ticks_processed > 1 {
            warn!("Processed {} ticks in one frame (tick #{})", event.ticks_processed, event.tick);
        }
    }

    for event in speed_reader.read() {
        info!(
            "Speed changed: {} -> {}",
            event.old_speed.name(),
            event.new_speed.name()
        );
    }

    for event in pause_reader.read() {
        info!("Simulation PAUSED at tick #{}", event.tick);
    }

    for event in resume_reader.read() {
        info!("Simulation RESUMED at tick #{}", event.tick);
    }
}

/// System that demonstrates using tick events to drive game logic
pub fn example_tick_driven_system(
    mut tick_reader: EventReader<TickEvent>,
) {
    for event in tick_reader.read() {
        // This system runs exactly once per simulation tick
        // regardless of framerate
        if event.tick % 50 == 0 {
            debug!("Processing game logic at tick #{}", event.tick);
        }
    }
}