mod inventory;
mod item_system;
pub mod plant_growth;
mod resource_types;

pub use inventory::Inventory;
pub use item_system::{ArmorType, Item, ItemRarity, ItemStack, ItemRegistry, ItemType, ToolType, WeaponType, ConsumableType};
pub use resource_types::{ResourceRegistry, ResourceType};

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
