/// Configuration for tick-based simulation
///
/// This module defines all constants related to the tick-based simulation engine.
/// Following the Factorio/Dwarf Fortress model where simulation runs at fixed
/// tick rate independent of rendering framerate.
use bevy::prelude::*;

/// Base tick rate - how many simulation ticks per second at 1x speed
/// 10 TPS gives good balance between responsiveness and performance
pub const TICKS_PER_SECOND: u32 = 10;

/// Duration of one tick in seconds
pub const TICK_DURATION: f32 = 1.0 / TICKS_PER_SECOND as f32;

// ============================================================================
// NEEDS SYSTEM CONSTANTS
// ============================================================================

/// Maximum values for need counters (similar to DF's 100,000 scale)
pub const MAX_HUNGER: u32 = 100_000;
pub const MAX_ENERGY: u32 = 100_000;
pub const MAX_MORALE: u32 = 100_000;

/// Per-tick increments for needs
pub const HUNGER_PER_TICK: u32 = 500; // At 10 TPS: 5000/sec, ~200 ticks to starve
pub const ENERGY_LOSS_PER_TICK: u32 = 300; // At 10 TPS: 3000/sec, ~333 ticks to exhaust
pub const MORALE_LOSS_PER_TICK: u32 = 200; // When unhappy

/// Thresholds for need states
pub const HUNGER_THRESHOLD_HUNGRY: u32 = 50_000; // 50% = hungry
pub const HUNGER_THRESHOLD_STARVING: u32 = 80_000; // 80% = starving
pub const ENERGY_THRESHOLD_TIRED: u32 = 30_000; // 30% = tired
pub const ENERGY_THRESHOLD_NAP: u32 = 15_000; // 15% = need to nap (prevents exhaustion)
pub const ENERGY_THRESHOLD_EXHAUSTED: u32 = 10_000; // 10% = exhausted

/// Recovery rates when resting/eating
pub const ENERGY_RECOVERY_PER_TICK: u32 = 2000; // When resting
pub const ENERGY_NAP_RECOVERY_PER_TICK: u32 = 8000; // When napping (4x faster than resting)
pub const HUNGER_REDUCTION_PER_FOOD: u32 = 20_000; // Per food item consumed

// ============================================================================
// WORK SYSTEM CONSTANTS
// ============================================================================

/// Work progress counters (similar to construction/crafting progress)
pub const MAX_WORK_PROGRESS: u32 = 10_000;

/// Per-tick work rates for different activities
pub const GATHER_PROGRESS_PER_TICK: u32 = 1000; // 10 ticks to gather
pub const BUILD_PROGRESS_PER_TICK: u32 = 500; // 20 ticks to build
pub const CRAFT_PROGRESS_PER_TICK: u32 = 750; // ~13 ticks to craft
pub const HARVEST_PROGRESS_PER_TICK: u32 = 1500; // ~7 ticks to harvest

// ============================================================================
// MOVEMENT CONSTANTS
// ============================================================================

/// Movement is grid-based at simulation level
pub const MOVE_TICKS_PER_TILE: u32 = 3; // Takes 3 ticks to move one tile
pub const MOVE_PROGRESS_PER_TICK: u32 = MAX_WORK_PROGRESS / MOVE_TICKS_PER_TILE;

// ============================================================================
// SIMULATION SPEED SETTINGS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimulationSpeed {
    Paused,
    Slow,      // 0.5x speed
    Normal,    // 1.0x speed
    Fast,      // 2.0x speed
    VeryFast,  // 5.0x speed
    UltraFast, // 10.0x speed (may not achieve if CPU-bound)
}

impl SimulationSpeed {
    pub fn multiplier(&self) -> f32 {
        match self {
            SimulationSpeed::Paused => 0.0,
            SimulationSpeed::Slow => 0.5,
            SimulationSpeed::Normal => 1.0,
            SimulationSpeed::Fast => 2.0,
            SimulationSpeed::VeryFast => 5.0,
            SimulationSpeed::UltraFast => 10.0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            SimulationSpeed::Paused => "Paused",
            SimulationSpeed::Slow => "Slow (0.5x)",
            SimulationSpeed::Normal => "Normal (1x)",
            SimulationSpeed::Fast => "Fast (2x)",
            SimulationSpeed::VeryFast => "Very Fast (5x)",
            SimulationSpeed::UltraFast => "Ultra Fast (10x)",
        }
    }
}

// ============================================================================
// TICK ACCUMULATOR
// ============================================================================

/// Accumulates frame time and determines when to run simulation ticks
/// This is the core of the tick-based simulation system
#[derive(Resource, Debug)]
pub struct TickAccumulator {
    /// Accumulated time since last tick
    accumulated: f32,
    /// Current simulation speed setting
    speed: SimulationSpeed,
    /// Number of ticks that should run this frame
    pub pending_ticks: u32,
    /// Performance tracking
    actual_tps: f32,
    last_tick_time: f32,
}

impl Default for TickAccumulator {
    fn default() -> Self {
        Self {
            accumulated: 0.0,
            speed: SimulationSpeed::Normal,
            pending_ticks: 0,
            actual_tps: 0.0,
            last_tick_time: 0.0,
        }
    }
}

impl TickAccumulator {
    /// Update the accumulator with frame delta time
    /// Returns the number of ticks that should execute this frame
    pub fn update(&mut self, delta_seconds: f32) -> u32 {
        // Don't accumulate if paused
        if self.speed == SimulationSpeed::Paused {
            self.pending_ticks = 0;
            return 0;
        }

        // Accumulate time based on speed multiplier
        self.accumulated += delta_seconds * self.speed.multiplier();

        // Calculate how many ticks to run
        let ticks = (self.accumulated / TICK_DURATION) as u32;

        // Remove the consumed time
        self.accumulated -= ticks as f32 * TICK_DURATION;

        // Cap accumulated time to prevent spiral of death
        // (when simulation can't keep up, don't accumulate infinite ticks)
        if self.accumulated > TICK_DURATION * 3.0 {
            self.accumulated = TICK_DURATION * 3.0;
        }

        // Track performance
        if ticks > 0 {
            let now = delta_seconds;
            self.actual_tps = ticks as f32 / (now - self.last_tick_time);
            self.last_tick_time = now;
        }

        self.pending_ticks = ticks;
        ticks
    }

    pub fn set_speed(&mut self, speed: SimulationSpeed) -> Option<(SimulationSpeed, SimulationSpeed)> {
        if self.speed != speed {
            let old_speed = self.speed;
            self.speed = speed;
            // Reset accumulator when changing speed to prevent jumps
            self.accumulated = 0.0;
            Some((old_speed, speed))
        } else {
            None
        }
    }

    pub fn get_speed(&self) -> SimulationSpeed {
        self.speed
    }

    pub fn get_actual_tps(&self) -> f32 {
        self.actual_tps
    }

    pub fn should_tick(&self) -> bool {
        self.pending_ticks > 0
    }
}

// ============================================================================
// HELPERS
// ============================================================================

/// Convert a counter value to a normalized float (0.0 - 1.0)
/// Used for backward compatibility and UI display
pub fn counter_to_float(counter: u32, max: u32) -> f32 {
    (counter as f32 / max as f32).clamp(0.0, 1.0)
}

/// Convert a normalized float to a counter value
/// Used during migration from float-based to counter-based
pub fn float_to_counter(value: f32, max: u32) -> u32 {
    (value * max as f32) as u32
}
