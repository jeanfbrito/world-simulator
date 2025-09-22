//! Interface definitions for the world simulation engine
//! 
//! This crate provides the shared types and traits used for communication
//! between the headless simulation engine and various visualizers.

pub mod commands;
pub mod entities;
pub mod events;
pub mod observer;
pub mod results;
pub mod state;
pub mod types;
pub mod ipc;

// Re-export commonly used types at the crate root
pub use commands::EngineCommand;
pub use entities::{
    BuildingType, DestroyReason, EntityType, PopulationChangeReason, ResourceType, Season,
    TaskAssignment, TaskType, Weather, WorkerState, Recipe, RecipeId,
};
pub use events::EngineEvent;
pub use observer::EngineObserver;
pub use results::{CommandResult, WorldConfig};
pub use state::{EntitySnapshot, GlobalState, SettlementSnapshot, WorldSnapshot};
pub use types::{EntityId, PlayerId, Position, SettlementId, Tick};
pub use ipc::{
    IpcMessage, MessagePayload, IpcError, PackMetadata, VisualRegistry,
    TileVisual, EntityVisual, UITheme, AnimationData, VariantSelector,
};