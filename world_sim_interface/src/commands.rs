//! Command definitions for engine input

use serde::{Deserialize, Serialize};
use super::{EntityId, Position, SettlementId};
use super::entities::*;
use std::collections::HashMap;

/// Commands that can be sent to the simulation engine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EngineCommand {
    // === Movement ===
    Move {
        entity_id: EntityId,
        target: Position,
    },
    
    // === Resource Management ===
    Harvest {
        worker_id: EntityId,
        resource_id: EntityId,
    },
    
    HarvestResource {
        worker_ids: Vec<EntityId>,
        target_id: EntityId,
    },
    
    GiveResources {
        entity_id: EntityId,
        resources: HashMap<ResourceType, u32>,
    },
    
    Store {
        worker_id: EntityId,
        building_id: EntityId,
    },
    
    // === Building ===
    Build {
        builder_id: EntityId,
        building_type: BuildingType,
        position: Position,
    },
    
    ConstructBuilding {
        building_type: BuildingType,
        position: Position,
        workers: Vec<EntityId>,
    },
    
    CancelConstruction {
        building_id: EntityId,
    },
    
    // === Worker Management ===
    AssignWorker {
        worker_id: EntityId,
        building_id: EntityId,
    },
    
    AssignWorkerTask {
        worker_id: EntityId,
        task: TaskAssignment,
    },
    
    SpawnWorker {
        position: Position,
        settlement_id: Option<SettlementId>,
    },
    
    // === Production ===
    StartRecipe {
        recipe_id: RecipeId,
        building_id: EntityId,
    },
    
    // === Movement (legacy) ===
    MoveEntity {
        entity_id: EntityId,
        target: Position,
    },
    
    // === Queries ===
    QueryArea {
        min: Position,
        max: Position,
    },
    
    QueryEntity {
        id: EntityId,
    },
    
    QueryResources {
        settlement_id: SettlementId,
    },
    
    QueryAvailableRecipes {
        building_id: Option<EntityId>,
    },
    
    // === Debug Commands ===
    #[cfg(debug_assertions)]
    SpawnEntity {
        entity_type: EntityType,
        position: Position,
    },
    
    #[cfg(debug_assertions)]
    SetResource {
        settlement_id: SettlementId,
        resource_type: ResourceType,
        amount: u32,
    },
}