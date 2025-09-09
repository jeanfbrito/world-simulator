# Claude Code Agent Instructions

## CRITICAL: Debug-First Development Workflow

**MANDATORY**: Every code modification MUST follow this debug validation workflow before moving to the next step:

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

1. **Use background builds** - Continue working while compiling:
   ```bash
   cargo build -p world_sim_simple &  # Run in background
   # Continue editing other files
   ```

2. **Check syntax quickly**:
   ```bash
   cargo check  # Faster than full build, catches most errors
   ```

3. **Build specific package**:
   ```bash
   cargo build -p world_sim_simple  # Don't rebuild entire workspace
   ```

4. **Monitor build progress**:
   ```bash
   # Check if cargo is still working (not stuck)
   ps aux | grep cargo
   # Watch for "Compiling" messages
   ```

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

## Summary

**Terminal debugging is not optional** - it's the required validation method for all code changes. HTML visualization is supplementary. Always validate through debug output before considering any task complete. Use Playwright MCP freely for browser-based testing and validation. Follow the incremental upgrade plan for sim_simple enhancements. Expect and plan for long Rust compilation times - they're normal and not a sign of problems.