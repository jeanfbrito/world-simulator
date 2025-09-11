# Task Tracking: Bevy Entitiles Feature Extraction

## Current Sprint: Phase 1-2 Pathfinding Improvements

### ✅ Completed Tasks

#### Phase 1: Tick-Based Movement
- [x] Add `Path::follow_tick()` method to `pathfinding.rs`
- [x] Convert movement from delta-time to tick-based (0.5 tiles per tick)
- [x] Maintain backward compatibility with legacy `follow()` method
- [x] Add Manhattan distance helper function

#### Phase 2: Enhanced A* Algorithm  
- [x] Separate orthogonal and diagonal neighbors
- [x] Implement proper movement costs (10 for straight, 14 for diagonal)
- [x] Add diagonal movement validation (check adjacent blocking tiles)
- [x] Add `euclidean_distance_squared()` utility function

### 🔄 In Progress

#### Testing & Validation
- [ ] Build and test with `cargo build -p world_sim_simple`
- [ ] Run with debug output: `RUST_LOG=debug cargo run -p world_sim_simple`
- [ ] Verify peasants use diagonal paths when available
- [ ] Check no performance regression
- [ ] Confirm workers reach destinations correctly

### 📋 Upcoming Tasks

#### Phase 3: Chunked Storage System
- [ ] Create `src/map/chunked_storage.rs` file
- [ ] Implement generic `ChunkedStorage<T>` struct
- [ ] Add chunk size configuration (default 16x16)
- [ ] Implement `transform_index()` for coordinate conversion
- [ ] Add efficient neighbor queries
- [ ] Create tests for chunked storage

#### Phase 3.1: Integrate Chunked Storage
- [ ] Modify `WorldMap` struct in `main.rs`
- [ ] Replace `Vec<Vec<TileType>>` with `ChunkedStorage<TileType>`
- [ ] Update all map access methods
- [ ] Add entity spatial indexing by chunk
- [ ] Test map generation with new storage

#### Phase 4: Advanced PathNode System
- [ ] Create enhanced `PathNode` struct with g_cost and h_cost
- [ ] Implement proper f_cost calculation (g + h)
- [ ] Add parent tracking for path reconstruction
- [ ] Optimize priority queue with better ordering
- [ ] Add path smoothing algorithm

#### Phase 5: Path Caching
- [ ] Create `src/ai/path_cache.rs`
- [ ] Implement LRU cache for frequently used paths
- [ ] Add cache invalidation on obstacle changes
- [ ] Add metrics for cache hit/miss rates
- [ ] Configure cache size limits

#### Phase 6: Incremental Pathfinding
- [ ] Implement `PathGrid` for step-by-step computation
- [ ] Add support for partial path computation
- [ ] Allow pathfinding over multiple ticks for long paths
- [ ] Add early termination for impossible paths

### 🚀 Future Enhancements

#### Wave Function Collapse (WFC)
- [ ] Study `bevy_entitiles/src/algorithm/wfc.rs`
- [ ] Extract WFC algorithm for procedural generation
- [ ] Create terrain generation patterns
- [ ] Implement building placement rules
- [ ] Add resource distribution patterns

#### Multi-threaded Pathfinding
- [ ] Study async pathfinding in bevy_entitiles
- [ ] Prepare data structures for parallel access
- [ ] Implement pathfinding job queue
- [ ] Add thread-safe path result handling

## Bug Tracking

### Known Issues
1. **Movement appears instant** - Need to verify tick-based movement is working
2. **Peasants may not use diagonal paths yet** - Test after compilation

### Fixed Issues
- [x] Query conflict with PositionComponent (added Without<WorkerTag> filters)
- [x] Peasants spawning at same position (added random positioning)

## Performance Benchmarks

### Baseline (Before Changes)
- Compilation time: ~5-15 minutes (normal for Bevy)
- Pathfinding: Orthogonal only
- Memory usage: Vec<Vec<TileType>> for 64x64 map

### Target Metrics
- Pathfinding: 30% faster with diagonal movement
- Memory: 20% reduction with chunked storage
- Path quality: Shorter paths with diagonal movement

## Code Quality Checklist

### Before Committing
- [ ] Code compiles without warnings
- [ ] Debug output shows expected behavior
- [ ] No performance regression
- [ ] Documentation updated
- [ ] Tests pass (when added)

### Code Review Points
- Tick-based movement consistency
- Proper error handling
- Memory efficiency
- Algorithm correctness
- Code clarity and comments

## Dependencies & References

### Bevy Entitiles Files (Reference Only)
- `/Users/jean/Github/bevy_entitiles/src/algorithm/pathfinding.rs`
- `/Users/jean/Github/bevy_entitiles/src/tilemap/chunking/storage.rs`
- `/Users/jean/Github/bevy_entitiles/src/math/ext.rs`
- `/Users/jean/Github/bevy_entitiles/src/algorithm/wfc.rs`

### Our Project Files
- `world_sim_simple/src/ai/pathfinding.rs` (modified)
- `world_sim_simple/src/main.rs` (to be modified)
- `world_sim_simple/src/map/` (to be created)

## Git Commit Messages

### Suggested Commit Pattern
```
feat(pathfinding): Add tick-based movement from bevy_entitiles

- Implement Path::follow_tick() for consistent simulation
- Add diagonal movement support with proper costs
- Extract Manhattan distance utility
- Inspired by bevy_entitiles pathfinding patterns
```

## Notes & Decisions

1. **No Direct Dependency**: We're extracting patterns, not adding bevy_entitiles as a dependency
2. **Headless Focus**: All rendering-related code is excluded
3. **Progressive Implementation**: Small, testable changes at each phase
4. **Backward Compatibility**: Keeping legacy methods during transition
5. **Performance First**: Optimizations should not compromise simulation accuracy

## Questions to Resolve

1. Should we make diagonal movement configurable per unit?
2. What's the optimal chunk size for our 64x64 map?
3. Should path caching be global or per-entity?
4. How to handle dynamic obstacles efficiently?

## Success Criteria

- [ ] Peasants move smoothly with tick-based system
- [ ] Diagonal paths are used when optimal
- [ ] No performance degradation
- [ ] Code is clean and well-documented
- [ ] All tests pass (when implemented)