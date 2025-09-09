//! Commands for managing scripts at runtime

use bevy_ecs::prelude::*;
use super::{recipe_loader::ReloadRecipeScriptsCommand, ai_scripts::ReloadAIScriptsCommand};

/// Commands for script management
pub struct ScriptCommands;

impl ScriptCommands {
    /// Reload all recipe scripts
    pub fn reload_recipes(world: &mut World) {
        world.send_event(ReloadRecipeScriptsCommand);
        tracing::info!("Triggered recipe scripts reload");
    }
    
    /// Reload all AI behavior scripts
    pub fn reload_ai_scripts(world: &mut World) {
        world.send_event(ReloadAIScriptsCommand);
        tracing::info!("Triggered AI scripts reload");
    }
    
    /// Reload all scripts
    pub fn reload_all(world: &mut World) {
        Self::reload_recipes(world);
        Self::reload_ai_scripts(world);
        tracing::info!("Triggered full scripts reload");
    }
}

/// System to handle script reload via keyboard (debug builds only)
#[cfg(debug_assertions)]
pub fn debug_script_reload_system(
    keyboard: Res<bevy_input::ButtonInput<bevy_input::keyboard::KeyCode>>,
    mut recipe_reload: EventWriter<ReloadRecipeScriptsCommand>,
    mut ai_reload: EventWriter<ReloadAIScriptsCommand>,
) {
    use bevy_input::keyboard::KeyCode;
    
    // Ctrl+R to reload recipes
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyR) {
        recipe_reload.send(ReloadRecipeScriptsCommand);
        tracing::info!("Reloading recipe scripts (Ctrl+R)");
    }
    
    // Ctrl+A to reload AI scripts
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyA) {
        ai_reload.send(ReloadAIScriptsCommand);
        tracing::info!("Reloading AI scripts (Ctrl+A)");
    }
    
    // Ctrl+Shift+R to reload all
    if keyboard.pressed(KeyCode::ControlLeft) && 
       keyboard.pressed(KeyCode::ShiftLeft) && 
       keyboard.just_pressed(KeyCode::KeyR) {
        recipe_reload.send(ReloadRecipeScriptsCommand);
        ai_reload.send(ReloadAIScriptsCommand);
        tracing::info!("Reloading all scripts (Ctrl+Shift+R)");
    }
}

/// Component to track script performance
#[derive(Component, Default)]
pub struct ScriptPerformance {
    pub execution_time_ms: f32,
    pub last_execution: Option<std::time::Instant>,
    pub execution_count: u32,
    pub errors: Vec<String>,
}

impl ScriptPerformance {
    /// Record a script execution
    pub fn record_execution(&mut self, start: std::time::Instant) {
        let duration = start.elapsed();
        self.execution_time_ms = duration.as_secs_f32() * 1000.0;
        self.last_execution = Some(std::time::Instant::now());
        self.execution_count += 1;
    }
    
    /// Record an error
    pub fn record_error(&mut self, error: String) {
        self.errors.push(error);
        // Keep only last 10 errors
        if self.errors.len() > 10 {
            self.errors.remove(0);
        }
    }
    
    /// Get average execution time
    pub fn average_time_ms(&self) -> f32 {
        if self.execution_count > 0 {
            self.execution_time_ms / self.execution_count as f32
        } else {
            0.0
        }
    }
}