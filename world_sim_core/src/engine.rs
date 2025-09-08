//! Core simulation engine implementation

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use world_sim_interface::{
    EngineCommand, EngineEvent, EngineObserver, WorldSnapshot, CommandResult,
    WorldConfig, EntityId, EntitySnapshot, Position, EntityType, 
    BuildingType, ResourceType, Recipe, RecipeId,
};
use crate::components::*;
use crate::systems;
use crate::recipes::RecipeRegistry;
use crate::resources::WorldState;
use std::collections::HashMap;

/// Core simulation engine
pub struct SimulationEngine {
    app: App,
    observers: Vec<Box<dyn EngineObserver>>,
    entity_id_counter: EntityId,
    tick_count: u64,
    recipes: Vec<Recipe>,
}

impl SimulationEngine {
    pub fn new() -> Self {
        let mut app = App::new();
        
        // Add resources
        app.init_resource::<WorldState>();
        app.init_resource::<systems::EventQueue>();
        app.init_resource::<systems::HarvestRequests>();
        app.init_resource::<systems::MoveRequests>();
        app.init_resource::<systems::BuildRequests>();
        app.init_resource::<systems::RecipeRequests>();
        app.insert_resource(RecipeRegistry::new());
        
        // Add systems
        app.add_systems(Update, (
            systems::handle_move_commands,
            systems::pathfinding_system,
            systems::movement_system,
            systems::start_harvest_system,
            systems::harvest_system,
            systems::handle_build_commands,
            systems::building_system,
            systems::handle_recipe_commands,
            systems::recipe_system,
        ));
        
        Self {
            app,
            observers: Vec::new(),
            entity_id_counter: 1,
            tick_count: 0,
            recipes: Vec::new(),
        }
    }
    
    pub fn new_world(&mut self, config: WorldConfig) -> Result<(), String> {
        // Clear existing world
        self.app.world_mut().clear_entities();
        self.entity_id_counter = 1;
        self.tick_count = 0;
        
        // Initialize world state
        let mut world_state = WorldState::new(config.clone());
        
        // Update world state resource
        self.app.world_mut().insert_resource(world_state);
        
        // Generate terrain and resources using the old method for now
        self.generate_world(&config);
        
        // Spawn starting workers
        for i in 0..config.starting_workers {
            let pos = Position::new(
                (config.width as i32 / 2 + i as i32) % config.width as i32,
                config.height as i32 / 2,
            );
            self.spawn_worker(pos);
        }
        
        // Emit world created event
        self.emit_event(EngineEvent::WorldCreated {
            width: config.width,
            height: config.height,
            seed: config.seed,
        });
        
        Ok(())
    }
    
    pub fn execute_command(&mut self, command: EngineCommand) -> CommandResult {
        self.emit_event(EngineEvent::CommandReceived {
            command: serde_json::to_value(&command).ok(),
        });
        
        let result = match command {
            EngineCommand::Move { entity_id, target } => {
                self.handle_move_command(entity_id, target)
            }
            EngineCommand::Harvest { worker_id, resource_id } => {
                self.handle_harvest_command(worker_id, resource_id)
            }
            EngineCommand::Build { builder_id, building_type, position } => {
                self.handle_build_command(builder_id, building_type, position)
            }
            EngineCommand::GiveResources { entity_id, resources } => {
                self.handle_give_resources(entity_id, resources)
            }
            EngineCommand::SpawnWorker { position, settlement_id } => {
                self.handle_spawn_worker(position, settlement_id)
            }
            EngineCommand::Store { worker_id, building_id } => {
                self.handle_store_command(worker_id, building_id)
            }
            EngineCommand::AssignWorker { worker_id, building_id } => {
                self.handle_assign_worker(worker_id, building_id)
            }
            EngineCommand::StartRecipe { recipe_id, building_id } => {
                self.handle_start_recipe(building_id, recipe_id)
            }
            _ => CommandResult::failure("Command not implemented"),
        };
        
        self.emit_event(EngineEvent::CommandExecuted {
            success: result.success,
            result: serde_json::to_value(&result).ok(),
        });
        
        result
    }
    
    pub fn tick(&mut self) {
        self.tick_count += 1;
        
        // Emit tick event
        self.emit_event(EngineEvent::Tick { tick: self.tick_count });
        
        // Update world
        self.app.update();
    }
    
    pub fn snapshot(&mut self) -> WorldSnapshot {
        let mut entities = Vec::new();
        
        // Collect all entities
        let world = self.app.world_mut();
        let mut query = world.query::<(Entity, Option<&PositionComponent>, Option<&WorkerComponent>, Option<&BuildingComponent>, Option<&ResourceNodeComponent>, Option<&InventoryComponent>)>();
        
        for (entity, pos, worker, building, resource, inventory) in query.iter(world) {
            let entity_type = if worker.is_some() {
                EntityType::Worker
            } else if let Some(building) = building {
                EntityType::Building(building.building_type.clone())
            } else if let Some(resource) = resource {
                match resource.resource_type {
                    ResourceType::Wood => EntityType::Tree,
                    ResourceType::Food => EntityType::BerryBush,
                    ResourceType::Stone => EntityType::StoneDeposit,
                    _ => EntityType::Tree,
                }
            } else {
                EntityType::Worker
            };
            
            let position = pos.map(|p| p.position).unwrap_or_default();
            
            let mut components = HashMap::new();
            
            // Add inventory to components if present
            if let Some(inv) = inventory {
                let inv_data: HashMap<String, u32> = inv.resources
                    .iter()
                    .map(|(k, v)| (format!("{:?}", k).to_lowercase(), *v))
                    .collect();
                components.insert("inventory".to_string(), serde_json::to_value(inv_data).unwrap());
            }
            
            // Add worker state if present
            if let Some(worker) = worker {
                components.insert("state".to_string(), serde_json::to_value(&worker.state).unwrap());
                components.insert("hunger".to_string(), serde_json::json!(worker.hunger));
                components.insert("happiness".to_string(), serde_json::json!(worker.happiness));
            }
            
            // Add resource amount if present
            if let Some(resource) = resource {
                components.insert("resource_amount".to_string(), serde_json::json!(resource.amount));
            }
            
            // Add building storage/output if present
            if building.is_some() {
                if let Some(storage) = world.get::<StorageComponent>(entity) {
                    let storage_data: HashMap<String, u32> = storage.resources
                        .iter()
                        .map(|(k, v)| (format!("{:?}", k).to_lowercase(), *v))
                        .collect();
                    components.insert("storage".to_string(), serde_json::to_value(storage_data).unwrap());
                }
                
                if let Some(production) = world.get::<ProductionComponent>(entity) {
                    let output_data: HashMap<String, u32> = production.output_storage
                        .iter()
                        .map(|(k, v)| (format!("{:?}", k).to_lowercase(), *v))
                        .collect();
                    components.insert("output".to_string(), serde_json::to_value(output_data).unwrap());
                }
            }
            
            entities.push(EntitySnapshot {
                id: entity.index() as EntityId,
                entity_type,
                position,
                components,
            });
        }
        
        WorldSnapshot {
            tick: self.tick_count,
            entities,
            settlements: Vec::new(),
            global: Default::default(),
        }
    }
    
    pub fn add_observer(&mut self, observer: Box<dyn EngineObserver>) {
        self.observers.push(observer);
    }
    
    pub fn register_recipe(&mut self, recipe: Recipe) {
        // Add to internal list
        self.recipes.push(recipe.clone());
        
        // Also add to registry resource
        if let Some(mut registry) = self.app.world_mut().get_resource_mut::<RecipeRegistry>() {
            registry.register(recipe);
        }
    }
    
    pub fn get_recipes(&self) -> Vec<Recipe> {
        self.recipes.clone()
    }
    
    pub fn get_population_cap(&mut self) -> usize {
        // Base cap + houses
        let mut cap = 5;
        
        let world = self.app.world_mut();
        let mut query = world.query::<&BuildingComponent>();
        for building in query.iter(world) {
            if building.is_complete && matches!(building.building_type, BuildingType::House) {
                cap += 4;
            }
        }
        
        cap
    }
    
    pub fn save_state(&mut self) -> Result<Vec<u8>, String> {
        // Serialize world state
        let snapshot = self.snapshot();
        serde_json::to_vec(&snapshot)
            .map_err(|e| format!("Failed to save state: {}", e))
    }
    
    pub fn load_state(&mut self, data: Vec<u8>) -> Result<(), String> {
        // Deserialize and restore world state
        let snapshot: WorldSnapshot = serde_json::from_slice(&data)
            .map_err(|e| format!("Failed to load state: {}", e))?;
        
        // Clear current world
        self.app.world_mut().clear_entities();
        
        // Restore entities
        // (Implementation would recreate entities from snapshot)
        
        self.tick_count = snapshot.tick;
        
        Ok(())
    }
    
    // Private helper methods
    
    fn generate_world(&mut self, config: &WorldConfig) {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        
        let mut rng = if let Some(seed) = config.seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };
        
        let num_resources = ((config.width * config.height) as f32 * config.resource_density) as usize;
        
        // Generate trees
        let num_trees = num_resources / 2;
        for _ in 0..num_trees {
            let x = rng.gen_range(0..config.width as i32);
            let y = rng.gen_range(0..config.height as i32);
            self.spawn_resource(Position::new(x, y), ResourceType::Wood, 50);
        }
        
        // Generate berry bushes
        let num_berries = num_resources / 3;
        for _ in 0..num_berries {
            let x = rng.gen_range(0..config.width as i32);
            let y = rng.gen_range(0..config.height as i32);
            self.spawn_resource(Position::new(x, y), ResourceType::Food, 20);
        }
        
        // Generate stone deposits
        let num_stones = num_resources / 6;
        for _ in 0..num_stones {
            let x = rng.gen_range(0..config.width as i32);
            let y = rng.gen_range(0..config.height as i32);
            self.spawn_resource(Position::new(x, y), ResourceType::Stone, 30);
        }
    }
    
    fn spawn_worker(&mut self, position: Position) -> EntityId {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        let world = self.app.world_mut();
        world.spawn((
            WorkerComponent::new(format!("Worker {}", id)),
            PositionComponent::at(position),
            InventoryComponent::new(20),
            MovementComponent::new(1.0),
            TaskQueueComponent::new(10),
        ));
        
        self.emit_event(EngineEvent::EntitySpawned {
            entity_id: id,
            entity_type: EntityType::Worker,
            position,
        });
        
        id
    }
    
    fn spawn_resource(&mut self, position: Position, resource_type: ResourceType, amount: u32) -> EntityId {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        let entity_type = match resource_type {
            ResourceType::Wood => EntityType::Tree,
            ResourceType::Food => EntityType::BerryBush,
            ResourceType::Stone => EntityType::StoneDeposit,
            _ => EntityType::Tree,
        };
        
        let world = self.app.world_mut();
        world.spawn((
            ResourceNodeComponent::new(resource_type, amount),
            PositionComponent::at(position),
        ));
        
        self.emit_event(EngineEvent::EntitySpawned {
            entity_id: id,
            entity_type,
            position,
        });
        
        id
    }
    
    fn emit_event(&mut self, event: EngineEvent) {
        // Send to observers
        let events = vec![event.clone()];
        for observer in &mut self.observers {
            observer.on_events(&events);
        }
        
        // Also add to internal event queue
        if let Some(mut queue) = self.app.world_mut().get_resource_mut::<systems::EventQueue>() {
            queue.push(event);
        }
    }
    
    // Command handlers
    
    fn handle_move_command(&mut self, entity_id: EntityId, target: Position) -> CommandResult {
        if let Some(mut requests) = self.app.world_mut().get_resource_mut::<systems::MoveRequests>() {
            requests.add(entity_id, target);
        }
        CommandResult::success()
    }
    
    fn handle_harvest_command(&mut self, worker_id: EntityId, resource_id: EntityId) -> CommandResult {
        if let Some(mut requests) = self.app.world_mut().get_resource_mut::<systems::HarvestRequests>() {
            requests.add(worker_id, resource_id);
        }
        CommandResult::success()
    }
    
    fn handle_build_command(&mut self, builder_id: EntityId, building_type: BuildingType, position: Position) -> CommandResult {
        if let Some(mut requests) = self.app.world_mut().get_resource_mut::<systems::BuildRequests>() {
            requests.add(builder_id, building_type, position);
        }
        CommandResult::success()
    }
    
    fn handle_give_resources(&mut self, entity_id: EntityId, resources: HashMap<ResourceType, u32>) -> CommandResult {
        let world = self.app.world_mut();
        
        // Find entity by ID (simplified - would need proper entity mapping)
        let mut query = world.query::<(Entity, Option<&mut InventoryComponent>, Option<&mut StorageComponent>)>();
        
        for (entity, inventory, storage) in query.iter_mut(world) {
            if entity.index() as EntityId == entity_id {
                if let Some(mut inv) = inventory {
                    for (resource_type, amount) in &resources {
                        inv.add_resource(*resource_type, *amount);
                    }
                    return CommandResult::success();
                }
                if let Some(mut stor) = storage {
                    for (resource_type, amount) in &resources {
                        stor.add_resource(*resource_type, *amount);
                    }
                    return CommandResult::success();
                }
            }
        }
        
        CommandResult::failure("Entity not found or has no inventory")
    }
    
    fn handle_spawn_worker(&mut self, position: Position, _settlement_id: Option<u64>) -> CommandResult {
        let current_pop = self.snapshot().entities
            .iter()
            .filter(|e| matches!(e.entity_type, EntityType::Worker))
            .count();
        
        let pop_cap = self.get_population_cap();
        if current_pop >= pop_cap {
            return CommandResult::failure("Population cap reached");
        }
        
        self.spawn_worker(position);
        CommandResult::success()
    }
    
    fn handle_store_command(&mut self, _worker_id: EntityId, _building_id: EntityId) -> CommandResult {
        CommandResult::success()
    }
    
    fn handle_assign_worker(&mut self, _worker_id: EntityId, _building_id: EntityId) -> CommandResult {
        CommandResult::success()
    }
    
    fn handle_start_recipe(&mut self, building_id: EntityId, recipe_id: RecipeId) -> CommandResult {
        // Add recipe to registry if needed
        if let Some(recipe) = self.recipes.iter().find(|r| r.id == recipe_id).cloned() {
            if let Some(mut registry) = self.app.world_mut().get_resource_mut::<RecipeRegistry>() {
                registry.register(recipe);
            }
        }
        
        // Queue the recipe request
        if let Some(mut requests) = self.app.world_mut().get_resource_mut::<systems::RecipeRequests>() {
            requests.add(building_id, recipe_id);
        }
        
        CommandResult::success()
    }
}

impl Default for SimulationEngine {
    fn default() -> Self {
        Self::new()
    }
}


