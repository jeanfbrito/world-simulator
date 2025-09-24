# AI Demonstration Example

This example showcases advanced AI behaviors and decision-making capabilities in the world-simulator. It demonstrates multiple AI systems working together to create complex, emergent behaviors.

## What This Example Shows

### AI Systems Demonstrated

- **GOAP (Goal-Oriented Action Planning)**: Agents create and execute plans to achieve goals
- **Utility AI**: Agents select behaviors based on weighted needs and environmental conditions
- **State Machines**: Agents transition between behavioral states based on conditions
- **Multi-Agent Coordination**: Multiple AI agents work together and compete for resources
- **Learning and Adaptation**: Agents adapt their behavior based on experience

### Scenarios Included

1. **Resource Gathering**: Agents compete and cooperate for limited resources
2. **Building Construction**: Coordinated construction projects with multiple agents
3. **Survival Challenge**: Agents must manage hunger, energy, and safety
4. **Territory Control**: Agents compete for control of strategic locations

## Features

### AI Agent Types

- **Strategic Planners**: Use GOAP for long-term goal achievement
- **Reactive Agents**: Use Utility AI for dynamic response to conditions
- **Behavioral Agents**: Use State Machines for predictable, patterned behavior
- **Hybrid Agents**: Combine multiple AI systems for complex decision making

### Advanced Behaviors

- **Dynamic Goal Prioritization**: Agents adjust goals based on changing conditions
- **Resource Competition**: Multiple agents compete for limited resources
- **Cooperative Behavior**: Agents work together to achieve common goals
- **Adaptive Learning**: Agents learn from experience and improve strategies
- **Emergent Behavior**: Complex behaviors arise from simple AI rules

## Configuration

The AI demonstration uses specialized configuration through Lua files and Rust code:

- **AI Parameters**: Personality traits, needs, and behavioral weights
- **World Setup**: Resource distribution and environmental challenges
- **Agent Configuration**: Different AI types and strategies
- **Scenario Parameters**: Goals, success conditions, and time limits

## Running the Example

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

### From the Project Root

```bash
# Run the AI demonstration
cargo run --example ai_demo
```

### From the Example Directory

```bash
cd examples/ai_demo
cargo run --bin ai_demo
```

## Expected Output

The demonstration will run for 800 ticks and display detailed AI behavior analysis:

```
🤖 Starting AI Demonstration Example
🧠 AI demonstration initialized. Running for 800 ticks...
📊 This will showcase various AI behaviors and decision-making patterns.
🎮 AI demonstration started. Monitoring agent behaviors...
📈 Watching for emergent behaviors and decision patterns...

🧠 Tick 100: 12 AI agents (GOAP: 4, Utility: 5, State Machines: 3)
   🎯 StrategicPlanner_1: Active plan with 5 actions - 3 goals
   ⚖️  ReactiveAgent_2: gather_wood - 8 behaviors
   🔄 BehavioralAgent_1: moving state - 4 transitions available

🔍 Deep AI Analysis at tick 200
   📊 GOAP Planning: 75.0% efficiency, 2.5 avg goals
   📊 Utility AI: 80.0% activity, 45 total behaviors
   📊 State Machines: 3.2 avg states, 5 unique current states
   🔮 Analysis complete - watching for emergent behaviors...

🎉 AI demonstration completed successfully!
📊 Final AI Analysis:
   • Total AI agents: 12
   • GOAP planners: 4
   • Utility AI agents: 5
   • State machine agents: 3
   • Simulation ticks: 800
🧠 Key AI Behaviors Observed:
   • Goal-oriented planning and execution
   • Dynamic behavior selection based on needs
   • State transitions and condition handling
   • Multi-agent coordination and cooperation
```

## Key Components

### Main Simulation (`main.rs`)

- Initializes the Bevy app with AI-specific plugins
- Sets up comprehensive monitoring and analysis systems
- Provides detailed AI behavior tracking and insights
- Demonstrates multiple AI systems working together

### AI Scenarios (`scenarios/`)

- **Resource Gathering**: Competitive and cooperative gathering behaviors
- **Building Construction**: Coordinated multi-agent construction projects
- **Survival Challenge**: Agents managing multiple needs simultaneously
- **Territory Control**: Strategic competition and defense behaviors

### Monitoring Systems

- **Real-time AI Tracking**: Monitors individual agent decisions and behaviors
- **Performance Analysis**: Tracks AI efficiency and success rates
- **Emergent Behavior Detection**: Identifies complex behaviors from simple rules
- **Comparative Analysis**: Compares different AI approaches and strategies

## Customization

### Add New AI Behaviors

Create new AI actions and behaviors:

```rust
// Add new GOAP action
let new_action = GoapAction {
    name: "specialized_behavior".to_string(),
    cost: 1.0,
    preconditions: vec![("has_tool".to_string(), true)],
    effects: vec![("task_completed".to_string(), true)],
    duration: 15,
};

// Add new Utility behavior
let new_behavior = UtilityBehavior {
    name: "adaptive_response".to_string(),
    utility_score: 0.7,
    considerations: vec![],
    weight: 1.0,
};
```

### Modify AI Parameters

Adjust AI decision-making parameters:

```rust
// Change personality traits
ai.personality = "aggressive"; // or "defensive", "balanced"

// Adjust needs weights
agent.needs.insert("hunger".to_string(), 0.9);
agent.needs.insert("safety".to_string(), 0.7);

// Modify behavior weights
behavior.weight = 1.5; // Higher priority
```

### Create Custom Scenarios

Design new AI challenge scenarios:

```rust
pub fn setup_custom_scenario(commands: &mut Commands) {
    // Create environmental challenges
    // Spawn specialized agents
    // Set up success conditions
    // Configure monitoring systems
}
```

## Learning Points

This example teaches you:

1. **Multiple AI Systems**: How different AI approaches (GOAP, Utility, State Machines) can work together
2. **Agent Coordination**: How multiple AI agents can cooperate and compete
3. **Emergent Behavior**: How complex behaviors arise from simple AI rules
4. **Performance Analysis**: How to measure and analyze AI effectiveness
5. **Adaptive Systems**: How AI agents can learn and adapt over time

## Advanced Concepts

### GOAP Planning

- Goal-oriented behavior with action planning
- Dynamic plan creation and execution
- Precondition and effect handling
- Plan failure and recovery

### Utility AI

- Weighted decision making
- Need-based behavior selection
- Environmental awareness
- Dynamic utility calculation

### State Machines

- Behavioral state management
- Condition-based transitions
- State-specific actions
- Complex behavioral patterns

## Next Steps

After understanding this AI demonstration, you can explore:

- **Custom World Example**: Procedural generation and world customization
- **WebSocket Client Example**: Real-time AI monitoring and control
- **Performance Testing**: AI system optimization and benchmarking
- **Modding Example**: Creating custom AI behaviors and scenarios

## Troubleshooting

### Common Issues

**AI agents not making decisions**: Check that AI plugins are properly initialized and components are configured.

**Poor AI performance**: Adjust AI parameters, weights, and action costs to improve decision making.

**Agents getting stuck**: Verify that preconditions and effects are properly defined and achievable.

**Memory issues**: Reduce the number of AI agents or optimize AI data structures.

### Getting Help

If you encounter issues:

1. Check the detailed AI monitoring output for behavior insights
2. Verify AI component configuration and initialization
3. Adjust logging level to debug for detailed AI decision traces
4. Review the AI system documentation for implementation details

## Contributing

This example is part of the world-simulator project. To contribute improvements or new AI scenarios:

1. Test new AI behaviors thoroughly
2. Ensure compatibility with existing AI systems
3. Add comprehensive monitoring and analysis
4. Document new AI features and behaviors

## Performance Considerations

- AI computation complexity increases with agent count
- GOAP planning can be CPU-intensive with many actions
- Utility AI requires frequent utility calculations
- State machines are generally the most efficient
- Consider agent LOD (Level of Detail) for large simulations

For more detailed information about AI systems in the world-simulator, see the main project documentation.