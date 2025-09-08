// Engine API Contract - Rust Interface Definitions
// This file defines the public API for the headless simulation engine

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// ============================================================================
// Core Types
// ============================================================================

pub type EntityId = u64;
pub type SettlementId = u64;
pub type PlayerId = u64;
pub type Tick = u64;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// ============================================================================
// Engine Events (Output)
// ============================================================================

/// Events emitted by the engine that visualizers can subscribe to
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EngineEvent {
    // === Entity Lifecycle ===
    EntitySpawned {
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
        settlement_id: SettlementId,
        old_count: u32,
        new_count: u32,
        reason: PopulationChangeReason,
    },
    
    // === Resource Events ===
    ResourcesChanged {
        settlement_id: SettlementId,
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

// ============================================================================
// Engine Commands (Input)
// ============================================================================

/// Commands that can be sent to the engine
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

// ============================================================================
// Command Results
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<serde_json::Value>,
}

// ============================================================================
// State Queries
// ============================================================================

/// Complete world state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub tick: Tick,
    pub entities: Vec<EntitySnapshot>,
    pub settlements: Vec<SettlementSnapshot>,
    pub global: GlobalState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub position: Position,
    pub components: HashMap<String, serde_json::Value>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    pub season: Season,
    pub weather: Weather,
    pub game_speed: f32,
}

// ============================================================================
// Enums and Types
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    // Resources
    Tree,
    BerryBush,
    StoneDeposit,
    IronOre,
    
    // Units
    Worker,
    
    // Buildings
    House,
    Stockpile,
    Granary,
    Sawmill,
    Quarry,
    Farm,
    
    // Items
    ResourceItem(ResourceType),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResourceType {
    // Raw
    Wood,
    Stone,
    Iron,
    Berries,
    Wheat,
    
    // Processed
    Planks,
    Bread,
    Tools,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BuildingType {
    House,
    Stockpile,
    Granary,
    Sawmill,
    Quarry,
    Farm,
    Bakery,
    Smithy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Idle,
    Harvesting,
    Building,
    Crafting,
    Hauling,
    Farming,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskAssignment {
    Harvest { target: EntityId },
    Build { building: EntityId },
    Craft { recipe: String, building: EntityId },
    Haul { from: Position, to: Position, resource: ResourceType },
    Farm { field: EntityId },
    Idle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DestroyReason {
    Harvested,
    Demolished,
    Decayed,
    Attacked,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PopulationChangeReason {
    Birth,
    Death,
    Immigration,
    Emigration,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Rain,
    Snow,
    Storm,
}

// ============================================================================
// Observer Pattern
// ============================================================================

/// Trait for components that want to observe engine events
pub trait EngineObserver: Send + Sync {
    /// Called when events occur
    fn on_events(&mut self, events: &[EngineEvent]);
    
    /// Called when a full snapshot is available
    fn on_snapshot(&mut self, snapshot: &WorldSnapshot);
    
    /// Return true if this observer wants full snapshots
    fn wants_snapshots(&self) -> bool { false }
}

// ============================================================================
// Engine Interface
// ============================================================================

/// Main interface to the simulation engine
pub trait SimulationEngine {
    /// Create a new world
    fn new_world(&mut self, config: WorldConfig) -> Result<(), String>;
    
    /// Load a saved world
    fn load_world(&mut self, save_data: &[u8]) -> Result<(), String>;
    
    /// Save current world state
    fn save_world(&self) -> Result<Vec<u8>, String>;
    
    /// Execute a command
    fn execute_command(&mut self, command: EngineCommand) -> CommandResult;
    
    /// Run one simulation tick
    fn tick(&mut self, delta_time: f32);
    
    /// Get current world snapshot
    fn snapshot(&self) -> WorldSnapshot;
    
    /// Add an observer
    fn add_observer(&mut self, observer: Box<dyn EngineObserver>);
    
    /// Remove an observer
    fn remove_observer(&mut self, id: usize);
}

// ============================================================================
// Configuration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub width: u32,
    pub height: u32,
    pub seed: Option<u64>,
    pub resource_density: f32,
    pub starting_workers: u32,
    pub seasons_enabled: bool,
    pub resource_regeneration: bool,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            seed: None,
            resource_density: 0.1,
            starting_workers: 5,
            seasons_enabled: false,
            resource_regeneration: true,
        }
    }
}