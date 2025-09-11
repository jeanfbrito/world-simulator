mod metrics;
mod spatial_index;

pub use metrics::{performance_metrics_system, PerformanceMetrics};
pub use spatial_index::{spatial_index_update_system, SpatialIndex};

use crate::debug::{DebugLevel, DebugSystem};
use bevy::prelude::*;

pub struct PerformancePlugin;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialIndex>()
            .init_resource::<PerformanceMetrics>()
            .add_systems(Startup, performance_init_system)
            .add_systems(
                Update,
                (spatial_index_update_system, performance_metrics_system).chain(),
            );

        info!("[PERF] Performance optimization systems initialized");
    }
}

fn performance_init_system(debug: Res<DebugSystem>) {
    debug.log(DebugLevel::Info, "PERF", "Performance systems initialized");
    info!("[PERF] Spatial indexing and metrics tracking enabled");
}
