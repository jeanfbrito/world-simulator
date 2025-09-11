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

/// System that manages behavior state transitions based on needs
pub fn behavior_state_machine_system(
    mut query: Query<
        (
            &mut BehaviorCycle,
            &crate::components::UnitNeeds,
            &crate::components::UnitInventory,
            &mut crate::ai::ActionPlan,
            &crate::components::NameComponent,
        ),
        With<crate::components::PeasantTag>,
    >,
    time: Res<Time>,
    debug: Res<crate::debug::DebugSystem>,
    sim_state: Res<crate::SimulationState>,
) {
    use crate::debug::DebugLevel;

    if !sim_state.just_ticked {
        return;
    }

    for (mut cycle, needs, inventory, mut plan, name) in query.iter_mut() {
        // Update timer
        cycle.state_timer += time.delta_secs();

        // Determine next state based on priorities
        let next_state = determine_next_state(
            &cycle.current_state,
            needs,
            inventory,
            cycle.state_timer >= cycle.state_duration,
        );

        // Transition to new state if needed
        if next_state != cycle.current_state {
            debug.log(
                DebugLevel::Info,
                "BEHAVIOR",
                &format!(
                    "{} transitioning from {:?} to {:?}",
                    name.name, cycle.current_state, next_state
                ),
            );

            // Clear any existing plan when changing states
            *plan = crate::ai::ActionPlan::new(Vec::new());

            // Set state duration based on the new state
            cycle.state_duration = match next_state {
                BehaviorState::Resting => 3.0,
                BehaviorState::Eating => 1.0,
                BehaviorState::Gathering => 5.0,
                BehaviorState::Building => 10.0,
                BehaviorState::Moving => 2.0,
                BehaviorState::Working => 5.0,
                BehaviorState::Idle => 1.0,
            };

            cycle.current_state = next_state;
            cycle.state_timer = 0.0;
        }
    }
}

fn determine_next_state(
    current: &BehaviorState,
    needs: &crate::components::UnitNeeds,
    inventory: &crate::components::UnitInventory,
    timeout: bool,
) -> BehaviorState {
    // Priority 1: Critical needs
    if needs.energy <= 0.1 {
        return BehaviorState::Resting;
    }

    if needs.hunger >= 0.8 && inventory.get_amount(crate::resources::ResourceType::Berries) > 0 {
        return BehaviorState::Eating;
    }

    // Priority 2: High needs
    if needs.energy < 0.3 && current != &BehaviorState::Resting {
        return BehaviorState::Resting;
    }

    if needs.hunger > 0.5 && inventory.get_amount(crate::resources::ResourceType::Berries) > 0 {
        return BehaviorState::Eating;
    }

    // Priority 3: Resource gathering when low on supplies
    if inventory.get_amount(crate::resources::ResourceType::Berries) < 2 && needs.energy > 0.3 {
        return BehaviorState::Gathering;
    }

    // Priority 4: Work when healthy
    if needs.energy > 0.5 && needs.hunger < 0.5 {
        if inventory.get_amount(crate::resources::ResourceType::Wood) < 10 {
            return BehaviorState::Gathering;
        }
        return BehaviorState::Working;
    }

    // Default: Stay in current state unless timeout
    if timeout {
        return BehaviorState::Idle;
    }

    current.clone()
}
