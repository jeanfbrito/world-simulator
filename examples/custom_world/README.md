# Custom World Generation Example

This example demonstrates advanced world generation capabilities in the world-simulator. It showcases procedural terrain generation, custom biome systems, dynamic resource distribution, and configurable world parameters.

## What This Example Shows

### World Generation Features

- **Procedural Terrain**: Dynamic terrain generation using noise functions
- **Custom Biomes**: Five distinct biome types with unique characteristics
- **Resource Distribution**: Intelligent resource placement based on biome preferences
- **Feature Generation**: Rivers, lakes, forests, and mountain ranges
- **Configurable Parameters**: Complete control over world generation settings

### Biome Types Demonstrated

1. **Plains**: Balanced terrain with moderate resources
2. **Forest**: Dense vegetation with abundant wood and food
3. **Mountains**: Rugged terrain with stone and ore deposits
4. **Desert**: Arid environment with scarce resources
5. **Tundra**: Cold climate with limited resources

### Advanced Features

- **Noise-Based Generation**: Uses Perlin noise for natural terrain
- **Biome Interpolation**: Smooth transitions between biomes
- **Resource Clustering**: Resources group in realistic deposits
- **Feature Placement**: Natural features like rivers and lakes
- **Spawn Optimization**: Intelligent entity placement based on terrain

## Configuration

The custom world uses advanced configuration through Lua files:

- **Terrain Parameters**: Noise settings and biome definitions
- **Resource Distribution**: Biome-based resource preferences and clustering
- **Entity Configuration**: Custom units and buildings for generated worlds
- **Feature Generation**: Natural world features and their parameters

## Running the Example

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

### From the Project Root

```bash
# Run the custom world generation example
cargo run --example custom_world
```

### From the Example Directory

```bash
cd examples/custom_world
cargo run --bin custom_world
```

## Expected Output

The demonstration will run for 600 ticks and display detailed world generation analysis:

```
🌍 Starting Custom World Generation Example
🏗️  Custom world generation initialized. Running for 600 ticks...
🗺️  This will showcase procedural world generation and customization.
🎮 Custom world generation started. Monitoring terrain and features...
📈 Tracking world generation progress and entity distribution...

🗺️  Tick 100: World Stats - 45 resources, 5 units, 2 buildings
🌍 Initial World Generation Analysis:
   • World size: 32x32
   • Resource nodes: 45
   • Active entities: 5
   • Structures: 2

🔍 Resource Distribution Analysis:
   • Wood resources: 20 (44.4%)
   • Stone resources: 15 (33.3%)
   • Food resources: 10 (22.2%)
   • Total resource volume: 2150

🏗️  World Development Analysis:
   • Development score: 240
   • Entity density: 0.022 entities/tile
   • Unit-to-building ratio: 2.5:1
   • Resources per unit: 9.0
   • Development stage: Growing village

🎉 Custom world generation completed successfully!
📊 Final World Analysis:
   • World dimensions: 32x32
   • Total resources: 42
   • Total units: 8
   • Total buildings: 4
   • Simulation ticks: 600
🌍 Key World Generation Features Demonstrated:
   • Procedural terrain generation
   • Dynamic resource distribution
   • Customizable biome systems
   • Advanced entity placement
   • Configurable world parameters
```

## Key Components

### Main Simulation (`main.rs`)

- Initializes the Bevy app with custom world generation plugins
- Sets up comprehensive monitoring and analysis systems
- Provides detailed world generation tracking and insights
- Demonstrates procedural generation and customization

### World Configuration (`world_config.lua`)

- **Terrain Generation**: Noise parameters and biome definitions
- **Resource Systems**: Biome-based resource distribution and clustering
- **Entity Definitions**: Custom units and buildings for generated worlds
- **Feature Systems**: Natural world features and placement algorithms

### Monitoring Systems

- **Terrain Analysis**: Tracks world generation progress and patterns
- **Resource Distribution**: Analyzes resource placement and balance
- **Development Tracking**: Monitors world development and growth stages
- **Biome Analysis**: Provides insights into biome distribution and characteristics

## Customization

### Modify Terrain Generation

Adjust terrain parameters in the world configuration:

```lua
-- Change terrain roughness
config.world.terrain.noise_scale = 0.15
config.world.terrain.noise_octaves = 6

-- Modify biome distribution
config.world.terrain.forest_density = 0.5
config.world.terrain.mountain_density = 0.3
```

### Create Custom Biomes

Add new biome types to the configuration:

```lua
{
    name = "swamp",
    humidity_range = {0.7, 1.0},
    temperature_range = {0.4, 0.8},
    color = {0.2, 0.4, 0.2},
    resources = {"wood", "food"},
    features = {"water", "vegetation"}
}
```

### Adjust Resource Distribution

Modify resource generation parameters:

```lua
-- Change resource abundance
config.world.resources.types.wood.max_amount = 150
config.world.resources.types.wood.cluster_size = 5

-- Add new resource types
ore = {
    name = "gold_ore",
    max_amount = 20,
    regeneration_rate = 0.001,
    biome_preference = {"mountains"}
}
```

### Custom Entity Creation

Define custom units for generated worlds:

```lua
explorer = {
    name = "explorer",
    display_name = "World Explorer",
    vision_range = 12,
    movement_speed = 1.5,
    abilities = {"explore", "survey", "map"}
}
```

## Learning Points

This example teaches you:

1. **Procedural Generation**: How to generate worlds using noise functions and algorithms
2. **Biome Systems**: Creating and managing different world biomes with unique characteristics
3. **Resource Distribution**: Intelligent resource placement based on environmental factors
4. **Feature Generation**: Creating natural world features like rivers and mountains
5. **World Configuration**: Comprehensive world generation through configuration files

## Advanced Concepts

### Noise-Based Terrain Generation

- Perlin noise for natural terrain
- Multi-octave noise for detail
- Biome interpolation and transitions
- Elevation and humidity mapping

### Resource Distribution Algorithms

- Biome-based resource preferences
- Resource clustering and deposits
- Regeneration and sustainability
- Balance and gameplay considerations

### Feature Generation Systems

- River generation using pathfinding
- Lake placement based on terrain
- Forest and vegetation distribution
- Mountain range generation

## Next Steps

After understanding this custom world generation example, you can explore:

- **AI Demonstration Example**: Advanced AI behaviors and decision making
- **Basic Simulation Example**: Core simulation features and monitoring
- **Performance Testing**: Benchmarking and optimization techniques
- **Modding Example**: Creating custom game modes and modifications

## Troubleshooting

### Common Issues

**World generation takes too long**: Reduce world size or complexity in configuration.

**Resources too sparse**: Adjust resource density and clustering parameters.

**Biome distribution uneven**: Modify noise parameters and biome ranges.

**Entities not spawning**: Check spawn requirements and building availability.

**Performance issues**: Reduce world size or disable certain features.

### Getting Help

If you encounter issues:

1. Check the detailed world generation output for insights
2. Verify configuration parameters and syntax
3. Adjust logging level to debug for detailed generation traces
4. Review the world generation documentation for implementation details

## Performance Considerations

- World generation complexity increases with world size
- Noise computation can be CPU-intensive with many octaves
- Resource clustering requires additional processing
- Feature generation adds significant computation time
- Consider generation caching for repeated worlds

For more detailed information about world generation in the world-simulator, see the main project documentation.

## Technical Details

### Generation Pipeline

1. **Terrain Base**: Generate noise-based height and humidity maps
2. **Biome Assignment**: Assign biomes based on terrain characteristics
3. **Feature Placement**: Place rivers, lakes, forests, and mountains
4. **Resource Distribution**: Place resources based on biome preferences
5. **Entity Spawning**: Spawn units and buildings based on world state

### Configuration System

- Lua-based configuration for flexibility
- Hierarchical parameter organization
- Type validation and defaults
- Hot-reloading support (in development)

### Performance Optimization

- Procedural generation with deterministic results
- Efficient noise computation
- Spatial partitioning for features
- Lazy loading of world components