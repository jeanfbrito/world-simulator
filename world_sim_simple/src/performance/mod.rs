mod spatial_index;
mod metrics;

pub use spatial_index::{SpatialIndex, SpatialHash, CellCoordinate, spatial_index_update_system};
pub use metrics::{PerformanceMetrics, FrameTimer, performance_metrics_system};

use bevy::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};

pub struct PerformancePlugin;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialIndex>()
            .init_resource::<PerformanceMetrics>()
            .add_systems(Startup, performance_init_system)
            .add_systems(Update, (
                spatial_index_update_system,
                performance_metrics_system,
            ).chain());
            
        info!("[PERF] Performance optimization systems initialized");
    }
}

fn performance_init_system(debug: Res<DebugSystem>) {
    debug.log(
        DebugLevel::Info,
        "PERF",
        "Performance systems initialized"
    );
    info!("[PERF] Spatial indexing and metrics tracking enabled");
}