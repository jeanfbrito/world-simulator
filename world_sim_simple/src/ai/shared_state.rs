use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use big_brain::prelude::*;

// Shared state components that both AI systems can read/write
// Note: We're using dogoap's Satiety/Energy components directly now

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct HasWood(pub i32);

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct HasStone(pub i32);

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct HasFood(pub i32);

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct NeedsShelter(pub bool);

#[derive(Component, Reflect, Clone, DatumComponent)]
#[reflect(Component)]
pub struct NeedsResources(pub bool);


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

// Sync dogoap Satiety/Energy values to UnitNeedsV2 for display and other systems
pub fn sync_dogoap_to_unit_needs(
    mut query: Query<(
        &crate::ai::bevy_dogoap_impl::Satiety,
        &crate::ai::bevy_dogoap_impl::Energy,
        &mut crate::components::UnitNeedsV2,
    )>,
) {
    for (satiety, energy, mut needs) in query.iter_mut() {
        // Convert GOAP values (0-100) to UnitNeeds (0.0-1.0)
        needs.set_hunger_from_dogoap(satiety.0 as f32);
        needs.set_energy_from_dogoap(energy.0 as f32);
    }
}

// Decide which AI mode to use based on current state
pub fn ai_mode_selection_system(
    mut query: Query<(
        &crate::ai::bevy_dogoap_impl::Satiety,
        &crate::ai::bevy_dogoap_impl::Energy,
        &mut AIMode,
        &GoalPriorities,
    )>,
    debug: Res<crate::debug::DebugSystem>,
) {
    for (satiety, energy, mut mode, priorities) in query.iter_mut() {
        // Use dogoap values directly (0 = bad, 100 = good)
        let new_mode = if satiety.0 < 30.0 || energy.0 < 30.0 {
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