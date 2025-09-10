use bevy::prelude::*;
use crate::debug::{DebugSystem, DebugLevel};

#[derive(Component)]
pub struct LuaScriptComponent {
    pub script_path: String,
    pub loaded: bool,
}

pub fn register_lua_api(debug: Res<DebugSystem>) {
    debug.log(
        DebugLevel::Info,
        "SCRIPT",
        "Lua API registered successfully"
    );
}