use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use big_brain::prelude::*;

// Shared state components that both AI systems can read/write

#[derive(Component, Reflect, Clone, DatumComponent)]
#[reflect(Component)]
pub struct IsHungry(pub bool);

#[derive(Component, Reflect, Clone, DatumComponent)]
#[reflect(Component)]
pub struct IsTired(pub bool);

#[derive(Component, Reflect, Clone, DatumComponent)]
#[reflect(Component)]
pub struct HasWood(pub i32);

#[derive(Component, Reflect, Clone, DatumComponent)]
#[reflect(Component)]
pub struct HasStone(pub i32);

#[derive(Component, Reflect, Clone, DatumComponent)]
#[reflect(Component)]
pub struct HasFood(pub i32);

#[derive(Component, Reflect, Clone, DatumComponent)]
#[reflect(Component)]
pub struct NeedsShelter(pub bool);

#[derive(Component, Reflect, Clone, DatumComponent)]
#[reflect(Component)]
pub struct NeedsResources(pub bool);

// Worker stats that both systems can monitor
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct WorkerStats {
    pub hunger: f32,    // 0.0 = starving, 1.0 = full
    pub energy: f32,    // 0.0 = exhausted, 1.0 = rested
    pub health: f32,    // 0.0 = dead, 1.0 = healthy
}

impl Default for WorkerStats {
    fn default() -> Self {
        Self {
            hunger: 1.0,
            energy: 1.0,
            health: 1.0,
        }
    }
}

// Current AI decision mode
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub enum AIMode {
    Reactive,    // Use big-brain for immediate needs
    Planning,    // Use bevy_dogoap for long-term goals
    Executing,   // Currently executing a plan
}

impl Default for AIMode {
    fn default() -> Self {
        Self::Reactive
    }
}

// Shared goal priorities
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct GoalPriorities {
    pub survive: f32,      // Eat, sleep, stay healthy
    pub gather: f32,       // Collect resources
    pub build: f32,        // Construct buildings
    pub craft: f32,        // Create items
}

impl Default for GoalPriorities {
    fn default() -> Self {
        Self {
            survive: 1.0,
            gather: 0.5,
            build: 0.3,
            craft: 0.2,
        }
    }
}

// Update worker stats over time
pub fn update_worker_stats_system(
    mut query: Query<(&mut WorkerStats, &mut IsHungry, &mut IsTired)>,
    time: Res<Time>,
    debug: Res<crate::debug::DebugSystem>,
) {
    let dt = time.delta_seconds();
    
    for (mut stats, mut is_hungry, mut is_tired) in query.iter_mut() {
        // Decrease hunger over time
        stats.hunger = (stats.hunger - dt * 0.02).max(0.0);
        is_hungry.0 = stats.hunger < 0.3;
        
        // Decrease energy over time
        stats.energy = (stats.energy - dt * 0.015).max(0.0);
        is_tired.0 = stats.energy < 0.3;
        
        // Log critical states
        if stats.hunger < 0.1 {
            debug.log(
                crate::debug::DebugLevel::Warning,
                "WORKER_STATE",
                "Worker is starving!"
            );
        }
        if stats.energy < 0.1 {
            debug.log(
                crate::debug::DebugLevel::Warning,
                "WORKER_STATE",
                "Worker is exhausted!"
            );
        }
    }
}

// Decide which AI mode to use based on current state
pub fn ai_mode_selection_system(
    mut query: Query<(&WorkerStats, &mut AIMode, &GoalPriorities)>,
    debug: Res<crate::debug::DebugSystem>,
) {
    for (stats, mut mode, priorities) in query.iter_mut() {
        let new_mode = if stats.hunger < 0.3 || stats.energy < 0.3 {
            // Critical needs - use reactive AI
            AIMode::Reactive
        } else if *mode == AIMode::Executing {
            // Keep executing current plan
            AIMode::Executing
        } else {
            // Safe to plan long-term
            AIMode::Planning
        };
        
        if *mode != new_mode {
            debug.log(
                crate::debug::DebugLevel::Info,
                "AI_MODE",
                &format!("Switching from {:?} to {:?}", *mode, new_mode)
            );
            *mode = new_mode;
        }
    }
}