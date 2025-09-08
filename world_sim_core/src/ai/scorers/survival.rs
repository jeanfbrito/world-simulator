//! Survival-related scorers for Utility AI

use bevy_ecs::prelude::*;
use big_brain::prelude::*;
use crate::components::*;

/// Scores based on hunger level
#[derive(Component, ScorerBuilder, Clone)]
pub struct HungerScorer;

#[derive(Component, ScorerBuilder, Clone)]
pub struct CriticalHungerScorer;

/// Scores based on fatigue/energy level
#[derive(Component, ScorerBuilder, Clone)]
pub struct FatigueScorer;

#[derive(Component, ScorerBuilder, Clone)]
pub struct ExhaustionScorer;

/// Scores based on nearby threats
#[derive(Component, ScorerBuilder, Clone)]
pub struct ThreatScorer {
    pub threat_range: f32,
}

#[derive(Component, ScorerBuilder, Clone)]
pub struct DangerScorer;

/// Scores based on nearby opportunities
#[derive(Component, ScorerBuilder, Clone)]
pub struct OpportunityScorer {
    pub opportunity_range: f32,
}

/// System that scores hunger need
pub fn hunger_scorer_system(
    mut query: Query<(&Actor, &mut Score, &HungerScorer)>,
    workers: Query<(&WorkerComponent, &IsHungry)>,
) {
    for (Actor(actor), mut score, _) in query.iter_mut() {
        if let Ok((worker, hunger)) = workers.get(*actor) {
            // Linear scoring: 0% hunger = 0.0 score, 100% hunger = 1.0 score
            score.set((hunger.0 / 100.0) as f32);
        }
    }
}

/// System for critical hunger (non-linear scoring)
pub fn critical_hunger_scorer_system(
    mut query: Query<(&Actor, &mut Score, &CriticalHungerScorer)>,
    workers: Query<(&WorkerComponent, &IsHungry)>,
) {
    for (Actor(actor), mut score, _) in query.iter_mut() {
        if let Ok((worker, hunger)) = workers.get(*actor) {
            // Exponential scoring for critical levels
            if hunger.0 > 80.0 {
                score.set(0.95); // Very high priority
            } else if hunger.0 > 60.0 {
                score.set(0.6);
            } else {
                score.set(0.0);
            }
        }
    }
}

/// System that scores fatigue/low energy
pub fn fatigue_scorer_system(
    mut query: Query<(&Actor, &mut Score, &FatigueScorer)>,
    workers: Query<(&WorkerComponent, &HasEnergy)>,
) {
    for (Actor(actor), mut score, _) in query.iter_mut() {
        if let Ok((worker, energy)) = workers.get(*actor) {
            // Inverse scoring: 100% energy = 0.0 score, 0% energy = 1.0 score
            score.set(1.0 - (energy.0 / 100.0) as f32);
        }
    }
}

/// System for exhaustion (critical energy levels)
pub fn exhaustion_scorer_system(
    mut query: Query<(&Actor, &mut Score, &ExhaustionScorer)>,
    workers: Query<(&WorkerComponent, &HasEnergy)>,
) {
    for (Actor(actor), mut score, _) in query.iter_mut() {
        if let Ok((worker, energy)) = workers.get(*actor) {
            if energy.0 < 10.0 {
                score.set(1.0); // Emergency
            } else if energy.0 < 25.0 {
                score.set(0.7);
            } else {
                score.set(0.0);
            }
        }
    }
}

/// Component marking dangerous entities (wolves, raiders, etc.)
#[derive(Component)]
pub struct Threat {
    pub danger_level: f32,
}

/// System that scores based on nearby threats
pub fn threat_scorer_system(
    mut query: Query<(&Actor, &mut Score, &ThreatScorer)>,
    workers: Query<&PositionComponent>,
    threats: Query<(&PositionComponent, &Threat)>,
) {
    for (Actor(actor), mut score, scorer) in query.iter_mut() {
        if let Ok(worker_pos) = workers.get(*actor) {
            let mut max_threat = 0.0;
            
            for (threat_pos, threat) in threats.iter() {
                let distance = worker_pos.distance_to(threat_pos);
                if distance < scorer.threat_range {
                    // Closer threats score higher
                    let proximity_score = 1.0 - (distance / scorer.threat_range);
                    let threat_score = proximity_score * threat.danger_level;
                    max_threat = max_threat.max(threat_score);
                }
            }
            
            score.set(max_threat);
        }
    }
}

/// Component marking valuable opportunities
#[derive(Component)]
pub struct Opportunity {
    pub value: f32,
}

/// System that scores based on nearby opportunities (valuable resources, etc.)
pub fn opportunity_scorer_system(
    mut query: Query<(&Actor, &mut Score, &OpportunityScorer)>,
    workers: Query<(&PositionComponent, &InventoryComponent)>,
    opportunities: Query<(&PositionComponent, &Opportunity, &ResourceNodeComponent)>,
) {
    for (Actor(actor), mut score, scorer) in query.iter_mut() {
        if let Ok((worker_pos, inventory)) = workers.get(*actor) {
            // Don't pursue opportunities if inventory is full
            if inventory.is_full() {
                score.set(0.0);
                continue;
            }
            
            let mut best_opportunity = 0.0;
            
            for (opp_pos, opportunity, resource) in opportunities.iter() {
                let distance = worker_pos.distance_to(opp_pos);
                if distance < scorer.opportunity_range {
                    // Score based on value and proximity
                    let proximity_score = 1.0 - (distance / scorer.opportunity_range);
                    let value_score = opportunity.value / 100.0; // Normalize value
                    let opportunity_score = proximity_score * value_score * 0.5; // Cap at 0.5
                    best_opportunity = best_opportunity.max(opportunity_score);
                }
            }
            
            score.set(best_opportunity);
        }
    }
}