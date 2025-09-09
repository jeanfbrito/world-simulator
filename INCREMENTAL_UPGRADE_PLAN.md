# Incremental Upgrade Plan: Enhancing sim_simple Step-by-Step

## Decision: Enhance sim_simple incrementally
After analysis, enhancing sim_simple by porting features from sim_core is the better approach because:
- sim_simple is working and we have full control
- sim_core is complex with many dependencies (LuaJIT, etc.)
- We can test each feature addition with our debug system
- We maintain a working simulation at every step

## Implementation Strategy
Each step must be:
1. Small and focused (1-2 hours of work)
2. Testable with terminal debug
3. Non-breaking to existing functionality
4. Validated before moving to next step

## Phase 1: Core Architecture (Day 1)

### Step 1.1: Add Component Registry (30 min)
```rust
// Create components/mod.rs
// Port: PositionComponent, HealthComponent from sim_core
```
- [ ] Create `src/components/mod.rs`
- [ ] Add `PositionComponent` with x, y, z
- [ ] Add `HealthComponent` with current/max
- [ ] Build and run: `RUST_LOG=debug cargo run`
- [ ] Verify: "Component registered" in green

### Step 1.2: Add Basic Plugin System (45 min)
```rust
// Create plugin.rs
// Simple trait: init(), update(), cleanup()
```
- [ ] Create `src/plugin.rs` with `SimulationPlugin` trait
- [ ] Add `PluginManager` to manage plugins
- [ ] Convert existing systems to plugins
- [ ] Debug output: "Plugin loaded: WorldPlugin"
- [ ] Test: All existing features still work

### Step 1.3: Refactor Entity System (45 min)
- [ ] Replace `Worker` struct with components
- [ ] Use `PositionComponent` instead of TileEntity x,y
- [ ] Add `NameComponent` for worker names
- [ ] Debug: Log entity creation with components
- [ ] Verify: Workers still move and display

## Phase 2: World Improvements (Day 2)

### Step 2.1: Chunk System Foundation (1 hour)
```rust
// Create tilemap/chunk.rs
// 16x16 tiles per chunk
```
- [ ] Create `src/tilemap/mod.rs` and `chunk.rs`
- [ ] Define `Chunk` struct with 16x16 tiles
- [ ] Add `ChunkCoordinate` type
- [ ] Debug: "Chunk (0,0) initialized"
- [ ] Test: World still renders correctly

### Step 2.2: Convert World to Chunks (1 hour)
- [ ] Replace flat tile array with chunk grid
- [ ] Update tile access methods
- [ ] Maintain backward compatibility
- [ ] Debug: "World: 4x4 chunks (64x64 tiles)"
- [ ] Verify: Tile interactions still work

### Step 2.3: Add Chunk Loading/Unloading (45 min)
- [ ] Track active chunks around camera
- [ ] Load chunks within radius
- [ ] Unload distant chunks
- [ ] Debug: "Chunk (1,0) loaded", "Chunk (-2,-2) unloaded"
- [ ] Test: Move camera, see chunk messages

## Phase 3: Agent Intelligence (Day 3)

### Step 3.1: Task Component (30 min)
```rust
// Port components/task.rs
```
- [ ] Add `TaskComponent` with task enum
- [ ] Define basic tasks: Idle, MoveTo, Harvest
- [ ] Add to workers
- [ ] Debug: "Worker 1: Task changed Idle -> MoveTo"
- [ ] Verify: Task appears in debug output

### Step 3.2: Simple Task System (1 hour)
- [ ] Create `systems/task_system.rs`
- [ ] Process task queue
- [ ] Execute current tasks
- [ ] Debug: "Task completed: MoveTo (5,5)"
- [ ] Test: Assign task via debug CLI

### Step 3.3: Basic Pathfinding (1.5 hours)
```rust
// Simplified A* from sim_core
```
- [ ] Port `ai/pathfinding.rs` (simplified)
- [ ] Add path request/response
- [ ] Integrate with MoveTo task
- [ ] Debug: "Path found: 12 nodes, cost: 15.5"
- [ ] Test: Workers navigate around obstacles

## Phase 4: Resources & Items (Day 4)

### Step 4.1: Resource Components (30 min)
- [ ] Add `ResourceComponent` for tiles
- [ ] Define resource types: Wood, Stone, Food
- [ ] Add to appropriate tiles
- [ ] Debug: "Resource spawned: Wood at (10,15)"
- [ ] Verify: Resources show in tile info

### Step 4.2: Inventory System (45 min)
```rust
// Port components/inventory.rs
```
- [ ] Add `InventoryComponent` 
- [ ] Support item stacking
- [ ] Add to workers
- [ ] Debug: "Worker 1 inventory: Wood x5"
- [ ] Test: Pick up items

### Step 4.3: Harvest Task (45 min)
- [ ] Add Harvest task type
- [ ] Implement resource gathering
- [ ] Update inventory on harvest
- [ ] Debug: "Harvest complete: +3 Wood"
- [ ] Test: Worker harvests tree

## Phase 5: Building System (Day 5)

### Step 5.1: Building Components (30 min)
- [ ] Add `BuildingComponent`
- [ ] Define building types: Storage, Workshop
- [ ] Add construction state
- [ ] Debug: "Building placed: Storage at (20,20)"
- [ ] Verify: Buildings render differently

### Step 5.2: Construction Tasks (1 hour)
- [ ] Add Build task type
- [ ] Require materials
- [ ] Track construction progress
- [ ] Debug: "Construction: 45% complete"
- [ ] Test: Build with materials

### Step 5.3: Building Functions (45 min)
- [ ] Storage: increase inventory capacity
- [ ] Workshop: enable crafting
- [ ] Add building interactions
- [ ] Debug: "Storage accessed: 50/100 capacity"
- [ ] Test: Use buildings

## Phase 6: Crafting (Day 6)

### Step 6.1: Recipe Registry (30 min)
```rust
// Simplified from recipes.rs
```
- [ ] Create `recipes.rs`
- [ ] Define Recipe struct
- [ ] Add basic recipes
- [ ] Debug: "Recipe loaded: Plank (Wood x1)"
- [ ] Verify: Recipes list in debug

### Step 6.2: Crafting System (1 hour)
- [ ] Add Craft task
- [ ] Check recipe requirements
- [ ] Consume ingredients, produce output
- [ ] Debug: "Crafting: Plank complete"
- [ ] Test: Craft items

## Phase 7: AI Behaviors (Day 7)

### Step 7.1: Need System (45 min)
- [ ] Add hunger, energy needs
- [ ] Decrease over time
- [ ] Affect task priorities
- [ ] Debug: "Worker 1: Hunger 45%, Energy 70%"
- [ ] Test: Workers seek food when hungry

### Step 7.2: Task Priority (45 min)
- [ ] Add priority to tasks
- [ ] Sort task queue by priority
- [ ] Urgent needs = high priority
- [ ] Debug: "Task queue: Eat(P:10), Build(P:5)"
- [ ] Verify: Urgent tasks execute first

### Step 7.3: Simple Behavior Tree (1 hour)
```rust
// Simplified from ai/behavior_tree.rs
```
- [ ] Basic selector/sequence nodes
- [ ] Condition checks
- [ ] Connect to task system
- [ ] Debug: "Behavior: Hungry -> FindFood -> Eat"
- [ ] Test: Complex behaviors work

## Phase 8: Save/Load (Day 8)

### Step 8.1: World Serialization (45 min)
- [ ] Serialize chunks to JSON
- [ ] Save world state
- [ ] Add save command
- [ ] Debug: "World saved: 64 chunks, 2.1MB"
- [ ] Test: Save file created

### Step 8.2: Entity Serialization (45 min)
- [ ] Serialize all components
- [ ] Save entity states
- [ ] Include in save file
- [ ] Debug: "Saved 5 workers, 10 buildings"
- [ ] Verify: Complete state saved

### Step 8.3: Load System (1 hour)
- [ ] Load world from file
- [ ] Restore all entities
- [ ] Rebuild component relationships
- [ ] Debug: "World loaded: tick 1523"
- [ ] Test: Game continues from save

## Phase 9: Performance (Day 9)

### Step 9.1: Spatial Index (45 min)
- [ ] Add spatial hash for entities
- [ ] Quick neighbor lookups
- [ ] Update on movement
- [ ] Debug: "Spatial index: 25 entities in 9 cells"
- [ ] Test: Performance improvement

### Step 9.2: System Parallelization (1 hour)
- [ ] Identify independent systems
- [ ] Run in parallel where safe
- [ ] Add timing metrics
- [ ] Debug: "Frame time: 8ms (was 15ms)"
- [ ] Verify: No race conditions

## Phase 10: WebSocket Integration (Day 10)

### Step 10.1: Efficient Updates (45 min)
- [ ] Send only changed data
- [ ] Compress large updates
- [ ] Batch small changes
- [ ] Debug: "WS: Sent 120 bytes (was 5KB)"
- [ ] Test with Playwright MCP

### Step 10.2: Client Sync (45 min)
- [ ] Track client state version
- [ ] Handle reconnection
- [ ] Sync on connect
- [ ] Debug: "Client synced: version 1523"
- [ ] Verify: Frontend stays in sync

## Debug Validation for Each Step

```bash
# Before starting any step:
git status  # Clean working directory

# After implementing:
cargo build  # Must compile without errors
RUST_LOG=debug cargo run  # Run with debug

# Check for:
✅ Expected debug message appears
✅ No red ERROR messages
✅ Existing features still work
✅ Performance acceptable (FPS > 30)

# If frontend involved:
mcp__playwright__browser_navigate("http://localhost:3000")
mcp__playwright__browser_console_messages()  # No errors
mcp__playwright__browser_take_screenshot()  # Visual check

# Only then:
git add -A && git commit -m "Step X.Y: Description"
```

## Success Metrics

Each step is complete when:
1. Code compiles without warnings
2. Debug output shows expected behavior
3. No regression in existing features
4. Terminal shows clear success indicators
5. Can demonstrate feature via debug logs

## Risk Mitigation

- **Feature flags**: Add toggles for new systems
- **Backward compatible**: Keep old code paths initially
- **Incremental**: Each step < 2 hours of work
- **Testable**: Every change visible in debug output
- **Revertable**: Git commit after each working step

## Timeline

- **Week 1**: Phases 1-3 (Architecture, World, AI)
- **Week 2**: Phases 4-6 (Resources, Building, Crafting)
- **Week 3**: Phases 7-10 (Behaviors, Save/Load, Performance, WebSocket)

Total: ~40-50 hours of focused development, with working simulation throughout.

## First Step To Start

```bash
# Start immediately with Step 1.1:
mkdir -p world_sim_simple/src/components
touch world_sim_simple/src/components/mod.rs
# Add PositionComponent and HealthComponent
# Run with debug to verify
```