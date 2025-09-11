use crate::resources::ResourceType;
/// Storage building components for resource management
///
/// Storage buildings hold resources with tick-based intake/output
/// using integer counters for deterministic behavior.
use bevy::prelude::*;
use std::collections::HashMap;

/// Main storage component for buildings that can hold resources
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct StorageBuilding {
    /// Current stored resources (type -> amount)
    pub stored: HashMap<ResourceType, u32>,

    /// Maximum capacity per resource type
    pub capacity_per_type: u32,

    /// Total maximum capacity (all resources combined)
    pub total_capacity: u32,

    /// Current total amount stored
    pub current_total: u32,

    /// Allowed resource types (None = all types)
    pub allowed_types: Option<Vec<ResourceType>>,

    /// Priority for this storage (higher = preferred)
    pub priority: i32,

    /// Whether storage accepts deposits
    pub accepting_deposits: bool,

    /// Whether storage allows withdrawals
    pub allowing_withdrawals: bool,
}

impl StorageBuilding {
    pub fn new(capacity: u32) -> Self {
        Self {
            stored: HashMap::new(),
            capacity_per_type: capacity / 4, // Default: split among types
            total_capacity: capacity,
            current_total: 0,
            allowed_types: None,
            priority: 0,
            accepting_deposits: true,
            allowing_withdrawals: true,
        }
    }

    /// Create a specialized storage for specific resources
    pub fn specialized(capacity: u32, types: Vec<ResourceType>) -> Self {
        let mut storage = Self::new(capacity);
        storage.allowed_types = Some(types);
        storage.capacity_per_type = capacity; // Full capacity for allowed types
        storage
    }

    /// Check if a resource type can be stored
    pub fn can_store(&self, resource_type: &ResourceType) -> bool {
        if !self.accepting_deposits {
            return false;
        }

        if let Some(ref allowed) = self.allowed_types {
            if !allowed.contains(resource_type) {
                return false;
            }
        }

        let current_amount = self.stored.get(resource_type).unwrap_or(&0);
        *current_amount < self.capacity_per_type && self.current_total < self.total_capacity
    }

    /// Deposit resources into storage
    pub fn deposit(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        if !self.can_store(&resource_type) {
            return 0;
        }

        let current = self.stored.get(&resource_type).unwrap_or(&0);
        let space_for_type = self.capacity_per_type - current;
        let space_total = self.total_capacity - self.current_total;
        let max_deposit = space_for_type.min(space_total).min(amount);

        if max_deposit > 0 {
            *self.stored.entry(resource_type).or_insert(0) += max_deposit;
            self.current_total += max_deposit;
        }

        max_deposit // Return amount actually deposited
    }

    /// Withdraw resources from storage
    pub fn withdraw(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        if !self.allowing_withdrawals {
            return 0;
        }

        let current = self.stored.get(&resource_type).unwrap_or(&0);
        let withdrawn = (*current).min(amount);

        if withdrawn > 0 {
            *self.stored.get_mut(&resource_type).unwrap() -= withdrawn;
            self.current_total -= withdrawn;

            // Remove entry if empty
            if self.stored[&resource_type] == 0 {
                self.stored.remove(&resource_type);
            }
        }

        withdrawn
    }

    /// Get amount of specific resource
    pub fn get_amount(&self, resource_type: &ResourceType) -> u32 {
        *self.stored.get(resource_type).unwrap_or(&0)
    }

    /// Check if storage is full
    pub fn is_full(&self) -> bool {
        self.current_total >= self.total_capacity
    }

    /// Check if storage is empty
    pub fn is_empty(&self) -> bool {
        self.current_total == 0
    }

    /// Get fill percentage (0.0 to 1.0)
    pub fn fill_percentage(&self) -> f32 {
        self.current_total as f32 / self.total_capacity as f32
    }
}

/// Stockpile - an open-air storage area
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct Stockpile {
    /// Size of the stockpile in tiles (e.g., 3x3)
    pub width: u32,
    pub height: u32,

    /// Resources stored per tile
    pub capacity_per_tile: u32,

    /// Whether this is a priority stockpile
    pub is_priority: bool,
}

impl Stockpile {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            capacity_per_tile: 100, // 100 items per tile
            is_priority: false,
        }
    }

    pub fn total_capacity(&self) -> u32 {
        self.width * self.height * self.capacity_per_tile
    }
}

/// Warehouse - enclosed storage building
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct Warehouse {
    /// Storage efficiency multiplier
    pub efficiency: f32,

    /// Protection from decay/damage
    pub protection_level: f32,

    /// Whether it needs workers to function
    pub requires_workers: bool,

    /// Current worker count
    pub assigned_workers: u32,

    /// Required workers for full efficiency
    pub required_workers: u32,
}

impl Warehouse {
    pub fn new() -> Self {
        Self {
            efficiency: 1.5,       // 50% more capacity than stockpiles
            protection_level: 0.9, // 90% protection from decay
            requires_workers: true,
            assigned_workers: 0,
            required_workers: 2,
        }
    }

    pub fn get_efficiency(&self) -> f32 {
        if !self.requires_workers {
            return self.efficiency;
        }

        let worker_ratio = self.assigned_workers as f32 / self.required_workers as f32;
        self.efficiency * worker_ratio.min(1.0)
    }
}

/// Silo - specialized storage for specific resources
#[derive(Component, Clone, Debug, Reflect)]
pub struct Silo {
    /// The specific resource this silo stores
    pub resource_type: ResourceType,

    /// High capacity for single resource
    pub capacity: u32,

    /// Preservation quality (reduces decay)
    pub preservation: f32,
}

impl Silo {
    pub fn new(resource_type: ResourceType, capacity: u32) -> Self {
        Self {
            resource_type,
            capacity,
            preservation: 0.95, // 95% preservation
        }
    }
}

/// Component for units carrying resources to storage
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct StorageTask {
    /// Target storage entity
    pub target_storage: Option<Entity>,

    /// Resource being transported
    pub carrying_type: Option<ResourceType>,

    /// Amount being transported
    pub carrying_amount: u32,

    /// Task state
    pub state: StorageTaskState,

    /// Ticks until next state change
    pub progress_counter: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Reflect)]
pub enum StorageTaskState {
    #[default]
    Idle,
    MovingToPickup,
    PickingUp,
    MovingToStorage,
    Depositing,
    Complete,
}

impl StorageTask {
    pub fn new(target: Entity, resource_type: ResourceType, amount: u32) -> Self {
        Self {
            target_storage: Some(target),
            carrying_type: Some(resource_type),
            carrying_amount: amount,
            state: StorageTaskState::MovingToPickup,
            progress_counter: 0,
        }
    }

    /// Update task progress (called on ticks)
    pub fn tick_update(&mut self) -> bool {
        if self.progress_counter > 0 {
            self.progress_counter -= 1;
            return false;
        }

        // State transition logic
        match self.state {
            StorageTaskState::PickingUp => {
                self.state = StorageTaskState::MovingToStorage;
                self.progress_counter = 50; // Time to walk (5 seconds at 10 TPS)
            }
            StorageTaskState::Depositing => {
                self.state = StorageTaskState::Complete;
                return true; // Task complete
            }
            _ => {}
        }

        false
    }
}

/// Tag for storage buildings that need regular updates
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct StorageUpdateTag;

/// Event for when storage contents change
#[derive(Event, Clone, Debug)]
pub struct StorageChangedEvent {
    pub storage_entity: Entity,
    pub resource_type: ResourceType,
    pub old_amount: u32,
    pub new_amount: u32,
    pub change_type: StorageChangeType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StorageChangeType {
    Deposit,
    Withdrawal,
    Decay,
    Transfer,
}
