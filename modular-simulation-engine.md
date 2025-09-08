# Modular Simulation Engine Technical Specification

## Executive Summary

This document outlines the technical specifications for a modular, headless simulation engine designed to power medieval economy and fortress simulation games. The system is architected as a decoupled backend simulation engine that can interface with multiple frontend presentation layers (web UI, Unity3D, Unreal Engine) through well-defined APIs.

## System Architecture Overview

### Core Design Principles

- **Headless Simulation Backend**: Game logic and simulation run independently of visualization
- **Entity-Component-System (ECS) Architecture**: Data-oriented design for optimal performance and scalability
- **Recipe-Based Everything**: All crafting, construction, and resource processing follows uniform recipe patterns
- **Modular Design**: Clear separation of concerns allowing easy extension and modification
- **API-First Approach**: Frontend-agnostic communication through standardized interfaces

### High-Level Architecture

```
+-------------------------------------------------------------+
|                    Frontend Layer                           |
|  +-------------+ +-------------+ +-----------------------+ |
|  |   Web UI    | |   Unity3D   | |    Unreal Engine      | |
|  |  (Simple)   | |  (Advanced) | |     (Advanced)        | |
|  +-------------+ +-------------+ +-----------------------+ |
+-------------------------------------------------------------+
                            |
                    +---------------+
                    |  API Gateway  |
                    | (REST/gRPC/   |
                    |  WebSocket)   |
                    +---------------+
                            |
+-------------------------------------------------------------+
|                 Simulation Backend                          |
|  +-------------------------------------------------------+  |
|  |              ECS Engine Core                          |  |
|  +-------------------------------------------------------+  |
|  +-------------+ +-------------+ +-----------------------+ |
|  |  Resource   | | Population  | |    Recipe & Crafting  | |
|  |   System    | |   System    | |        System         | |
|  +-------------+ +-------------+ +-----------------------+ |
|  +-------------+ +-------------+ +-----------------------+ |
|  |   Trade &   | |  Building   | |      Event System     | |
|  |  Logistics  | |   System    | |                       | |
|  +-------------+ +-------------+ +-----------------------+ |
+-------------------------------------------------------------+
                            |
                    +---------------+
                    |   Data Layer  |
                    | (PostgreSQL + |
                    |    Redis)     |
                    +---------------+
```

## Technology Stack

### Backend Core
- **Language**: Rust (for memory safety, performance, and concurrency)
- **Game Engine**: Bevy Engine (Rust-based ECS framework)
- **Database**: PostgreSQL for persistent data, Redis for caching
- **API**: gRPC for high-performance communication, WebSocket for real-time updates
- **Build System**: Cargo (Rust package manager)

### Frontend Options
- **MVP/Simple**: Web-based dashboard (React/Vue + WebSocket)
- **Advanced**: Unity3D or Unreal Engine 5 integration
- **Cross-platform**: Support for multiple simultaneous frontend clients

### Development Tools
- **Version Control**: Git with CI/CD pipelines
- **Testing**: Automated unit and integration testing
- **Documentation**: API documentation with OpenAPI/Swagger
- **Monitoring**: Performance profiling and logging systems

## Core Systems Design

### 1. Entity-Component-System (ECS) Architecture

#### Components
```rust
// Example component definitions
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

struct ResourceNode {
    resource_type: ResourceType,
    remaining_quantity: u32,
    regeneration_rate: f32,
}

struct Worker {
    assigned_task: Option<TaskType>,
    efficiency: f32,
    skills: HashMap<SkillType, u32>,
}

struct Inventory {
    items: HashMap<ResourceType, u32>,
    capacity: u32,
}
```

#### Systems
- **Resource Collection System**: Handles worker interactions with resource nodes
- **Production System**: Processes crafting recipes and building construction
- **Logistics System**: Manages resource transportation and storage
- **Population System**: Tracks worker assignments and population dynamics
- **Event System**: Handles random events and triggers

### 2. Recipe-Based Resource System

#### Recipe Data Structure
```json
{
  "recipe_id": "house_basic",
  "name": "Basic House",
  "type": "construction",
  "inputs": [
    {"resource": "wood", "quantity": 5},
    {"resource": "stone", "quantity": 2}
  ],
  "outputs": [
    {"entity": "house", "quantity": 1}
  ],
  "requirements": {
    "tools": ["hammer"],
    "skills": {"construction": 1}
  },
  "processing_time": 120
}
```

#### Resource Drop Tables
```json
{
  "entity_id": "oak_tree",
  "name": "Oak Tree",
  "interaction_type": "chop",
  "requirements": {
    "tools": ["axe"]
  },
  "drop_table": [
    {"resource": "wood", "quantity": 3, "chance": 100.0},
    {"resource": "oak_seed", "quantity": 1, "chance": 15.0},
    {"resource": "rare_resin", "quantity": 1, "chance": 2.0}
  ]
}
```

### 3. World Entity System

#### Entity Definition Schema
```json
{
  "entity_type": "gold_vein",
  "name": "Gold Vein",
  "category": "mineral_deposit",
  "interaction_actions": ["mine"],
  "prerequisites": {
    "tools": ["iron_pickaxe", "steel_pickaxe"],
    "skills": {"mining": 3}
  },
  "resource_drops": [
    {"resource": "gold_ore", "min_quantity": 1, "max_quantity": 3, "chance": 90.0},
    {"resource": "precious_gems", "quantity": 1, "chance": 5.0}
  ],
  "depletion": {
    "total_resources": 50,
    "regeneration_rate": 0
  }
}
```

### 4. Population and Labor System

#### Worker Assignment Logic
- Workers are entities with skill components and task assignments
- Dynamic task allocation based on priority, skills, and availability
- Efficiency modifiers based on tools, skills, and happiness levels

#### Happiness System (Stronghold-inspired)
```json
{
  "happiness_factors": {
    "food_variety": {"weight": 0.3, "sources": ["bread", "meat", "cheese"]},
    "housing_quality": {"weight": 0.2, "calculation": "houses_per_population"},
    "taxation": {"weight": -0.4, "calculation": "tax_rate"},
    "safety": {"weight": 0.1, "factors": ["walls", "guards"]}
  }
}
```

### 5. API and Communication Layer

#### REST API Endpoints
```
GET    /api/world/state           # Current world state snapshot
GET    /api/entities/{id}         # Specific entity details
POST   /api/commands/build        # Issue construction command
POST   /api/commands/assign       # Assign worker to task
GET    /api/resources/inventory   # Current resource inventory
GET    /api/population/status     # Population and happiness metrics
```

#### WebSocket Events
```json
{
  "event_type": "resource_collected",
  "timestamp": "2025-09-08T13:16:00Z",
  "data": {
    "worker_id": 123,
    "resource": "wood",
    "quantity": 3,
    "location": {"x": 45, "y": 78}
  }
}
```

## Development Phases

### Phase 1: MVP (6-16 weeks, $30,000-$150,000)
- Core ECS architecture implementation
- Basic resource collection and recipe system
- Simple web-based frontend for visualization
- Core API endpoints for state queries and commands

#### MVP Scope
- 5-10 basic resource types (wood, stone, food, iron, gold)
- 3-5 building types (house, storage, production buildings)
- Basic worker assignment system
- Simple web dashboard showing resource flows and population

### Phase 2: Stronghold-like Features
- Advanced building construction system
- Happiness and population dynamics
- Trade and logistics improvements
- Enhanced frontend visualization options

### Phase 3: Complex Simulation Features
- Multi-tier crafting chains
- Environmental factors and seasons
- Dynamic events and challenges
- Advanced AI behaviors

### Phase 4: Dwarf Fortress-level Complexity
- Complex material systems with qualities
- Advanced skill and technology trees
- Sophisticated world generation
- Deep social and political simulations

## Performance Considerations

### Multi-threading Architecture
- Job-based parallelism for scalable performance
- Separate threads for simulation logic and API serving
- Lock-free data structures for entity access patterns

### Data Optimization
- Component arrays for cache-friendly entity processing
- Spatial partitioning (octrees/quadtrees) for efficient spatial queries
- Optimized serialization for network communication

### Scalability Targets
- Support for 1,000+ entities in MVP
- Target 10,000+ entities for full implementation
- Sub-100ms response times for API calls
- Real-time updates with minimal latency

## Integration Patterns

### Frontend Integration Examples

#### Unity3D Integration
```csharp
public class SimulationClient : MonoBehaviour {
    private WebSocket ws;
    
    void Start() {
        // Connect to simulation backend
        ws = new WebSocket("ws://localhost:8080/ws");
        ws.OnMessage += OnSimulationUpdate;
    }
    
    void OnSimulationUpdate(string data) {
        // Parse and apply state changes to Unity scene
        var update = JsonUtility.FromJson<StateUpdate>(data);
        UpdateGameObjects(update);
    }
}
```

#### Web Frontend Integration
```javascript
// React component for real-time resource display
const ResourceMonitor = () => {
    const [resources, setResources] = useState({});
    
    useEffect(() => {
        const ws = new WebSocket('ws://localhost:8080/ws');
        ws.onmessage = (event) => {
            const update = JSON.parse(event.data);
            if (update.type === 'resource_update') {
                setResources(update.data);
            }
        };
    }, []);
    
    return (
        <div>
            {Object.entries(resources).map(([type, quantity]) => (
                <div key={type}>{type}: {quantity}</div>
            ))}
        </div>
    );
};
```

## Quality Assurance and Testing

### Testing Strategy
- Unit tests for all core systems
- Integration tests for API endpoints
- Performance benchmarks for simulation scale
- Automated testing in CI/CD pipeline

### Monitoring and Analytics
- Real-time performance metrics
- Error tracking and logging
- Player behavior analytics (if applicable)
- Resource usage monitoring

## Security and Data Management

### Data Protection
- Secure API authentication (JWT tokens)
- Input validation and sanitization
- Rate limiting for API endpoints
- Encrypted data transmission

### Save Game Management
- Versioned save format for backward compatibility
- Incremental save system for large worlds
- Backup and recovery mechanisms
- Export/import functionality for modding

## Modding and Extensibility

### Modding Support
- Lua scripting integration for game logic modification
- Hot-reloading for rapid iteration during development
- Plugin architecture for community extensions
- Comprehensive API documentation for modders

### Content Creation Tools
- Recipe editor for custom crafting systems
- Entity definition tools
- Event scripting interface
- Resource and building customization

## Conclusion

This modular simulation engine provides a robust foundation for creating complex economic and fortress simulation games. By starting with a Stronghold-inspired economy and building toward Dwarf Fortress-level complexity, the system balances immediate playability with long-term extensibility.

The recipe-based, data-driven architecture ensures that content creation and game balancing can be performed without deep technical knowledge, while the modular design allows for incremental complexity increases and easy integration with various frontend technologies.