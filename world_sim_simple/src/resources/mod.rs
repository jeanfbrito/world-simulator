mod resource_types;
mod item_system;
mod inventory;
pub mod plant_growth;

pub use resource_types::{ResourceType, ResourceCategory, ResourceProperties};
pub use item_system::{Item, ItemStack, ItemType, ItemRarity, ToolType, WeaponType, ArmorType};
pub use inventory::{Inventory, InventorySlot, create_starter_inventory};
pub use plant_growth::{PlantGrowth, PlantProduce, PlantYield, ProcessingType, ProduceQuality, Season};

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