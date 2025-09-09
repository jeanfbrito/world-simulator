//! Utility AI implementation using big-brain for reactive behaviors

use bevy_ecs::prelude::*;
use big_brain::prelude::*;
use world_sim_interface::Position;
use crate::components::*;
use super::scorers::*;
use super::utility_actions::*;
use super::coordinator::AICoordinator;

/// Thinker configuration for basic worker utility AI
pub fn create_worker_thinker() -> ThinkerBuilder {
    Thinker::build()
        .label("WorkerUtilityAI")
        .picker(FirstToScore::new(0.8)) // Actions trigger at 80% score
        .when(
            HungerScorer,
            EmergencyEatAction,
        )
        .when(
            FatigueScorer, 
            EmergencyRestAction,
        )
        .when(
            ThreatScorer { threat_range: 10.0 },
            FleeAction,
        )
        .when(
            OpportunityScorer { opportunity_range: 15.0 },
            GrabResourceAction,
        )
}

/// Advanced thinker with more nuanced behaviors
pub fn create_advanced_worker_thinker() -> ThinkerBuilder {
    Thinker::build()
        .label("AdvancedWorkerAI")
        .picker(Highest) // Always pick highest scoring action
        .when(
            CriticalHungerScorer,
            EmergencyEatAction,
        )
        .when(
            ExhaustionScorer,
            EmergencySleepAction,
        )
        .when(
            DangerScorer,
            DefensiveAction,
        )
        .when(
            ProfitScorer,
            TradeAction,
        )
        .when(
            SocialScorer { help_range: 20.0 },
            HelpAllyAction,
        )
}

/// Spawn a worker with hybrid AI (both GOAP and Utility)
pub fn spawn_hybrid_worker(
    commands: &mut Commands,
    position: Position,
    name: String,
) -> Entity {
    // First create the GOAP planner
    let (goap_planner, goap_components) = crate::ai::create_worker_planner();
    
    // Then add utility AI thinker
    let utility_thinker = create_worker_thinker();
    
    commands.spawn((
        // Core components
        WorkerComponent::new(name.clone()),
        PositionComponent::new(position.x, position.y),
        MovementComponent::new(5.0),
        InventoryComponent::new(20),
        
        // GOAP components
        goap_planner,
        goap_components,
        
        // Utility AI components
        utility_thinker.build(),
        
        // Coordinator to manage both systems
        AICoordinator::new(),
        
        // Worker component with name
        // Name component removed - name is stored in WorkerComponent
    )).id()
}