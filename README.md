# World Simulator - Headless Medieval Economy Engine

A pure headless simulation engine for medieval economy and fortress games, built with Rust and Bevy ECS. The engine has **zero rendering dependencies** and can be visualized with any frontend (Unity, Godot, Terminal, Web).

## Features

- 🎮 **Headless Core**: Pure simulation, no graphics dependencies
- 🔌 **Pluggable Visualizers**: Connect any frontend via events/commands
- ⚡ **High Performance**: Handle 10,000+ entities at 60 ticks/second
- 🏰 **Medieval Economy**: Resource gathering, crafting, building, population
- 🔄 **Deterministic**: Same inputs always produce same outputs
- 🎯 **Event-Driven**: Clean API for observation and control

## Quick Start

```bash
# Clone the repository
git clone https://github.com/jeanfbrito/world-simulator.git
cd world-simulator

# Run the simple simulation with GUI and GOAP AI
cargo run -p world_sim_simple

# Run with debug output to see AI decisions
RUST_LOG=info cargo run -p world_sim_simple

# Run headless simulation (no graphics)
cargo run --example headless

# Run with ASCII terminal display
cargo run --example terminal

# Run with simple 2D graphics (optional)
cargo run --example with_graphics
```

## Architecture

```
┌─────────────────────────────────────────┐
│        HEADLESS ENGINE CORE             │
│     (Pure Simulation, No Graphics)      │
│                                         │
│  ECS Systems → Events → Commands        │
└─────────────────────────────────────────┘
                    ↕
    ┌──────────────────────────────┐
    │      PLUGGABLE FRONTENDS      │
    ├──────────────────────────────┤
    │ • Terminal ASCII              │
    │ • Bevy 2D/3D                  │
    │ • Unity (via FFI)             │
    │ • Godot (via GDNative)        │
    │ • Web (via WASM)              │
    └──────────────────────────────┘
```

## Project Structure

- `world_sim_core/` - Headless simulation engine
- `world_sim_interface/` - Shared types and API
- `world_sim_bevy_viz/` - Optional Bevy 2D renderer
- `world_sim_terminal/` - Optional ASCII display
- `examples/` - Usage examples
- `specs/` - Design documentation

## Development

```bash
# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code quality
cargo clippy
cargo fmt --check

# Build release version
cargo build --release
```

## License

MIT - See LICENSE file for details