use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};

pub mod lua_api;
pub mod recipe_scripts;
pub mod tree_generation;
// pub mod goap_loader;  // TODO: Enable when bevy_mod_scripting integration is ready
// pub mod storage_loader; // TODO: Enable when mlua is configured

// Re-export key types for easier use
pub use recipe_scripts::{RecipeScript, ReloadRecipeScriptsCommand};
pub use tree_generation::{ScriptedTree, GenerateTreesCommand, TreeGenerationState};
// pub use goap_loader::{ScriptedGoapActions, ReloadGoapActionsCommand};

#[derive(Event)]
pub struct ScriptReloadEvent {
    pub script_type: ScriptType,
}

#[derive(Debug, Clone)]
pub enum ScriptType {
    Recipe,
    Worker,
    World,
    Goap,
}

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BMSPlugin)
            .add_event::<ScriptReloadEvent>()
            .add_event::<ReloadRecipeScriptsCommand>()
            .add_event::<GenerateTreesCommand>()
            // .add_event::<ReloadGoapActionsCommand>()  // TODO: Enable when ready
            .init_resource::<TreeGenerationState>()
            // .init_resource::<ScriptedGoapActions>()  // TODO: Enable when ready
            .add_systems(Startup, scripting_init_system)
            .add_systems(Update, (
                script_reload_system,
                recipe_scripts::load_recipe_scripts,
                recipe_scripts::process_recipe_scripts,
                tree_generation::generate_trees_system,
                // goap_loader::load_goap_actions_system,  // TODO: Enable when ready
                // goap_loader::merge_goap_actions_system,  // TODO: Enable when ready
            ));
    }
}

fn scripting_init_system(
    debug: Res<DebugSystem>,
    mut reload_events: EventWriter<ReloadRecipeScriptsCommand>,
    mut tree_events: EventWriter<GenerateTreesCommand>,
    // mut goap_events: EventWriter<ReloadGoapActionsCommand>,  // TODO: Enable when ready
) {
    debug.log(
        DebugLevel::Info,
        "SCRIPT",
        "Scripting system initialized with asset-based script loading"
    );
    
    // Automatically load recipe scripts on startup
    reload_events.send(ReloadRecipeScriptsCommand {});
    
    debug.log(
        DebugLevel::Info,
        "SCRIPT",
        "Triggering initial recipe script loading"
    );
    
    // Automatically generate trees on startup
    tree_events.send(GenerateTreesCommand {
        area: None,
        force_regenerate: false,
    });
    
    debug.log(
        DebugLevel::Info,
        "SCRIPT", 
        "Triggering initial tree generation"
    );
    
    // TODO: Load GOAP actions when ready
    // goap_events.send(ReloadGoapActionsCommand {
    //     pack_name: Some("stronghold".to_string()),
    // });
    
    // debug.log(
    //     DebugLevel::Info,
    //     "SCRIPT",
    //     "Triggering initial GOAP actions loading"
    // );
}

fn script_reload_system(
    mut reload_events: EventReader<ScriptReloadEvent>,
    mut recipe_reload_events: EventWriter<ReloadRecipeScriptsCommand>,
    // mut goap_reload_events: EventWriter<ReloadGoapActionsCommand>,  // TODO: Enable when ready
    debug: Res<DebugSystem>,
) {
    for event in reload_events.read() {
        debug.log(
            DebugLevel::Info,
            "SCRIPT",
            &format!("Reloading {:?} scripts", event.script_type)
        );
        
        match event.script_type {
            ScriptType::Recipe => {
                recipe_reload_events.send(ReloadRecipeScriptsCommand {});
            }
            ScriptType::Worker => {
                debug.log(
                    DebugLevel::Debug,
                    "SCRIPT",
                    "Worker script reloading not yet implemented"
                );
            }
            ScriptType::Goap => {
                // TODO: Enable when ready
                // goap_reload_events.send(ReloadGoapActionsCommand {
                //     pack_name: None,  // Use default pack
                // });
                debug.log(
                    DebugLevel::Debug,
                    "SCRIPT",
                    "GOAP script reloading not yet implemented"
                );
            }
            ScriptType::World => {
                debug.log(
                    DebugLevel::Debug,
                    "SCRIPT",
                    "World script reloading not yet implemented"
                );
            }
        }
    }
}