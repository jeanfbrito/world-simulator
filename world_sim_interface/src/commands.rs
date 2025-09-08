//! Command definitions for engine input

use serde::{Deserialize, Serialize};
use super::{EntityId, Position, SettlementId};
use super::entities::*;

/// Commands that can be sent to the simulation engine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EngineCommand {
    // === Direct Orders ===
    HarvestResource {
        worker_ids: Vec<EntityId>,
        target_id: EntityId,
    },
    
    ConstructBuilding {
        building_type: BuildingType,
        position: Position,
        workers: Vec<EntityId>,
    },
    
    CancelConstruction {
        building_id: EntityId,
    },
    
    AssignWorker {
        worker_id: EntityId,
        task: TaskAssignment,
    },
    
    MoveEntity {
        entity_id: EntityId,
        target: Position,
    },
    
    StartRecipe {
        recipe_id: String,
        building_id: EntityId,
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