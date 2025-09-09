//! Tilemap rendering and spatial management using bevy_entitiles

use bevy::prelude::*;
use bevy_entitiles::prelude::*;
use world_sim_interface::Position;

pub mod layers;
pub mod world_grid;
pub mod pathfinding;

/// Plugin for tilemap functionality
pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add bevy_entitiles plugin
            .add_plugins(EntiTilesPlugin)
            
            // Resources
            .init_resource::<world_grid::WorldGrid>()
            .init_resource::<pathfinding::PathfindingManager>()
            
            // Events
            .add_event::<TileUpdateEvent>()
            .add_event::<PathRequestEvent>()
            
            // Systems
            .add_systems(Startup, setup_tilemap)
            .add_systems(Update, (
                world_grid::sync_positions_to_tilemap,
                pathfinding::process_path_requests,
                layers::update_tile_layers,
            ));
    }
}

/// Event for tile updates
#[derive(Event)]
pub struct TileUpdateEvent {
    pub position: Position,
    pub layer: layers::LayerType,
    pub tile_type: TileType,
}

/// Event for pathfinding requests
#[derive(Event)]
pub struct PathRequestEvent {
    pub entity: Entity,
    pub start: Position,
    pub target: Position,
    pub callback: PathCallback,
}

/// Callback for completed paths
pub enum PathCallback {
    Movement,
    AIPlanning,
}

/// Types of tiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    // Terrain
    Grass,
    Stone,
    Sand,
    Water,
    DeepWater,
    
    // Resources
    Tree,
    OreNode,
    BerryBush,
    
    // Buildings
    Wall,
    Floor,
    Door,
    Storage,
    Workshop,
    
    // Special
    Empty,
    Blocked,
}

impl TileType {
    /// Get the texture index for this tile type
    pub fn texture_index(&self) -> u32 {
        match self {
            TileType::Grass => 0,
            TileType::Stone => 1,
            TileType::Sand => 2,
            TileType::Water => 3,
            TileType::DeepWater => 4,
            TileType::Tree => 10,
            TileType::OreNode => 11,
            TileType::BerryBush => 12,
            TileType::Wall => 20,
            TileType::Floor => 21,
            TileType::Door => 22,
            TileType::Storage => 23,
            TileType::Workshop => 24,
            TileType::Empty => 255,
            TileType::Blocked => 254,
        }
    }
    
    /// Check if this tile is walkable
    pub fn is_walkable(&self) -> bool {
        !matches!(self, 
            TileType::Water | 
            TileType::DeepWater | 
            TileType::Wall | 
            TileType::Blocked |
            TileType::Tree |
            TileType::OreNode
        )
    }
    
    /// Get movement cost for pathfinding
    pub fn movement_cost(&self) -> f32 {
        match self {
            TileType::Grass => 1.0,
            TileType::Sand => 1.5,
            TileType::Floor => 0.8,
            TileType::Door => 1.2,
            _ if self.is_walkable() => 1.0,
            _ => f32::INFINITY,
        }
    }
}

/// Setup the initial tilemap
fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load tileset texture
    let texture = asset_server.load("textures/tileset.png");
    
    // Create tilemap entity
    let tilemap = commands.spawn((
        TilemapBundle {
            tile_render_size: TileRenderSize(Vec2::new(32.0, 32.0)),
            slot_size: TilemapSlotSize(Vec2::new(32.0, 32.0)),
            ty: TilemapType::Square,
            storage: TilemapStorage::new(64, commands.spawn_empty().id()),
            texture: TilemapTexture::Single(texture),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Name::new("World Tilemap"),
    )).id();
    
    // Store tilemap entity for later use
    commands.insert_resource(TilemapEntity(tilemap));
}

/// Resource to store the main tilemap entity
#[derive(Resource)]
pub struct TilemapEntity(pub Entity);

/// Convert world position to tile position
pub fn world_to_tile(position: &Position) -> TilePos {
    TilePos {
        x: position.x as u32,
        y: position.y as u32,
    }
}

/// Convert tile position to world position
pub fn tile_to_world(tile_pos: &TilePos) -> Position {
    Position {
        x: tile_pos.x as i32,
        y: tile_pos.y as i32,
    }
}