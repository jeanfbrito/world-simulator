//! Scripting infrastructure for dynamic content

pub mod lua_api;
pub mod recipe_loader;
pub mod ai_scripts;
pub mod commands;
pub mod item_loader;
pub mod building_loader;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_lua::prelude::*;

/// Plugin for integrating Lua scripting into the simulation
pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        // Add the main scripting plugin with Lua support
        app.add_plugins(ScriptingPlugins)
           .add_plugins(LuaScriptingPlugin);
        
        // Register our custom Lua API
        app.add_systems(Startup, lua_api::register_lua_api);
        
        // Add reload command events
        app.add_event::<recipe_loader::ReloadRecipeScriptsCommand>()
           .add_event::<ai_scripts::ReloadAIScriptsCommand>()
           .add_event::<item_loader::ReloadItemScriptsCommand>()
           .add_event::<building_loader::ReloadBuildingScriptsCommand>();
        
        // Initialize registries
        app.init_resource::<item_loader::ItemRegistry>()
           .init_resource::<building_loader::BuildingRegistry>();
        
        // Add script loading systems (triggered by commands)
        app.add_systems(Update, (
            recipe_loader::load_recipe_scripts,
            ai_scripts::load_ai_scripts,
            item_loader::load_item_scripts,
            building_loader::load_building_scripts,
        ));
        
        // Add script processing systems
        app.add_systems(Update, (
            recipe_loader::process_recipe_scripts,
            ai_scripts::process_ai_scripts,
            item_loader::process_item_scripts,
            building_loader::process_building_scripts,
            lua_api::apply_lua_recipe_modifiers,
            lua_api::apply_lua_worker_modifiers,
            lua_api::apply_lua_ai_modifiers,
        ));
        
        // Add component update systems
        app.add_systems(Update, (
            item_loader::update_item_decay,
            building_loader::update_building_maintenance,
            building_loader::construction_system,
        ));
        
        // Register script events
        app.add_event::<RecipeScriptEvent>()
           .add_event::<AIScriptEvent>()
           .add_event::<WorldScriptEvent>();
        
        // Add script debugging resources
        #[cfg(debug_assertions)]
        app.init_resource::<ScriptDebugger>();
    }
}

/// Event triggered by recipe scripts
#[derive(Event)]
pub struct RecipeScriptEvent {
    pub recipe_id: String,
    pub event_type: RecipeEventType,
    pub data: ScriptEventData,
}

#[derive(Debug, Clone)]
pub enum RecipeEventType {
    Started,
    Completed,
    Failed,
    Modified,
}

/// Event triggered by AI scripts  
#[derive(Event)]
pub struct AIScriptEvent {
    pub entity: Entity,
    pub event_type: AIEventType,
    pub data: ScriptEventData,
}

#[derive(Debug, Clone)]
pub enum AIEventType {
    BehaviorChange,
    GoalModified,
    PersonalityTrait,
    Decision,
}

/// Event triggered by world scripts
#[derive(Event)]
pub struct WorldScriptEvent {
    pub event_type: WorldEventType,
    pub data: ScriptEventData,
}

#[derive(Debug, Clone)]
pub enum WorldEventType {
    Disaster,
    SeasonChange,
    QuestStarted,
    QuestCompleted,
}

/// Generic data container for script events
#[derive(Debug, Clone, Default)]
pub struct ScriptEventData {
    pub values: std::collections::HashMap<String, ScriptValue>,
}

#[derive(Debug, Clone)]
pub enum ScriptValue {
    Number(f64),
    Text(String),
    Boolean(bool),
    Entity(Entity),
}

/// Resource for script debugging (debug builds only)
#[cfg(debug_assertions)]
#[derive(Resource, Default)]
pub struct ScriptDebugger {
    pub enabled: bool,
    pub log_level: ScriptLogLevel,
    pub performance_tracking: bool,
    pub script_errors: Vec<ScriptError>,
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone)]
pub enum ScriptLogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

#[cfg(debug_assertions)]
impl Default for ScriptLogLevel {
    fn default() -> Self {
        Self::Warning
    }
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone)]
pub struct ScriptError {
    pub script_name: String,
    pub line: Option<u32>,
    pub message: String,
    pub timestamp: std::time::Instant,
}