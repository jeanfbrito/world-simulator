mod inventory;
mod item_system;
pub mod plant_growth;
mod resource_types;

pub use inventory::Inventory;
pub use item_system::{ArmorType, Item, ItemStack, ItemType, ToolType, WeaponType};
pub use resource_types::ResourceType;

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
