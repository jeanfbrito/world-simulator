use bevy::prelude::*;
use colored::*;
use crate::{WorldMap, TileEntity, TILE_SIZE, MAP_SIZE};
use crate::plugin::PluginManager;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        println!("{}", "[WORLD] Initializing world plugin...".cyan());
        
        app.add_systems(Startup, init_world_plugin)
           .add_systems(Update, world_update_system);
    }
}

fn init_world_plugin(mut manager: ResMut<PluginManager>) {
    manager.register("WorldPlugin");
    println!("{}", "[WORLD] World systems initialized".green());
}

fn world_update_system(
    world_map: Res<WorldMap>,
    time: Res<Time>,
) {
    // Future world update logic
    // For now, just track that the system runs
    static mut LAST_LOG: f32 = 0.0;
    unsafe {
        LAST_LOG += time.delta_secs();
        if LAST_LOG > 5.0 {
            println!("{}", "[WORLD] World update tick".dimmed());
            LAST_LOG = 0.0;
        }
    }
}