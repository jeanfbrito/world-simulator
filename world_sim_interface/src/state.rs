//! State snapshot structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{EntityId, Position, SettlementId, Tick};
use super::entities::*;

/// Complete world state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub tick: Tick,
    pub entities: Vec<EntitySnapshot>,
    pub settlements: Vec<SettlementSnapshot>,
    pub global: GlobalState,
}

/// Snapshot of a single entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub position: Position,
    pub components: HashMap<String, serde_json::Value>,
}

/// Snapshot of a settlement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementSnapshot {
    pub id: SettlementId,
    pub name: String,
    pub position: Position,
    pub population: u32,
    pub happiness: f32,
    pub resources: HashMap<ResourceType, u32>,
    pub buildings: Vec<EntityId>,
}

/// Global game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    pub season: Season,
    pub weather: Weather,
    pub game_speed: f32,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            season: Season::Spring,
            weather: Weather::Clear,
            game_speed: 1.0,
        }
    }
}

impl Default for WorldSnapshot {
    fn default() -> Self {
        Self {
            tick: 0,
            entities: Vec::new(),
            settlements: Vec::new(),
            global: GlobalState::default(),
        }
    }
}