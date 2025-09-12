use bevy::prelude::*;

/// The high-level behavioral state of a unit
#[derive(Component, Clone, Debug, PartialEq, Reflect, Default)]
pub enum BehaviorState {
    /// Resting to recover energy
    Resting,
    /// Eating to reduce hunger
    Eating,
    /// Gathering resources (food, wood, stone)
    Gathering,
    /// Building structures
    Building,
    /// Moving to a location
    Moving,
    /// Idle, deciding what to do next
    #[default]
    Idle,
    /// Working on a task
    Working,
}

/// Component that tracks the current behavior cycle
#[derive(Component, Debug, Reflect)]
pub struct BehaviorCycle {
    pub current_state: BehaviorState,
    pub state_timer: f32,
    pub state_duration: f32,
}

impl Default for BehaviorCycle {
    fn default() -> Self {
        Self {
            current_state: BehaviorState::Idle,
            state_timer: 0.0,
            state_duration: 1.0,
        }
    }
}

// Behavior state machine system removed - now handled by dogoap