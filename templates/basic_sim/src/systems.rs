//! Custom simulation systems

use std::collections::HashMap;
use world_sim::prelude::*;

/// Movement system for entity movement
pub struct MovementSystem {
    query: Query<(With<Position>, With<Movement>)>,
}

impl MovementSystem {
    pub fn new() -> Self {
        Self {
            query: Query::new(),
        }
    }
}

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        for (entity, (position, movement)) in self.query.iter(world) {
            // Update position based on movement
            if let Some(target) = movement.target {
                let dx = target.0 - position.x;
                let dy = target.1 - position.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance > 0.1 {
                    // Move towards target
                    let move_distance = movement.speed * dt;
                    let ratio = (move_distance / distance).min(1.0);

                    position.x += dx * ratio;
                    position.y += dy * ratio;
                } else {
                    // Reached target
                    position.x = target.0;
                    position.y = target.1;
                    movement.target = None;
                }
            }
        }
    }
}

/// Resource gathering system
pub struct ResourceGatheringSystem {
    query: Query<(With<Position>, With<Inventory>, With<GatheringSkill>)>,
    resource_query: Query<With<Resource>>,
}

impl ResourceGatheringSystem {
    pub fn new() -> Self {
        Self {
            query: Query::new(),
            resource_query: Query::new(),
        }
    }
}

impl System for ResourceGatheringSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Process gathering for entities
        for (entity, (position, inventory, gathering_skill)) in self.query.iter(world) {
            // Find nearby resources
            let mut nearest_resource = None;
            let mut nearest_distance = f32::MAX;

            for (resource_entity, resource) in self.resource_query.iter(world) {
                let resource_position = world.get_component::<Position>(resource_entity).unwrap();
                let distance = ((position.x - resource_position.x).powi(2)
                    + (position.y - resource_position.y).powi(2)).sqrt();

                if distance < 5.0 && distance < nearest_distance {
                    nearest_distance = distance;
                    nearest_resource = Some((resource_entity, resource));
                }
            }

            // Gather from nearest resource
            if let Some((resource_entity, resource)) = nearest_resource {
                let gathering_rate = gathering_skill.skill * dt;

                if inventory.can_add_resource(&resource.resource_type, gathering_rate as u32) {
                    let gathered = resource.take_amount(gathering_rate as u32);
                    inventory.add_resource(&resource.resource_type, gathered);

                    // Update skill through practice
                    gathering_skill.experience += gathering_rate * 0.1;
                    gathering_skill.update_skill();
                }
            }
        }
    }
}

/// Crafting system for item creation
pub struct CraftingSystem {
    query: Query<With<Inventory>>,
    recipes: HashMap<String, Recipe>,
}

impl CraftingSystem {
    pub fn new() -> Self {
        let mut recipes = HashMap::new();

        // Add basic recipes
        recipes.insert("wooden_plank".to_string(), Recipe {
            name: "Wooden Plank".to_string(),
            inputs: vec![RecipeInput {
                resource_type: "wood".to_string(),
                amount: 2,
            }],
            outputs: vec![RecipeOutput {
                resource_type: "wooden_plank".to_string(),
                amount: 4,
            }],
            crafting_time: 5.0,
            required_tools: vec![],
        });

        recipes.insert("stone_tool".to_string(), Recipe {
            name: "Stone Tool".to_string(),
            inputs: vec![
                RecipeInput {
                    resource_type: "wood".to_string(),
                    amount: 1,
                },
                RecipeInput {
                    resource_type: "stone".to_string(),
                    amount: 2,
                },
            ],
            outputs: vec![RecipeOutput {
                resource_type: "stone_tool".to_string(),
                amount: 1,
            }],
            crafting_time: 10.0,
            required_tools: vec![],
        });

        Self {
            query: Query::new(),
            recipes,
        }
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.insert(recipe.name.clone(), recipe);
    }
}

impl System for CraftingSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        for (entity, inventory) in self.query.iter(world) {
            // Check for ongoing crafting
            if let Some(crafting) = world.get_component::<Crafting>(entity) {
                crafting.progress += dt;

                if crafting.progress >= crafting.recipe.crafting_time {
                    // Complete crafting
                    for output in &crafting.recipe.outputs {
                        inventory.add_resource(&output.resource_type, output.amount);
                    }

                    // Remove crafting component
                    world.remove_component::<Crafting>(entity);
                }
            } else {
                // Start new crafting if we have enough resources
                for recipe in self.recipes.values() {
                    if self.can_craft(inventory, recipe) {
                        // Consume inputs
                        for input in &recipe.inputs {
                            inventory.remove_resource(&input.resource_type, input.amount);
                        }

                        // Start crafting
                        let crafting = Crafting {
                            recipe: recipe.clone(),
                            progress: 0.0,
                        };
                        world.add_component(entity, crafting);
                        break;
                    }
                }
            }
        }
    }
}

impl CraftingSystem {
    fn can_craft(&self, inventory: &Inventory, recipe: &Recipe) -> bool {
        for input in &recipe.inputs {
            if inventory.get_resource_amount(&input.resource_type) < input.amount {
                return false;
            }
        }
        true
    }
}

/// AI system for entity behavior
pub struct AISystem {
    query: Query<With<AIComponent>>,
    goap_planner: GOAPPlanner,
    utility_ai: UtilityAI,
}

impl AISystem {
    pub fn new() -> Self {
        let mut planner = GOAPPlanner::new();

        // Add GOAP actions
        planner.add_action(GOAPAction {
            name: "gather_wood".to_string(),
            cost: 5.0,
            preconditions: vec![|world| world.has_nearby_resource("wood")],
            effects: vec![|world| world.add_to_inventory("wood", 1)],
        });

        planner.add_action(GOAPAction {
            name: "gather_stone".to_string(),
            cost: 5.0,
            preconditions: vec![|world| world.has_nearby_resource("stone")],
            effects: vec![|world| world.add_to_inventory("stone", 1)],
        });

        let mut utility_ai = UtilityAI::new();

        // Add utility behaviors
        utility_ai.add_behavior(UtilityBehavior {
            name: "resource_gathering".to_string(),
            utility: |world| {
                if world.get_inventory_total() < 5 {
                    0.8
                } else {
                    0.2
                }
            },
            action: |world| world.start_gathering(),
        });

        utility_ai.add_behavior(UtilityBehavior {
            name: "crafting".to_string(),
            utility: |world| {
                if world.can_craft_any() && world.get_inventory_total() >= 3 {
                    0.6
                } else {
                    0.1
                }
            },
            action: |world| world.start_crafting(),
        });

        Self {
            query: Query::new(),
            goap_planner: planner,
            utility_ai,
        }
    }
}

impl System for AISystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        for (entity, ai) in self.query.iter(world) {
            // Update AI state
            ai.update(dt);

            // Select behavior based on AI type
            match ai.ai_type {
                AIType::GOAP => {
                    // Goal-oriented action planning
                    if ai.needs_replanning() {
                        if let Some(plan) = self.goap_planner.plan(world, &ai.current_goal) {
                            ai.set_plan(plan);
                        }
                    }
                    ai.execute_plan(world);
                }
                AIType::Utility => {
                    // Utility-based AI
                    let behavior = self.utility_ai.select_behavior(world);
                    behavior.execute(world);
                }
                AIType::StateBased => {
                    // State machine-based AI
                    ai.update_state_machine(world);
                }
            }
        }
    }
}

/// Helper components and structures

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub target: Option<(f32, f32)>,
}

impl Movement {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            target: None,
        }
    }
}

#[derive(Component)]
pub struct Inventory {
    pub items: HashMap<String, u32>,
    pub capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: HashMap::new(),
            capacity,
        }
    }

    pub fn can_add_resource(&self, resource_type: &str, amount: u32) -> bool {
        let current = self.items.get(resource_type).unwrap_or(&0);
        (*current + amount) as usize <= self.capacity
    }

    pub fn add_resource(&mut self, resource_type: &str, amount: u32) {
        let entry = self.items.entry(resource_type.to_string()).or_insert(0);
        *entry += amount;
    }

    pub fn remove_resource(&mut self, resource_type: &str, amount: u32) {
        if let Some(entry) = self.items.get_mut(resource_type) {
            *entry = entry.saturating_sub(amount);
        }
    }

    pub fn get_resource_amount(&self, resource_type: &str) -> u32 {
        *self.items.get(resource_type).unwrap_or(&0)
    }
}

#[derive(Component)]
pub struct GatheringSkill {
    pub skill: f32,
    pub experience: f32,
}

impl GatheringSkill {
    pub fn new(skill: f32) -> Self {
        Self {
            skill,
            experience: 0.0,
        }
    }

    pub fn update_skill(&mut self) {
        // Simple skill progression
        let required_exp = self.skill * 100.0;
        if self.experience >= required_exp {
            self.skill += 0.1;
            self.experience = 0.0;
        }
    }
}

#[derive(Component)]
pub struct Crafting {
    pub recipe: Recipe,
    pub progress: f32,
}

#[derive(Component)]
pub struct AIComponent {
    pub ai_type: AIType,
    pub current_goal: Goal,
    pub plan: Option<Vec<GOAPAction>>,
    pub state: AIState,
}

impl AIComponent {
    pub fn new(ai_type: AIType) -> Self {
        Self {
            ai_type,
            current_goal: Goal::survival(),
            plan: None,
            state: AIState::Idle,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update AI internal state
    }

    pub fn needs_replanning(&self) -> bool {
        self.plan.is_none() || self.state == AIState::Planning
    }

    pub fn set_plan(&mut self, plan: Vec<GOAPAction>) {
        self.plan = Some(plan);
        self.state = AIState::Executing;
    }

    pub fn execute_plan(&mut self, world: &mut World) {
        if let Some(ref mut plan) = self.plan {
            if let Some(action) = plan.first() {
                // Execute action
                if self.execute_action(action, world) {
                    plan.remove(0);
                }
            }
        }
    }

    pub fn update_state_machine(&mut self, world: &mut World) {
        match self.state {
            AIState::Idle => {
                // Decide on next action
                self.state = AIState::Planning;
            }
            AIState::Gathering => {
                // Continue gathering
            }
            AIState::Crafting => {
                // Continue crafting
            }
            AIState::Moving => {
                // Continue moving
            }
            _ => {}
        }
    }

    fn execute_action(&self, action: &GOAPAction, world: &mut World) -> bool {
        // Execute the specific action
        // Return true if action completed
        false
    }
}

#[derive(Debug, Clone)]
pub enum AIType {
    GOAP,
    Utility,
    StateBased,
}

#[derive(Debug, Clone)]
pub enum AIState {
    Idle,
    Planning,
    Executing,
    Gathering,
    Crafting,
    Moving,
}

#[derive(Debug, Clone)]
pub struct Goal {
    pub name: String,
    pub priority: f32,
}

impl Goal {
    pub fn survival() -> Self {
        Self {
            name: "survival".to_string(),
            priority: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub inputs: Vec<RecipeInput>,
    pub outputs: Vec<RecipeOutput>,
    pub crafting_time: f32,
    pub required_tools: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RecipeInput {
    pub resource_type: String,
    pub amount: u32,
}

#[derive(Debug, Clone)]
pub struct RecipeOutput {
    pub resource_type: String,
    pub amount: u32,
}

#[derive(Debug, Clone)]
pub struct GOAPAction {
    pub name: String,
    pub cost: f32,
    pub preconditions: Vec<fn(&World) -> bool>,
    pub effects: Vec<fn(&mut World)>,
}

#[derive(Debug, Clone)]
pub struct UtilityBehavior {
    pub name: String,
    pub utility: fn(&World) -> f32,
    pub action: fn(&mut World),
}

impl UtilityBehavior {
    pub fn execute(&self, world: &mut World) {
        (self.action)(world);
    }
}

pub struct GOAPPlanner {
    actions: Vec<GOAPAction>,
}

impl GOAPPlanner {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: GOAPAction) {
        self.actions.push(action);
    }

    pub fn plan(&self, world: &World, goal: &Goal) -> Option<Vec<GOAPAction>> {
        // Simplified A* planning
        // In a real implementation, this would be more sophisticated
        Some(vec![])
    }
}

pub struct UtilityAI {
    behaviors: Vec<UtilityBehavior>,
}

impl UtilityAI {
    pub fn new() -> Self {
        Self {
            behaviors: Vec::new(),
        }
    }

    pub fn add_behavior(&mut self, behavior: UtilityBehavior) {
        self.behaviors.push(behavior);
    }

    pub fn select_behavior(&self, world: &World) -> &UtilityBehavior {
        // Select behavior with highest utility
        self.behaviors
            .iter()
            .max_by(|a, b| {
                (a.utility)(world).partial_cmp(&(b.utility)(world)).unwrap()
            })
            .unwrap()
    }
}