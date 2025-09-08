//! Social and cooperation-related scorers for Utility AI

use bevy_ecs::prelude::*;
use big_brain::prelude::*;
use crate::components::*;

/// Scores based on nearby allies needing help
#[derive(Component, ScorerBuilder, Clone)]
pub struct SocialScorer {
    pub help_range: f32,
}

/// Scores based on worker morale/happiness
#[derive(Component, ScorerBuilder, Clone)]
pub struct MoraleScorer;

/// Scores based on isolation (too far from other workers)
#[derive(Component, ScorerBuilder, Clone)]
pub struct IsolationScorer {
    pub comfort_range: f32,
}

/// System that scores need to help nearby allies
pub fn social_scorer_system(
    mut query: Query<(&Actor, &mut Score, &SocialScorer)>,
    workers: Query<(&PositionComponent, &WorkerComponent)>,
    needy_workers: Query<(&PositionComponent, &IsHungry, &HasEnergy), With<WorkerComponent>>,
) {
    for (Actor(actor), mut score, scorer) in query.iter_mut() {
        if let Ok((my_pos, _)) = workers.get(*actor) {
            let mut help_score = 0.0;
            
            for (other_pos, hunger, energy) in needy_workers.iter() {
                let distance = my_pos.distance_to(other_pos);
                if distance < scorer.help_range && distance > 0.1 {
                    // Check if ally needs help
                    let need_level = if hunger.0 > 70.0 || energy.0 < 20.0 {
                        0.8
                    } else if hunger.0 > 50.0 || energy.0 < 40.0 {
                        0.4
                    } else {
                        0.0
                    };
                    
                    if need_level > 0.0 {
                        let proximity_factor = 1.0 - (distance / scorer.help_range);
                        help_score = help_score.max(need_level * proximity_factor * 0.6);
                    }
                }
            }
            
            score.set(help_score);
        }
    }
}

/// System that scores based on worker morale
pub fn morale_scorer_system(
    mut query: Query<(&Actor, &mut Score, &MoraleScorer)>,
    workers: Query<&WorkerComponent>,
) {
    for (Actor(actor), mut score, _) in query.iter_mut() {
        if let Ok(worker) = workers.get(*actor) {
            // Low happiness increases score for morale-boosting actions
            if worker.happiness < 0.3 {
                score.set(0.7);
            } else if worker.happiness < 0.5 {
                score.set(0.4);
            } else {
                score.set(0.0);
            }
        }
    }
}

/// System that scores based on isolation from other workers
pub fn isolation_scorer_system(
    mut query: Query<(&Actor, &mut Score, &IsolationScorer)>,
    workers: Query<(Entity, &PositionComponent), With<WorkerComponent>>,
) {
    for (Actor(actor), mut score, scorer) in query.iter_mut() {
        if let Ok((_, my_pos)) = workers.get(*actor) {
            let mut nearby_count = 0;
            
            for (other_entity, other_pos) in workers.iter() {
                if other_entity != *actor {
                    let distance = my_pos.distance_to(other_pos);
                    if distance < scorer.comfort_range {
                        nearby_count += 1;
                    }
                }
            }
            
            // Score higher when isolated
            if nearby_count == 0 {
                score.set(0.6);
            } else if nearby_count == 1 {
                score.set(0.3);
            } else {
                score.set(0.0);
            }
        }
    }
}