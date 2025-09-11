mod crafting_station;
mod crafting_system;
mod recipes;

pub use crafting_station::CraftingStationType;
pub use crafting_system::CraftingSystem;
pub use recipes::{Recipe, RecipeRegistry};

use crate::debug::{DebugLevel, DebugSystem};
use bevy::prelude::*;

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RecipeRegistry>()
            .init_resource::<CraftingSystem>()
            .add_systems(
                Startup,
                (crafting_init_system, register_recipes_system).chain(),
            )
            .add_systems(Update, crafting_update_system);
    }
}

fn crafting_init_system(debug: Res<DebugSystem>) {
    debug.log(DebugLevel::Info, "CRAFTING", "Crafting system initialized");
}

fn register_recipes_system(mut registry: ResMut<RecipeRegistry>, debug: Res<DebugSystem>) {
    registry.register_default_recipes();
    debug.log(
        DebugLevel::Info,
        "CRAFTING",
        &format!("Registered {} recipes", registry.count()),
    );
}

fn crafting_update_system(
    mut crafting_system: ResMut<CraftingSystem>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    crafting_system.update(time.delta_secs(), &debug);
}
