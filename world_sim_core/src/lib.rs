//! World Simulation Core - Headless ECS simulation engine
//! 
//! This crate provides a pure simulation engine with ZERO rendering dependencies.
//! It uses Bevy ECS for entity management and emits events for visualization.

pub mod ai;
pub mod components;
pub mod engine;
pub mod plugin;
pub mod recipes;
pub mod resources;
pub mod scripting;
pub mod systems;
pub mod world;

// Re-export main types
pub use engine::SimulationEngine;
pub use plugin::SimulationPlugin;
pub use recipes::RecipeRegistry;
pub use scripting::commands::ScriptCommands;

// Re-export interface types for convenience
pub use world_sim_interface::{
    WorldConfig,
    EntityType, ResourceType, BuildingType,
    EngineCommand, EngineEvent, EngineObserver,
    WorldSnapshot, CommandResult, WorkerState,
    Recipe, RecipeId,
};

// Re-export components
pub use components::*;
