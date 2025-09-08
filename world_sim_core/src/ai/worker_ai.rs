//! Worker AI using GOAP for intelligent behavior

use bevy_ecs::prelude::*;
use bevy_dogoap::prelude::*;
use crate::components::*;

/// Create a GOAP planner for a basic worker
pub fn create_worker_planner() -> (Planner, impl Bundle) {
    // Define the main goal - survive and be productive
    let survival_goal = Goal::from_reqs(&[
        IsHungry::is_less(30.0),
        HasEnergy::is_more(50.0),
    ]);
    
    let production_goal = Goal::from_reqs(&[
        HasWood::is_more(5),
    ]);
    
    // Define actions with preconditions and effects
    
    // Eat action - reduces hunger when we have food
    let eat_action = EatAction::new()
        .add_precondition(IsHungry::is_more(50.0))
        .add_precondition(HasFood::is_more(0))
        .add_mutator(IsHungry::decrease(25.0))
        .add_mutator(HasFood::decrease(1))
        .set_cost(1);
    
    // Rest action - restores energy
    let rest_action = RestAction::new()
        .add_precondition(HasEnergy::is_less(30.0))
        .add_mutator(HasEnergy::increase(30.0))
        .set_cost(2);
    
    // Sleep action - fully restores energy at home
    let sleep_action = SleepAction::new()
        .add_precondition(HasEnergy::is_less(20.0))
        .add_precondition(AtHome::is(true))
        .add_mutator(HasEnergy::set(100.0))
        .set_cost(3);
    
    // Harvest wood action
    let harvest_wood_action = HarvestWoodAction::new()
        .add_precondition(AtResource::is(true))
        .add_precondition(HasEnergy::is_more(20.0))
        .add_precondition(InventoryFull::is(false))
        .add_mutator(HasWood::increase(5))
        .add_mutator(HasEnergy::decrease(10.0))
        .set_cost(3);
    
    // Gather food action
    let gather_food_action = GatherFoodAction::new()
        .add_precondition(AtResource::is(true))
        .add_precondition(HasEnergy::is_more(10.0))
        .add_mutator(HasFood::increase(3))
        .add_mutator(HasEnergy::decrease(5.0))
        .set_cost(2);
    
    // Go to resource action
    let go_to_resource_action = GoToResourceAction::new()
        .add_precondition(AtResource::is(false))
        .add_mutator(AtResource::set(true))
        .add_mutator(AtStorage::set(false))
        .add_mutator(AtHome::set(false))
        .add_mutator(HasEnergy::decrease(5.0))
        .set_cost(2);
    
    // Go to storage action
    let go_to_storage_action = GoToStorageAction::new()
        .add_precondition(AtStorage::is(false))
        .add_mutator(AtStorage::set(true))
        .add_mutator(AtResource::set(false))
        .add_mutator(AtHome::set(false))
        .add_mutator(HasEnergy::decrease(5.0))
        .set_cost(2);
    
    // Go home action
    let go_to_home_action = GoToHomeAction::new()
        .add_precondition(AtHome::is(false))
        .add_mutator(AtHome::set(true))
        .add_mutator(AtResource::set(false))
        .add_mutator(AtStorage::set(false))
        .add_mutator(HasEnergy::decrease(5.0))
        .set_cost(2);
    
    // Store resources action
    let store_resources_action = StoreResourcesAction::new()
        .add_precondition(AtStorage::is(true))
        .add_precondition(HasWood::is_more(0))
        .add_mutator(HasWood::set(0))
        .add_mutator(InventoryEmpty::set(true))
        .add_mutator(InventoryFull::set(false))
        .set_cost(1);
    
    // Idle action - fallback when nothing to do
    let idle_action = IdleAction::new()
        .add_mutator(IsIdle::set(true))
        .set_cost(10);
    
    // Create the planner with all actions and initial state
    let (mut planner, components) = create_planner!({
        actions: [
            (EatAction, eat_action),
            (RestAction, rest_action),
            (SleepAction, sleep_action),
            (HarvestWoodAction, harvest_wood_action),
            (GatherFoodAction, gather_food_action),
            (GoToResourceAction, go_to_resource_action),
            (GoToStorageAction, go_to_storage_action),
            (GoToHomeAction, go_to_home_action),
            (StoreResourcesAction, store_resources_action),
            (IdleAction, idle_action)
        ],
        state: [
            IsHungry(30.0),
            HasEnergy(75.0),
            AtResource(false),
            AtStorage(false),
            AtHome(false),
            IsWorking(false),
            IsIdle(true),
            HasWood(0),
            HasFood(2),
            HasStone(0),
            InventoryFull(false),
            InventoryEmpty(true),
            HouseAvailable(true),
            StorageAvailable(true)
        ],
        goals: [survival_goal, production_goal],
    });
    
    // Configure planner behavior
    planner.remove_goal_on_no_plan_found = false;
    planner.always_plan = true;
    planner.current_goal = Some(survival_goal.clone());
    
    (planner, components)
}

/// Create an advanced worker planner with more complex behaviors
pub fn create_advanced_worker_planner() -> (Planner, impl Bundle) {
    // Multiple prioritized goals
    let survival_goal = Goal::from_reqs(&[
        IsHungry::is_less(20.0),
        HasEnergy::is_more(60.0),
    ]);
    
    let production_goal = Goal::from_reqs(&[
        SettlementWood::is_more(50),
        SettlementFood::is_more(30),
    ]);
    
    let construction_goal = Goal::from_reqs(&[
        HouseAvailable::is(true),
        StorageAvailable::is(true),
    ]);
    
    // More complex actions including building
    let build_house_action = BuildHouseAction::new()
        .add_precondition(HasWood::is_more(10))
        .add_precondition(HasEnergy::is_more(40.0))
        .add_precondition(HouseAvailable::is(false))
        .add_mutator(HouseAvailable::set(true))
        .add_mutator(HasWood::decrease(10))
        .add_mutator(HasEnergy::decrease(30.0))
        .set_cost(5);
    
    let build_storage_action = BuildStorageAction::new()
        .add_precondition(HasWood::is_more(15))
        .add_precondition(HasStone::is_more(5))
        .add_precondition(HasEnergy::is_more(50.0))
        .add_precondition(StorageAvailable::is(false))
        .add_mutator(StorageAvailable::set(true))
        .add_mutator(HasWood::decrease(15))
        .add_mutator(HasStone::decrease(5))
        .add_mutator(HasEnergy::decrease(40.0))
        .set_cost(6);
    
    // Create planner with basic actions plus building actions
    let (mut planner, components) = create_worker_planner();
    
    // Add construction goal
    planner.goals.push(construction_goal);
    
    (planner, components)
}

/// System to spawn workers with GOAP AI
pub fn spawn_worker_with_goap(
    commands: &mut Commands,
    position: Position,
    name: String,
) -> Entity {
    let (planner, goap_components) = create_worker_planner();
    
    commands.spawn((
        // Core components
        WorkerComponent::new(name),
        PositionComponent::new(position.x, position.y),
        MovementComponent::new(5.0),
        InventoryComponent::new(20),
        
        // GOAP components
        planner,
        goap_components,
        
        // Additional components
        Name::new("Worker"),
    )).id()
}

/// System to update GOAP state based on world state
pub fn sync_goap_state_system(
    mut query: Query<(
        &WorkerComponent,
        &InventoryComponent,
        &PositionComponent,
        &mut IsHungry,
        &mut HasEnergy,
        &mut HasWood,
        &mut HasFood,
        &mut HasStone,
        &mut InventoryFull,
        &mut InventoryEmpty,
    )>,
) {
    for (
        worker,
        inventory,
        _pos,
        mut is_hungry,
        mut has_energy,
        mut has_wood,
        mut has_food,
        mut has_stone,
        mut inventory_full,
        mut inventory_empty,
    ) in query.iter_mut() {
        // Sync hunger and energy
        is_hungry.0 = (worker.hunger * 100.0) as f64;
        has_energy.0 = (worker.energy * 100.0) as f64;
        
        // Sync inventory
        has_wood.0 = inventory.get_resource_amount(world_sim_interface::ResourceType::Wood);
        has_food.0 = inventory.get_resource_amount(world_sim_interface::ResourceType::Food);
        has_stone.0 = inventory.get_resource_amount(world_sim_interface::ResourceType::Stone);
        
        // Update inventory status
        inventory_full.0 = inventory.is_full();
        inventory_empty.0 = inventory.is_empty();
    }
}