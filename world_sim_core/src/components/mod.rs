//! ECS components for the simulation engine

pub mod position;
pub mod resources;
pub mod workers;
pub mod buildings;
pub mod inventory;
pub mod tasks;
pub mod goap_states;
pub mod goap_actions;

// Re-export commonly used components
pub use position::PositionComponent;
pub use resources::{ResourceNodeComponent, HarvestingComponent};
pub use workers::{WorkerComponent, MovementComponent};
pub use buildings::{BuildingComponent, StorageComponent, ProductionComponent};
pub use inventory::{InventoryComponent, CarryWeightComponent};
pub use tasks::{TaskComponent, TaskQueueComponent, TaskExecutorComponent, TaskType};

// Re-export GOAP components
pub use goap_states::*;
pub use goap_actions::*;