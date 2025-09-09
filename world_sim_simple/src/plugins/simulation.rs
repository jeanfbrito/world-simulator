use bevy::prelude::*;
use colored::*;
use crate::SimulationState;
use crate::plugin::PluginManager;
use crate::debug::{DebugSystem, DebugLevel};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        println!("{}", "[SIMULATION] Initializing simulation plugin...".cyan());
        
        app.add_systems(Startup, init_simulation_plugin)
           .add_systems(Update, (
               simulation_tick_system,
               simulation_stats_system,
           ));
    }
}

fn init_simulation_plugin(mut manager: ResMut<PluginManager>) {
    manager.register("SimulationPlugin");
    println!("{}", "[SIMULATION] Simulation systems initialized".green());
}

fn simulation_tick_system(
    mut sim_state: ResMut<SimulationState>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    if !sim_state.running {
        return;
    }
    
    sim_state.accumulated_time += time.delta_seconds() * sim_state.speed;
    
    if sim_state.accumulated_time >= 1.0 {
        sim_state.accumulated_time = 0.0;
        let old_tick = sim_state.tick;
        sim_state.tick += 1;
        
        // Log every 10 ticks
        if sim_state.tick % 10 == 0 {
            debug.log(
                DebugLevel::Debug,
                "SIMULATION",
                &format!("Tick {} → {}", old_tick, sim_state.tick)
            );
        }
    }
}

fn simulation_stats_system(
    sim_state: Res<SimulationState>,
    debug: Res<DebugSystem>,
    time: Res<Time>,
) {
    static mut LAST_STATS: f32 = 0.0;
    unsafe {
        LAST_STATS += time.delta_seconds();
        if LAST_STATS > 10.0 {  // Every 10 seconds
            debug.log(
                DebugLevel::Info,
                "STATS",
                &format!("Tick: {}, Speed: {:.1}x, Running: {}", 
                    sim_state.tick, sim_state.speed, sim_state.running)
            );
            LAST_STATS = 0.0;
        }
    }
}