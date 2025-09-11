/// GOAP Action Loader from Lua Scripts
/// 
/// This module loads GOAP actions from Lua scripts to make AI behavior
/// fully data-driven and moddable.

use bevy::prelude::*;
use mlua::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use crate::ai::{GoapAction, StateValue};
use crate::debug::{DebugSystem, DebugLevel};

/// Resource to store loaded GOAP actions from scripts
#[derive(Resource, Default)]
pub struct ScriptedGoapActions {
    pub actions: Vec<GoapAction>,
    pub loaded_from: String,
}

/// Event to trigger reloading of GOAP actions from scripts
#[derive(Event)]
pub struct ReloadGoapActionsCommand {
    pub pack_name: Option<String>,
}

/// Load GOAP actions from a Lua script file
pub fn load_goap_actions_from_file(path: &Path) -> Result<Vec<GoapAction>, String> {
    let lua = Lua::new();
    
    // Read the script file
    let script = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read script: {}", e))?;
    
    // Execute the script and get the returned table
    let actions_table: LuaTable = lua.load(&script)
        .eval()
        .map_err(|e| format!("Failed to execute script: {}", e))?;
    
    let mut actions = Vec::new();
    
    // Iterate through the actions table
    for pair in actions_table.pairs::<LuaString, LuaTable>() {
        let (name, action_data) = pair.map_err(|e| format!("Failed to iterate actions: {}", e))?;
        let action_name = name.to_str().map_err(|e| format!("Failed to convert name: {}", e))?.to_string();
        
        // Extract action properties
        let cost: f32 = action_data.get("cost")
            .unwrap_or(1.0);
        
        let mut goap_action = GoapAction::new(&action_name, cost);
        
        // Load preconditions
        if let Ok(preconditions) = action_data.get::<LuaTable>("preconditions") {
            for pair in preconditions.pairs::<LuaString, LuaTable>() {
                let (key, value_data) = pair.map_err(|e| format!("Failed to parse precondition: {}", e))?;
                let key_str = key.to_str().map_err(|e| format!("Failed to convert key: {}", e))?.to_string();
                
                let state_value = parse_state_value(&value_data)?;
                goap_action = goap_action.with_precondition(&key_str, state_value);
            }
        }
        
        // Load effects
        if let Ok(effects) = action_data.get::<LuaTable>("effects") {
            for pair in effects.pairs::<LuaString, LuaTable>() {
                let (key, value_data) = pair.map_err(|e| format!("Failed to parse effect: {}", e))?;
                let key_str = key.to_str().map_err(|e| format!("Failed to convert key: {}", e))?.to_string();
                
                let state_value = parse_state_value(&value_data)?;
                goap_action = goap_action.with_effect(&key_str, state_value);
            }
        }
        
        actions.push(goap_action);
    }
    
    Ok(actions)
}

/// Parse a StateValue from a Lua table
fn parse_state_value(value_data: &LuaTable) -> Result<StateValue, String> {
    let type_str: String = value_data.get("type")
        .map_err(|e| format!("Missing type field: {}", e))?;
    
    match type_str.as_str() {
        "Bool" => {
            let value: bool = value_data.get("value")
                .map_err(|e| format!("Failed to get bool value: {}", e))?;
            Ok(StateValue::Bool(value))
        }
        "Float" => {
            let value: f64 = value_data.get("value")
                .map_err(|e| format!("Failed to get float value: {}", e))?;
            Ok(StateValue::Float(value))
        }
        "Int" => {
            let value: i64 = value_data.get("value")
                .map_err(|e| format!("Failed to get int value: {}", e))?;
            Ok(StateValue::Int(value as u32))
        }
        "IntDelta" => {
            let value: i64 = value_data.get("value")
                .map_err(|e| format!("Failed to get int delta value: {}", e))?;
            Ok(StateValue::IntDelta(value as i32))
        }
        _ => Err(format!("Unknown StateValue type: {}", type_str))
    }
}

/// System to load GOAP actions from scripts
pub fn load_goap_actions_system(
    mut commands: Commands,
    mut reload_events: EventReader<ReloadGoapActionsCommand>,
    debug: Res<DebugSystem>,
) {
    for event in reload_events.read() {
        let pack_name = event.pack_name.as_deref().unwrap_or("stronghold");
        let script_path = format!("assets/packs/{}/scripts/ai/goap_actions.lua", pack_name);
        
        debug.log(
            DebugLevel::Info,
            "GOAP_LOADER",
            &format!("Loading GOAP actions from {}", script_path)
        );
        
        match load_goap_actions_from_file(Path::new(&script_path)) {
            Ok(actions) => {
                let action_count = actions.len();
                
                // Log each loaded action
                for action in &actions {
                    debug.log(
                        DebugLevel::Debug,
                        "GOAP_LOADER",
                        &format!("Loaded action: {} (cost: {})", action.name, action.cost)
                    );
                }
                
                // Store the loaded actions
                commands.insert_resource(ScriptedGoapActions {
                    actions,
                    loaded_from: pack_name.to_string(),
                });
                
                debug.log(
                    DebugLevel::Info,
                    "GOAP_LOADER",
                    &format!("Successfully loaded {} GOAP actions from {}", action_count, pack_name)
                );
            }
            Err(e) => {
                debug.log(
                    DebugLevel::Info,  // Warning level doesn't exist, use Info
                    "GOAP_LOADER",
                    &format!("Failed to load GOAP actions: {}", e)
                );
            }
        }
    }
}

/// System to merge scripted actions with hardcoded ones
pub fn merge_goap_actions_system(
    scripted: Option<Res<ScriptedGoapActions>>,
    mut action_set: ResMut<crate::ai::ActionSet>,
    debug: Res<DebugSystem>,
) {
    if let Some(scripted_actions) = scripted {
        if !scripted_actions.is_changed() {
            return;
        }
        
        debug.log(
            DebugLevel::Info,
            "GOAP_LOADER",
            &format!("Merging {} scripted actions into ActionSet", scripted_actions.actions.len())
        );
        
        // Clear existing actions and replace with scripted ones
        // In production, you might want to merge instead of replace
        action_set.actions.clear();
        action_set.actions.extend(scripted_actions.actions.clone());
        
        debug.log(
            DebugLevel::Info,
            "GOAP_LOADER",
            &format!("ActionSet now contains {} actions", action_set.actions.len())
        );
    }
}