# Architecture Clarification: Game Engine vs Web Service

## The Fundamental Difference

### ❌ Web Service Architecture (Wrong for Game Engine)
```
Client → HTTP/WebSocket → Server → Database → Server → Client
         (100-500ms)       (5-50ms)  (5-50ms)
```
- Total latency: 110-600ms per action
- Database becomes bottleneck
- Not suitable for real-time simulation

### ✅ Game Engine Architecture (Correct Approach)
```
Client ← Binary Protocol → Headless Engine (In-Memory ECS)
         (<1ms locally)     All state in RAM, no DB calls
         (<50ms network)    
```
- Tick-based deterministic simulation
- State replication, not request/response
- Database only for saves/loads

## Why Bevy/Rust is the Right Choice

### Performance Requirements
- **10,000+ entities**: Needs zero-cost abstractions (Rust)
- **60 ticks/second**: Needs predictable performance (no GC)
- **Deterministic sim**: Needs precise memory control
- **Multiplayer sync**: Needs efficient binary serialization

### Bevy ECS Benefits
```rust
// Efficient component iteration
fn harvest_system(
    mut workers: Query<(&mut Worker, &Position)>,
    mut resources: Query<(&mut ResourceNode, &Position)>,
) {
    // Processes thousands of entities in microseconds
    // Cache-friendly memory layout
    // Automatic parallelization
}
```

## Corrected Technology Stack

### Core Engine (Rust/Bevy)
- **bevy**: ECS framework (not a web framework!)
- **bevy_renet**: UDP networking for games
- **bevy_rapier**: Physics if needed
- **rmp-serde**: MessagePack for binary serialization

### Client Connections
- **Native**: Direct Bevy client
- **Web**: WebAssembly build + WebSockets
- **Unity/Unreal**: Binary protocol bridge

### Storage Strategy
```rust
// Runtime: Everything in ECS
world.spawn((
    Position { x: 10, y: 20 },
    ResourceNode { 
        resource_type: ResourceType::Tree,
        quantity: 100,
    },
));

// Saves: Serialize world to SQLite/binary
fn save_game(world: &World) {
    let snapshot = world.serialize();
    // Write to SQLite or .save file
}
```

## Common Misconceptions

### "Games need databases for multiplayer"
**FALSE**: Games use authoritative servers with in-memory state. Databases are for:
- Account management (separate service)
- Leaderboards (separate service)  
- Save files (not runtime)

### "Redis is good for game state"
**FALSE**: Network latency kills performance. Games need:
- Nanosecond component access
- Cache-coherent data structures
- Zero serialization overhead during play

### "PostgreSQL for game entities"
**FALSE**: SQL queries during gameplay are absurd. Real games:
- Process 10,000+ entities per frame
- Need spatial queries (quadtrees, not SQL)
- Require deterministic ordering

## Examples from Real Game Engines

### Unity (C#)
```csharp
// Everything in memory via MonoBehaviours
GameObject tree = Instantiate(treePrefab);
tree.GetComponent<ResourceNode>().quantity = 100;
// No database involved!
```

### Godot (GDScript)
```gdscript
# Nodes in scene tree (memory)
var tree = ResourceNode.new()
tree.quantity = 100
add_child(tree)
# Database? Never during gameplay
```

### Unreal (C++)
```cpp
// Actors and components in memory
AResourceNode* Tree = GetWorld()->SpawnActor<AResourceNode>();
Tree->Quantity = 100;
// Database would destroy performance
```

## Migration Path from Web Mindset

### Phase 1: Understand Game Loops
```rust
// Game engines work in ticks, not requests
fn main() {
    App::new()
        .add_systems(Update, (
            input_system,
            movement_system,
            harvest_system,
            render_system,
        ))
        .run(); // Runs 60+ times per second
}
```

### Phase 2: Learn ECS Patterns
- Entities: Just IDs
- Components: Pure data
- Systems: Pure logic
- Resources: Shared state

### Phase 3: Implement Networking
- Snapshot interpolation
- Client prediction
- Rollback/reconciliation
- Delta compression

## Conclusion

This is a **game engine**, not a web application. It needs:
- Microsecond performance, not millisecond
- Deterministic simulation, not eventual consistency  
- Binary protocols, not JSON/HTTP
- In-memory ECS, not database queries

The original spec correctly identified Rust/Bevy. We should follow game engine patterns, not web service patterns. PostgreSQL and Redis have no place in the runtime of a game engine - they're only useful for auxiliary services like accounts, leaderboards, and save files.