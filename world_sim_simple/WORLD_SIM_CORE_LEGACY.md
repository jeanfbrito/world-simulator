# World Sim Core - Legacy Code Archive

This document preserves the valuable concepts and ideas from the deleted `world_sim_core` module.
The code never compiled but contained interesting architectural patterns worth remembering.

**Deletion Commit**: `ca66702` - "chore: Remove world_sim_core legacy code and preserve concepts"  
To access the deleted source code, use: `git checkout ca66702^ -- world_sim_core/`

## Table of Contents
1. [Level of Detail (LOD) AI System](#level-of-detail-lod-ai-system)
2. [Parallel AI Processing](#parallel-ai-processing)
3. [Squad-Based Planning](#squad-based-planning)
4. [Social Alert System](#social-alert-system)
5. [AI Coordinator (Hybrid AI)](#ai-coordinator-hybrid-ai)
6. [Wave Function Collapse World Generation](#wave-function-collapse-world-generation)
7. [Advanced Scorers](#advanced-scorers)
8. [Test Suite Structure](#test-suite-structure)

---

## 1. Level of Detail (LOD) AI System

**File**: `src/ai/lod_system.rs` (270 lines)

### Concept
Dynamically adjusts AI complexity based on entity importance and distance from player. Reduces computational load for distant/unimportant entities.

### Key Features
- **5 Complexity Levels**: Full → Reactive → Simple → Minimal → Dormant
- **Importance Scoring**: Based on distance, combat status, valuable items, tasks
- **Dynamic Update Frequency**: 0.1s (Full) to 5.0s (Dormant)
- **Staggered Updates**: Spreads entity updates across frames to avoid spikes

### Implementation Highlights
```rust
pub enum AIComplexity {
    Full,      // GOAP + Utility + all features
    Reactive,  // Utility AI only
    Simple,    // Basic scripted behaviors
    Minimal,   // Basic needs only
    Dormant,   // Almost no processing
}

// Importance factors:
// - Distance < 20: score +1.0
// - In combat: score +0.5
// - Player ally: score +0.3
// - Has valuables: score +0.2
// - Important task: score +0.2
```

### Potential Benefits
- Could handle 10,000+ entities by reducing far-away AI complexity
- Maintains illusion of full simulation while saving CPU
- Smooth transitions between complexity levels

---

## 2. Parallel AI Processing

**File**: `src/ai/parallel_processing.rs` (386 lines)

### Concept
Uses multiple CPU cores and SIMD instructions for AI computations. Batches entity processing for cache efficiency.

### Key Features
- **Multi-threaded Planning**: Distributes GOAP planning across worker threads
- **SIMD Score Evaluation**: Processes 4 entities at once using AVX instructions
- **Cache-Friendly Layout**: Aligns components to 64-byte cache lines
- **Memory Pooling**: Pre-allocated buffers to reduce allocations
- **Rayon Integration**: Parallel spatial queries and batch updates

### Implementation Highlights
```rust
// Uses 75% of CPU cores for AI
let ai_threads = (cpu_count * 0.75).max(2.0);

// Cache-aligned component for better performance
#[repr(C, align(64))]
pub struct CacheFriendlyWorkerData {
    // Hot data (frequently accessed) - 64 bytes
    pub position_x: f32,
    pub position_y: f32,
    pub hunger: f32,
    pub energy: f32,
    // ... padding to cache line
    
    // Cold data (rarely accessed) - separate cache line
    pub name: [u8; 32],
    pub settlement_id: u32,
}

// SIMD processing of 4 scores simultaneously
unsafe {
    let scores = _mm256_add_pd(weighted_hunger, weighted_fatigue);
}
```

### Performance Claims
- 4x speedup for score calculations with SIMD
- Near-linear scaling with CPU cores for planning
- Reduced cache misses with aligned data layout

---

## 3. Squad-Based Planning

**File**: `src/ai/squad_planning.rs` (358 lines)

### Concept
Groups nearby workers into squads for coordinated behavior. Leader plans, members follow.

### Key Features
- **Automatic Squad Formation**: Groups 3-5 nearby workers
- **Shared Goals**: HarvestArea, ConstructBuilding, DefendLocation, PatrolRoute
- **Formation Movement**: Line, Column, Circle, Wedge, Scatter
- **Resource Sharing**: Squad members share food with hungry teammates
- **Morale System**: Success/failure affects squad performance

### Implementation Highlights
```rust
pub enum SquadGoal {
    HarvestArea { center: Position, radius: f32 },
    ConstructBuilding { building_type, position },
    DefendLocation { position },
    PatrolRoute { waypoints: Vec<Position> },
    GatherResources { resource_type, quota: u32 },
}

pub enum Formation {
    Line,     // Side by side
    Column,   // Single file
    Circle,   // Defensive circle
    Wedge,    // Offensive formation
    Scatter,  // Loose grouping
}

// Leader makes decisions, members follow
if shared.is_leader {
    planner.current_goal = complex_goal;
} else {
    planner.current_goal = simplified_leader_goal;
}
```

### Use Cases
- Large construction projects
- Area resource gathering
- Combat formations
- Patrol routes

---

## 4. Social Alert System

**File**: `src/ai/social_alerts.rs` (200+ lines)

### Concept
Alerts (danger, opportunities) propagate through groups like real social information.

### Key Features
- **Alert Types**: Danger, Opportunity, NeedHelp, EnemySpotted, ResourceFound
- **Spatial Propagation**: Uses spatial grid for efficient range queries
- **Alert Decay**: Intensity decreases with distance and time
- **Role-Based Reception**: Warriors respond to combat, workers to resources
- **Alert Chains**: Received alerts can be re-emitted (gossip)

### Implementation Highlights
```rust
pub enum AlertType {
    Danger(f32),        // Danger level
    Opportunity(f32),   // Value
    NeedHelp,          // Worker needs assistance
    EnemySpotted,      // Combat alert
    ResourceFound,     // Valuable resource
}

// Alert intensity based on distance
let intensity = alert.intensity * (1.0 - distance / alert.range);

// Warriors amplify combat alerts
if warrior.is_some() && matches!(alert.alert_type, AlertType::EnemySpotted) {
    intensity *= 1.5;
}

// Chain propagation with decay
if spreader.should_spread {
    commands.spawn(AlertEmitter {
        intensity: received_alert.intensity * 0.8, // Decay
        range: spreader.spread_range,
    });
}
```

### Emergent Behaviors
- Panic spreads through crowds
- Resource discoveries attract workers
- Combat alerts mobilize defenders
- Help requests create rescue chains

---

## 5. AI Coordinator (Hybrid AI)

**File**: `src/ai/coordinator.rs` (150+ lines)

### Concept
Manages switching between GOAP (planning) and Utility AI (reactive) based on situation.

### Key Features
- **3 Modes**: UtilityDriven, GoalDriven, Hybrid
- **Interrupt System**: Critical needs can pause GOAP plans
- **Goal Persistence**: Resistance to interruption for important goals
- **Mode Switch Cooldown**: Prevents thrashing between systems
- **State Preservation**: Can resume GOAP plans after interruption

### Implementation Highlights
```rust
pub enum AIMode {
    UtilityDriven,  // Critical needs handling
    GoalDriven,     // Executing planned goals
    Hybrid,         // Both active, utility can interrupt
}

pub struct AICoordinator {
    pub mode: AIMode,
    pub interrupt_threshold: f32,    // When utility overrides GOAP
    pub goal_persistence: f32,        // How strongly GOAP resists
    pub min_switch_interval: f32,     // Prevent rapid switching
    pub stored_goal: Option<String>,  // Resume after interrupt
}

// Switching logic
if utility_score > coordinator.interrupt_threshold {
    if goap_importance < coordinator.goal_persistence {
        // Pause GOAP, switch to Utility
        coordinator.stored_goal = current_goal;
        coordinator.mode = AIMode::UtilityDriven;
    }
}
```

### Benefits
- Best of both AI systems
- Handles emergencies without losing long-term goals
- Smooth transitions between planning and reacting

---

## 6. Wave Function Collapse World Generation

**File**: `src/tilemap/world_generation.rs` (200+ lines)

### Concept
Uses Wave Function Collapse algorithm for procedural world generation with coherent biomes.

### Key Features
- **Biome Types**: Forest, Mountain, Plains, Desert, Mixed
- **WFC Rules**: Tile adjacency constraints for realistic terrain
- **Resource Placement**: Biome-appropriate resource distribution
- **Starting Structures**: Automatic placement of initial buildings

### Implementation Highlights
```rust
// Adjacency rules for Forest biome
rules.add_adjacency(TileType::Grass, TileType::Grass, 1.0);
rules.add_adjacency(TileType::Grass, TileType::Tree, 0.7);
rules.add_adjacency(TileType::Tree, TileType::Tree, 0.5);
rules.add_adjacency(TileType::Grass, TileType::Water, 0.2);

// Mountain biome prefers stone clustering
rules.add_adjacency(TileType::Stone, TileType::Stone, 1.0);
rules.add_adjacency(TileType::Stone, TileType::OreNode, 0.4);

// Rivers and lakes form naturally
rules.add_adjacency(TileType::Water, TileType::Water, 0.9);
```

### Advantages over Simple Random
- Coherent biomes instead of random noise
- Natural-looking terrain features
- Controllable through rule weights
- Guaranteed solvability

---

## 7. Advanced Scorers

**Files**: `src/ai/scorers/` (economic.rs, social.rs, survival.rs)

### Survival Scorers
- `HungerScorer`: Urgency increases exponentially above 70%
- `FatigueScorer`: Considers time of day and recent work
- `HealthScorer`: Factors in injuries and illness
- `ThreatScorer`: Distance-based danger evaluation
- `ShelterScorer`: Weather and time of day considerations

### Economic Scorers
- `ResourceValueScorer`: Market prices and scarcity
- `EfficiencyScorer`: Tool quality and skill level
- `OpportunityScorer`: Nearby valuable resources
- `TradeScorer`: Profit margins and relationships

### Social Scorers
- `LonelinessScorer`: Time since social interaction
- `MoraleScorer`: Recent successes/failures
- `RelationshipScorer`: Helping friends vs strangers
- `LeadershipScorer`: Following vs leading tendencies

---

## 8. Test Suite Structure

**Files**: `tests/*.rs` (8 test files)

### Test Coverage Areas
1. **determinism_test.rs**: Ensures same seed = same world
2. **engine_test.rs**: Core engine functionality
3. **harvest_system_test.rs**: Resource gathering mechanics
4. **building_system_test.rs**: Construction and ownership
5. **recipe_system_test.rs**: Crafting chains
6. **population_test.rs**: Birth, death, growth
7. **world_generation_test.rs**: Terrain generation
8. **event_emission_test.rs**: Event system integrity

### Testing Patterns
```rust
// Test determinism
for _ in 0..3 {
    let engine = SimulationEngine::new();
    engine.new_world(same_config);
    snapshots.push(engine.snapshot());
}
assert_all_equal!(snapshots);

// Test system isolation
#[test]
fn harvest_reduces_resource() {
    // Setup world with known state
    // Execute harvest
    // Verify resource depleted
    // Verify inventory increased
}
```

---

## Other Notable Features

### Data Files
- `src/data/recipes.json`: Crafting recipes (wood→planks, wheat→bread)
- `src/data/buildings.json`: Building definitions and requirements
- `src/data/resources.json`: Resource types and properties

### Performance Optimizations
- Spatial indexing for range queries
- Component batching for cache efficiency
- Lazy evaluation of expensive calculations
- Task pooling for async operations

### Scripting Integration
- Lua API bindings (never fully implemented)
- Hot-reloading support planned
- Recipe definitions in Lua
- AI behavior scripts

---

## Lessons Learned

### What Didn't Work
- **bevy_entitiles integration**: Too complex, version conflicts
- **Bevy 0.14**: Already outdated, world_sim_simple uses 0.16
- **Over-engineering**: Too many systems before basics worked
- **LuaJIT complications**: Build issues on different platforms

### What Could Be Valuable
- LOD system for massive entity counts
- Squad coordination for group tasks
- Social alerts for emergent behaviors
- Parallel processing for performance
- WFC for better world generation

### Recommendations for world_sim_simple
1. **Start Simple**: Get basics working before advanced features
2. **Incremental Optimization**: Add parallel processing only when needed
3. **Measure First**: Profile before optimizing
4. **Test Early**: TDD approach from world_sim_core was good
5. **Avoid External Complexity**: Minimize complex dependencies

---

## Conclusion

World_sim_core represented an ambitious but ultimately unsuccessful attempt at a highly optimized simulation engine. While it never compiled, the architectural patterns and optimization strategies provide valuable reference material for future enhancements to world_sim_simple.

The key lesson: **Working code beats perfect architecture every time.**

When world_sim_simple needs any of these features, refer back to this document and the git history for inspiration, but implement them incrementally with a focus on maintaining a working system at each step.
