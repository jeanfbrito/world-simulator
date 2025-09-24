# World Simulator API Reference

## Table of Contents

1. [Core Simulation API](#core-simulation-api)
2. [Entity Component System (ECS)](#entity-component-system-ecs)
3. [AI Systems](#ai-systems)
4. [Economic System](#economic-system)
5. [Networking and IPC](#networking-and-ipc)
6. [World Generation](#world-generation)
7. [Configuration System](#configuration-system)
8. [Testing Framework](#testing-framework)
9. [Examples and Templates](#examples-and-templates)

## Core Simulation API

### Simulation

The main simulation orchestrator that manages all systems and game state.

```rust
use world_sim_simple::simulation::Simulation;

// Create new simulation
let mut simulation = Simulation::new(config).await?;

// Run simulation ticks
simulation.tick().await?;

// Get current state
let state = simulation.get_state().await?;

// Save/restore state
let saved_state = simulation.save_state().await?;
simulation.restore_state(saved_state).await?;
```

#### Methods

- `new(config: SimulationConfig) -> Result<Simulation, SimulationError>` - Create new simulation
- `tick() -> Result<(), SimulationError>` - Execute one simulation tick
- `get_state() -> Result<SimulationState, SimulationError>` - Get current simulation state
- `save_state() -> Result<SavedState, SimulationError>` - Save simulation state
- `restore_state(state: SavedState) -> Result<(), SimulationError>` - Restore from saved state
- `startup() -> Result<(), SimulationError>` - Initialize simulation systems
- `shutdown() -> Result<(), SimulationError>` - Clean up simulation resources

### SimulationConfig

Configuration for simulation parameters.

```rust
let config = SimulationConfig {
    world_size: (100, 100),
    initial_entities: 50,
    resource_density: 0.3,
    tick_rate_ms: 100,
    max_concurrent_actions: 1000,
    enable_ai: true,
    enable_economy: true,
    enable_networking: false,
};
```

### SimulationState

Current state of the simulation.

```rust
struct SimulationState {
    pub tick: u64,
    pub entities: Vec<EntityState>,
    pub resources: Vec<ResourceState>,
    pub buildings: Vec<BuildingState>,
    pub world_state: WorldState,
}
```

## Entity Component System (ECS)

### Entity

Base entity type with unique identifier.

```rust
use world_sim_simple::ecs::Entity;

// Create new entity
let entity = Entity::new();

// Get entity ID
let id = entity.id();

// Check if entity is valid
let is_alive = entity.is_alive();
```

### Components

Data components that define entity properties.

```rust
use world_sim_simple::components::*;

// Position component
let position = Position { x: 10, y: 20 };

// Movement component
let movement = Movement {
    speed: 1.0,
    destination: Some((15, 25)),
    path: vec![(12, 22), (13, 23)],
};

// Inventory component
let inventory = Inventory {
    capacity: 10,
    items: vec![
        Item { name: "wood".to_string(), quantity: 5 },
        Item { name: "stone".to_string(), quantity: 3 },
    ],
};

// Add components to entity
entity.add_component(position);
entity.add_component(movement);
entity.add_component(inventory);
```

### Systems

Systems that process entities with specific components.

```rust
use world_sim_simple::systems::*;

// Movement system
let movement_system = MovementSystem::new();

// Resource gathering system
let gathering_system = ResourceGatheringSystem::new();

// Register systems with simulation
simulation.add_system(movement_system);
simulation.add_system(gathering_system);
```

## AI Systems

### GOAP (Goal-Oriented Action Planning)

```rust
use world_sim_simple::ai::goap::*;

// Define goals
let goal = Goal::new("Gather Resources", 1.0);

// Define actions
let actions = vec![
    Action::new("Find Tree")
        .with_cost(5.0)
        .with_precondition(|world| world.has_nearby_trees())
        .with_effect(|world| world.set_target_tree()),

    Action::new("Chop Wood")
        .with_cost(10.0)
        .with_precondition(|world| world.is_at_tree())
        .with_effect(|world| world.add_wood(1)),
];

// Create GOAP planner
let planner = GOAPPlanner::new(actions);

// Plan actions
let plan = planner.plan(&world_state, &goal)?;
```

### Utility AI

```rust
use world_sim_simple::ai::utility::*;

// Define behaviors
let behaviors = vec![
    Behavior::new("Gather")
        .with_weight(0.7)
        .with_condition(|world| world.needs_resources())
        .with_action(|world| world.gather_resources()),

    Behavior::new("Build")
        .with_weight(0.5)
        .with_condition(|world| world.has_enough_resources())
        .with_action(|world| world.build_structure()),
];

// Create utility AI
let utility_ai = UtilityAI::new(behaviors);

// Select action
let selected_behavior = utility_ai.select_action(&world_state);
selected_behavior.execute(&mut world_state);
```

### State Machine

```rust
use world_sim_simple::ai::state_machine::*;

// Define states
let idle_state = State::new("Idle")
    .with_on_enter(|entity| log!("Entity idle"))
    .with_on_update(|entity, world| {
        if world.has_nearby_resources() {
            entity.transition_to("Gathering");
        }
    });

let gathering_state = State::new("Gathering")
    .with_on_enter(|entity| log!("Start gathering"))
    .with_on_update(|entity, world| {
        if entity.inventory_full() {
            entity.transition_to("Returning");
        }
    });

// Create state machine
let mut state_machine = StateMachine::new(idle_state)
    .add_state(gathering_state);

// Update state machine
state_machine.update(entity, &world_state);
```

## Economic System

### Resource Management

```rust
use world_sim_simple::economy::resources::*;

// Define resource types
let wood = ResourceType::new("wood")
    .with_density(0.3)
    .with_max_per_tile(10)
    .with_regen_rate(0.01);

let stone = ResourceType::new("stone")
    .with_density(0.2)
    .with_max_per_tile(5)
    .with_regen_rate(0.005);

// Resource manager
let mut resource_manager = ResourceManager::new();
resource_manager.register_resource(wood);
resource_manager.register_resource(stone);

// Spawn resources in world
resource_manager.spawn_resources(&mut world, 100);
```

### Crafting System

```rust
use world_sim_simple::economy::crafting::*;

// Define recipes
let plank_recipe = Recipe::new("Wood Planks")
    .with_requirements(vec![
        ("wood".to_string(), 2),
    ])
    .with_results(vec![
        ("planks".to_string(), 4),
    ])
    .with_crafting_time(5.0);

let house_recipe = Recipe::new("House")
    .with_requirements(vec![
        ("planks".to_string(), 10),
        ("stone".to_string(), 5),
    ])
    .with_results(vec![
        ("house".to_string(), 1),
    ])
    .with_crafting_time(30.0);

// Crafting manager
let mut crafting_manager = CraftingManager::new();
crafting_manager.register_recipe(plank_recipe);
crafting_manager.register_recipe(house_recipe);

// Craft items
let result = crafting_manager.craft(
    &mut entity.inventory,
    "Wood Planks",
    2
)?;
```

### Market System

```rust
use world_sim_simple::economy::market::*;

// Create market
let market = Market::new();

// Set prices based on supply/demand
market.set_price("wood", 5.0);
market.set_price("stone", 8.0);

// Trading
entity.sell_to_market(&market, "wood", 5)?;
entity.buy_from_market(&market, "stone", 3)?;

// Update market prices
market.update_prices();
```

## Networking and IPC

### WebSocket Server

```rust
use world_sim_simple::networking::websocket::*;

// Start WebSocket server
let server = WebSocketServer::new(8080).await?;

// Handle connections
server.on_connection(|client| {
    println!("Client connected: {}", client.id());
});

// Handle messages
server.on_message(|client, message| {
    match message {
        Message::Text(text) => {
            // Handle text message
        }
        Message::Binary(data) => {
            // Handle binary data
        }
    }
});

// Broadcast to all clients
server.broadcast("Simulation update").await?;
```

### IPC Communication

```rust
use world_sim_simple::ipc::*;

// Create IPC message
let message = IPCMessage::EntityUpdate {
    entity_id: 123,
    position: (10, 20),
    state: "moving".to_string(),
};

// Send message
let ipc = IPCManager::new();
ipc.send_message(message).await?;

// Receive messages
while let Some(message) = ipc.receive_message().await? {
    match message {
        IPCMessage::EntityUpdate { entity_id, position, state } => {
            println!("Entity {} moved to {:?} (state: {})", entity_id, position, state);
        }
        IPCMessage::ResourceUpdate { resource_type, amount, location } => {
            println!("Resource {} changed by {} at {:?}", resource_type, amount, location);
        }
        IPCMessage::WorldUpdate { tick, entities } => {
            println!("World tick {}: {} entities", tick, entities);
        }
    }
}
```

## World Generation

### Terrain Generation

```rust
use world_sim_simple::world::terrain::*;

// Configure terrain generator
let config = TerrainConfig {
    width: 200,
    height: 200,
    seed: 42,
    octaves: 4,
    persistence: 0.5,
    scale: 50.0,
};

// Generate terrain
let terrain = TerrainGenerator::new(config)
    .with_height_map(NoiseType::Perlin)
    .with_moisture_map(NoiseType::Simplex)
    .with_temperature_map(NoiseType::Value)
    .generate()?;

// Get tile properties
let tile = terrain.get_tile(50, 50);
println!("Tile type: {:?}, elevation: {:.2}", tile.tile_type, tile.elevation);
```

### Biome System

```rust
use world_sim_simple::world::biome::*;

// Define biomes
let forest = Biome::new("Forest")
    .with_temperature_range(0.3..0.7)
    .with_moisture_range(0.5..0.9)
    .with_terrain_types(vec![TerrainType::Grass, TerrainType::Dirt])
    .with_resources(vec!["tree", "berry_bush"])
    .with_color(0x228B22);

let desert = Biome::new("Desert")
    .with_temperature_range(0.7..1.0)
    .with_moisture_range(0.0..0.3)
    .with_terrain_types(vec![TerrainType::Sand])
    .with_resources(vec!["cactus", "rock"])
    .with_color(0xF4A460);

// Generate biomes
let biome_generator = BiomeGenerator::new()
    .add_biome(forest)
    .add_biome(desert);

let biome_map = biome_generator.generate(&terrain)?;
```

### Resource Generation

```rust
use world_sim_simple::world::resources::*;

// Configure resource spawning
let config = ResourceConfig {
    tree_density: 0.1,
    stone_density: 0.05,
    iron_density: 0.02,
    cluster_size: 3,
    min_distance: 5,
};

// Generate resources
let resource_generator = ResourceGenerator::new(config);
let resources = resource_generator.generate(&terrain, &biome_map)?;

// Spawn resources in world
for resource in resources {
    world.spawn_resource_at(resource.location, resource.resource_type, resource.amount);
}
```

## Configuration System

### Lua Configuration

```lua
-- config.lua
config = {
    world = {
        width = 100,
        height = 100,
        seed = 42,
        tile_size = 32,
    },

    entities = {
        peasant = {
            count = 20,
            health = 100,
            movement_speed = 1.0,
            inventory_size = 10,
            gathering_rate = 0.1,
        },
    },

    resources = {
        wood = {
            density = 0.3,
            respawn_time = 30.0,
            max_per_tile = 5,
        },
        stone = {
            density = 0.2,
            respawn_time = 60.0,
            max_per_tile = 3,
        },
    },

    systems = {
        movement = {
            enabled = true,
            update_interval = 0.1,
        },
        gathering = {
            enabled = true,
            update_interval = 0.2,
        },
        ai = {
            enabled = true,
            update_interval = 0.5,
        },
    },
}

return config
```

### Rust Configuration Loading

```rust
use world_sim_simple::config::ConfigManager;

// Load configuration
let config_manager = ConfigManager::new();
let config = config_manager.load_from_file("config.lua")?;

// Access configuration values
let world_width = config.get::<i32>("world.width")?;
let peasant_count = config.get::<i32>("entities.peasant.count")?;
let wood_density = config.get::<f64>("resources.wood.density")?;

// Update configuration at runtime
config.set("world.width", 150)?;
config.set("entities.peasant.count", 30)?;

// Save configuration
config_manager.save_to_file("modified_config.lua")?;
```

## Testing Framework

### Integration Tests

```rust
use world_sim_simple::testing::*;

// Create test context
let mut context = TestContext::new().await;

// Initialize test simulation
let config = TestConfig::default()
    .with_world_size(50, 50)
    .with_peasant_count(10);

let simulation = context.initialize_simulation(&config).await?;

// Run test scenario
let result = context.run_test_scenario(simulation, |sim| async move {
    // Test setup
    for _ in 0..100 {
        sim.tick().await?;

        // Test assertions
        let state = sim.get_state().await?;
        assert!(state.entities.len() > 0, "Entities should exist");
    }

    Ok(())
}).await?;

// Validate test results
assert!(result.is_ok(), "Test scenario should pass");
```

### Performance Tests

```rust
use world_sim_simple::testing::performance::*;

// Configure performance test
let config = PerformanceTestConfig {
    duration: Duration::from_secs(30),
    entity_count: 100,
    world_size: (100, 100),
    metrics_to_collect: vec![
        MetricType::TickTime,
        MetricType::MemoryUsage,
        MetricType::EntityCount,
    ],
};

// Run performance test
let test = PerformanceTest::new(config);
let results = test.run().await?;

// Analyze results
assert!(results.avg_tick_time < Duration::from_millis(50), "Tick time too slow");
assert!(results.max_memory_usage < 1024 * 1024 * 1024, "Memory usage too high");
```

### Benchmarks

```rust
#[bench]
fn benchmark_entity_spawning(b: &mut test::Bencher) {
    let mut context = TestContext::new().await;
    let simulation = context.initialize_simulation(&TestConfig::default()).await.unwrap();

    b.iter(|| {
        // Spawn entity
        let entity = simulation.spawn_entity().unwrap();

        // Ensure entity is properly cleaned up
        simulation.despawn_entity(entity.id());
    });
}

#[bench]
fn benchmark_pathfinding(b: &mut test::Bencher) {
    let mut context = TestContext::new().await;
    let simulation = context.initialize_simulation(&TestConfig::default()).await.unwrap();

    b.iter(|| {
        // Find path
        let path = simulation.find_path((0, 0), (50, 50)).unwrap();

        // Verify path
        assert!(!path.is_empty());
    });
}
```

## Examples and Templates

### Basic Simulation

```rust
use world_sim_simple::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create simulation
    let config = SimulationConfig::default()
        .with_world_size(100, 100)
        .with_entity_count(20);

    let mut simulation = Simulation::new(config).await?;

    // Run simulation
    for tick in 0..1000 {
        simulation.tick().await?;

        if tick % 100 == 0 {
            let state = simulation.get_state().await?;
            println!("Tick {}: {} entities", tick, state.entities.len());
        }
    }

    Ok(())
}
```

### Custom Entity Type

```rust
use world_sim_simple::prelude::*;

// Define custom components
#[derive(Component)]
struct Blacksmith {
    skill_level: u32,
    crafting_speed: f32,
}

#[derive(Component)]
struct Workshop {
    owner: EntityId,
    tools: Vec<String>,
    materials: HashMap<String, u32>,
}

// Define custom system
#[derive(System)]
struct CraftingSystem {
    query: Query<With<Blacksmith>>,
}

impl System for CraftingSystem {
    fn update(&mut self, world: &mut World) {
        for (entity, blacksmith) in self.query.iter(world) {
            if let Some(workshop) = world.get_component::<Workshop>(entity) {
                // Process crafting
                blacksmith.craft_items(&mut workshop.materials);
            }
        }
    }
}

// Register custom components and systems
world.register_component::<Blacksmith>();
world.register_component::<Workshop>();
world.add_system(CraftingSystem::new());
```

### Custom World Generation

```rust
use world_sim_simple::world::*;

struct CustomWorldGenerator {
    config: WorldConfig,
}

impl WorldGenerator for CustomWorldGenerator {
    fn generate(&self, rng: &mut impl Rng) -> Result<World, GenerationError> {
        let mut world = World::new(self.config.size);

        // Generate custom terrain
        for x in 0..self.config.size.width {
            for y in 0..self.config.size.height {
                let height = self.generate_height(x, y, rng);
                let biome = self.determine_biome(height, x, y);

                world.set_tile(x, y, Tile::new(biome, height));
            }
        }

        // Add custom features
        self.add_rivers(&mut world, rng)?;
        self.add_mountains(&mut world, rng)?;
        self.add_forests(&mut world, rng)?;

        Ok(world)
    }
}
```

### Modding System

```rust
use world_sim_simple::modding::*;

// Define mod
#[derive(Mod)]
struct MagicMod {
    name: String,
    version: String,
    components: Vec<ComponentRegistration>,
    systems: Vec<SystemRegistration>,
}

impl Mod for MagicMod {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn load(&mut self, loader: &mut ModLoader) -> Result<(), ModError> {
        // Register magic components
        loader.register_component("Mana", ComponentInfo::new::<ManaComponent>())?;
        loader.register_component("Spell", ComponentInfo::new::<SpellComponent>())?;

        // Register magic systems
        loader.register_system("MagicSystem", Box::new(MagicSystem::new()))?;

        Ok(())
    }

    fn unload(&mut self, loader: &mut ModLoader) -> Result<(), ModError> {
        // Clean up mod resources
        Ok(())
    }
}

// Load and enable mods
let mut mod_manager = ModManager::new();
mod_manager.load_mod(MagicMod::new("magic", "1.0.0"))?;
mod_manager.enable_mod("magic")?;
```

This API reference covers the core functionality of the world simulator. For more detailed examples and advanced usage patterns, please refer to the examples in the `examples/` directory and the integration tests in `tests/`.