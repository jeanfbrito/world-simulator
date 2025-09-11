use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CraftingStationType {
    None,         // Hand crafting
    Workbench,    // Basic crafting
    Furnace,      // Smelting and cooking
    Anvil,        // Advanced metalworking
    Kitchen,      // Food preparation
    AlchemyTable, // Potions and magic
}

impl CraftingStationType {
    pub fn name(&self) -> &str {
        match self {
            Self::None => "Hand Crafting",
            Self::Workbench => "Workbench",
            Self::Furnace => "Furnace",
            Self::Anvil => "Anvil",
            Self::Kitchen => "Kitchen",
            Self::AlchemyTable => "Alchemy Table",
        }
    }

    pub fn speed_multiplier(&self) -> f32 {
        match self {
            Self::None => 1.0,
            Self::Workbench => 1.5,
            Self::Furnace => 1.2,
            Self::Anvil => 2.0,
            Self::Kitchen => 1.3,
            Self::AlchemyTable => 1.1,
        }
    }

    pub fn quality_bonus(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Workbench => 0.1,
            Self::Furnace => 0.15,
            Self::Anvil => 0.25,
            Self::Kitchen => 0.1,
            Self::AlchemyTable => 0.2,
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CraftingStation {
    pub station_type: CraftingStationType,
    pub position: (i32, i32),
    pub in_use: bool,
    pub user: Option<Entity>,
    pub queue_capacity: usize,
}

impl CraftingStation {
    pub fn new(station_type: CraftingStationType, position: (i32, i32)) -> Self {
        let capacity = match station_type {
            CraftingStationType::None => 1,
            CraftingStationType::Workbench => 3,
            CraftingStationType::Furnace => 5,
            CraftingStationType::Anvil => 2,
            CraftingStationType::Kitchen => 4,
            CraftingStationType::AlchemyTable => 2,
        };

        Self {
            station_type,
            position,
            in_use: false,
            user: None,
            queue_capacity: capacity,
        }
    }

    pub fn start_use(&mut self, user: Entity) -> bool {
        if !self.in_use {
            self.in_use = true;
            self.user = Some(user);
            info!(
                "[STATION] {} now in use at {:?}",
                self.station_type.name(),
                self.position
            );
            true
        } else {
            false
        }
    }

    pub fn stop_use(&mut self) {
        if self.in_use {
            info!(
                "[STATION] {} no longer in use at {:?}",
                self.station_type.name(),
                self.position
            );
            self.in_use = false;
            self.user = None;
        }
    }

    pub fn is_available(&self) -> bool {
        !self.in_use
    }
}
