//! AI Coordinator - Manages interaction between GOAP and Utility AI systems

use bevy_ecs::prelude::*;
use bevy::prelude::{Time, Name};
use bevy_dogoap::prelude::Planner;
use big_brain::prelude::*;

/// Defines which AI system is currently in control
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AIMode {
    /// Utility AI is handling critical needs
    UtilityDriven,
    /// GOAP is executing planned goals
    GoalDriven,
    /// Both systems active, utility can interrupt
    Hybrid,
}

/// Coordinates between GOAP and Utility AI systems
#[derive(Component)]
pub struct AICoordinator {
    /// Current AI control mode
    pub mode: AIMode,
    
    /// Utility score threshold for interrupting GOAP (0.0-1.0)
    pub interrupt_threshold: f32,
    
    /// How strongly GOAP resists interruption (0.0-1.0)
    pub goal_persistence: f32,
    
    /// Time since last mode switch
    pub mode_switch_cooldown: f32,
    
    /// Minimum time between mode switches (prevents thrashing)
    pub min_switch_interval: f32,
    
    /// Was GOAP paused by utility interrupt?
    pub goap_paused: bool,
    
    /// Stored GOAP state for resuming
    pub stored_goal: Option<String>,
}

impl AICoordinator {
    pub fn new() -> Self {
        Self {
            mode: AIMode::Hybrid,
            interrupt_threshold: 0.8,
            goal_persistence: 0.3,
            mode_switch_cooldown: 0.0,
            min_switch_interval: 2.0,
            goap_paused: false,
            stored_goal: None,
        }
    }
    
    /// Check if utility should interrupt GOAP
    pub fn should_interrupt(&self, utility_score: f32, goap_importance: f32) -> bool {
        if self.mode_switch_cooldown > 0.0 {
            return false; // Still in cooldown
        }
        
        // Adjust threshold based on GOAP importance
        let adjusted_threshold = self.interrupt_threshold + (goap_importance * self.goal_persistence);
        
        utility_score > adjusted_threshold
    }
    
    /// Switch to utility-driven mode
    pub fn switch_to_utility(&mut self) {
        if self.mode != AIMode::UtilityDriven {
            self.mode = AIMode::UtilityDriven;
            self.mode_switch_cooldown = self.min_switch_interval;
            self.goap_paused = true;
        }
    }
    
    /// Switch to goal-driven mode
    pub fn switch_to_goal(&mut self) {
        if self.mode != AIMode::GoalDriven {
            self.mode = AIMode::GoalDriven;
            self.mode_switch_cooldown = self.min_switch_interval;
            self.goap_paused = false;
        }
    }
    
    /// Return to hybrid mode
    pub fn switch_to_hybrid(&mut self) {
        self.mode = AIMode::Hybrid;
        self.mode_switch_cooldown = self.min_switch_interval;
        self.goap_paused = false;
    }
    
    /// Update cooldown timer
    pub fn update(&mut self, delta_time: f32) {
        if self.mode_switch_cooldown > 0.0 {
            self.mode_switch_cooldown -= delta_time;
        }
    }
}

/// System that coordinates between GOAP and Utility AI
pub fn ai_coordination_system(
    time: Res<Time>,
    mut query: Query<(
        &mut AICoordinator,
        &mut Planner,
        &Score, // From big-brain
        Option<&ActionState>, // Current utility action state
    )>,
) {
    for (mut coordinator, mut planner, utility_score, action_state) in query.iter_mut() {
        coordinator.update(time.delta_secs());
        
        match coordinator.mode {
            AIMode::Hybrid => {
                // Check if utility needs should interrupt
                let goap_importance = if planner.current_goal.is_some() { 0.5 } else { 0.0 };
                
                if coordinator.should_interrupt(utility_score.0, goap_importance) {
                    // High utility score - pause GOAP and handle immediate need
                    coordinator.switch_to_utility();
                    
                    // Store current GOAP goal for later
                    if let Some(goal) = &planner.current_goal {
                        coordinator.stored_goal = Some(format!("{:?}", goal));
                    }
                    
                    // Pause GOAP planning
                    planner.always_plan = false;
                } else if utility_score.0 < 0.3 && coordinator.goap_paused {
                    // Utility needs satisfied, resume GOAP
                    coordinator.switch_to_hybrid();
                    planner.always_plan = true;
                }
            }
            
            AIMode::UtilityDriven => {
                // Check if we can return control to GOAP
                if utility_score.0 < 0.4 {
                    // Immediate needs handled
                    if let Some(_) = coordinator.stored_goal {
                        coordinator.switch_to_goal();
                        planner.always_plan = true;
                    } else {
                        coordinator.switch_to_hybrid();
                    }
                }
            }
            
            AIMode::GoalDriven => {
                // Check for critical utility needs
                if utility_score.0 > 0.9 {
                    // Emergency! Switch to utility
                    coordinator.switch_to_utility();
                    planner.always_plan = false;
                }
            }
        }
    }
}

/// System to visualize current AI mode (for debugging)
pub fn debug_ai_mode_system(
    query: Query<(&Name, &AICoordinator)>,
) {
    for (name, coordinator) in query.iter() {
        match coordinator.mode {
            AIMode::UtilityDriven => {
                tracing::debug!("{}: UTILITY mode (handling immediate needs)", name.as_str());
            }
            AIMode::GoalDriven => {
                tracing::debug!("{}: GOAL mode (executing plan)", name.as_str());
            }
            AIMode::Hybrid => {
                tracing::debug!("{}: HYBRID mode (balanced)", name.as_str());
            }
        }
    }
}