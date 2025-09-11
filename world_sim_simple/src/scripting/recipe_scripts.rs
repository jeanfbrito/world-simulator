use crate::debug::{DebugLevel, DebugSystem};
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

/// Component to mark entities with recipe scripts
#[derive(Component)]
pub struct RecipeScript {
    pub script_path: String,
    pub loaded: bool,
}

/// Command event to trigger script reloading
#[derive(Event)]
pub struct ReloadRecipeScriptsCommand;

/// System to load recipe scripts on command only
pub fn load_recipe_scripts(
    mut commands: Commands,
    mut reload_events: EventReader<ReloadRecipeScriptsCommand>,
    asset_server: Res<AssetServer>,
    existing_scripts: Query<Entity, With<RecipeScript>>,
    debug: Res<DebugSystem>,
) {
    // Only load when commanded
    for _event in reload_events.read() {
        debug.log(
            DebugLevel::Info,
            "SCRIPT",
            "Loading recipe scripts from assets...",
        );

        // Clear existing scripts first
        for entity in existing_scripts.iter() {
            commands.entity(entity).despawn();
        }

        // Load all recipe scripts from the packs directory
        let recipe_scripts = vec!["packs/stronghold/scripts/recipes/wood_processing.lua"];

        for script_path in recipe_scripts {
            // Load script asset using bevy_mod_scripting
            let script_handle: Handle<ScriptAsset> = asset_server.load(script_path);

            // Create an entity with the script component
            commands.spawn((
                RecipeScript {
                    script_path: script_path.to_string(),
                    loaded: false,
                },
                ScriptComponent::new(vec![script_handle]),
                Name::new(format!("RecipeScript_{}", script_path.replace("/", "_"))),
            ));

            debug.log(
                DebugLevel::Info,
                "SCRIPT",
                &format!("Loading recipe script: {}", script_path),
            );
        }

        break; // Only process one reload event at a time
    }
}

/// System to process loaded recipe scripts
pub fn process_recipe_scripts(
    mut scripts: Query<(&mut RecipeScript, &ScriptComponent), Changed<ScriptComponent>>,
    debug: Res<DebugSystem>,
) {
    for (mut script, script_component) in scripts.iter_mut() {
        if !script.loaded {
            script.loaded = true;

            debug.log(
                DebugLevel::Info,
                "SCRIPT",
                &format!("Recipe script loaded: {}", script.script_path),
            );

            // Script is now loaded and ready to be executed by bevy_mod_scripting
            debug.log(
                DebugLevel::Debug,
                "SCRIPT",
                &format!("Recipe script '{}' ready for execution", script.script_path),
            );
        }
    }
}
