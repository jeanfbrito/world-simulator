# Claude Code Agent Instructions

## CRITICAL: Debug-First Development Workflow

**MANDATORY**: Every code modification MUST follow this debug validation workflow before moving to the next step:

### 0. Log Analysis - The DEFAULT Debugging Method

**ALWAYS analyze logs by comparing sequential ticks** to identify stuck behavior:

```bash
# Look at 10 sequential ticks to see what each unit is doing
tail -n 5000 /tmp/world_sim_*.log | grep -E "^\[196\.[6-8]" | head -50

# Track specific peasant behavior across ticks
tail -n 10000 /tmp/world_sim_*.log | grep "Peasant 1" | head -20

# Monitor work progress to spot overflow issues
tail -n 1000 /tmp/world_sim_*.log | grep "progress:" | head -20
```

**Key patterns to identify stuck behavior:**
- Same action repeating without state change
- Progress counters exceeding limits (e.g., 9990/30)
- Units at same position for many ticks
- No variation in behavior between different units
- Actions completing but not triggering replanning

When asked to analyze logs:
1. Compare multiple sequential ticks (not spaced out)
2. Track individual unit behavior changes
3. Look for progress/counter anomalies
4. Identify when behavior becomes repetitive

### 1. Terminal Debug is PRIMARY Method
Always use terminal debugging as the primary debugging method. It's faster and more efficient than HTML visualization.

```bash
# Run with debug output
RUST_LOG=debug cargo run --manifest-path world_sim_simple/Cargo.toml

# Or use the debug script
./run_debug.sh
```

### 2. Development Workflow - MUST FOLLOW

For EVERY code change, agents MUST:

1. **WRITE** - Make the code modification
2. **BUILD** - Compile and check for errors
   ```bash
   cargo build
   ```
3. **DEBUG** - Run with terminal debug output to validate
   ```bash
   RUST_LOG=info cargo run
   ```
4. **VERIFY** - Check debug output confirms expected behavior
5. **ITERATE** - Fix any issues found in debug output
6. **COMMIT** - Only after validation passes

### 3. Debug Output Validation Checklist

Before considering any task complete, verify in terminal output:
- [ ] No ERROR messages (red text)
- [ ] Expected INFO logs appear (green text)  
- [ ] State changes are logged correctly
- [ ] Performance metrics are acceptable (FPS > 30)
- [ ] No unexpected warnings (yellow text)

### 4. Terminal Debug Features to Use

#### Real-time Monitoring
- Watch for state changes in colored output
- Monitor FPS and performance metrics
- Track entity counts and positions

#### Interactive Commands (type while running)
- `verbosity debug` - Increase detail when investigating issues
- `stats` - Toggle performance monitoring
- `clear` - Clear buffer when output gets cluttered
- `pause` - Pause to examine state
- `step` - Step through frames one at a time

#### Log Levels for Different Stages
- `RUST_LOG=error` - Production/final testing
- `RUST_LOG=info` - General development (default)
- `RUST_LOG=debug` - Investigating issues
- `RUST_LOG=trace` - Deep debugging

### 5. Common Debug Patterns

#### Adding New Feature
```bash
# 1. Add feature code
# 2. Add debug log in new code
self.log(DebugLevel::Info, "FEATURE", "New feature activated");
# 3. Build and run with debug
RUST_LOG=info cargo run
# 4. Verify log appears in green
# 5. Test feature behavior
```

#### Fixing Bug
```bash
# 1. Set verbose logging
RUST_LOG=debug cargo run
# 2. Reproduce issue
# 3. Add targeted debug logs
self.log(DebugLevel::Debug, "BUG", &format!("State: {:?}", state));
# 4. Run again and examine output
# 5. Fix based on debug info
# 6. Verify fix with clean run
```

#### Performance Issues
```bash
# 1. Enable stats display (F1 in game)
# 2. Monitor FPS in terminal output
# 3. Add timing logs around suspected code
let start = Instant::now();
// ... code ...
self.log(DebugLevel::Debug, "PERF", &format!("Operation took: {:?}", start.elapsed()));
# 4. Identify bottlenecks from timing
```

### 6. NEVER Skip Debug Validation

**IMPORTANT**: Do NOT:
- Move to next task without validating current one
- Commit code without running debug verification  
- Assume code works without checking terminal output
- Ignore warnings or errors in debug output

### 7. HTML Debug as Secondary Tool

Use HTML visualization (`test_debug.html`) only when:
- Need visual representation of spatial data
- Debugging complex UI interactions
- Creating screenshots for documentation
- Terminal output insufficient for specific issue

### 8. Example Session

```bash
# Start development
$ ./run_debug.sh

# See initial state
[0.001] [INIT] INFO: System initialized
[0.002] [WORLD] INFO: Map generated 64x64

# Make code change, rebuild
$ cargo build

# Run with debug
$ RUST_LOG=debug cargo run

# Verify change in output
[1.234] [FEATURE] INFO: New pathfinding enabled
[1.235] [PATH] DEBUG: Path found: 15 nodes

# If good, continue. If not, fix and repeat.
```

## 9. Playwright MCP for Browser Testing

**IMPORTANT**: The Playwright MCP tools are available and should be used freely for:
- Automated browser testing of the web frontend
- Taking screenshots of the simulation UI
- Validating WebSocket communication
- Testing user interactions in the browser
- Verifying visual elements render correctly
- Debugging browser console errors
- Monitoring network requests

### Playwright Usage Examples

```bash
# Navigate to frontend
mcp__playwright__browser_navigate(url: "http://localhost:3000")

# Take screenshots for validation
mcp__playwright__browser_take_screenshot(filename: "simulation-state.png")

# Check console for errors
mcp__playwright__browser_console_messages()

# Interact with UI elements
mcp__playwright__browser_click(element: "Start Simulation button")

# Monitor WebSocket messages
mcp__playwright__browser_network_requests()
```

Use Playwright MCP whenever you need to:
- Verify frontend changes work correctly
- Debug client-side issues
- Validate UI state matches backend state
- Test user workflows end-to-end
- Generate visual documentation

## 10. Incremental Upgrade Strategy for sim_simple

**CRITICAL**: We are enhancing sim_simple incrementally, NOT fixing sim_core. This decision was made because:
- sim_simple is working and stable
- sim_core has complex dependencies (LuaJIT, etc.)
- We can test each feature with our debug system
- We maintain a working simulation at every step

### Development Approach

1. **Small Steps** - Each task should take 30min-2hrs maximum
2. **Non-Breaking** - System must work after every change
3. **Debug First** - Validate with terminal output before moving on
4. **Commit Often** - Git commit after each working step

### The 10-Phase Plan

We're following `INCREMENTAL_UPGRADE_PLAN.md` which includes:
- Phase 1-3: Core Architecture (Components, Plugins, Chunks)
- Phase 4-6: Economy (Resources, Buildings, Crafting)
- Phase 7-8: AI & Persistence (Behaviors, Save/Load)
- Phase 9-10: Optimization (Performance, WebSocket)

### Validation for Each Step

```bash
# Before starting any step
git status  # Ensure clean working directory

# After implementing
cargo build  # Must compile
RUST_LOG=debug cargo run  # Run with debug

# Check for:
✅ Expected debug message in green
✅ No red ERROR messages
✅ FPS > 30
✅ Existing features still work

# Only then proceed to next step
```

## 11. Rust/Bevy Compilation Best Practices

### Understanding Compilation Times

**IMPORTANT**: Long compilation times (5-15 minutes) are NORMAL for Rust + Bevy projects:
- **First build after `cargo clean`**: Downloads and compiles 200+ dependencies
- **Release builds**: Take 2-3x longer due to optimizations
- **Bevy specifically**: Heavy use of generics and compile-time type checking
- **Incremental builds**: Much faster (10-30 seconds) after initial build

### Build Strategies

**CRITICAL: Don't let long compilation block your work!**

1. **ALWAYS use background builds** - Continue working while compiling:
   ```bash
   cargo build -p world_sim_simple 2>&1 &  # Run in background
   # or use run_in_background: true with Bash tool
   
   # Then immediately continue with other tasks:
   # - Write documentation
   # - Plan next features
   # - Refactor other files
   # - Research solutions
   ```

2. **Monitor compilation without blocking**:
   ```bash
   # Use BashOutput tool to check progress periodically
   # Look for "Compiling" messages - means it's working
   # No output for 30+ seconds is still normal for Bevy
   ```

3. **Parallel workflow pattern**:
   ```bash
   # START: Launch build in background
   cargo build --release 2>&1 &
   
   # WORK: Continue with non-dependent tasks
   # - Implement next feature in different module
   # - Update documentation
   # - Write tests
   # - Fix issues in other files
   
   # CHECK: Periodically verify build progress (every 5-10 min)
   # - Use BashOutput to see compilation messages
   # - If seeing "Compiling crate_name", it's working
   # - If seeing repeated errors, fix and restart
   ```

4. **Signs build is actually stuck** (rare):
   ```bash
   # Check CPU usage - should be high if compiling
   ps aux | grep cargo
   # No CPU usage + no output for 10+ minutes = possibly stuck
   # Safe to kill and restart if truly stuck
   ```

### Productive During Compilation

While Rust compiles (5-15 minutes is normal!), you can:
- ✅ Implement features in other modules
- ✅ Write or update documentation
- ✅ Plan architecture for next components
- ✅ Research algorithms or patterns
- ✅ Review and refactor existing code
- ✅ Create or update tests
- ✅ Fix linting warnings in other files

Never just wait for compilation - always have parallel work ready!

### Common Build Issues and Solutions

- **"Build seems stuck"** - It's probably not! Bevy compilation is just slow
- **Out of memory** - Close other applications, use `--jobs 2` to limit parallelism
- **Clean build needed** - Only when dependency versions conflict
- **Release vs Debug** - Use debug for development (faster compilation)

## 12. GOAP Implementation Guidelines

### Building GOAP Without External Dependencies

When implementing GOAP (Goal-Oriented Action Planning), you can build it from scratch:

1. **State Components** - Use Bevy's Component system:
   ```rust
   #[derive(Component, Clone, Debug, Default, Reflect)]
   pub struct IsHungry(pub f64);
   ```

2. **Action System** - Simple structs with preconditions/effects:
   ```rust
   pub struct GoapAction {
       pub name: String,
       pub cost: f32,
       pub preconditions: HashMap<String, StateValue>,
       pub effects: HashMap<String, StateValue>,
   }
   ```

3. **Planner** - A* search algorithm works well:
   - Use BinaryHeap for open set
   - Track visited states to avoid cycles
   - Limit search depth to prevent infinite loops

4. **Integration Tips**:
   - Add `#[derive(Resource)]` for shared action sets
   - Add `#[derive(Component)]` for per-entity states
   - Use `#[derive(Reflect)]` for debug inspection

### Common GOAP Pitfalls to Avoid

- **Missing derives** - Always add Component/Resource/Reflect as needed
- **Wrong component types** - Check actual struct names (BuildingComponent vs Building)
- **Inventory assumptions** - Modern inventories are slot-based, not field-based
- **Debug levels** - Use DebugLevel::Info (Warning may not exist)

## 13. Debugging Complex Systems

### When Systems Don't Compile

1. **Read error messages carefully** - Rust errors are very descriptive
2. **Check imports** - Ensure all types are properly imported
3. **Verify component registration** - Components need proper derives
4. **Match existing patterns** - Look at similar working code

### Incremental Development Strategy

When adding complex features like GOAP:

1. **Phase 1**: Add basic components and state tracking
2. **Phase 2**: Implement actions and planning logic
3. **Phase 3**: Connect to existing systems
4. **Phase 4**: Test and optimize

Each phase should compile and run independently!

### Entity Component System (ECS) Best Practices

- **Components are data only** - No logic in components
- **Systems contain logic** - Keep systems focused and small
- **Resources are shared** - Use for global state/configuration
- **Queries must match exactly** - Component types must be registered

## 14. Project Organization Best Practices

### File Size Guidelines

**CRITICAL**: Keep files small and focused for better maintainability:
- **Maximum file size**: 300-400 lines (preferred), 500 lines absolute maximum
- **Single responsibility**: Each file should handle ONE concept
- **Break up large files**: Split into submodules when exceeding limits

### Module Structure

```rust
// Instead of one giant file:
// ❌ src/ai/mod.rs (2000+ lines)

// Use this structure:
// ✅ src/ai/
//     mod.rs          (module declarations and public API)
//     behaviors/
//         mod.rs      (behavior subsystem)
//         goap.rs     (GOAP specific logic)
//         utility.rs  (Utility AI logic)
//     actions/
//         mod.rs      (action subsystem)
//         movement.rs (movement actions)
//         work.rs     (work actions)
//         combat.rs   (combat actions)
//     scorers/
//         mod.rs      (scorer subsystem)
//         needs.rs    (need-based scorers)
//         environment.rs (environment scorers)
//     planners/
//         mod.rs      (planning subsystem)
//         goap_planner.rs
//         task_scheduler.rs
```

### Component Organization

Split large component files:
```rust
// ❌ components/mod.rs with 15+ components

// ✅ components/
//     mod.rs           (re-exports)
//     unit_state.rs    (UnitNeeds, UnitWorkState)
//     inventory.rs     (UnitInventory)
//     location.rs      (UnitLocation)
//     ownership.rs     (UnitOwnership)
//     combat.rs        (CombatStats, Weapon)
//     building.rs      (BuildingComponent)
```

### System Organization

Keep systems focused:
```rust
// Each system in its own file:
// systems/
//     movement.rs      (movement_system - 100 lines)
//     combat.rs        (combat_system - 150 lines)
//     needs.rs         (needs_update_system - 80 lines)
//     building.rs      (building_system - 120 lines)
```

### Configuration Files

Keep data files small and specific:
```lua
-- ❌ Single giant config.lua (1000+ lines)

-- ✅ Split by domain:
-- assets/packs/[pack_name]/scripts/
--     units/
--         peasant.lua
--         soldier.lua
--     buildings/
--         house.lua
--         stockpile.lua
--     ai/
--         goap_actions.lua
--         utility_scorers.lua
--         goap_goals.lua
```

### Benefits of Modular Organization

1. **Easier navigation**: Find code quickly
2. **Better collaboration**: Less merge conflicts
3. **Faster compilation**: Incremental builds work better
4. **Clearer dependencies**: Easy to see what depends on what
5. **Testability**: Easier to unit test small modules

### When to Split Files

Split when you see:
- File exceeds 300 lines
- Multiple unrelated structs/functions
- Complex nested modules
- Difficulty finding specific code
- Frequent merge conflicts

### Module Best Practices

1. **Public API at top**: Put pub declarations first
2. **Tests in same file**: Keep unit tests with code
3. **Documentation**: Each module needs clear docs
4. **Re-exports**: Use mod.rs for clean public API
5. **Feature flags**: Use for optional dependencies

## 15. AI System References

When implementing AI behaviors, refer to these key examples:

### Dogoap (Goal-Oriented Action Planning)
- **Repository**: ~/Github/dogoap/
- **Key Example**: `crates/bevy_dogoap/examples/miner.rs`
- **Usage**: Long-term planning, goal-driven behavior
- **Pattern**: Define states with derive macros, actions with preconditions/effects

### Big-Brain (Utility AI)
- **Repository**: ~/Github/big-brain/
- **Key Examples**: 
  - `examples/thirst.rs` (simple utility AI)
  - `examples/farming_sim.rs` (complex behaviors with sequences)
- **Usage**: Reactive behaviors, immediate needs, utility scoring
- **Pattern**: Thinkers with Scorers and Actions, state machine execution

### Hybrid Approach
Combine both systems:
1. Dogoap for strategic planning (what goals to achieve)
2. Big-Brain for tactical execution (how to achieve them)
3. Use consolidated state components instead of 15+ separate ones

## 16. Monitoring Running Simulations

**CRITICAL**: Never start multiple simulation instances! The monitor_world.sh script now automatically handles this.

### Automatic Monitoring with monitor_world.sh

The updated `monitor_world.sh` script now **automatically**:
1. **Detects multiple running simulations** and offers to clean them up
2. **Prevents duplicate instances** from starting
3. **Handles existing simulations** appropriately
4. **Logs output** to `/tmp/world_sim_*.log` for later review

### Just Run the Monitor:
```bash
# The script handles everything automatically
cd /Users/jean/Github/world-simulator && ./monitor_world.sh
```

### What the Monitor Does:

#### If Multiple Simulations Found:
- Shows warning with PID count
- Offers options:
  1. Kill all and start fresh (recommended)
  2. Keep one and kill others
  3. Exit without changes

#### If One Simulation Running:
- Detects it automatically
- Restarts with monitoring enabled
- Shows clean world state output

#### If No Simulation Running:
- Starts a new one with monitoring
- Logs to `/tmp/world_sim_TIMESTAMP.log`

### Output Shows:
- Peasant positions and states (🧍 standing, 🚶 walking)
- Hunger/Energy levels with visual bars
- Inventory contents (🪵 wood, 🍖 food, ⛏️ stone)
- Current actions and plans
- Resource availability (🌲 trees, 🫐 berry bushes)
- Tiles walked counter (📏)

### Manual Commands (if needed):

Check for running simulations:
```bash
ps aux | grep "target/debug/world_sim_simple" | grep -v grep
```

Kill all simulations:
```bash
pkill -f "world_sim_simple"
```

View saved logs:
```bash
ls -la /tmp/world_sim_*.log
tail -f /tmp/world_sim_*.log  # Follow latest log
```

### Why This Matters:
- **Prevents CPU overload** from multiple 100%+ CPU processes
- **Saves compilation time** (5-15 minutes each time!)
- **Clean output** without overlapping processes
- **Automatic management** - no manual checking needed
- **Historical logs** saved for debugging

### Example Workflow:
```bash
# Just run the monitor - it handles everything!
cd /Users/jean/Github/world-simulator && ./monitor_world.sh

# If you see multiple simulations warning, choose option 1 (kill all and restart)
# Monitor shows clean world state
# Press Ctrl+C to stop monitoring (simulation continues)
# Logs are saved to /tmp/ for later review
```

## 17. World Simulator Viewer

The HTML viewer for the simulation is located at:
```
file:///Users/jean/Github/world-simulator/world_sim_simple/viewer.html
```

The viewer provides:
- Real-time visualization of the map and entities
- Peasant details showing UnitMind states with icons
- WebSocket connection to the simulation at ws://localhost:8080
- Mind state icons that match the terminal debug output
- Movement detection and visual indicators (🚶 walking, 🧍 standing)

To use the viewer:
1. Start the simulation with WebSocket enabled (happens by default)
2. Open the viewer HTML file in a browser
3. The viewer will automatically connect and display the simulation state

## Summary

**Terminal debugging is not optional** - it's the required validation method for all code changes. HTML visualization is supplementary. Always validate through debug output before considering any task complete. Use Playwright MCP freely for browser-based testing and validation. Follow the incremental upgrade plan for sim_simple enhancements. Expect and plan for long Rust compilation times - they're normal and not a sign of problems. Refer to dogoap and big-brain repositories for AI implementation patterns. **Always monitor existing simulations instead of starting new ones.** The viewer at file:///Users/jean/Github/world-simulator/world_sim_simple/viewer.html provides visual representation of the simulation state.