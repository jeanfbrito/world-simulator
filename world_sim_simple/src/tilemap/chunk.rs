use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::TileType;

pub const CHUNK_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoordinate {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_world_position(world_x: f32, world_y: f32, tile_size: f32) -> Self {
        let tile_x = (world_x / tile_size).floor() as i32;
        let tile_y = (world_y / tile_size).floor() as i32;
        Self::from_tile_position(tile_x, tile_y)
    }

    pub fn from_tile_position(tile_x: i32, tile_y: i32) -> Self {
        Self {
            x: tile_x.div_euclid(CHUNK_SIZE as i32),
            y: tile_y.div_euclid(CHUNK_SIZE as i32),
        }
    }

    pub fn to_world_position(&self, tile_size: f32) -> (f32, f32) {
        let world_x = self.x as f32 * CHUNK_SIZE as f32 * tile_size;
        let world_y = self.y as f32 * CHUNK_SIZE as f32 * tile_size;
        (world_x, world_y)
    }
}

#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub coordinate: ChunkCoordinate,
    pub tiles: [[TileType; CHUNK_SIZE]; CHUNK_SIZE],
    pub is_dirty: bool,
}

impl Chunk {
    pub fn new(coordinate: ChunkCoordinate) -> Self {
        let mut chunk = Self {
            coordinate,
            tiles: [[TileType::Grass; CHUNK_SIZE]; CHUNK_SIZE],
            is_dirty: true,
        };
        chunk.generate_terrain();
        chunk
    }

    fn generate_terrain(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let world_x = self.coordinate.x * CHUNK_SIZE as i32 + x as i32;
                let world_y = self.coordinate.y * CHUNK_SIZE as i32 + y as i32;
                
                let noise = ((world_x as f32 * 0.1).sin() + (world_y as f32 * 0.1).cos()) * 0.5 + 0.5;
                
                self.tiles[y][x] = if noise < 0.3 {
                    TileType::Water
                } else if noise < 0.4 {
                    TileType::Sand
                } else if noise < 0.7 {
                    TileType::Grass
                } else if rng.gen_bool(0.3) {
                    TileType::Tree
                } else if rng.gen_bool(0.1) {
                    TileType::Stone
                } else {
                    TileType::Grass
                };
            }
        }
        
        info!("[CHUNK] Generated chunk ({}, {}) with {} tiles", 
            self.coordinate.x, self.coordinate.y, CHUNK_SIZE * CHUNK_SIZE);
    }

    pub fn get_tile(&self, local_x: usize, local_y: usize) -> Option<TileType> {
        if local_x < CHUNK_SIZE && local_y < CHUNK_SIZE {
            Some(self.tiles[local_y][local_x])
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, local_x: usize, local_y: usize, tile_type: TileType) -> bool {
        if local_x < CHUNK_SIZE && local_y < CHUNK_SIZE {
            self.tiles[local_y][local_x] = tile_type;
            self.is_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn world_to_local(world_x: i32, world_y: i32) -> (usize, usize) {
        let local_x = world_x.rem_euclid(CHUNK_SIZE as i32) as usize;
        let local_y = world_y.rem_euclid(CHUNK_SIZE as i32) as usize;
        (local_x, local_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_coordinate_conversion() {
        let coord = ChunkCoordinate::from_tile_position(17, -5);
        assert_eq!(coord.x, 1);
        assert_eq!(coord.y, -1);

        let coord = ChunkCoordinate::from_tile_position(15, 15);
        assert_eq!(coord.x, 0);
        assert_eq!(coord.y, 0);

        let coord = ChunkCoordinate::from_tile_position(-1, -1);
        assert_eq!(coord.x, -1);
        assert_eq!(coord.y, -1);
    }

    #[test]
    fn test_world_to_local() {
        let (x, y) = Chunk::world_to_local(17, 5);
        assert_eq!(x, 1);
        assert_eq!(y, 5);

        let (x, y) = Chunk::world_to_local(-1, -1);
        assert_eq!(x, 15);
        assert_eq!(y, 15);
    }
}