//! Tilemap layer management for terrain, resources, buildings, and units

use bevy::prelude::*;
use bevy_entitiles::prelude::*;
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
    changed_tiles: Query<(&TilePos, &TileType, &TilemapLayer), Changed<TileType>>,
    mut tilemap_query: Query<&mut TilemapStorage>,
    mut commands: Commands,
) {
    for (tile_pos, tile_type, layer) in changed_tiles.iter() {
        // Update the appropriate layer based on the tile type
        if let Ok(mut storage) = tilemap_query.get_single_mut() {
            // Get or create tile entity
            let tile_entity = storage.get(tile_pos).unwrap_or_else(|| {
                commands.spawn_empty().id()
            });
            
            // Update tile texture based on type
            commands.entity(tile_entity).insert(TileTexture(tile_type.texture_index()));
            
            // Set the tile in storage if it's new
            if storage.get(tile_pos).is_none() {
                storage.set(tile_pos, tile_entity);
            }
        }
    }
}

/// Bundle for creating a layered tile
#[derive(Bundle)]
pub struct LayeredTileBundle {
    pub position: TilePos,
    pub texture: TileTexture,
    pub layer: TilemapLayer,
    pub tile_type: TileType,
}

impl LayeredTileBundle {
    pub fn new(x: u32, y: u32, tile_type: TileType, layer: LayerType) -> Self {
        Self {
            position: TilePos { x, y },
            texture: TileTexture(tile_type.texture_index()),
            layer: TilemapLayer { layer_type: layer },
            tile_type,
        }
    }
}