//! Event definitions for engine output

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{EntityId, Position, Tick};
use super::entities::*;

/// Events emitted by the simulation engine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EngineEvent {
    // === World Events ===
    WorldCreated {
        width: u32,
        height: u32,
        seed: Option<u64>,
    },
    
    Tick {
        tick: u64,
    },
    
    // === Command Events ===
    CommandReceived {
        command: Option<serde_json::Value>,
    },
    
    CommandExecuted {
        success: bool,
        result: Option<serde_json::Value>,
    },
    
    // === Entity Lifecycle ===
    EntitySpawned {
        entity_id: EntityId,
        entity_type: EntityType,
        position: Position,
    },
    
    EntitySpawnedDetailed {
        id: EntityId,
        entity_type: EntityType,
        position: Position,
        components: HashMap<String, serde_json::Value>,
    },
    
    EntityMoved {
        id: EntityId,
        from: Position,
        to: Position,
        path: Vec<Position>,
        duration: f32,
    },
    
    EntityDestroyed {
        id: EntityId,
        reason: DestroyReason,
    },
    
    ComponentChanged {
        entity_id: EntityId,
        component: String,
        old_value: serde_json::Value,
        new_value: serde_json::Value,
    },
    
    // === Game Events ===
    HarvestStarted {
        worker_id: EntityId,
        resource_id: EntityId,
    },
    
    HarvestCompleted {
        worker_id: EntityId,
        resource_id: EntityId,
        amount: u32,
    },
    
    ResourceCollected {
        worker_id: EntityId,
        resource_type: ResourceType,
        amount: u32,
    },
    
    ResourceHarvested {
        worker_id: EntityId,
        resource_id: EntityId,
        resource_type: ResourceType,
        amount: u32,
    },
    
    ResourceDepleted {
        resource_id: EntityId,
        will_regenerate: bool,
        regeneration_time: Option<Tick>,
    },
    
    ConstructionStarted {
        building_type: BuildingType,
        position: Position,
        builder_id: EntityId,
    },
    
    ConstructionCompleted {
        building_id: EntityId,
        building_type: BuildingType,
        position: Position,
    },
    
    BuildingConstructionStarted {
        building_id: EntityId,
        building_type: BuildingType,
        position: Position,
        workers: Vec<EntityId>,
        estimated_completion: Tick,
    },
    
    BuildingConstructionProgress {
        building_id: EntityId,
        progress: f32, // 0.0 to 1.0
    },
    
    BuildingCompleted {
        building_id: EntityId,
        building_type: BuildingType,
    },
    
    RecipeStarted {
        recipe_id: String,
        building_id: EntityId,
        inputs_consumed: Vec<(ResourceType, u32)>,
        estimated_completion: Tick,
    },
    
    RecipeCompleted {
        recipe_id: String,
        building_id: EntityId,
        outputs: Vec<(ResourceType, u32)>,
    },
    
    // === Population Events ===
    WorkerAssigned {
        worker_id: EntityId,
        task: TaskType,
        target: Option<EntityId>,
    },
    
    WorkerIdle {
        worker_id: EntityId,
    },
    
    PopulationChanged {
        settlement_id: super::SettlementId,
        old_count: u32,
        new_count: u32,
        reason: PopulationChangeReason,
    },
    
    // === Resource Events ===
    ResourcesChanged {
        settlement_id: super::SettlementId,
        resource_type: ResourceType,
        old_amount: u32,
        new_amount: u32,
    },
    
    StorageCapacityReached {
        building_id: EntityId,
        resource_type: ResourceType,
    },
    
    // === Time Events ===
    TickCompleted {
        tick: Tick,
        delta_time: f32,
    },
    
    SeasonChanged {
        old_season: Season,
        new_season: Season,
    },
}