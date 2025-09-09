# Upgrade Plan: sim_simple → sim_core Architecture

## Overview
Upgrade world_sim_simple to use the modular architecture from world_sim_core, implementing step-by-step with debug validation at each stage.

## Phase 1: Foundation (Days 1-2)

### 1.1 Plugin System Architecture
- [ ] Create `plugin.rs` with trait-based plugin system
- [ ] Port `SimulationPlugin`, `WorldPlugin`, `EntityPlugin` traits
- [ ] Set up plugin registration and lifecycle
- **Debug**: Verify plugins load in terminal output
- **Test**: `RUST_LOG=debug cargo run` - see "Plugin loaded" messages

### 1.2 Component System
- [ ] Create `components/` module structure
- [ ] Port core components: `Position`, `Health`, `Inventory`, `Task`
- [ ] Add component registration system
- **Debug**: Log component creation/destruction
- **Test**: Verify ECS queries work correctly

## Phase 2: World Management (Days 3-4)

### 2.1 Chunk-Based World
- [ ] Port `tilemap/chunk.rs` - 16x16 chunk system
- [ ] Implement `ChunkManager` for dynamic loading
- [ ] Add chunk serialization/deserialization
- **Debug**: Log chunk loading/unloading events
- **Test**: Move camera, verify chunks load in terminal

### 2.2 Tilemap System
- [ ] Port `tilemap/tile.rs` with tile properties
- [ ] Add `TileRegistry` for tile types
- [ ] Implement tile state changes
- **Debug**: Log tile modifications with coordinates
- **Test**: Modify tiles, verify in debug output

## Phase 3: Entity Systems (Days 5-6)

### 3.1 Agent Management
- [ ] Port `components/agent.rs` with full agent state
- [ ] Add `AgentSpawnSystem` with configurable spawning
- [ ] Implement agent lifecycle (spawn/death/respawn)
- **Debug**: Track agent count changes
- **Test**: Spawn agents, verify in terminal stats

### 3.2 Task System
- [ ] Port `ai/task_system.rs` with task queue
- [ ] Add task priorities and dependencies
- [ ] Implement task assignment algorithm
- **Debug**: Log task assignments and completions
- **Test**: Assign tasks, watch execution in terminal

## Phase 4: AI & Behavior (Days 7-8)

### 4.1 Pathfinding
- [ ] Port A* pathfinding from `ai/pathfinding.rs`
- [ ] Add path caching and optimization
- [ ] Implement dynamic obstacle avoidance
- **Debug**: Log path calculations with node counts
- **Test**: Watch agents navigate in debug output

### 4.2 Behavior Trees
- [ ] Port `ai/behavior_tree.rs` structure
- [ ] Add condition nodes and action nodes
- [ ] Implement behavior switching
- **Debug**: Log behavior state changes
- **Test**: Verify behaviors execute correctly

## Phase 5: Resources & Economy (Days 9-10)

### 5.1 Resource System
- [ ] Port `resources/resource_manager.rs`
- [ ] Add resource types and properties
- [ ] Implement resource spawning/depletion
- **Debug**: Track resource counts by type
- **Test**: Harvest resources, verify in terminal

### 5.2 Inventory Management
- [ ] Port `components/inventory.rs` with stacking
- [ ] Add weight/volume constraints
- [ ] Implement item transfer system
- **Debug**: Log inventory changes
- **Test**: Transfer items, verify in debug

## Phase 6: Building & Construction (Days 11-12)

### 6.1 Building System
- [ ] Port `systems/building_system.rs`
- [ ] Add building blueprints and requirements
- [ ] Implement construction progress
- **Debug**: Log construction stages
- **Test**: Build structures, monitor progress

### 6.2 Crafting System
- [ ] Port `recipes.rs` with recipe registry
- [ ] Add crafting stations and tools
- [ ] Implement recipe discovery
- **Debug**: Log crafting attempts and results
- **Test**: Craft items, verify outputs

## Phase 7: Advanced Features (Days 13-14)

### 7.1 Save/Load System
- [ ] Port save/load from `data/save_system.rs`
- [ ] Add world state serialization
- [ ] Implement save versioning
- **Debug**: Log save/load operations
- **Test**: Save, restart, load, verify state

### 7.2 Scripting Integration
- [ ] Port Lua scripting from `scripting/`
- [ ] Add script hot-reloading
- [ ] Implement script API bindings
- **Debug**: Log script events and errors
- **Test**: Load scripts, verify execution

## Phase 8: Integration & Polish (Days 15-16)

### 8.1 WebSocket Updates
- [ ] Update WebSocket protocol for new features
- [ ] Add efficient state diff streaming
- [ ] Implement client state synchronization
- **Debug**: Log WebSocket message types
- **Test**: Use Playwright to verify frontend updates

### 8.2 Performance Optimization
- [ ] Profile with `RUST_LOG=trace`
- [ ] Optimize hot paths identified in debug
- [ ] Add performance metrics to debug system
- **Debug**: Monitor FPS and frame times
- **Test**: Stress test with many entities

## Debug Validation Process

For EACH component above:

1. **Write**: Implement the feature
2. **Build**: `cargo build` - fix any errors
3. **Debug**: `RUST_LOG=debug cargo run`
4. **Verify**: Check expected logs appear
5. **Test**: Run specific test scenario
6. **Monitor**: Watch terminal for issues
7. **Browser**: Use Playwright MCP if UI involved
8. **Commit**: Only after validation passes

## Terminal Debug Checkpoints

Each phase must show in terminal:
- ✅ Green INFO logs for successful operations
- ✅ No red ERROR messages
- ✅ Expected state changes logged
- ✅ Performance metrics acceptable (FPS > 30)
- ✅ Memory usage stable

## Example Debug Session

```bash
# Start with debug
$ RUST_LOG=debug cargo run

# Expected output for Phase 2.1:
[0.001] [WORLD] INFO: ChunkManager initialized
[0.002] [CHUNK] DEBUG: Loading chunk (0, 0)
[0.003] [CHUNK] INFO: Chunk (0, 0) loaded with 256 tiles
[0.010] [CHUNK] DEBUG: Unloading chunk (-2, -2)
[0.011] [MEMORY] INFO: Active chunks: 9, Memory: 2.1MB
```

## Browser Testing with Playwright

After each phase:
```javascript
// Navigate to frontend
mcp__playwright__browser_navigate("http://localhost:3000")

// Take screenshot
mcp__playwright__browser_take_screenshot("phase-X-complete.png")

// Check console
mcp__playwright__browser_console_messages()

// Verify no errors
```

## Success Criteria

Phase complete when:
1. All terminal debug output shows expected behavior
2. No compilation warnings or errors
3. Performance metrics meet targets
4. Browser tests pass (if applicable)
5. Can demonstrate feature working via debug logs

## Risk Mitigation

- Keep sim_simple working throughout upgrade
- Create feature flags for gradual rollout
- Maintain backwards compatibility
- Test each system in isolation first
- Use debug output to catch issues early

## Timeline

- **Week 1**: Phases 1-3 (Foundation & World)
- **Week 2**: Phases 4-6 (AI & Economy)
- **Week 3**: Phases 7-8 (Advanced & Polish)

Each phase validated through terminal debugging before proceeding.