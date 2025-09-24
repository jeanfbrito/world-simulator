# World Simulator Architecture

## Table of Contents

1. [Overview](#overview)
2. [Core Architecture](#core-architecture)
3. [Entity Component System (ECS)](#entity-component-system-ecs)
4. [Simulation Engine](#simulation-engine)
5. [AI Systems](#ai-systems)
6. [Economic System](#economic-system)
7. [World Generation](#world-generation)
8. [Networking Layer](#networking-layer)
9. [Configuration System](#configuration-system)
10. [Performance Architecture](#performance-architecture)
11. [Data Flow](#data-flow)
12. [Extensibility](#extensibility)

## Overview

The World Simulator is built using a modular, component-based architecture that emphasizes performance, scalability, and extensibility. The system uses Entity Component System (ECS) for flexible entity management and a message-passing architecture for inter-system communication.

### Key Design Principles

- **Component-Based**: All game objects are entities with components
- **Data-Oriented**: Optimize for data locality and cache efficiency
- **Parallel Processing**: Systems run in parallel where possible
- **Event-Driven**: Systems communicate through events and messages
- **Modular**: Each system is independent and pluggable
- **Extensible**: Easy to add new systems, components, and behaviors

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Application Layer                        │
├─────────────────────────────────────────────────────────────┤
│  Examples  │  Tests  │  Web Viewer  │  CLI Tools  │  Mods    │
├─────────────────────────────────────────────────────────────┤
│                     Simulation API                           │
├─────────────────────────────────────────────────────────────┤
│  Simulation Engine  │  AI Systems  │  Economic System       │
│  World Generation   │  Networking  │  Configuration System   │
├─────────────────────────────────────────────────────────────┤
│                   Entity Component System                    │
├─────────────────────────────────────────────────────────────┤
│                     Core Library                             │
├─────────────────────────────────────────────────────────────┤
│  ECS Framework  │  Math Library  │  Utilities  │  Logging    │
└─────────────────────────────────────────────────────────────┘
```

## Core Architecture

### Layered Architecture

The system is organized in layers with clear separation of concerns:

1. **Core Layer**: Fundamental utilities and ECS framework
2. **Simulation Layer**: Game logic and systems
3. **Application Layer**: Examples, tools, and user interfaces

### Module Dependencies

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   world_sim_    │    │   web-viewer    │    │    examples     │
│     simple      │◄───│                 │◄───│                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │   world_sim     │
                    │     (lib)       │
                    └─────────────────┘
                           │
            ┌──────────────┴──────────────┐
            │                             │
    ┌─────────────┐               ┌─────────────┐
    │   ecs       │               │  simulation  │
    └─────────────┘               └─────────────┘
```

## Entity Component System (ECS)

### Core ECS Architecture

The ECS architecture provides a flexible, data-oriented approach to entity management:

```rust
// Component storage optimized for cache efficiency
pub struct ComponentStorage<T> {
    entities: Vec<EntityId>,
    data: Vec<T>,
    entity_index: HashMap<EntityId, usize>,
}

// System execution with parallel processing
pub struct SystemExecutor {
    systems: Vec<Box<dyn System>>,
    schedule: ExecutionSchedule,
    thread_pool: ThreadPool,
}

// Query system for efficient component access
pub struct Query<T> {
    marker: PhantomData<T>,
    archetype_ids: Vec<ArchetypeId>,
}
```

### Archetype-Based ECS

The system uses an archetype-based ECS for optimal performance:

- **Archetypes**: Groups of entities with the same component types
- **Chunked Storage**: Components stored in contiguous memory blocks
- **Sparse Sets**: Fast entity lookup and iteration
- **Parallel Queries**: Systems can process entities in parallel

```rust
// Archetype definition
pub struct Archetype {
    id: ArchetypeId,
    components: Vec<ComponentTypeId>,
    entities: Vec<EntityId>,
    chunks: Vec<Chunk>,
}

// Chunk for cache-friendly storage
pub struct Chunk {
    capacity: usize,
    count: usize,
    data: Box<[u8]>,
}
```

### Component Lifecycle

1. **Registration**: Component types are registered with the ECS
2. **Creation**: Components are created and assigned to entities
3. **Update**: Systems modify component data each frame
4. **Query**: Systems access components through queries
5. **Destruction**: Components are removed when entities are destroyed

## Simulation Engine

### Core Simulation Loop

The simulation engine runs a deterministic tick-based loop:

```rust
pub struct SimulationEngine {
    world: World,
    systems: SystemExecutor,
    scheduler: TaskScheduler,
    delta_time: f64,
    tick_number: u64,
}

impl SimulationEngine {
    pub fn tick(&mut self) -> Result<(), SimulationError> {
        // Fixed timestep with interpolation
        let fixed_delta = Duration::from_secs_f64(1.0 / 60.0);

        // Update systems
        self.systems.execute(&mut self.world, fixed_delta);

        // Process events
        self.process_events();

        // Update tick counter
        self.tick_number += 1;

        Ok(())
    }
}
```

### System Scheduling

Systems are scheduled based on dependencies and resource access:

```rust
pub struct SystemSchedule {
    systems: Vec<SystemDescriptor>,
    dependencies: DependencyGraph,
    execution_order: Vec<SystemId>,
}

pub enum SystemPhase {
    Input,           // Handle user input
    Update,          // Update game logic
    Physics,         // Physics simulation
    Render,          // Rendering preparation
    PostProcess,     // Post-processing
}
```

### Deterministic Simulation

The simulation is deterministic across runs:

- **Fixed Timestep**: Consistent physics and AI updates
- **Seed-Based RNG**: Random number generation with fixed seeds
- **Order-Independent**: System execution order doesn't affect results
- **Snapshot Support**: Save and restore exact simulation state

## AI Systems

### AI Architecture Overview

The AI system uses a hybrid approach combining multiple techniques:

```
┌─────────────────────────────────────────────────────────────┐
│                     AI System Manager                        │
├─────────────────────────────────────────────────────────────┤
│  GOAP System  │  Utility AI  │  State Machine  │  Behavior Tree │
├─────────────────────────────────────────────────────────────┤
│  Pathfinding  │  Perception  │  Memory System   │  Learning      │
├─────────────────────────────────────────────────────────────┤
│                World State and Knowledge Base               │
└─────────────────────────────────────────────────────────────┘
```

### Goal-Oriented Action Planning (GOAP)

GOAP provides intelligent goal-driven behavior:

```rust
pub struct GOAPSystem {
    agents: Vec<GOAPAgent>,
    world_state: WorldState,
    action_registry: ActionRegistry,
}

pub struct GOAPAgent {
    entity: EntityId,
    goals: Vec<Goal>,
    current_plan: Option<ActionPlan>,
    blackboard: HashMap<String, Value>,
}

impl GOAPAgent {
    pub fn plan(&self, world_state: &WorldState) -> Option<ActionPlan> {
        // A* search for optimal action sequence
        let planner = AStarPlanner::new(self.goals.clone());
        planner.find_plan(world_state, &self.action_registry)
    }
}
```

### Utility AI

Utility AI provides flexible, context-aware decision making:

```rust
pub struct UtilitySystem {
    behaviors: Vec<Behavior>,
    curves: HashMap<String, UtilityCurve>,
}

pub struct Behavior {
    name: String,
    considerations: Vec<Consideration>,
    action: Box<dyn Action>,
}

impl Behavior {
    pub fn evaluate(&self, context: &Context) -> f64 {
        let score = self.considerations.iter()
            .map(|c| c.evaluate(context))
            .product();
        score * self.action.get_multiplier(context)
    }
}
```

### Pathfinding and Navigation

The pathfinding system supports multiple algorithms:

```rust
pub enum PathfindingAlgorithm {
    AStar,
    Dijkstra,
    FlowField,
    JumpPointSearch,
}

pub struct PathfindingSystem {
    grid: NavigationGrid,
    agents: HashMap<EntityId, PathfindingAgent>,
    algorithm: PathfindingAlgorithm,
}
```

## Economic System

### Economic Architecture

The economic system simulates resource flow and market dynamics:

```
┌─────────────────────────────────────────────────────────────┐
│                     Economic Manager                         │
├─────────────────────────────────────────────────────────────┤
│  Resource    │  Crafting     │  Market       │  Production  │
│  Manager     │  System       │  System       │  System     │
├─────────────────────────────────────────────────────────────┤
│               Resource Flow and Supply Chains                │
├─────────────────────────────────────────────────────────────┤
│              Economic Agents and Behaviors                   │
└─────────────────────────────────────────────────────────────┘
```

### Resource Management

Resources are managed through a sophisticated system:

```rust
pub struct ResourceManager {
    resources: HashMap<ResourceType, ResourceData>,
    deposits: SpatialIndex<ResourceDeposit>,
    flow_network: FlowNetwork,
}

pub struct ResourceData {
    total_amount: u64,
    available_amount: u64,
    production_rate: f64,
    consumption_rate: f64,
    price: f64,
}
```

### Market Simulation

The market system simulates supply and demand dynamics:

```rust
pub struct MarketSystem {
    commodities: HashMap<String, Commodity>,
    agents: Vec<EconomicAgent>,
    price_history: HashMap<String, Vec<f64>>,
}

impl MarketSystem {
    pub fn update_prices(&mut self) {
        for (commodity_name, commodity) in &mut self.commodities {
            let supply = self.calculate_supply(commodity_name);
            let demand = self.calculate_demand(commodity_name);

            // Supply and demand pricing
            let new_price = self.calculate_equilibrium_price(supply, demand);
            commodity.current_price = new_price;

            // Record price history
            self.price_history
                .entry(commodity_name.clone())
                .or_insert_with(Vec::new)
                .push(new_price);
        }
    }
}
```

### Crafting and Production

The crafting system supports complex production chains:

```rust
pub struct CraftingSystem {
    recipes: HashMap<String, Recipe>,
    workshops: HashMap<EntityId, Workshop>,
    production_queue: Vec<ProductionOrder>,
}

pub struct Recipe {
    name: String,
    inputs: Vec<RecipeInput>,
    outputs: Vec<RecipeOutput>,
    crafting_time: Duration,
    required_tools: Vec<String>,
}
```

## World Generation

### World Generation Pipeline

The world is generated through a series of stages:

```
┌─────────────────────────────────────────────────────────────┐
│                  World Generator                            │
├─────────────────────────────────────────────────────────────┤
│  Height Map  │  Biome Map   │  Resource Map   │  Feature Map │
├─────────────────────────────────────────────────────────────┤
│  Terrain      │  Climate     │  Ecosystems     │  Structures  │
├─────────────────────────────────────────────────────────────┤
│                Post-processing and Validation                │
└─────────────────────────────────────────────────────────────┘
```

### Procedural Generation

The system uses multiple noise functions for realistic terrain:

```rust
pub struct TerrainGenerator {
    height_noise: NoiseGenerator,
    moisture_noise: NoiseGenerator,
    temperature_noise: NoiseGenerator,
    features: Vec<FeatureGenerator>,
}

impl TerrainGenerator {
    pub fn generate(&self, size: (u32, u32), seed: u64) -> Terrain {
        let mut terrain = Terrain::new(size);

        // Generate height map using multiple octaves
        for x in 0..size.0 {
            for y in 0..size.1 {
                let height = self.generate_height(x, y);
                let moisture = self.generate_moisture(x, y);
                let temperature = self.generate_temperature(x, y);

                terrain.set_tile(x, y, Tile::new(height, moisture, temperature));
            }
        }

        terrain
    }
}
```

### Biome System

Biomes are generated based on climate parameters:

```rust
pub struct BiomeSystem {
    biomes: Vec<Biome>,
    biome_map: Grid<BiomeId>,
    transition_zones: Vec<TransitionZone>,
}

pub struct Biome {
    name: String,
    climate_range: ClimateRange,
    resources: Vec<ResourceType>,
    features: Vec<Feature>,
    color: Color,
}
```

## Networking Layer

### Network Architecture

The networking layer supports real-time multiplayer:

```
┌─────────────────────────────────────────────────────────────┐
│                    Network Manager                           │
├─────────────────────────────────────────────────────────────┤
│  WebSocket    │  TCP          │  UDP           │  IPC         │
│  Server       │  Connections  │  Broadcasts    │  Messages    │
├─────────────────────────────────────────────────────────────┤
│  Message      │  Session      │  State         │  Event       │
│  Serialization│  Management   │  Synchronization│  Handling    │
├─────────────────────────────────────────────────────────────┤
│                Protocol and Security                         │
└─────────────────────────────────────────────────────────────┘
```

### Message System

Messages are serialized and routed efficiently:

```rust
pub struct NetworkMessage {
    id: MessageId,
    timestamp: SystemTime,
    channel: Channel,
    payload: MessagePayload,
}

pub enum MessagePayload {
    EntityUpdate(EntityUpdate),
    WorldState(WorldState),
    PlayerAction(PlayerAction),
    SystemEvent(SystemEvent),
}
```

### State Synchronization

The system uses delta compression for efficient state sync:

```rust
pub struct StateSynchronizer {
    last_state: WorldState,
    delta_compression: bool,
    update_rate: f64,
    clients: HashMap<ClientId, ClientConnection>,
}

impl StateSynchronizer {
    pub fn generate_delta(&self, current_state: &WorldState) -> Vec<StateDelta> {
        // Compare with last state and generate deltas
        let mut deltas = Vec::new();

        // Entity deltas
        for entity in &current_state.entities {
            if let Some(last_entity) = self.last_state.get_entity(entity.id) {
                let delta = self.calculate_entity_delta(last_entity, entity);
                if !delta.is_empty() {
                    deltas.push(StateDelta::Entity(delta));
                }
            }
        }

        deltas
    }
}
```

## Configuration System

### Configuration Architecture

The configuration system uses Lua for flexibility:

```
┌─────────────────────────────────────────────────────────────┐
│                    Config Manager                            │
├─────────────────────────────────────────────────────────────┤
│  Lua Parser   │  Schema Validator  │  Hot Reload     │  Caching  │
├─────────────────────────────────────────────────────────────┤
│  Type System  │  Default Values    │  User Overrides │  Mods    │
├─────────────────────────────────────────────────────────────┤
│                Configuration Files and Sources                │
└─────────────────────────────────────────────────────────────┘
```

### Lua Configuration

Configuration files use Lua for expressiveness:

```lua
-- world.lua
world = {
    size = { width = 200, height = 200 },
    seed = 42,
    biomes = {
        forest = {
            density = 0.3,
            resources = { "tree", "berry_bush" },
            climate = { temperature = 0.5, moisture = 0.7 }
        },
        desert = {
            density = 0.1,
            resources = { "cactus", "stone" },
            climate = { temperature = 0.8, moisture = 0.2 }
        }
    },
    entities = {
        peasant = {
            count = 50,
            health = 100,
            speed = 1.0,
            inventory_size = 10
        }
    }
}
```

### Hot Reloading

The system supports configuration changes at runtime:

```rust
pub struct ConfigManager {
    configs: HashMap<String, Config>,
    watchers: Vec<FileSystemWatcher>,
    schema_validator: SchemaValidator,
}

impl ConfigManager {
    pub fn watch_config(&mut self, path: &Path) -> Result<(), ConfigError> {
        let watcher = FileSystemWatcher::new(path)?;
        watcher.on_change(move |path| {
            if let Ok(new_config) = self.load_config(path) {
                self.update_config(new_config);
                self.notify_listeners();
            }
        });
        self.watchers.push(watcher);
        Ok(())
    }
}
```

## Performance Architecture

### Performance Optimization Strategies

The system uses multiple optimization techniques:

```
┌─────────────────────────────────────────────────────────────┤
│                    Performance Layer                       │
├─────────────────────────────────────────────────────────────┤
│  Cache        │  Memory      │  Threading    │  SIMD        │
│  Optimization │  Management  │  Model         │  Optimization │
├─────────────────────────────────────────────────────────────┤
│  Data-Oriented │  Batch       │  Load          │  Monitoring  │
│  Design       │  Processing  │  Balancing     │  and Profiling │
├─────────────────────────────────────────────────────────────┤
│                Algorithm and Data Structure Choices         │
└─────────────────────────────────────────────────────────────┘
```

### Memory Management

The system uses efficient memory allocation patterns:

```rust
pub struct MemoryPool<T> {
    free_list: Vec<*mut T>,
    capacity: usize,
    chunk_size: usize,
}

pub struct ArenaAllocator {
    chunks: Vec<MemoryChunk>,
    current_chunk: usize,
    chunk_size: usize,
}

// Zero-copy serialization
pub struct ZeroCopySerializer {
    buffer: Vec<u8>,
    layout: MemoryLayout,
}
```

### Parallel Processing

The system maximizes parallel execution:

```rust
pub struct ParallelExecutor {
    thread_pool: ThreadPool,
    task_queue: TaskQueue,
    scheduler: WorkStealingScheduler,
}

impl ParallelExecutor {
    pub fn execute_parallel<T, F>(&self, data: &mut [T], operation: F)
    where
        F: Fn(&mut T) + Send + Sync + 'static,
        T: Send + 'static,
    {
        let chunk_size = (data.len() + self.thread_pool.len() - 1) / self.thread_pool.len();

        data.par_chunks_mut(chunk_size)
            .for_each(|chunk| {
                chunk.iter_mut().for_each(&operation);
            });
    }
}
```

## Data Flow

### Event System

Events drive communication between systems:

```rust
pub struct EventBus {
    subscribers: HashMap<EventType, Vec<Box<dyn EventHandler>>>,
    event_queue: VecDeque<Event>,
}

pub enum Event {
    EntityCreated(EntityCreatedEvent),
    EntityDestroyed(EntityDestroyedEvent),
    ComponentChanged(ComponentChangedEvent),
    ResourceDepleted(ResourceDepletedEvent),
}
```

### State Management

State is managed through a sophisticated system:

```rust
pub struct StateManager {
    current_state: WorldState,
    state_history: VecDeque<WorldState>,
    checkpoints: HashMap<u64, SavedState>,
}

impl StateManager {
    pub fn save_state(&mut self) -> StateId {
        let state_id = self.generate_state_id();
        let compressed_state = self.compress_state(&self.current_state);
        self.checkpoints.insert(state_id, compressed_state);
        state_id
    }

    pub fn load_state(&mut self, state_id: StateId) -> Result<(), StateError> {
        if let Some(compressed_state) = self.checkpoints.get(&state_id) {
            self.current_state = self.decompress_state(compressed_state)?;
            Ok(())
        } else {
            Err(StateError::NotFound(state_id))
        }
    }
}
```

## Extensibility

### Plugin System

The system supports plugins for extension:

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError>;
    fn update(&mut self, context: &PluginContext) -> Result<(), PluginError>;
    fn shutdown(&mut self) -> Result<(), PluginError>;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    plugin_api: PluginApi,
}
```

### Modding Support

The system supports runtime modding:

```rust
pub struct ModManager {
    mods: HashMap<String, Mod>,
    load_order: Vec<String>,
    mod_api: ModApi,
}

pub struct Mod {
    name: String,
    version: String,
    components: Vec<ComponentRegistration>,
    systems: Vec<SystemRegistration>,
    resources: Vec<ResourceDefinition>,
}
```

### Scripting Integration

Lua scripting enables dynamic behavior:

```rust
pub struct ScriptEngine {
    lua: Lua,
    script_cache: HashMap<String, LuaChunk>,
    sandbox: ScriptSandbox,
}

impl ScriptEngine {
    pub fn execute_script(&mut self, script: &str, context: &ScriptContext) -> Result<ScriptResult, ScriptError> {
        let chunk = self.compile_script(script)?;
        let result = chunk.call(context.to_lua_table())?;
        Ok(ScriptResult::from_lua(result))
    }
}
```

This architecture provides a solid foundation for a scalable, performant, and extensible world simulation system. The modular design allows for easy addition of new features and systems while maintaining clean separation of concerns.