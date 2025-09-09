mod chunk;
mod terrain;

pub use chunk::{Chunk, ChunkCoordinate, CHUNK_SIZE};
pub use terrain::{TerrainType, TerrainProperties};

use bevy::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, tilemap_init_system);
    }
}

fn tilemap_init_system(debug: Res<DebugSystem>) {
    debug.log(
        DebugLevel::Info,
        "TILEMAP",
        "Tilemap system initialized with chunk support"
    );
}