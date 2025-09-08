# Headless Simulation Engine Architecture

## Core Philosophy

The World Simulator is a **pure simulation engine** with zero rendering dependencies. Any frontend (2D, 3D, text, web) can connect and visualize the simulation through a clean event-driven API.

```
┌─────────────────────────────────────────────────────────────┐
│                    HEADLESS ENGINE CORE                      │
│                  (No UI, No Rendering, Pure Logic)           │
│                                                               │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐       │
│  │   ECS    │ │ Resources│ │Population│ │  Recipes │       │
│  │  System  │ │  System  │ │  System  │ │  System  │       │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘       │
│                                                               │
│                    ▼ Events & State API ▼                    │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
   ┌─────────┐          ┌─────────┐          ┌─────────┐
   │ Bevy 2D │          │Unity 3D │          │Terminal │
   │Visualizer│         │ Client  │          │  ASCII  │
   └─────────┘          └─────────┘          └─────────┘
        │                     │                     │
   ┌─────────┐          ┌─────────┐          ┌─────────┐
   │ Godot   │          │ Unreal  │          │   Web   │
   │ Client  │          │ Client  │          │   API   │
   └─────────┘          └─────────┘          └─────────┘
```

## Project Structure

```
world-simulator/
├── Cargo.toml                          # Workspace configuration
│
├── world_sim_core/                     # HEADLESS ENGINE (No Bevy rendering!)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                      # Core plugin exports
│       ├── components/                 # ECS Components
│       │   ├── mod.rs
│       │   ├── position.rs            # Spatial components
│       │   ├── resources.rs           # Resource nodes, items
│       │   ├── workers.rs             # Population units
│       │   └── buildings.rs           # Structures
│       ├── systems/                    # Game Logic Systems
│       │   ├── mod.rs
│       │   ├── harvest_system.rs
│       │   ├── movement_system.rs
│       │   ├── recipe_system.rs
│       │   ├── population_system.rs
│       │   └── building_system.rs
│       ├── resources/                  # Shared Resources
│       │   ├── world_state.rs
│       │   ├── recipes.rs
│       │   └── game_config.rs
│       ├── events/                     # Event Definitions
│       │   ├── mod.rs
│       │   ├── entity_events.rs       # Spawn, move, destroy
│       │   ├── game_events.rs         # Harvest, build, craft
│       │   └── state_events.rs        # Resource changes
│       └── api/                        # Public API
│           ├── mod.rs
│           ├── commands.rs            # Input commands
│           ├── queries.rs             # State queries
│           └── observer.rs            # Observer pattern
│
├── world_sim_interface/                # INTERFACE DEFINITIONS
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── events.rs                  # Shared event types
│       ├── commands.rs                # Command protocol
│       └── state.rs                   # State snapshots
│
├── world_sim_bevy_viz/                 # OPTIONAL: Bevy 2D Renderer
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── sprite_sync.rs
│       ├── camera.rs
│       └── ui_overlay.rs
│
├── world_sim_terminal/                 # OPTIONAL: Terminal Display
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── ascii_renderer.rs
│       └── terminal_commands.rs
│
├── world_sim_network/                  # OPTIONAL: Network Server
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── server.rs                  # Headless server
│       ├── protocol.rs                # Binary protocol
│       └── replication.rs             # State sync
│
├── world_sim_unity_bridge/             # OPTIONAL: Unity Integration
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── ffi.rs                     # C FFI for Unity
│       └── serialization.rs
│
└── examples/
    ├── headless.rs                    # Pure simulation
    ├── headless_server.rs             # Network server
    ├── with_bevy_graphics.rs          # Bevy visualization
    ├── terminal_game.rs               # ASCII version
    └── benchmark.rs                   # Performance testing
```

## Core Engine API

### Events (Output from Engine)

```rust
// world_sim_interface/src/events.rs
use serde::{Serialize, Deserialize};

/// All events are serializable for network/FFI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineEvent {
    // Entity lifecycle
    EntitySpawned {
        id: EntityId,
        entity_type: EntityType,
        position: Position,
        components: ComponentData,
    },
    EntityMoved {
        id: EntityId,
        from: Position,
        to: Position,
        path: Vec<Position>,
    },
    EntityDestroyed {
        id: EntityId,
        reason: DestroyReason,
    },
    
    // Game events
    ResourceHarvested {
        worker_id: EntityId,
        resource_id: EntityId,
        resource_type: ResourceType,
        amount: u32,
    },
    BuildingConstructed {
        building_id: EntityId,
        building_type: BuildingType,
        position: Position,
    },
    RecipeCompleted {
        recipe_id: RecipeId,
        building_id: EntityId,
        outputs: Vec<(ResourceType, u32)>,
    },
    
    // State changes
    ResourcesChanged {
        settlement_id: SettlementId,
        changes: HashMap<ResourceType, i32>,
    },
    PopulationChanged {
        settlement_id: SettlementId,
        delta: i32,
        new_total: u32,
    },
}
```

### Commands (Input to Engine)

```rust
// world_sim_interface/src/commands.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineCommand {
    // Direct orders
    HarvestResource {
        worker_id: EntityId,
        target_id: EntityId,
    },
    ConstructBuilding {
        building_type: BuildingType,
        position: Position,
        workers: Vec<EntityId>,
    },
    AssignWorker {
        worker_id: EntityId,
        task: TaskType,
    },
    
    // Queries (synchronous responses)
    QueryArea {
        min: Position,
        max: Position,
    },
    QueryEntity {
        id: EntityId,
    },
    QueryResources {
        settlement_id: SettlementId,
    },
}
```

### State Snapshots

```rust
// world_sim_interface/src/state.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub tick: u64,
    pub entities: Vec<EntitySnapshot>,
    pub settlements: Vec<SettlementSnapshot>,
    pub global_state: GlobalState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub position: Position,
    pub components: HashMap<String, Value>, // JSON-like for flexibility
}
```

## Integration Examples

### 1. Headless Server

```rust
// examples/headless_server.rs
use world_sim_core::SimulationEngine;
use world_sim_network::NetworkServer;

fn main() {
    let mut engine = SimulationEngine::new();
    let server = NetworkServer::new("0.0.0.0:7777");
    
    // Connect server to engine
    engine.add_observer(server.observer());
    
    // Run simulation at 20 ticks/second
    loop {
        engine.tick();
        server.broadcast_events();
        std::thread::sleep(Duration::from_millis(50));
    }
}
```

### 2. Unity Integration (C# Side)

```csharp
// Unity C# code
using System.Runtime.InteropServices;

public class WorldSimulator : MonoBehaviour {
    [DllImport("world_sim_unity_bridge")]
    private static extern IntPtr create_engine();
    
    [DllImport("world_sim_unity_bridge")]
    private static extern void tick_engine(IntPtr engine);
    
    [DllImport("world_sim_unity_bridge")]
    private static extern IntPtr get_events(IntPtr engine);
    
    private IntPtr engine;
    
    void Start() {
        engine = create_engine();
    }
    
    void Update() {
        tick_engine(engine);
        string events = Marshal.PtrToStringAnsi(get_events(engine));
        ProcessEvents(JsonUtility.FromJson<EngineEvent[]>(events));
    }
}
```

### 3. Terminal ASCII

```rust
// world_sim_terminal/src/lib.rs
use world_sim_core::{SimulationEngine, EngineEvent};

pub struct TerminalRenderer {
    grid: [[char; 80]; 40],
}

impl TerminalRenderer {
    pub fn handle_event(&mut self, event: &EngineEvent) {
        match event {
            EngineEvent::EntitySpawned { position, entity_type, .. } => {
                self.grid[position.y][position.x] = match entity_type {
                    EntityType::Tree => '🌳',
                    EntityType::Worker => '@',
                    EntityType::House => '⌂',
                    _ => '?',
                };
            }
            _ => {}
        }
    }
    
    pub fn render(&self) {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        for row in &self.grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
}
```

### 4. Godot GDScript Integration

```gdscript
# Godot integration via GDNative
extends Node

var world_sim = preload("res://world_sim.gdns")
var engine

func _ready():
    engine = world_sim.new()
    engine.connect("event_received", self, "_on_engine_event")

func _process(delta):
    engine.tick(delta)

func _on_engine_event(event):
    match event.type:
        "EntitySpawned":
            var sprite = Sprite.new()
            sprite.position = Vector2(event.position.x * 32, event.position.y * 32)
            sprite.texture = load("res://sprites/" + event.entity_type + ".png")
            add_child(sprite)
```

### 5. Web API

```rust
// world_sim_web/src/main.rs
use axum::{Json, extract::State};
use world_sim_core::SimulationEngine;

async fn get_world_state(
    State(engine): State<Arc<Mutex<SimulationEngine>>>,
) -> Json<WorldSnapshot> {
    let engine = engine.lock().unwrap();
    Json(engine.snapshot())
}

async fn send_command(
    State(engine): State<Arc<Mutex<SimulationEngine>>>,
    Json(command): Json<EngineCommand>,
) -> Json<CommandResult> {
    let mut engine = engine.lock().unwrap();
    Json(engine.execute_command(command))
}
```

## Plugin Development Guide

### Creating a New Visualizer

```rust
// your_visualizer/src/lib.rs
use world_sim_interface::{EngineEvent, EngineCommand, WorldSnapshot};

pub trait Visualizer {
    /// Called when engine produces events
    fn handle_events(&mut self, events: &[EngineEvent]);
    
    /// Called to get commands from user
    fn poll_commands(&mut self) -> Vec<EngineCommand>;
    
    /// Optional: Handle full state snapshot
    fn sync_snapshot(&mut self, snapshot: &WorldSnapshot);
}

// Implement for your renderer
impl Visualizer for MyCustomRenderer {
    fn handle_events(&mut self, events: &[EngineEvent]) {
        for event in events {
            // Update your visual representation
        }
    }
    
    fn poll_commands(&mut self) -> Vec<EngineCommand> {
        // Return user inputs as commands
        vec![]
    }
}
```

## Performance Considerations

### Zero-Copy Events
```rust
// Use Arc for large data
pub struct EngineEvent {
    pub data: Arc<EventData>, // Shared between observers
}
```

### Batch Processing
```rust
// Engine batches events per tick
pub struct TickEvents {
    pub tick: u64,
    pub events: Vec<EngineEvent>,
}
```

### Optional Components
```rust
// Only serialize what visualizers need
#[cfg_attr(feature = "network", derive(Serialize))]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
```

## Testing Strategy

```rust
// Test engine without any visualization
#[test]
fn test_harvest_system() {
    let mut engine = SimulationEngine::new();
    let worker = engine.spawn_worker(Position::new(0, 0));
    let tree = engine.spawn_tree(Position::new(1, 0));
    
    engine.execute_command(EngineCommand::HarvestResource {
        worker_id: worker,
        target_id: tree,
    });
    
    // Advance simulation
    for _ in 0..10 {
        engine.tick();
    }
    
    // Check results via queries
    let resources = engine.query_resources();
    assert_eq!(resources.get(&ResourceType::Wood), Some(&5));
}
```

## Benefits

1. **True Engine Independence**: Core runs anywhere (server, WASM, embedded)
2. **Any Frontend**: Unity, Unreal, Godot, custom engines, web, terminal
3. **Network Ready**: Events serialize for multiplayer
4. **Test in Isolation**: No graphics needed for logic tests
5. **Performance**: No rendering overhead in simulation
6. **Modding**: Replace any visualizer without touching engine

This is how games like Minecraft Server, Factorio Headless, and Dwarf Fortress Remote work - pure simulation with pluggable frontends!