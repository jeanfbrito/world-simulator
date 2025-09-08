# Quickstart Guide: Headless World Simulator

## Prerequisites

- Rust 1.75+ (install from [rustup.rs](https://rustup.rs/))
- Git
- Optional: Any text editor with Rust support

## Quick Setup (2 minutes)

```bash
# Clone the repository
git clone https://github.com/jeanfbrito/world-simulator.git
cd world-simulator

# Build the project
cargo build --release

# Run headless simulation (no graphics)
cargo run --example headless

# Run with ASCII terminal display
cargo run --example terminal

# Run with Bevy 2D graphics (if you want visuals)
cargo run --example with_graphics
```

## Project Structure

```
world-simulator/
├── Cargo.toml                    # Workspace configuration
├── world_sim_core/               # HEADLESS ENGINE (no graphics!)
│   ├── src/
│   │   ├── lib.rs               # Core plugin
│   │   ├── components/          # ECS data
│   │   ├── systems/            # Game logic
│   │   └── events/             # Event definitions
│   └── tests/
├── world_sim_interface/          # Shared API types
├── world_sim_bevy_viz/          # Optional: 2D sprites
├── world_sim_terminal/          # Optional: ASCII display
└── examples/
    ├── headless.rs              # No UI, pure simulation
    ├── terminal.rs              # ASCII like Dwarf Fortress
    └── with_graphics.rs         # Simple 2D sprites
```

## Choose Your Interface

### Option 1: Headless (Testing/Server)
```bash
cargo run --example headless
# Runs pure simulation, outputs events to console
```

### Option 2: Terminal ASCII (Retro Style)
```bash
cargo run --example terminal
```
```
World (50x50) Tick: 1234
┌──────────────────────────┐
│....T...T......@....T.....│  T = Tree
│..T....@....#####.....T...│  @ = Worker  
│.....T.....#######....B...│  # = House
│..@.......#######..T......│  B = Berry Bush
│....T......#####......T....│  S = Stockpile
│....................S......│  G = Granary
└──────────────────────────┘
Resources: Wood: 45 | Food: 23 | Population: 8
Commands: (H)arvest (B)uild (M)ove (Q)uit
```

### Option 3: Simple 2D Graphics (Bevy)
```bash
cargo run --example with_graphics
# WASD to move camera, click to select, right-click to command
```

## Basic Usage

### 1. Running Tests First (TDD)

```bash
# Run all tests
cargo test

# Run core engine tests only
cargo test -p world_sim_core

# Watch mode for TDD
cargo watch -x test

# Run specific test
cargo test harvest_system
```

### 2. Create and Run a World

```rust
// examples/minimal.rs
use world_sim_core::{SimulationEngine, WorldConfig};

fn main() {
    // Create engine (no graphics needed!)
    let mut engine = SimulationEngine::new();
    
    // Configure world
    let config = WorldConfig {
        width: 100,
        height: 100,
        starting_workers: 5,
        resource_density: 0.1,
        ..Default::default()
    };
    
    // Initialize
    engine.new_world(config).unwrap();
    
    // Run simulation
    for tick in 0..1000 {
        engine.tick(0.05); // 20 ticks per second
        
        // Get events (this is what visualizers would use)
        let events = engine.get_events();
        for event in events {
            println!("Tick {}: {:?}", tick, event);
        }
    }
}
```

### 3. Send Commands to Engine

```rust
use world_sim_interface::{EngineCommand, Position, BuildingType};

// Harvest a tree
let cmd = EngineCommand::HarvestResource {
    worker_ids: vec![worker_1, worker_2],
    target_id: tree_entity,
};
engine.execute_command(cmd);

// Build a house
let cmd = EngineCommand::ConstructBuilding {
    building_type: BuildingType::House,
    position: Position { x: 50, y: 50 },
    workers: vec![worker_3],
};
engine.execute_command(cmd);

// Query area
let cmd = EngineCommand::QueryArea {
    min: Position { x: 0, y: 0 },
    max: Position { x: 10, y: 10 },
};
let result = engine.execute_command(cmd);
```

## Verification Checklist

### Core Engine Features

- [ ] **World Generation**: Creates entities without graphics
  ```bash
  cargo test world_generation
  ```

- [ ] **Resource Harvesting**: Workers harvest resources
  ```bash
  cargo test harvest_system
  ```

- [ ] **Recipe Processing**: Transform resources
  ```bash
  cargo test recipe_system
  ```

- [ ] **Building Construction**: Place and build structures
  ```bash
  cargo test building_system
  ```

- [ ] **Population Management**: Growth and food consumption
  ```bash
  cargo test population_system
  ```

- [ ] **Event System**: All changes emit events
  ```bash
  cargo test event_emission
  ```

### Performance Targets

- [ ] **Entity Count**: Handle 10,000+ entities
  ```bash
  cargo bench entity_processing
  ```

- [ ] **Tick Performance**: <1ms for 1,000 entities
  ```bash
  cargo run --release --example benchmark
  ```

- [ ] **Memory Usage**: <200MB for full world
  ```bash
  cargo run --example memory_test
  ```

- [ ] **Deterministic**: Same inputs = same outputs
  ```bash
  cargo test determinism
  ```

## Adding Your Own Visualizer

### Step 1: Create a New Crate
```bash
cargo new world_sim_my_viz --lib
```

### Step 2: Implement the Observer
```rust
// world_sim_my_viz/src/lib.rs
use world_sim_interface::{EngineEvent, EngineObserver};

pub struct MyVisualizer {
    // Your visualization state
}

impl EngineObserver for MyVisualizer {
    fn on_events(&mut self, events: &[EngineEvent]) {
        for event in events {
            match event {
                EngineEvent::EntitySpawned { position, entity_type, .. } => {
                    // Draw your entity however you want!
                },
                EngineEvent::ResourceHarvested { amount, .. } => {
                    // Show harvesting animation
                },
                // ... handle other events
            }
        }
    }
    
    fn on_snapshot(&mut self, snapshot: &WorldSnapshot) {
        // Optional: handle full state updates
    }
}
```

### Step 3: Connect to Engine
```rust
// examples/my_viz.rs
use world_sim_core::SimulationEngine;
use world_sim_my_viz::MyVisualizer;

fn main() {
    let mut engine = SimulationEngine::new();
    let visualizer = Box::new(MyVisualizer::new());
    
    engine.add_observer(visualizer);
    engine.new_world(Default::default()).unwrap();
    
    // Run your game loop
    loop {
        engine.tick(0.05);
        // Your visualization updates automatically via events!
    }
}
```

## Integration Examples

### Unity 3D Client
```csharp
// Use the FFI bridge
[DllImport("world_sim_unity_bridge")]
static extern IntPtr create_engine();

[DllImport("world_sim_unity_bridge")]
static extern void tick_engine(IntPtr engine, float dt);
```

### Godot Integration
```gdscript
# Via GDNative
var engine = preload("res://world_sim.gdns").new()
engine.tick(delta)
```

### Web Frontend (via WASM)
```javascript
import init, { SimulationEngine } from './world_sim_wasm.js';

await init();
const engine = new SimulationEngine();
engine.tick(0.05);
```

## Common Tasks

### Add a New Resource Type
1. Add variant to `ResourceType` enum in `world_sim_interface`
2. Add harvesting logic in `harvest_system.rs`
3. Add to drop tables in `resources.rs`
4. Write tests first!

### Add a New Building Type
1. Add variant to `BuildingType` enum
2. Create construction recipe
3. Add building system logic
4. Write tests first!

### Debug Commands
```rust
// Enable debug features
#[cfg(debug_assertions)]
let cmd = EngineCommand::SpawnEntity {
    entity_type: EntityType::Tree,
    position: Position { x: 10, y: 10 },
};
```

## Performance Profiling

```bash
# CPU profiling
cargo build --release
perf record --call-graph=dwarf cargo run --release --example benchmark
perf report

# Memory profiling  
cargo build --release
valgrind --tool=massif cargo run --release --example benchmark
ms_print massif.out.*

# Flame graph
cargo install flamegraph
cargo flamegraph --example benchmark
```

## Testing Without Graphics

```bash
# Run headless server
cargo run --example headless_server

# In another terminal, connect test client
cargo run --example test_client

# Or use netcat to send raw commands
echo '{"type":"HarvestResource","worker_ids":[1],"target_id":5}' | nc localhost 7777
```

## Next Steps

1. **Try Different Visualizers**: Terminal → 2D → 3D
2. **Add Resources**: Stone, iron, more food types
3. **Complex Buildings**: Workshops, farms, defenses
4. **Multiplayer**: Run headless server with multiple clients
5. **Custom Frontend**: Make your own visualizer!

## Troubleshooting

### "No graphics device found"
You're trying to run `with_graphics` on a headless system. Use:
```bash
cargo run --example headless  # or terminal
```

### Performance Issues
```bash
# Use release mode!
cargo run --release --example your_example

# Check you're not using DefaultPlugins in core
grep -r "DefaultPlugins" world_sim_core/  # Should return nothing
```

### Determinism Issues
```bash
# Test determinism
cargo test test_deterministic_simulation
```

## Support

- Engine API: See `contracts/engine-api.rs`
- Architecture: See `headless-architecture.md`
- Examples: Check `examples/` directory
- Tests: Look at `world_sim_core/tests/`