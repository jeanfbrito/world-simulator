# Project Templates and Scaffolding

This directory contains templates and scaffolding for creating world simulator projects and mods.

## Available Templates

### Basic Simulation Template (`basic_sim/`)
- Simple world simulation with basic features
- Perfect for beginners and learning the API

### AI Extension Template (`ai_extension/`)
- Template for adding new AI behaviors
- Includes custom GOAP actions and utility behaviors

### Economic Mod Template (`economic_mod/`)
- Template for economic simulation mods
- Custom resources, recipes, and market systems

### Custom World Generator (`world_gen/`)
- Template for custom world generation
- Includes terrain, biome, and resource generation

### Performance Test Template (`perf_test/`)
- Template for performance testing
- Includes benchmarking and analysis tools

### Web Integration Template (`web_integration/`)
- Template for web-based simulators
- WebSocket support and web viewer

## Quick Start

### Using Templates

1. **Copy template to new project**
   ```bash
   cp -r templates/basic_sim my_simulation
   cd my_simulation
   ```

2. **Customize the configuration**
   ```bash
   # Edit config.lua to match your needs
   nano config.lua
   ```

3. **Run the simulation**
   ```bash
   cargo run
   ```

### Creating Mods

1. **Copy mod template**
   ```bash
   cp -r templates/economic_mod my_mod
   cd my_mod
   ```

2. **Implement mod functionality**
   ```rust
   // src/lib.rs
   impl Mod for MyMod {
       fn initialize(&mut self, context: &ModContext) -> Result<(), ModError> {
           // Your mod initialization
           Ok(())
       }
   }
   ```

3. **Build and test**
   ```bash
   cargo build
   cargo test
   ```

## Template Structure

```
templates/
├── basic_sim/          # Basic simulation template
├── ai_extension/       # AI extension template
├── economic_mod/       # Economic mod template
├── world_gen/          # World generator template
├── perf_test/          # Performance test template
├── web_integration/    # Web integration template
└── create_project.sh   # Project creation script
```

## Template Features

### Basic Simulation Template
- Entity spawning and management
- Basic AI with GOAP planning
- Simple economic system
- Resource gathering and crafting
- WebSocket support
- Performance monitoring

### AI Extension Template
- Custom AI components
- GOAP action definitions
- Utility AI behaviors
- State machine integration
- Decision trees
- Learning systems

### Economic Mod Template
- Custom resource types
- Crafting recipes
- Market mechanics
- Supply and demand
- Price dynamics
- Trade systems

### World Generator Template
- Terrain generation
- Biome systems
- Resource distribution
- Feature placement
- Climate simulation
- Custom algorithms

### Performance Test Template
- Benchmarking framework
- Performance monitoring
- Memory tracking
- Load testing
- Scalability analysis
- Reporting tools

### Web Integration Template
- WebSocket server
- Real-time updates
- Web viewer
- API endpoints
- Client-side code
- Deployment scripts

## Project Creation Script

Use the included script to create new projects:

```bash
./templates/create_project.sh my_simulation --template basic_sim
```

This will:
- Copy the template to a new directory
- Initialize a Git repository
- Set up the project structure
- Create initial configuration
- Set up development tools

## Customizing Templates

### Adding New Components

```rust
// src/components.rs
#[derive(Component)]
pub struct CustomComponent {
    pub value: f32,
    pub state: String,
}
```

### Adding New Systems

```rust
// src/systems.rs
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

### Adding New Resources

```lua
-- config/resources.lua
config.resources.my_resource = {
    density = 0.1,
    max_per_tile = 5,
    respawn_time = 30.0,
    color = "#FF0000",
}
```

## Best Practices

### Template Customization
- Keep templates generic and reusable
- Use configuration files for customization
- Document customization options
- Provide clear examples

### Performance Considerations
- Use efficient data structures
- Minimize allocations in hot paths
- Consider cache efficiency
- Profile and optimize critical sections

### Code Organization
- Separate concerns into modules
- Use clear naming conventions
- Document public APIs
- Follow Rust best practices

### Testing
- Include comprehensive tests
- Test edge cases
- Use property-based testing
- Ensure backward compatibility

## Contributing Templates

To contribute new templates:

1. **Create template in appropriate directory**
2. **Add comprehensive documentation**
3. **Include examples and tests**
4. **Test with multiple scenarios**
5. **Submit pull request**

## Support

For help with templates:
- Check the documentation
- Look at existing examples
- Ask questions in GitHub Discussions
- Report issues on GitHub Issues

Happy simulating! 🎮