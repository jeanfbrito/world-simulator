use super::{Chunk, ChunkCoordinate};
use crate::components::PositionComponent;
use crate::debug::{DebugLevel, DebugSystem};
use bevy::prelude::*;
use std::collections::HashMap;

pub const VIEW_DISTANCE: i32 = 3; // Load chunks within 3 chunks of the player
pub const UNLOAD_DISTANCE: i32 = 5; // Unload chunks beyond 5 chunks

#[derive(Resource)]
pub struct ChunkManager {
    loaded_chunks: HashMap<ChunkCoordinate, Entity>,
    view_distance: i32,
    unload_distance: i32,
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self {
            loaded_chunks: HashMap::new(),
            view_distance: VIEW_DISTANCE,
            unload_distance: UNLOAD_DISTANCE,
        }
    }
}

impl ChunkManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_loaded(&self, coord: &ChunkCoordinate) -> bool {
        self.loaded_chunks.contains_key(coord)
    }

    pub fn get_chunk_entity(&self, coord: &ChunkCoordinate) -> Option<Entity> {
        self.loaded_chunks.get(coord).copied()
    }

    pub fn register_chunk(&mut self, coord: ChunkCoordinate, entity: Entity) {
        self.loaded_chunks.insert(coord, entity);
    }

    pub fn unregister_chunk(&mut self, coord: &ChunkCoordinate) -> Option<Entity> {
        self.loaded_chunks.remove(coord)
    }

    pub fn get_chunks_to_load(&self, center: ChunkCoordinate) -> Vec<ChunkCoordinate> {
        let mut chunks_to_load = Vec::new();

        for dx in -self.view_distance..=self.view_distance {
            for dy in -self.view_distance..=self.view_distance {
                let coord = ChunkCoordinate {
                    x: center.x + dx,
                    y: center.y + dy,
                };

                if !self.is_loaded(&coord) {
                    chunks_to_load.push(coord);
                }
            }
        }

        chunks_to_load
    }

    pub fn get_chunks_to_unload(&self, center: ChunkCoordinate) -> Vec<ChunkCoordinate> {
        let mut chunks_to_unload = Vec::new();

        for coord in self.loaded_chunks.keys() {
            let dx = (coord.x - center.x).abs();
            let dy = (coord.y - center.y).abs();

            if dx > self.unload_distance || dy > self.unload_distance {
                chunks_to_unload.push(*coord);
            }
        }

        chunks_to_unload
    }
}

pub fn chunk_loading_system(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    query_positions: Query<&PositionComponent>,
    debug: Res<DebugSystem>,
) {
    // Find the center of all entities (simplified - in real game, track player)
    if query_positions.is_empty() {
        return;
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut count = 0;

    for pos in query_positions.iter() {
        sum_x += pos.x;
        sum_y += pos.y;
        count += 1;
    }

    if count == 0 {
        return;
    }

    let center_x = sum_x / count as f32;
    let center_y = sum_y / count as f32;

    // Convert to chunk coordinates
    let center_chunk = ChunkCoordinate::from_world_position(center_x, center_y, 10.0);

    // Load new chunks
    let chunks_to_load = chunk_manager.get_chunks_to_load(center_chunk);
    for coord in chunks_to_load {
        let chunk = Chunk::new(coord);
        let entity = commands.spawn(chunk).id();
        chunk_manager.register_chunk(coord, entity);

        debug.log(
            DebugLevel::Debug,
            "CHUNK",
            &format!("Loaded chunk ({}, {})", coord.x, coord.y),
        );
    }
}

pub fn chunk_unloading_system(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    query_positions: Query<&PositionComponent>,
    debug: Res<DebugSystem>,
) {
    // Find the center of all entities
    if query_positions.is_empty() {
        return;
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut count = 0;

    for pos in query_positions.iter() {
        sum_x += pos.x;
        sum_y += pos.y;
        count += 1;
    }

    if count == 0 {
        return;
    }

    let center_x = sum_x / count as f32;
    let center_y = sum_y / count as f32;

    // Convert to chunk coordinates
    let center_chunk = ChunkCoordinate::from_world_position(center_x, center_y, 10.0);

    // Unload distant chunks
    let chunks_to_unload = chunk_manager.get_chunks_to_unload(center_chunk);
    for coord in chunks_to_unload {
        if let Some(entity) = chunk_manager.unregister_chunk(&coord) {
            commands.entity(entity).despawn();

            debug.log(
                DebugLevel::Debug,
                "CHUNK",
                &format!("Unloaded chunk ({}, {})", coord.x, coord.y),
            );
        }
    }
}
