# World Simulator - Medieval Economy & AI Simulation Engine

A sophisticated medieval economy simulation engine with intelligent AI agents, built with Rust and Bevy ECS. Features advanced AI behaviors using GOAP (Goal-Oriented Action Planning) and Utility AI systems for realistic peasant simulation.

<img width="1416" height="1099" alt="image" src="https://github.com/user-attachments/assets/8703d922-7fa0-4480-8df2-a40d52a8ae83" />

## Features

- 🎮 **Headless Simulation**: Pure simulation engine with optional visualization
- 🤖 **Advanced AI**: GOAP planning + Utility AI for intelligent agent behaviors
- ⚡ **High Performance**: Optimized ECS architecture with spatial indexing
- 🏰 **Medieval Economy**: Resource gathering, crafting, building, storage management
- 🌍 **Dynamic World**: Procedural terrain generation with biomes and resources
- 🔌 **WebSocket API**: Real-time frontend communication for custom visualizers
- 📊 **Debug Monitoring**: Built-in ASCII world visualization and AI decision tracking
- 💾 **Save/Load System**: Persistent world state with bincode serialization
- 🎯 **Event-Driven**: Clean command/event API for external control

## 📚 Documentation

Comprehensive documentation is available in the `docs/` folder:

### Getting Started
- [**Quick Start Guide**](docs/guides/quick-start.md) - Get up and running in minutes
- [**Debugging Guide**](docs/guides/debugging.md) - Tools and techniques for troubleshooting

### Architecture
- [**System Overview**](docs/architecture/overview.md) - Core architecture and design principles
- [**Tick System**](docs/architecture/tick-system.md) - Understanding tick-based simulation
- [**Core Components**](docs/architecture/components.md) - Entity Component System details

### AI & Behavior
- [**Behavior System Overview**](docs/behavior-system/README.md) - Dual-AI system explanation
- [**GOAP Planning**](docs/behavior-system/goap-planning.md) - Goal-Oriented Action Planning
- [**Big Brain Reactive**](docs/behavior-system/big-brain-reactive.md) - Emergency response system
- [**Actions & Tasks**](docs/behavior-system/actions-and-tasks.md) - All available unit actions

### Systems
- [**Movement & Pathfinding**](docs/movement-pathfinding.md) - Grid movement and A* pathfinding
- [**Energy Management**](docs/needs-system/energy-management.md) - Three-layer energy protection
- [**Hunger System**](docs/needs-system/hunger-system.md) - Food gathering and consumption
- [**Work System**](docs/needs-system/work-system.md) - Resource gathering mechanics

## Quick Start

```bash
# Clone the repository
git clone https://github.com/jeanfbrito/world-simulator.git
cd world-simulator

# Run the simulation with AI agents
cargo run -p world_sim_simple

# Monitor the living world (recommended - shows peasant behaviors)
./monitor_world.sh

# Run with detailed AI debug output
RUST_LOG=debug cargo run -p world_sim_simple

# Run with minimal output
RUST_LOG=error cargo run -p world_sim_simple

# Run headless simulation examples
cargo run --example headless      # Basic headless mode
cargo run --example terminal      # ASCII visualization
cargo run --example goap_workers  # GOAP AI demonstration
```

### Web Frontend

```bash
# Start the simulation with WebSocket server
cargo run -p world_sim_simple

# Open frontend in browser
open frontend/index.html
# Or serve with Python
cd frontend && python -m http.server 8000
```

## Architecture

```
┌─────────────────────────────────────────┐
│          SIMULATION CORE                │
│         (Bevy ECS Engine)               │
├─────────────────────────────────────────┤
│ • AI Systems (GOAP + Utility)           │
│ • Resource Management                   │
│ • Crafting & Building                   │
│ • Tilemap & Spatial Index               │
│ • Save/Load System                      │
└─────────────────────────────────────────┘
                    ↕
         ┌──────────────────┐
         │  WebSocket API    │
         └──────────────────┘
                    ↕
    ┌──────────────────────────────┐
    │      VISUALIZATION OPTIONS     │
    ├──────────────────────────────┤
    │ • Terminal Monitor (ASCII)     │
    │ • Web Frontend (HTML5/JS)      │
    │ • Debug CLI (Built-in)         │
    │ • Custom Clients via WebSocket │
    └──────────────────────────────┘
```

### AI System Architecture

The simulation features a sophisticated dual-AI system:

- **GOAP (Goal-Oriented Action Planning)**: Strategic planning for long-term goals
  - Dynamic goal selection based on needs
  - A* planning for optimal action sequences
  - Actions: GatherWood, GatherFood, BuildHouse, Eat, Rest
  
- **Utility AI**: Reactive behaviors for immediate needs
  - Real-time scoring of available actions
  - Smooth transitions between behaviors
  - Handles interruptions and emergencies

### Component System

- **Unit Components**: Position, Needs (hunger/energy), Inventory, Work state
- **Building Components**: Type, ownership, storage capacity
- **Resource Components**: Type (wood/food/stone), regeneration rates
- **AI Components**: Behavior state, current plan, task queue

## Project Structure

```
world-simulator/
├── world_sim_simple/       # Main simulation implementation
│   ├── src/
│   │   ├── ai/            # AI systems (GOAP, Utility, Pathfinding)
│   │   ├── buildings/     # Building types and construction
│   │   ├── components/    # ECS components
│   │   ├── crafting/      # Crafting recipes and stations
│   │   ├── performance/   # Metrics and spatial indexing
│   │   ├── resources/     # Resource types and inventory
│   │   ├── save_load/     # Persistence system
│   │   ├── scripting/     # Lua configuration loading
│   │   ├── systems/       # Core game systems
│   │   ├── tilemap/       # World generation and chunks
│   │   └── websocket.rs   # WebSocket server for frontends
│   └── Cargo.toml
├── world_sim_core/         # Original core engine (legacy)
├── world_sim_interface/    # Shared types and API
├── frontend/              # Web-based visualization
│   ├── index.html
│   ├── js/               # Game client and WebSocket handling
│   └── css/              # Styling
├── examples/              # Usage examples
├── assets/               # Game assets and configurations
└── scripts/              # Development and utility scripts
```

## Key Features in Detail

### Intelligent Peasant AI
- Peasants autonomously manage hunger and energy needs
- Plan efficient paths to resources using A* pathfinding
- Gather wood from trees and food from berry bushes
- Build houses when resources are available
- Store excess resources in buildings
- Make decisions based on current needs and available actions

### Resource System
- **Trees**: Provide wood, regenerate over time
- **Berry Bushes**: Provide food, seasonal growth
- **Stone Deposits**: Mining resources (planned)
- **Inventory Management**: Slot-based system with stacking

### Building System
- **Houses**: Provide shelter and storage
- **Stockpiles**: Dedicated storage buildings
- **Workshops**: Crafting stations (planned)
- **Construction**: Multi-step building process with material requirements

### World Generation
- 64x64 tile worlds with multiple biomes
- Procedural placement of resources
- Chunk-based management for performance
- Configurable terrain types (grass, forest, water, mountains)

## Development

```bash
# Run tests
cargo test

# Run with specific log levels
RUST_LOG=error cargo run -p world_sim_simple   # Minimal output
RUST_LOG=info cargo run -p world_sim_simple    # Standard output
RUST_LOG=debug cargo run -p world_sim_simple   # Detailed debugging

# Monitor running simulation
./monitor_world.sh

# Build optimized release version
cargo build --release -p world_sim_simple

# Check code quality
cargo clippy
cargo fmt --check

# Run specific examples
cargo run --example goap_workers    # Test GOAP AI
cargo run --example headless        # Headless simulation
```

## Performance

- Handles 100+ AI agents at 60 FPS
- Spatial indexing for efficient queries
- Chunk-based world management
- Optimized pathfinding with caching
- Minimal memory footprint per entity

## Configuration

The simulation can be configured through:
- Lua scripts in `assets/` for AI behaviors and recipes
- JSON files for resource and building definitions
- Runtime parameters via environment variables
- WebSocket commands for dynamic control

## License

MIT - See LICENSE file for details
