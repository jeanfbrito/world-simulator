# Bevy Entitiles Feature Extraction Documentation

## Overview
This document tracks the progressive extraction and adaptation of useful features from the bevy_entitiles crate (`/Users/jean/Github/bevy_entitiles`) for our headless world simulation. We're only extracting non-rendering components that enhance our simulation.

## Knowledge Source
- **Repository**: `/Users/jean/Github/bevy_entitiles` (used as reference, not dependency)
- **Version**: Updated to Bevy 0.16
- **Purpose**: Extract algorithms and patterns for headless simulation

## Implementation Phases

### Phase 1: Tick-Based Movement ✅ COMPLETED
**Status**: Completed
**Files Modified**: 
- `src/ai/pathfinding.rs`

**Changes Made**:
1. Added `Path::follow_tick()` method for tick-based movement
   - Uses `tiles_per_tick` parameter instead of delta time
   - Consistent with our 10 TPS simulation
2. Added Manhattan distance helper function
   - Cleaner distance calculations
   - Foundation for better heuristics

### Phase 2: Enhanced A* Pathfinding ✅ COMPLETED  
**Status**: Completed
**Files Modified**:
- `src/ai/pathfinding.rs`

**Changes Made**:
1. Improved A* algorithm with proper diagonal movement
   - Separate orthogonal (cost: 10) and diagonal (cost: 14) neighbors
   - Diagonal movement validation (checks for blocking adjacent tiles)
2. Added utility functions:
   - `manhattan_distance()` - for heuristic calculations
   - `euclidean_distance_squared()` - for future use

**Inspired by**: `bevy_entitiles/src/algorithm/pathfinding.rs`

### Phase 3: Chunked Storage System 🔄 PENDING
**Status**: Not Started
**Target Files**:
- Create: `src/map/chunked_storage.rs`
- Modify: `src/main.rs` (WorldMap struct)

**Planned Features**:
1. ChunkedStorage<T> generic structure
   - 16x16 chunk size (optimal for 64x64 map)
   - HashMap-based chunk storage
   - Efficient neighbor queries
2. Replace `Vec<Vec<TileType>>` with chunked storage
3. Add spatial indexing for entities

**Reference**: `bevy_entitiles/src/tilemap/chunking/storage.rs`

### Phase 4: Advanced PathNode System 🔄 PENDING
**Status**: Not Started
**Target Files**:
- `src/ai/pathfinding.rs`

**Planned Features**:
1. Enhanced PathNode struct with:
   - g_cost: actual cost from start
   - h_cost: heuristic cost to goal
   - parent tracking for path reconstruction
2. Better priority queue management
3. Path smoothing for more natural movement

**Reference**: `bevy_entitiles/src/algorithm/pathfinding.rs` (PathNode struct)

### Phase 5: Path Caching & Optimization 🔄 PENDING
**Status**: Not Started
**Target Files**:
- Create: `src/ai/path_cache.rs`

**Planned Features**:
1. Cache frequently used paths
2. Incremental pathfinding for dynamic obstacles
3. Multi-threaded pathfinding preparation (single-threaded execution)

## Key Algorithms Extracted

### 1. Manhattan Distance
```rust
fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
```

### 2. Diagonal Movement Validation
```rust
// Check if diagonal movement is blocked by adjacent tiles
let blocked_x = obstacles.contains(&(position.0 + dx, position.1));
let blocked_y = obstacles.contains(&(position.0, position.1 + dy));
if blocked_x || blocked_y {
    continue; // Can't move diagonally if adjacent tiles are blocked
}
```

### 3. Tick-Based Movement
```rust
pub fn follow_tick(&mut self, transform: &mut Transform, tiles_per_tick: f32) -> bool {
    // Move by fixed amount per tick
    let move_distance = tiles_per_tick * 10.0; // Convert tiles to world units
    // ... movement logic
}
```

## Testing Checklist

- [x] Tick-based movement compiles
- [ ] Tick-based movement tested in simulation
- [x] Diagonal pathfinding compiles
- [ ] Diagonal pathfinding produces shorter paths
- [ ] No performance regression
- [ ] Workers reach destinations correctly

## Performance Metrics

### Before Extraction
- Pathfinding time: TBD
- Memory usage: TBD
- Path quality: Direct or orthogonal only

### After Phase 1-2
- Pathfinding time: TBD
- Memory usage: TBD
- Path quality: Diagonal movement supported

### Expected After All Phases
- Pathfinding time: -30% (chunked storage)
- Memory usage: -20% (sparse storage)
- Path quality: Optimal with caching

## Next Steps

1. **Immediate**: Test current changes with debug output
2. **Short-term**: Implement chunked storage for better performance
3. **Medium-term**: Add path caching for repeated destinations
4. **Long-term**: Consider Wave Function Collapse for world generation

## Code References

### Bevy Entitiles Files Studied
- `/src/algorithm/pathfinding.rs` - A* implementation
- `/src/tilemap/chunking/storage.rs` - Chunked storage pattern
- `/src/math/ext.rs` - Math utilities
- `/src/algorithm/wfc.rs` - Wave Function Collapse

### Our Modified Files
- `world_sim_simple/src/ai/pathfinding.rs` - Enhanced with tick-based and diagonal movement
- Future: `world_sim_simple/src/map/chunked_storage.rs`

## Notes

- We're NOT adding bevy_entitiles as a dependency
- We're extracting patterns and adapting them for headless simulation
- All rendering-related code is excluded
- Focus on performance and simulation accuracy

## Dependencies to Avoid
- Render pipelines
- Material systems
- Texture handling
- Camera systems
- Any GPU-related code

## Safe to Extract
- Algorithms (pathfinding, WFC)
- Data structures (chunked storage)
- Math utilities
- Coordinate systems
- Non-rendering tile management