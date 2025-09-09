use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use crate::ai::shared_state::*;
use crate::debug::{DebugSystem, DebugLevel};

// GOAP Actions as Components

#[derive(Component, Reflect, Clone, Default, ActionComponent)]
#[reflect(Component)]
pub struct EatAction;

#[derive(Component, Reflect, Clone, Default, ActionComponent)]
#[reflect(Component)]
pub struct RestAction;

#[derive(Component, Reflect, Clone, Default, ActionComponent)]
#[reflect(Component)]
pub struct GatherWoodAction;

#[derive(Component, Reflect, Clone, Default, ActionComponent)]
#[reflect(Component)]
pub struct GatherStoneAction;

#[derive(Component, Reflect, Clone, Default, ActionComponent)]
#[reflect(Component)]
pub struct GatherFoodAction;

#[derive(Component, Reflect, Clone, Default, ActionComponent)]
#[reflect(Component)]
pub struct BuildShelterAction;

// Action execution systems

pub fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<(Entity, &EatAction, &mut WorkerStats, &mut IsHungry, &mut HasFood)>,
    debug: Res<DebugSystem>,
) {
    for (entity, _action, mut stats, mut is_hungry, mut has_food) in query.iter_mut() {
        if has_food.0 > 0 {
            has_food.0 -= 1;
            stats.hunger = 1.0;
            is_hungry.0 = false;
            
            debug.log(DebugLevel::Info, "GOAP_ACTION", "Worker ate food");
            commands.entity(entity).remove::<EatAction>();
        }
    }
}

pub fn handle_rest_action(
    mut commands: Commands,
    mut query: Query<(Entity, &RestAction, &mut WorkerStats, &mut IsTired)>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    for (entity, _action, mut stats, mut is_tired) in query.iter_mut() {
        // Rest over time
        stats.energy = (stats.energy + time.delta_seconds() * 0.3).min(1.0);
        
        if stats.energy >= 1.0 {
            is_tired.0 = false;
            debug.log(DebugLevel::Info, "GOAP_ACTION", "Worker fully rested");
            commands.entity(entity).remove::<RestAction>();
        }
    }
}

pub fn handle_gather_wood_action(
    mut commands: Commands,
    mut query: Query<(Entity, &GatherWoodAction, &mut HasWood)>,
    debug: Res<DebugSystem>,
) {
    for (entity, _action, mut has_wood) in query.iter_mut() {
        // Simulate gathering wood
        has_wood.0 += 5;
        debug.log(DebugLevel::Info, "GOAP_ACTION", &format!("Gathered wood, total: {}", has_wood.0));
        commands.entity(entity).remove::<GatherWoodAction>();
    }
}

pub fn handle_gather_stone_action(
    mut commands: Commands,
    mut query: Query<(Entity, &GatherStoneAction, &mut HasStone)>,
    debug: Res<DebugSystem>,
) {
    for (entity, _action, mut has_stone) in query.iter_mut() {
        has_stone.0 += 3;
        debug.log(DebugLevel::Info, "GOAP_ACTION", &format!("Gathered stone, total: {}", has_stone.0));
        commands.entity(entity).remove::<GatherStoneAction>();
    }
}

pub fn handle_gather_food_action(
    mut commands: Commands,
    mut query: Query<(Entity, &GatherFoodAction, &mut HasFood)>,
    debug: Res<DebugSystem>,
) {
    for (entity, _action, mut has_food) in query.iter_mut() {
        has_food.0 += 3;
        debug.log(DebugLevel::Info, "GOAP_ACTION", &format!("Gathered food, total: {}", has_food.0));
        commands.entity(entity).remove::<GatherFoodAction>();
    }
}

pub fn handle_build_shelter_action(
    mut commands: Commands,
    mut query: Query<(Entity, &BuildShelterAction, &mut HasWood, &mut HasStone, &mut NeedsShelter)>,
    debug: Res<DebugSystem>,
) {
    for (entity, _action, mut has_wood, mut has_stone, mut needs_shelter) in query.iter_mut() {
        if has_wood.0 >= 10 && has_stone.0 >= 5 {
            has_wood.0 -= 10;
            has_stone.0 -= 5;
            needs_shelter.0 = false;
            debug.log(DebugLevel::Info, "GOAP_ACTION", "Built shelter!");
            commands.entity(entity).remove::<BuildShelterAction>();
        }
    }
}

// Type aliases for clarity
type WorkerPlanner = bevy_dogoap::prelude::Planner;
type WorkerComponents = Bundle;

// Create a planner for a worker entity
pub fn create_worker_planner(ai_mode: &AIMode) -> (WorkerPlanner, impl Bundle) {
    match ai_mode {
        AIMode::Planning => {
            // Complex planning goals
            let goal_build = Goal::from_reqs(&[
                NeedsShelter::is(false),
            ]);
            
            let goal_resources = Goal::from_reqs(&[
                HasWood::is_more_than(20),
                HasStone::is_more_than(10),
            ]);
            
            // Actions with preconditions and effects
            let eat_action = EatAction::new()
                .add_precondition(HasFood::is_more_than(0))
                .add_mutator(IsHungry::set(false));
            
            let rest_action = RestAction::new()
                .add_mutator(IsTired::set(false));
            
            let gather_wood = GatherWoodAction::new()
                .add_precondition(IsTired::is(false))
                .add_mutator(HasWood::increase(5));
            
            let gather_stone = GatherStoneAction::new()
                .add_precondition(IsTired::is(false))
                .add_mutator(HasStone::increase(3));
            
            let gather_food = GatherFoodAction::new()
                .add_precondition(IsTired::is(false))
                .add_mutator(HasFood::increase(3));
            
            let build_shelter = BuildShelterAction::new()
                .add_precondition(HasWood::is_more_than(10))
                .add_precondition(HasStone::is_more_than(5))
                .add_mutator(NeedsShelter::set(false))
                .add_mutator(HasWood::decrease(10))
                .add_mutator(HasStone::decrease(5));
            
            create_planner!({
                actions: [
                    (EatAction, eat_action),
                    (RestAction, rest_action),
                    (GatherWoodAction, gather_wood),
                    (GatherStoneAction, gather_stone),
                    (GatherFoodAction, gather_food),
                    (BuildShelterAction, build_shelter),
                ],
                state: [
                    IsHungry(false),
                    IsTired(false),
                    HasWood(0),
                    HasStone(0),
                    HasFood(5),
                    NeedsShelter(true),
                ],
                goals: [goal_build, goal_resources],
            })
        }
        _ => {
            // Simple reactive goals for immediate needs
            let goal_not_hungry = Goal::from_reqs(&[IsHungry::is(false)]);
            let goal_not_tired = Goal::from_reqs(&[IsTired::is(false)]);
            
            let eat_action = EatAction::new()
                .add_precondition(HasFood::is_more_than(0))
                .add_mutator(IsHungry::set(false));
            
            let rest_action = RestAction::new()
                .add_mutator(IsTired::set(false));
            
            let gather_food = GatherFoodAction::new()
                .add_mutator(HasFood::increase(3));
            
            create_planner!({
                actions: [
                    (EatAction, eat_action),
                    (RestAction, rest_action),
                    (GatherFoodAction, gather_food),
                ],
                state: [
                    IsHungry(true),
                    IsTired(false),
                    HasFood(0),
                ],
                goals: [goal_not_hungry, goal_not_tired],
            })
        }
    }
}

// System to run GOAP planning
pub fn dogoap_planning_system(
    mut commands: Commands,
    query: Query<(Entity, &AIMode), (With<WorkerStats>, Without<WorkerPlanner>)>,
    debug: Res<DebugSystem>,
) {
    for (entity, ai_mode) in query.iter() {
        if *ai_mode == AIMode::Planning {
            debug.log(DebugLevel::Info, "DOGOAP", "Creating planner for worker");
            
            let (planner, components) = create_worker_planner(ai_mode);
            commands.entity(entity)
                .insert(planner)
                .insert(components);
        }
    }
}