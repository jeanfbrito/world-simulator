/// Systems module organizing all game systems
/// 
/// Systems are separated into tick-based (simulation) and frame-based (presentation)

pub mod needs_update;
pub mod needs_update_v2;
pub mod movement;
pub mod movement_effects;
pub mod work;

pub use needs_update::*;
pub use needs_update_v2::*;
pub use movement::*;
pub use movement_effects::*;
pub use work::*;

use bevy::prelude::*;

/// Plugin that registers all systems
pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add migration systems to run once at startup
        app.add_systems(PostStartup, (
            crate::components::migrate_needs_system,
            crate::components::migrate_positions_system,
            add_movement_components_system,
            configure_unit_speeds_system,
            add_work_components_system,
        ));
        
        // Add tick-based systems (simulation)
        app.add_systems(
            Update,
            (
                // Needs systems
                update_unit_needs_tick_system,
                sync_needs_v2_to_worldstate_system,
                eating_action_system,
                
                // Movement systems (tick-based)
                update_movement_effects_system,
                movement_request_system,
                tick_movement_system,
                sync_tile_entity_system,
                sync_position_component_system,
                
                // Work systems (tick-based)
                work_assignment_system,
                tick_work_system,
                auto_gather_system,
                work_effects_system,
                
                // Performance monitoring
                needs_performance_monitor_system,
                movement_performance_monitor_system,
                work_performance_monitor_system,
            ).chain().run_if(crate::simulation::on_simulation_tick_legacy)
        );
        
        // Add frame-based systems (presentation)
        app.add_systems(
            Update,
            visual_interpolation_system, // Runs every frame for smooth movement
        );
    }
}