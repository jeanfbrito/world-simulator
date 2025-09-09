mod chunk;
mod terrain;
mod biome;
mod chunk_manager;

pub use chunk::{Chunk, ChunkCoordinate, CHUNK_SIZE};
pub use terrain::{TerrainType, TerrainProperties};
pub use biome::{BiomeType, BiomeGenerator};
pub use chunk_manager::{ChunkManager, chunk_loading_system, chunk_unloading_system};

use bevy::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkManager>()
            .add_systems(Startup, tilemap_init_system)
            .add_systems(Update, (
                chunk_loading_system,
                chunk_unloading_system,
            ).chain());
    }
}

fn tilemap_init_system(debug: Res<DebugSystem>) {
    debug.log(
        DebugLevel::Info,
        "TILEMAP",
        "Tilemap system initialized with chunk support"
    );
}