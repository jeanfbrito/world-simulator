mod biome;
mod chunk;
mod chunk_manager;
mod terrain;

pub use biome::BiomeType;
pub use chunk::{Chunk, ChunkCoordinate, CHUNK_SIZE};
pub use chunk_manager::{chunk_loading_system, chunk_unloading_system, ChunkManager};
pub use terrain::TerrainType;

use crate::debug::{DebugLevel, DebugSystem};
use bevy::prelude::*;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkManager>()
            .add_systems(Startup, tilemap_init_system)
            .add_systems(
                Update,
                (chunk_loading_system, chunk_unloading_system).chain(),
            );
    }
}

fn tilemap_init_system(debug: Res<DebugSystem>) {
    debug.log(
        DebugLevel::Info,
        "TILEMAP",
        "Tilemap system initialized with chunk support",
    );
}
