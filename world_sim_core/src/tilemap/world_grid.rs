//! World grid management and position synchronization

use bevy::prelude::*;
use bevy_entitiles::prelude::*;
use world_sim_interface::Position;
use crate::components::PositionComponent;
use super::{TileType, world_to_tile, tile_to_world};
use std::collections::HashMap;

/// Resource that manages the world grid state
#[derive(Resource, Default)]
pub struct WorldGrid {
    /// Map of positions to tile data
    tiles: HashMap<Position, TileData>,
    /// Entities at each position
    entities_at: HashMap<Position, Vec<Entity>>,
    /// Size of the world
    width: u32,
    height: u32,
}

/// Data for a single tile
#[derive(Clone)]
pub struct TileData {
    pub terrain: TileType,
    pub resource: Option<TileType>,
    pub building: Option<TileType>,
    pub walkable: bool,
    pub movement_cost: f32,
}

impl Default for TileData {
    fn default() -> Self {
        Self {
            terrain: TileType::Grass,
            resource: None,
            building: None,
            walkable: true,
            movement_cost: 1.0,
        }
    }
}

impl WorldGrid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut grid = Self {
            tiles: HashMap::new(),
            entities_at: HashMap::new(),
            width,
            height,
        };
        
        // Initialize with default terrain
        for x in 0..width {
            for y in 0..height {
                let pos = Position::new(x as i32, y as i32);
                grid.tiles.insert(pos, TileData::default());
            }
        }
        
        grid
    }
    
    /// Get tile data at position
    pub fn get_tile(&self, pos: &Position) -> Option<&TileData> {
        self.tiles.get(pos)
    }
    
    /// Get mutable tile data at position
    pub fn get_tile_mut(&mut self, pos: &Position) -> Option<&mut TileData> {
        self.tiles.get_mut(pos)
    }
    
    /// Set terrain at position
    pub fn set_terrain(&mut self, pos: Position, terrain: TileType) {
        if let Some(tile) = self.tiles.get_mut(&pos) {
            tile.terrain = terrain;
            tile.walkable = terrain.is_walkable();
            tile.movement_cost = terrain.movement_cost();
        }
    }
    
    /// Set resource at position
    pub fn set_resource(&mut self, pos: Position, resource: Option<TileType>) {
        if let Some(tile) = self.tiles.get_mut(&pos) {
            tile.resource = resource;
            // Resources typically block movement
            tile.walkable = tile.terrain.is_walkable() && resource.is_none();
        }
    }
    
    /// Set building at position
    pub fn set_building(&mut self, pos: Position, building: Option<TileType>) {
        if let Some(tile) = self.tiles.get_mut(&pos) {
            tile.building = building;
            // Update walkability based on building
            tile.walkable = if let Some(b) = building {
                b.is_walkable()
            } else {
                tile.terrain.is_walkable() && tile.resource.is_none()
            };
        }
    }
    
    /// Check if position is walkable
    pub fn is_walkable(&self, pos: &Position) -> bool {
        self.tiles.get(pos)
            .map(|tile| tile.walkable)
            .unwrap_or(false)
    }
    
    /// Get movement cost at position
    pub fn movement_cost(&self, pos: &Position) -> f32 {
        self.tiles.get(pos)
            .map(|tile| tile.movement_cost)
            .unwrap_or(f32::INFINITY)
    }
    
    /// Add entity at position
    pub fn add_entity(&mut self, pos: Position, entity: Entity) {
        self.entities_at.entry(pos).or_default().push(entity);
    }
    
    /// Remove entity from position
    pub fn remove_entity(&mut self, pos: Position, entity: Entity) {
        if let Some(entities) = self.entities_at.get_mut(&pos) {
            entities.retain(|&e| e != entity);
        }
    }
    
    /// Get entities at position
    pub fn entities_at(&self, pos: &Position) -> Vec<Entity> {
        self.entities_at.get(pos).cloned().unwrap_or_default()
    }
    
    /// Check if position is in bounds
    pub fn in_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 &&
        pos.y >= 0 && pos.y < self.height as i32
    }
}

/// System to sync entity positions with the tilemap
pub fn sync_positions_to_tilemap(
    mut world_grid: ResMut<WorldGrid>,
    mut moved_entities: Query<(Entity, &PositionComponent), Changed<PositionComponent>>,
    mut removed: RemovedComponents<PositionComponent>,
) {
    // Handle moved entities
    for (entity, pos_comp) in moved_entities.iter_mut() {
        // Remove from old position (would need to track previous position)
        // For now, just ensure it's at the new position
        world_grid.add_entity(pos_comp.position, entity);
    }
    
    // Handle removed position components
    for entity in removed.read() {
        // Remove from all positions (inefficient but safe)
        for entities in world_grid.entities_at.values_mut() {
            entities.retain(|&e| e != entity);
        }
    }
}