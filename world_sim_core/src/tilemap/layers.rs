//! Tilemap layer management for terrain, resources, buildings, and units

use bevy::prelude::*;
use bevy::math::IVec2;
use bevy_entitiles::tilemap::tile::{TileTexture, TileBuilder};
use bevy_entitiles::tilemap::map::TilemapStorage;
use super::TileType;

/// Different rendering layers in the tilemap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayerType {
    Terrain,    // Base terrain (grass, stone, water)
    Resources,  // Trees, ore nodes, etc.
    Buildings,  // Structures and walls
    Units,      // Workers and other mobile entities
}

impl LayerType {
    /// Get the Z-order for this layer
    pub fn z_order(&self) -> f32 {
        match self {
            LayerType::Terrain => 0.0,
            LayerType::Resources => 1.0,
            LayerType::Buildings => 2.0,
            LayerType::Units => 3.0,
        }
    }
    
    /// Get the layer index for bevy_entitiles (0-3)
    pub fn layer_index(&self) -> u32 {
        match self {
            LayerType::Terrain => 0,
            LayerType::Resources => 1,
            LayerType::Buildings => 2,
            LayerType::Units => 3,
        }
    }
}

/// Component to track which layer an entity belongs to
#[derive(Component)]
pub struct TilemapLayer {
    pub layer_type: LayerType,
}

/// System to update tile layers based on game state
pub fn update_tile_layers(
    changed_tiles: Query<(&Transform, &TileType, &TilemapLayer), Changed<TileType>>,
    mut tilemap_query: Query<&mut TilemapStorage>,
    mut commands: Commands,
) {
    for (transform, tile_type, layer) in changed_tiles.iter() {
        // Convert transform to tile position
        let tile_pos = IVec2::new(
            (transform.translation.x / 32.0) as i32,
            (transform.translation.y / 32.0) as i32,
        );
        
        // Update the appropriate layer based on the tile type
        if let Ok(mut storage) = tilemap_query.get_single_mut() {
            // Create tile using TileBuilder
            let tile = TileBuilder::new()
                .with_layer(layer.layer_index())
                .with_texture_index(tile_type.texture_index())
                .build();
            
            // Set the tile in storage
            storage.set(&tile_pos, tile);
        }
    }
}

/// Bundle for creating a layered tile entity
#[derive(Bundle)]
pub struct LayeredTileBundle {
    pub transform: Transform,
    pub texture: TileTexture,
    pub layer: TilemapLayer,
    pub tile_type: TileType,
}

impl LayeredTileBundle {
    pub fn new(x: i32, y: i32, tile_type: TileType, layer: LayerType) -> Self {
        Self {
            transform: Transform::from_xyz(x as f32 * 32.0, y as f32 * 32.0, layer.z_order()),
            texture: TileTexture::new(tile_type.texture_index()),
            layer: TilemapLayer { layer_type: layer },
            tile_type,
        }
    }
}