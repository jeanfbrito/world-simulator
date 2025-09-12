use crate::ai::{TaskPriority, TaskStatus, TaskType};
use crate::buildings::BuildingType;
use crate::resources::{Item, ResourceType};
use crate::tilemap::{BiomeType, ChunkCoordinate, TerrainType};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveState {
    pub version: String,
    pub timestamp: u64,
    pub tick: u32,
    pub chunks: Vec<ChunkData>,
    pub entities: Vec<EntityData>,
    pub buildings: Vec<BuildingData>,
    pub metadata: SaveMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMetadata {
    pub world_seed: u64,
    pub play_time: f32,
    pub save_name: String,
    pub difficulty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkData {
    pub coordinate: ChunkCoordinate,
    pub biome: BiomeType,
    pub tiles: Vec<Vec<TerrainType>>,
    pub resources: HashMap<(usize, usize), ResourceType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
    pub id: u64,
    pub entity_type: EntityType,
    pub position: (f32, f32, f32),
    pub components: ComponentData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Worker,
    Resource,
    Building,
    Item,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
    pub name: Option<String>,
    pub health: Option<f32>,
    pub max_health: Option<f32>,
    pub energy: Option<f32>,
    pub max_energy: Option<f32>,
    pub inventory: Option<InventoryData>,
    pub unit_stats: Option<UnitStatsData>,
    pub current_task: Option<TaskData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryData {
    pub items: Vec<(Item, u32)>,
    pub max_weight: f32,
    pub max_slots: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitStatsData {
    pub work_speed: f32,
    pub move_speed: f32,
    pub carry_capacity: f32,
    pub experience: u32,
    pub level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub id: usize,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub progress: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingData {
    pub id: u64,
    pub building_type: BuildingType,
    pub position: (usize, usize),
    pub health: f32,
    pub construction_progress: f32,
    pub is_complete: bool,
}

impl SaveState {
    pub fn new(save_name: String) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        info!("[SAVE] Creating new save state: {}", save_name);

        Self {
            version: "0.1.0".to_string(),
            timestamp,
            tick: 0,
            chunks: Vec::new(),
            entities: Vec::new(),
            buildings: Vec::new(),
            metadata: SaveMetadata {
                world_seed: 0,
                play_time: 0.0,
                save_name,
                difficulty: "Normal".to_string(),
            },
        }
    }

    pub fn from_world(
        save_name: String,
        tick: u32,
        chunks: Vec<ChunkData>,
        entities: Vec<EntityData>,
        buildings: Vec<BuildingData>,
    ) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        info!(
            "[SAVE] Creating save state from world: {} chunks, {} entities, {} buildings",
            chunks.len(),
            entities.len(),
            buildings.len()
        );

        Self {
            version: "0.1.0".to_string(),
            timestamp,
            tick,
            chunks,
            entities,
            buildings,
            metadata: SaveMetadata {
                world_seed: 0,
                play_time: 0.0,
                save_name,
                difficulty: "Normal".to_string(),
            },
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // Validate save data integrity
        if self.version.is_empty() {
            return Err("Invalid version".to_string());
        }

        if self.metadata.save_name.is_empty() {
            return Err("Invalid save name".to_string());
        }

        info!("[SAVE] Save state validated successfully");
        Ok(())
    }
}
