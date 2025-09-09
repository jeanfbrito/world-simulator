use bevy::prelude::*;
// use bevy_dogoap::prelude::*; // Temporarily disabled for testing

// Basic needs and states for workers
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct IsHungry(pub f64);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct HasEnergy(pub f64);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct IsWorking(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct IsIdle(pub bool);

// Location states
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct AtResource(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct AtStorage(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct AtHome(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct AtCraftingStation(pub bool);

// Inventory states
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct HasWood(pub u32);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct HasFood(pub u32);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct HasStone(pub u32);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct InventoryFull(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct InventoryEmpty(pub bool);

// Building availability states
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct HasHouse(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct HouseAvailable(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct StorageAvailable(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct FarmAvailable(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct WorkshopAvailable(pub bool);

// Task completion states
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct HarvestComplete(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct BuildingComplete(pub bool);

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct CraftingComplete(pub bool);

// Global settlement states (for shared goals)
#[derive(Resource, Clone, Debug, Default)]
pub struct SettlementState {
    pub food_supply: u32,
    pub wood_supply: u32,
    pub stone_supply: u32,
    pub population_count: i32,
    pub building_count: i32,
}

// GOAP state definitions for the planner
pub fn register_goap_states(app: &mut App) {
    // Register all state components manually for now
    app.register_type::<IsHungry>()
        .register_type::<HasEnergy>()
        .register_type::<IsWorking>()
        .register_type::<IsIdle>()
        .register_type::<AtResource>()
        .register_type::<AtStorage>()
        .register_type::<AtHome>()
        .register_type::<AtCraftingStation>()
        .register_type::<HasWood>()
        .register_type::<HasFood>()
        .register_type::<HasStone>()
        .register_type::<InventoryFull>()
        .register_type::<InventoryEmpty>()
        .register_type::<HasHouse>()
        .register_type::<HouseAvailable>()
        .register_type::<StorageAvailable>()
        .register_type::<FarmAvailable>()
        .register_type::<WorkshopAvailable>()
        .register_type::<HarvestComplete>()
        .register_type::<BuildingComplete>()
        .register_type::<CraftingComplete>();
    
    // Add settlement state resource
    app.insert_resource(SettlementState::default());
}

// Helper functions to update states based on worker conditions
impl IsHungry {
    pub fn update(&mut self, hunger_level: f64) {
        self.0 = hunger_level.clamp(0.0, 1.0);
    }
    
    pub fn is_hungry(&self) -> bool {
        self.0 > 0.3 // Hungry when above 30%
    }
}

impl HasEnergy {
    pub fn update(&mut self, energy_level: f64) {
        self.0 = energy_level.clamp(0.0, 1.0);
    }
    
    pub fn is_tired(&self) -> bool {
        self.0 < 0.3 // Tired when below 30%
    }
}

// State transition helpers
pub fn update_location_states(
    entity: Entity,
    at_resource: bool,
    at_storage: bool,
    at_home: bool,
    at_crafting: bool,
    commands: &mut Commands,
) {
    commands.entity(entity)
        .insert(AtResource(at_resource))
        .insert(AtStorage(at_storage))
        .insert(AtHome(at_home))
        .insert(AtCraftingStation(at_crafting));
}

pub fn update_inventory_states(
    entity: Entity,
    wood: u32,
    food: u32,
    stone: u32,
    max_capacity: u32,
    commands: &mut Commands,
) {
    let total = wood + food + stone;
    commands.entity(entity)
        .insert(HasWood(wood))
        .insert(HasFood(food))
        .insert(HasStone(stone))
        .insert(InventoryFull(total >= max_capacity))
        .insert(InventoryEmpty(total == 0));
}