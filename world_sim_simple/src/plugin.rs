use bevy::prelude::*;
use colored::*;
use std::collections::HashMap;

/// Base trait for all simulation plugins
pub trait SimulationPlugin {
    /// Plugin name for debug output
    fn name(&self) -> &str;

    /// Initialize the plugin
    fn build(&self, app: &mut App);
}

/// Manages all simulation plugins
#[derive(Resource, Default)]
pub struct PluginManager {
    plugins: HashMap<String, bool>,
    load_order: Vec<String>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            load_order: Vec::new(),
        }
    }

    pub fn register(&mut self, name: &str) {
        if !self.plugins.contains_key(name) {
            self.plugins.insert(name.to_string(), true);
            self.load_order.push(name.to_string());
            println!("{}", format!("[PLUGIN] ✓ {} registered", name).green());
        }
    }

    pub fn is_loaded(&self, name: &str) -> bool {
        self.plugins.get(name).copied().unwrap_or(false)
    }

    pub fn list_plugins(&self) {
        println!("{}", "[PLUGIN] Loaded plugins:".cyan().bold());
        for name in &self.load_order {
            let status = if self.plugins[name] { "✓" } else { "✗" };
            println!("  {} {}", status.green(), name);
        }
    }
}

/// System to track plugin initialization
pub fn plugin_init_system(manager: ResMut<PluginManager>) {
    manager.list_plugins();
}
