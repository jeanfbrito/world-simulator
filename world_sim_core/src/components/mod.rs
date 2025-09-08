//! ECS components for the simulation engine

pub mod position;
pub mod resources;
pub mod workers;
pub mod buildings;
pub mod inventory;
pub mod tasks;

// Re-export commonly used components
pub use position::PositionComponent;
pub use resources::{ResourceNodeComponent, HarvestingComponent};
pub use workers::{WorkerComponent, MovementComponent};
pub use buildings::{BuildingComponent, StorageComponent, ProductionComponent};
pub use inventory::{InventoryComponent, CarryWeightComponent};
pub use tasks::{TaskComponent, TaskQueueComponent, TaskExecutorComponent, TaskType};