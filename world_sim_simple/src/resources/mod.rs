mod resource_types;
mod item_system;
mod inventory;

pub use resource_types::{ResourceType, ResourceCategory, ResourceProperties};
pub use item_system::{Item, ItemStack, ItemType, ItemRarity};
pub use inventory::{Inventory, InventorySlot, create_starter_inventory};

use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, resources_init_system);
    }
}

fn resources_init_system() {
    info!("[RESOURCES] Resource and item systems initialized");
}