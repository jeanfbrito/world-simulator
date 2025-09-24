# Basic Simulation Example

This example demonstrates how to set up and run a basic world simulation with AI agents, resources, and buildings. It shows the core functionality of the world simulator in a simple, easy-to-understand way.

## What This Example Shows

- **World Initialization**: Setting up a basic simulation world
- **Entity Management**: Automatic spawning of units, resources, and buildings
- **AI Systems**: GOAP planning and Utility AI behaviors
- **Simulation Loop**: Running the simulation for a specified duration
- **Progress Monitoring**: Tracking simulation state and printing status updates

## Running the Example

### Prerequisites

Make sure you have the following installed:
- Rust (latest stable version)
- Cargo (Rust's package manager)

### Build and Run

```bash
# Navigate to the project root
cd /path/to/world-simulator

# Build the example
cargo build --example basic_simulation

# Run the example
cargo run --example basic_simulation
```

### Expected Output

The example will run for 500 simulation ticks and display progress updates:

```
🚀 Starting Basic World Simulation Example
🏗️  Setting up basic simulation...
✅ Basic simulation setup complete!
🎮 Simulation started. Monitoring progress...
📊 Tick 50: 5 units, 15 resources, 2 buildings
📊 Tick 100: 5 units, 14 resources, 3 buildings
📊 Tick 150: 5 units, 13 resources, 4 buildings
...
🎉 Basic simulation example completed successfully!
📈 Final state:
   • Total simulation ticks: 500
   • Active units: 5
   • Available resources: 10
   • Constructed buildings: 6
```

## Code Structure

### Main Components

1. **`main()`**: Sets up the Bevy app and plugins
2. **`setup_basic_simulation()`**: Initializes the simulation world
3. **`simulation_monitor_system()`**: Monitors progress and prints status

### Key Features

- **Headless Operation**: No window or graphics, pure simulation
- **Automatic Entity Spawning**: Uses the pack system to spawn entities
- **Progress Tracking**: Monitors simulation state every 50 ticks
- **Clean Exit**: Automatically exits after completion

## Configuration

The example uses default simulation configuration. You can modify the behavior by:

### Changing Simulation Duration

Edit the `simulation_monitor_system()` function to change the tick limit:

```rust
// Stop after 1000 ticks instead of 500
if sim_state.tick >= 1000 {
    println!("🎉 Extended simulation example completed successfully!");
    // ... rest of the code
}
```

### Adjusting Logging Level

Change the log level in `main()`:

```rust
// For more detailed logging
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
    .format_timestamp_millis()
    .init();

// For minimal logging
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
    .format_timestamp_millis()
    .init();
```

### Modifying World Configuration

You can modify world generation parameters by editing the `WorldMap` resource initialization or by creating custom world generation logic.

## Troubleshooting

### Common Issues

1. **Compilation Errors**: Make sure all dependencies are up to date
   ```bash
   cargo update
   ```

2. **Runtime Errors**: Check that all required asset files are present
   ```bash
   ls assets/packs/dev-world/
   ```

3. **No Output**: Ensure logging is properly configured
   ```bash
   RUST_LOG=info cargo run --example basic_simulation
   ```

### Performance Issues

If the simulation runs slowly, try:
- Reducing the number of entities in the world
- Decreasing the simulation tick rate
- Running in release mode: `cargo run --release --example basic_simulation`

## Next Steps

After understanding this basic example, you can explore:

1. **AI Demo Example**: Shows advanced AI behaviors and scenarios
2. **Custom World Example**: Demonstrates custom world generation
3. **WebSocket Client Example**: Shows how to connect to the simulation via WebSocket
4. **Performance Test Example**: Demonstrates performance testing and benchmarking

## Contributing

If you find issues with this example or have suggestions for improvement:

1. Check the [project documentation](../../docs/)
2. Review the [development workflow](../../docs/DEVELOPMENT_WORKFLOW.md)
3. Submit issues or pull requests to the GitHub repository

## License

This example is part of the world-simulator project and is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.
