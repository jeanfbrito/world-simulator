# Modding Example

This example demonstrates the modding and customization capabilities of the world-simulator. It showcases custom entity types, mod-based gameplay systems, scriptable behaviors, and a plugin architecture.

## What This Example Shows

### Modding Features

- **Custom Components**: Magic, Technology, Quest, and Custom AI components
- **Mod Systems**: Pluggable systems that extend gameplay
- **Entity Customization**: Custom entity types with mod-specific behaviors
- **Event System**: Scriptable events and handlers
- **UI Extensions**: Custom UI elements and interfaces

### Core Mods Demonstrated

1. **Magic System Mod**: Adds spell casting, mana management, and magical entities
2. **Technology Mod**: Research progression, technology trees, and scientific advancement
3. **Quest System Mod**: Quest generation, objectives, and reward systems

### Custom Components

- **MagicComponent**: Mana, spells, and spell power
- **TechnologyComponent**: Research points, unlocked technologies, and research efficiency
- **CustomAIComponent**: Personality types, behavior weights, and learning systems
- **QuestComponent**: Active quests, progress tracking, and reputation management

### Advanced Modding Concepts

- **Mod Dependencies**: Managing relationships between mods
- **Load Order**: Controlling mod initialization sequence
- **Event System**: Cross-mod communication and scripting
- **Component Registration**: Dynamic component registration
- **System Plugins**: Pluggable gameplay systems

## Configuration

The modding example uses comprehensive configuration for mod management:

- **Mod Management**: Discovery, loading, and dependency resolution
- **Custom Components**: Field definitions and update systems
- **Event System**: Scriptable events with parameters and handlers
- **UI Extensions**: Custom interface elements and data binding

## Running the Example

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

### From the Project Root

```bash
# Run the modding example
cargo run --example modding_example
```

### From the Example Directory

```bash
cd examples/modding_example
cargo run --bin modding_example
```

## Expected Output

The demonstration will run for 400 ticks with a visualization window:

```
🔧 Starting Modding Example
🔧 Modding example initialized. Running custom gameplay systems...
🎮 This will showcase modding capabilities and custom gameplay mechanics.
🔧 Setting up Modding Example...
✨ Magic system initialized
🔬 Technology system initialized
📜 Quest system initialized
✅ Modding example setup complete!
🎯 Loaded mods: 3
🔧 Active systems: 3
🎮 Custom entities with magic, technology, and quest systems
🎮 Modding example started. Monitoring custom systems...
📈 Tracking mod performance and entity interactions...

🔧 Mod Entities - 1 magic users, 1 scientists, 1 quest holders
✨ Magic event: Mana regeneration active
🔬 Technology event: Research progress updated
📜 Quest event: New objectives available

🔧 Mod Performance - 3 active mods, 3 mod systems

🎉 Modding example completed successfully!
📊 Final Mod Analysis:
   • Loaded mods: 3
   • Active mods: 3
   • Mod systems: 3
   • Simulation ticks: 400
🔧 Key Modding Features Demonstrated:
   • Custom entity types and components
   • Mod-based gameplay systems
   • Scriptable behaviors and events
   • Plugin architecture
   • Custom UI and visualization
```

## Key Components

### Main Application (`main.rs`)

- **Mod Manager**: Handles mod loading, dependencies, and execution
- **Custom Components**: Magic, Technology, Quest, and Custom AI components
- **Mod Systems**: Pluggable systems for extended gameplay
- **Event System**: Scriptable events and cross-mod communication
- **UI Framework**: Custom interface elements and data binding

### Configuration (`config.lua`)

- **Mod Definitions**: Core mods with complete functionality
- **Component Schema**: Field definitions and validation
- **System Configuration**: Update intervals and query requirements
- **Event System**: Parameters, handlers, and routing
- **UI Extensions**: Custom interface definitions

### Core Mods

1. **Magic System**: Spell casting, mana management, magical entities
2. **Technology Mod**: Research trees, scientific advancement, laboratories
3. **Quest System**: Quest generation, objectives, reward systems

## Customization

### Create Custom Mods

Define new mods in the configuration:

```lua
-- Add custom mod
config.mods.custom_mod = {
    enabled = true,
    version = "1.0.0",
    description = "Custom gameplay mod",
    dependencies = {"magic_system"},
    load_order = 4,

    -- Custom mod content
    custom_content = {
        entities = {...},
        components = {...},
        systems = {...}
    }
}
```

### Define Custom Components

Create new component types:

```lua
-- Custom component definition
config.components.my_component = {
    component_type = "MyComponent",
    fields = {
        custom_field = {type = "f32", default = 1.0, min = 0.0},
        another_field = {type = "string", default = "default"}
    },
    update_system = "my_system",
    save_data = true,
    network_sync = true
}
```

### Implement Custom Systems

Create pluggable gameplay systems:

```lua
-- Custom system configuration
config.systems.my_system = {
    system_type = "update",
    update_interval = 0.5,
    priority = 60,
    parallel = true,
    query_requirements = {
        required_components = {"MyComponent"},
        optional_components = {"PositionComponent"}
    },
    operations = {
        "custom_operation_1",
        "custom_operation_2"
    }
}
```

### Add Custom Events

Define scriptable events:

```lua
-- Custom event definition
config.events.custom_event = {
    event_type = "custom",
    parameters = {
        source = {type = "entity", required = true},
        target = {type = "entity", required = false},
        intensity = {type = "f32", required = true}
    },
    handlers = {
        "handle_custom_event",
        "update_ui_for_custom_event"
    }
}
```

### Create Custom UI Elements

Extend the user interface:

```lua
-- Custom UI element
config.ui.custom_ui = {
    enabled = true,
    elements = {
        custom_panel = {
            type = "panel",
            position = {x = 10, y = 10},
            size = {width = 300, height = 200},
            binding = "custom_data"
        }
    }
}
```

## Learning Points

This example teaches you:

1. **Mod Architecture**: Design patterns for extensible gameplay systems
2. **Component System**: Dynamic component registration and management
3. **Event System**: Scriptable events and cross-mod communication
4. **Plugin Architecture**: Pluggable systems and mod loading
5. **UI Framework**: Custom interface elements and data binding

## Advanced Concepts

### Mod Lifecycle Management

- **Discovery**: Finding and identifying available mods
- **Loading**: Loading mod data and dependencies
- **Initialization**: Setting up mod systems and components
- **Execution**: Running mod systems and processing events
- **Cleanup**: Proper shutdown and resource cleanup

### Component Architecture

- **Dynamic Registration**: Runtime component type registration
- **Schema Validation**: Field type checking and constraint validation
- **Data Binding**: Automatic synchronization between components and UI
- **Serialization**: Save/load support for mod components
- **Network Synchronization**: Multiplayer support for mod data

### Event System

- **Event Types**: Action, state change, achievement, interaction events
- **Parameter System**: Typed parameters with validation
- **Handler System**: Event routing and processing
- **Cross-Mod Communication**: Event-based mod interaction
- **Scripting Support**: Lua-based event handlers

### UI Framework

- **Custom Elements**: Buttons, panels, progress bars, and custom widgets
- **Data Binding**: Automatic UI updates based on game state
- **Event Handling**: User interaction processing
- **Layout System**: Positioning, sizing, and anchoring
- **Theming**: Customizable appearance and styling

## Next Steps

After understanding this modding example, you can explore:

- **Custom World Generation**: Advanced procedural generation techniques
- **AI Demonstration**: Complex AI behaviors and decision making
- **WebSocket Client**: Real-time monitoring and control
- **Performance Testing**: Benchmarking and optimization

## Troubleshooting

### Common Issues

**Mods not loading**: Check mod directory structure and file permissions.

**Component conflicts**: Verify component type names and field definitions.

**System crashes**: Review system requirements and query configurations.

**Event handling fails**: Check event parameters and handler definitions.

**UI not displaying**: Verify UI element positioning and binding expressions.

### Getting Help

If you encounter issues:

1. Check the mod loading output for errors and warnings
2. Verify mod dependencies and load order
3. Review component and system configurations
4. Enable debug logging for detailed troubleshooting
5. Check UI element definitions and data bindings

## Best Practices

- **Mod Design**: Keep mods focused and modular
- **Component Design**: Use clear field names and validation
- **System Design**: Optimize update intervals and query efficiency
- **Event Design**: Use descriptive event names and clear parameters
- **UI Design**: Create intuitive and responsive interfaces

## Performance Considerations

- **Mod Loading**: Minimize loading time and memory usage
- **Component Updates**: Optimize update intervals and query complexity
- **Event Processing**: Use efficient event routing and handler logic
- **UI Updates**: Minimize redraw frequency and data binding overhead
- **Memory Management**: Proper cleanup of mod resources

## Security Considerations

- **Input Validation**: Validate all mod-provided data
- **Resource Limits**: Prevent mod resource exhaustion
- **Sandboxing**: Limit mod access to system resources
- **Compatibility**: Ensure mod compatibility and version checking
- **Error Handling**: Graceful handling of mod errors and failures

For more detailed information about modding in the world-simulator, see the main project documentation.

## Technical Details

### Mod Architecture

1. **Discovery**: Scan mod directories for manifest files
2. **Validation**: Check compatibility and dependencies
3. **Loading**: Parse mod data and register components
4. **Initialization**: Create mod systems and set up event handlers
5. **Execution**: Run mod systems and process game events

### Component System

- **Registration**: Dynamic component type registration
- **Storage**: Efficient component data storage and access
- **Queries**: Fast entity queries with component filtering
- **Updates**: Optimized component update scheduling
- **Serialization**: Automatic save/load support

### Event System

- **Event Types**: Typed events with parameter validation
- **Routing**: Efficient event delivery to handlers
- **Handlers**: System and script-based event processing
- **Prioritization**: Event processing order and priority
- **Threading**: Thread-safe event processing

### UI Framework

- **Elements**: Custom UI widgets and containers
- **Layout**: Flexible positioning and sizing system
- **Data Binding**: Automatic UI state synchronization
- **Events**: User interaction handling and routing
- **Rendering**: Efficient UI rendering and updates