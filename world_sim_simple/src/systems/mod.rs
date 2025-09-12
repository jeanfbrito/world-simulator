pub mod movement;
pub mod movement_effects;
/// Systems module organizing all game systems
///
/// Systems are separated into tick-based (simulation) and frame-based (presentation)
pub mod needs_update;
pub mod needs_update_v2;
pub mod resource_regeneration;
pub mod resource_growth;
pub mod storage;
pub mod unit_mind;
pub mod work;

pub use movement::*;
pub use movement_effects::*;
pub use needs_update_v2::*;
pub use resource_regeneration::*;
pub use resource_growth::*;
pub use storage::*;
pub use unit_mind::*;
pub use work::*;

use bevy::prelude::*;

/// Plugin that registers all systems
pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Register events
        app.add_event::<crate::components::StorageChangedEvent>()
            .add_event::<crate::components::ResourceRegeneratedEvent>()
            .add_event::<crate::components::ResourceDepletedEvent>()
            .add_event::<crate::components::ResourceGrowthEvent>();

        // Add migration systems to run once at startup
        app.add_systems(
            PostStartup,
            (
                crate::components::migrate_needs_system,
                crate::components::migrate_positions_system,
                add_movement_components_system,
                configure_unit_speeds_system,
                add_work_components_system,
                spawn_storage_buildings_system,
                spawn_regenerating_resources_system,
                add_regeneration_to_trees_system,
                ensure_unit_mind_system,
            ),
        );

        // Add tick-based systems (simulation) - split into smaller groups
        app.add_systems(
            Update,
            (
                // Needs systems
                update_unit_needs_tick_system,
                sync_needs_v2_to_worldstate_system,
                sync_has_energy_to_needs_system,
                eating_action_system,
            )
                .chain()
                .run_if(crate::simulation::on_simulation_tick_legacy),
        );

        app.add_systems(
            Update,
            (
                // Movement systems (tick-based)
                update_movement_effects_system,
                movement_request_system,
                tick_movement_system,
                sync_tile_entity_system,
                sync_position_component_system,
            )
                .chain()
                .run_if(crate::simulation::on_simulation_tick_legacy),
        );

        app.add_systems(
            Update,
            (
                // Work systems (tick-based)
                work_assignment_system,
                tick_work_system,
                auto_gather_system,
                work_effects_system,
                update_unit_mind_system,  // Update unit minds after work
                log_mind_changes_system,  // Log mind state changes for debugging
            )
                .chain()
                .run_if(crate::simulation::on_simulation_tick_legacy),
        );

        app.add_systems(
            Update,
            (
                // Resource systems (tick-based)
                resource_growth_system,
                harvest_growing_resource_system,
                resource_regeneration_system,
                resource_harvest_system,
                resource_status_display_system,
            )
                .chain()
                .run_if(crate::simulation::on_simulation_tick_legacy),
        );

        app.add_systems(
            Update,
            (
                // Storage systems (tick-based)
                storage_task_assignment_system,
                storage_task_update_system,
                storage_deposit_system,
                storage_withdrawal_system,
                storage_display_system,
            )
                .chain()
                .run_if(crate::simulation::on_simulation_tick_legacy),
        );

        app.add_systems(
            Update,
            (
                // Performance monitoring
                needs_performance_monitor_system,
                movement_performance_monitor_system,
                work_performance_monitor_system,
                storage_performance_monitor_system,
                resource_performance_monitor_system,
            )
                .chain()
                .run_if(crate::simulation::on_simulation_tick_legacy),
        );

        // Add frame-based systems (presentation)
        app.add_systems(
            Update,
            visual_interpolation_system, // Runs every frame for smooth movement
        );
    }
}
