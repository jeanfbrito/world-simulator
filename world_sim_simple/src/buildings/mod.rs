mod building_types;
mod placement;
mod construction;

pub use building_types::{BuildingType, BuildingComponent, BuildingSize, BuildingRequirements};
pub use placement::{BuildingPlacementSystem, PlacementValidation, can_place_building};
pub use construction::{ConstructionQueue, ConstructionTask, ConstructionStatus};

use bevy::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConstructionQueue>()
            .init_resource::<BuildingPlacementSystem>()
            .add_systems(Startup, buildings_init_system)
            .add_systems(Update, (
                construction_update_system,
                building_placement_update_system,
            ));
    }
}

fn buildings_init_system(debug: Res<DebugSystem>) {
    debug.log(
        DebugLevel::Info,
        "BUILDINGS",
        "Building systems initialized"
    );
}

fn construction_update_system(
    mut queue: ResMut<ConstructionQueue>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    queue.update(time.delta_seconds(), &debug);
}

fn building_placement_update_system(
    placement: Res<BuildingPlacementSystem>,
    debug: Res<DebugSystem>,
) {
    // Placeholder for placement updates
    if placement.is_changed() {
        debug.log(
            DebugLevel::Debug,
            "PLACEMENT",
            "Building placement system updated"
        );
    }
}