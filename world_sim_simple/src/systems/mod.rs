/// Systems module organizing all game systems
/// 
/// Systems are separated into tick-based (simulation) and frame-based (presentation)

pub mod needs_update;
pub mod needs_update_v2;

pub use needs_update::*;
pub use needs_update_v2::*;

use bevy::prelude::*;

/// Plugin that registers all systems
pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add migration system to run once at startup
        app.add_systems(PostStartup, crate::components::migrate_needs_system);
        
        // Add tick-based systems
        app.add_systems(
            Update,
            (
                // New tick-based needs system
                update_unit_needs_tick_system,
                sync_needs_v2_to_worldstate_system,
                eating_action_system,
                needs_performance_monitor_system,
            ).chain().run_if(crate::simulation::on_simulation_tick_legacy)
        );
    }
}