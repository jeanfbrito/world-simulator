# Basic World Simulation Template

A template for creating basic world simulations using the world-simulator framework.

## Features

- Entity Component System (ECS) architecture
- Basic AI with GOAP and Utility AI
- Resource gathering and management
- Simple economic system with crafting
- Movement and navigation
- Performance monitoring
- Configurable simulation parameters

## Quick Start

### Prerequisites

- Rust 1.70+
- Cargo

### Building

```bash
cargo build --release
```

### Running

```bash
# Run with default configuration
cargo run

# Run with custom configuration
cargo run -- --config custom_config.toml

# Run with debug mode
cargo run --features debug
```

### Configuration

Edit `config.toml` to customize the simulation:

```toml
[world]
width = 150
height = 150
seed = 42

[entities]
peasant_count = 30

[[resources.types]]
name = "wood"
density = 0.15
```

## Project Structure

```
basic_sim/
├── src/
│   ├── main.rs          # Main simulation loop
│   ├── config.rs        # Configuration management
│   ├── systems.rs       # Simulation systems
│   ├── components.rs    # ECS components
│   └── lib.rs           # Library exports
├── config.toml          # Simulation configuration
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## Key Components

### Simulation Systems

- **MovementSystem**: Handles entity movement and navigation
- **ResourceGatheringSystem**: Manages resource collection
- **CraftingSystem**: Handles item creation and recipes
- **AISystem**: Controls entity behavior with multiple AI types

### ECS Components

- **Position**: Entity location in the world
- **Movement**: Entity velocity and movement targets
- **Health**: Entity health and regeneration
- **Inventory**: Item storage and management
- **Skills**: Entity abilities and progression

### Configuration Options

- World size and generation parameters
- Entity counts and behaviors
- Resource types and spawn rates
- System enable/disable and priorities
- Performance monitoring settings

## Customization Guide

### Adding New Components

1. Define the component in `components.rs`:

```rust
#[derive(Component)]
pub struct CustomComponent {
    pub value: f32,
    pub state: String,
}
```

2. Add it to entities:

```rust
let entity = simulation.spawn_entity("custom")?
    .with_component(CustomComponent::new(1.0, "active"));
```

### Creating New Systems

1. Implement the system in `systems.rs`:

```rust
pub struct CustomSystem {
    query: Query<With<CustomComponent>>,
}

impl System for CustomSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        for (entity, component) in self.query.iter(world) {
            // Update logic
        }
    }
}
```

2. Register it in the main simulation:

```rust
builder = builder.add_system(CustomSystem::new());
```

### Adding New Resources

1. Update `config.toml`:

```toml
[[resources.types]]
name = "custom_resource"
density = 0.1
max_per_tile = 3
respawn_time = 45.0
color = "#FF00FF"
```

2. Handle the resource in systems:

```rust
if resource.resource_type == "custom_resource" {
    // Custom resource handling
}
```

### Extending AI Behaviors

1. Add new GOAP actions:

```rust
planner.add_action(GOAPAction {
    name: "custom_action".to_string(),
    cost: 3.0,
    preconditions: vec![|world| world.can_perform_action()],
    effects: vec![|world| world.perform_action()],
});
```

2. Add utility behaviors:

```rust
utility_ai.add_behavior(UtilityBehavior {
    name: "custom_behavior".to_string(),
    utility: |world| world.should_perform_behavior(),
    action: |world| world.perform_behavior(),
});
```

## Examples

### Basic Entity Spawning

```rust
// Spawn a peasant with custom components
let peasant = simulation.spawn_entity("peasant")?
    .with_component(Position::new(10.0, 10.0))
    .with_component(Inventory::new(10))
    .with_component(Skills::new())
    .with_component(Health::new(100.0));
```

### Resource Management

```rust
// Add resource to inventory
inventory.add_item("wood", 5);

// Check if we have enough resources
if inventory.has_item("wood", 3) {
    // Craft something
    inventory.remove_item("wood", 3);
    inventory.add_item("planks", 6);
}
```

### AI Configuration

```rust
// Create GOAP-based AI
let ai = AIComponent::new(AIType::GOAP);
ai.current_goal = Goal::survival();

// Create Utility-based AI
let ai = AIComponent::new(AIType::Utility);
// The system will automatically select behaviors
```

## Performance Considerations

- Use appropriate entity counts for your world size
- Monitor FPS and adjust tick duration accordingly
- Use spatial partitioning for large worlds
- Batch operations where possible
- Profile memory usage with the monitoring system

## Testing

Run the test suite:

```bash
cargo test
```

Run benchmarks:

```bash
cargo bench
```

## Deployment

### Building for Release

```bash
cargo build --release
```

### Distribution

The binary will be available at:
- Linux/macOS: `target/release/basic_simulation`
- Windows: `target/release/basic_simulation.exe`

## Troubleshooting

### Common Issues

1. **Simulation runs slowly**
   - Reduce entity count in config
   - Increase tick duration
   - Disable unnecessary systems

2. **No entities spawning**
   - Check entity configuration
   - Verify world size is appropriate
   - Check resource density settings

3. **AI not working**
   - Verify AI system is enabled
   - Check if goals are properly set
   - Review action preconditions

### Debug Mode

Enable debug mode in config.toml:

```toml
[simulation]
debug_mode = true
```

This will provide additional logging and debug information.

## Contributing

1. Fork the template
2. Make your changes
3. Test thoroughly
4. Submit a pull request

## License

This template is licensed under the MIT License.

## Support

For issues and questions:
- Check the main project documentation
- Review existing examples
- Open an issue on GitHub

Happy simulating! 🎮