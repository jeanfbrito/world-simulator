//! Social alert propagation - dangers and opportunities spread through groups

use bevy_ecs::prelude::*;
use std::collections::HashMap;
use world_sim_interface::Position;
use bevy::prelude::Time;
use crate::components::*;
use std::collections::HashSet;

/// Types of alerts that can spread socially
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertType {
    Danger(f32),        // Danger level
    Opportunity(f32),   // Value
    NeedHelp,          // Worker needs assistance
    EnemySpotted,      // Combat alert
    ResourceFound,     // Valuable resource discovered
}

/// Component for entities that can emit alerts
#[derive(Component)]
pub struct AlertEmitter {
    pub alert_type: AlertType,
    pub range: f32,
    pub intensity: f32,
    pub duration: f32,
}

/// Component for received alerts
#[derive(Component)]
pub struct ReceivedAlert {
    pub alert_type: AlertType,
    pub source_position: Position,
    pub received_at: f32,
    pub intensity: f32,
}

/// Component for workers who spread alerts
#[derive(Component)]
pub struct AlertSpreader {
    pub spread_range: f32,
    pub spread_delay: f32,
    pub last_spread: f32,
}

impl Default for AlertSpreader {
    fn default() -> Self {
        Self {
            spread_range: 15.0,
            spread_delay: 0.5,
            last_spread: 0.0,
        }
    }
}

/// System that propagates alerts between nearby entities
pub fn propagate_alerts_system(
    mut commands: Commands,
    time: Res<Time>,
    emitters: Query<(Entity, &PositionComponent, &AlertEmitter)>,
    mut receivers: Query<(
        Entity,
        &PositionComponent,
        Option<&mut ReceivedAlert>,
        Option<&WorkerComponent>,
        Option<&WarriorComponent>,
    ), Without<AlertEmitter>>,
    spatial_grid: Res<SpatialGrid>,
) {
    let current_time = time.elapsed_secs();
    
    for (emitter_entity, emitter_pos, alert) in emitters.iter() {
        // Get nearby entities from spatial grid (much faster than checking all)
        let nearby = spatial_grid.get_entities_in_range(emitter_pos, alert.range);
        
        for nearby_entity in nearby {
            if let Ok((entity, pos, existing_alert, worker, warrior)) = receivers.get_mut(nearby_entity) {
                let distance = emitter_pos.distance_to(pos);
                
                if distance <= alert.range {
                    // Calculate alert intensity based on distance
                    let intensity = alert.intensity * (1.0 - distance / alert.range);
                    
                    // Different entity types react differently
                    let should_receive = match alert.alert_type {
                        AlertType::Danger(_) => true, // Everyone reacts to danger
                        AlertType::EnemySpotted => warrior.is_some() || worker.is_some(),
                        AlertType::NeedHelp => worker.is_some(),
                        AlertType::Opportunity(_) => worker.is_some() && warrior.is_none(),
                        AlertType::ResourceFound => worker.is_some(),
                    };
                    
                    if should_receive {
                        // Warriors amplify combat alerts
                        let final_intensity = if warrior.is_some() && 
                            matches!(alert.alert_type, AlertType::EnemySpotted | AlertType::Danger(_)) {
                            intensity * 1.5
                        } else {
                            intensity
                        };
                        
                        // Update or create alert
                        if let Some(mut existing) = existing_alert {
                            if final_intensity > existing.intensity {
                                existing.alert_type = alert.alert_type;
                                existing.source_position = Position { 
                                    x: emitter_pos.x(), 
                                    y: emitter_pos.y() 
                                };
                                existing.intensity = final_intensity;
                                existing.received_at = current_time;
                            }
                        } else {
                            commands.entity(entity).insert(ReceivedAlert {
                                alert_type: alert.alert_type,
                                source_position: Position { 
                                    x: emitter_pos.x(), 
                                    y: emitter_pos.y() 
                                },
                                received_at: current_time,
                                intensity: final_intensity,
                            });
                            
                            // Mark for immediate AI processing
                            commands.entity(entity).insert(super::priority_queue::ProcessAlert);
                        }
                    }
                }
            }
        }
    }
}

/// System that makes entities react to alerts
pub fn react_to_alerts_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &ReceivedAlert,
        &mut crate::ai::AICoordinator,
        Option<&mut crate::ai::priority_queue::AIPriorityComponent>,
        Option<&WorkerComponent>,
        Option<&WarriorComponent>,
    )>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();
    
    for (entity, alert, mut coordinator, priority, worker, warrior) in query.iter_mut() {
        // Alert decay - old alerts are ignored
        if current_time - alert.received_at > 5.0 {
            commands.entity(entity).remove::<ReceivedAlert>();
            continue;
        }
        
        match alert.alert_type {
            AlertType::Danger(level) if level > 0.5 => {
                // High danger - immediate utility takeover
                coordinator.switch_to_utility();
                
                if let Some(mut p) = priority {
                    p.current_priority = super::priority_queue::AIPriority::Critical;
                }
            }
            AlertType::EnemySpotted => {
                // Enemy spotted - immediate utility takeover
                coordinator.switch_to_utility();
                
                if let Some(mut p) = priority {
                    p.current_priority = super::priority_queue::AIPriority::Critical;
                }
                
                // Warriors move toward danger, workers flee
                if warrior.is_some() {
                    commands.entity(entity).insert(MoveToPosition {
                        target: alert.source_position,
                        speed_multiplier: 1.5,
                    });
                } else {
                    commands.entity(entity).insert(FleeFromPosition {
                        danger_pos: alert.source_position,
                        speed_multiplier: 1.5,
                    });
                }
            }
            
            AlertType::Opportunity(value) if value > 30.0 => {
                // Valuable opportunity - consider interrupting
                if coordinator.mode == crate::ai::AIMode::GoalDriven {
                    // Only interrupt if really valuable
                    if value > 50.0 {
                        coordinator.switch_to_utility();
                    }
                }
            }
            
            AlertType::NeedHelp if worker.is_some() => {
                // Help request - increase social scorer weight
                commands.entity(entity).insert(PrioritizeHelping {
                    target_position: alert.source_position,
                });
            }
            
            _ => {}
        }
    }
}

/// System for alert spreading (word-of-mouth)
pub fn spread_alerts_system(
    mut spreaders: Query<(
        Entity,
        &mut AlertSpreader,
        &ReceivedAlert,
        &PositionComponent,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();
    
    for (entity, mut spreader, alert, pos) in spreaders.iter_mut() {
        // Only spread recent, intense alerts
        if alert.intensity > 0.5 && 
           current_time - alert.received_at < 2.0 &&
           current_time - spreader.last_spread > spreader.spread_delay {
            
            // Become an emitter temporarily
            commands.entity(entity).insert(AlertEmitter {
                alert_type: alert.alert_type,
                range: spreader.spread_range,
                intensity: alert.intensity * 0.8, // Reduce intensity when spreading
                duration: 1.0,
            });
            
            spreader.last_spread = current_time;
        }
    }
}

/// Clean up expired alert emitters
pub fn cleanup_alerts_system(
    mut commands: Commands,
    mut emitters: Query<(Entity, &mut AlertEmitter)>,
    time: Res<Time>,
) {
    for (entity, mut emitter) in emitters.iter_mut() {
        emitter.duration -= time.delta_secs();
        if emitter.duration <= 0.0 {
            commands.entity(entity).remove::<AlertEmitter>();
        }
    }
}

/// Spatial grid for efficient range queries
#[derive(Resource)]
pub struct SpatialGrid {
    cell_size: f32,
    grid: HashMap<(i32, i32), HashSet<Entity>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            grid: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, entity: Entity, pos: &PositionComponent) {
        let cell = self.get_cell(pos);
        self.grid.entry(cell).or_insert_with(HashSet::new).insert(entity);
    }
    
    pub fn remove(&mut self, entity: Entity, pos: &PositionComponent) {
        let cell = self.get_cell(pos);
        if let Some(entities) = self.grid.get_mut(&cell) {
            entities.remove(&entity);
        }
    }
    
    pub fn get_entities_in_range(&self, pos: &PositionComponent, range: f32) -> Vec<Entity> {
        let mut result = Vec::new();
        let cell_range = (range / self.cell_size).ceil() as i32;
        let center_cell = self.get_cell(pos);
        
        for dx in -cell_range..=cell_range {
            for dy in -cell_range..=cell_range {
                let cell = (center_cell.0 + dx, center_cell.1 + dy);
                if let Some(entities) = self.grid.get(&cell) {
                    result.extend(entities.iter());
                }
            }
        }
        
        result
    }
    
    fn get_cell(&self, pos: &PositionComponent) -> (i32, i32) {
        (
            (pos.x as f32 / self.cell_size) as i32,
            (pos.y as f32 / self.cell_size) as i32,
        )
    }
}

impl Default for SpatialGrid {
    fn default() -> Self {
        Self::new(10.0) // 10 unit cells
    }
}

/// Component for warriors
#[derive(Component)]
pub struct WarriorComponent {
    pub combat_skill: f32,
    pub courage: f32,
}

/// Movement commands from alerts
#[derive(Component)]
pub struct MoveToPosition {
    pub target: Position,
    pub speed_multiplier: f32,
}

#[derive(Component)]
pub struct FleeFromPosition {
    pub danger_pos: Position,
    pub speed_multiplier: f32,
}

#[derive(Component)]
pub struct PrioritizeHelping {
    pub target_position: Position,
}

/// System to update spatial grid
pub fn update_spatial_grid_system(
    mut grid: ResMut<SpatialGrid>,
    moved: Query<(Entity, &PositionComponent), Changed<PositionComponent>>,
    mut last_positions: Local<HashMap<Entity, PositionComponent>>,
) {
    for (entity, pos) in moved.iter() {
        // Remove from old position
        if let Some(old_pos) = last_positions.get(&entity) {
            grid.remove(entity, old_pos);
        }
        
        // Add to new position
        grid.insert(entity, pos);
        last_positions.insert(entity, pos.clone());
    }
}