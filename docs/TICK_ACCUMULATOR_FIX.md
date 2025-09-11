# Critical Fix: Tick Accumulator Time Remainder Handling

> **Note:** This article is also available in the [project wiki](https://github.com/jeanfbrito/world-simulator/wiki/Tick-Accumulator-Time-Remainder-Fix)

## Problem Discovered

During debugging of the world simulator, we discovered the simulation was running at 1 tick per second instead of the intended 10 ticks per second. Investigation revealed two critical issues in the tick accumulator implementation.

## Issue 1: Incorrect Tick Threshold

The original code used `1.0` as the threshold instead of `TICK_RATE` (0.1):

```rust
// WRONG - waits for 1 full second before ticking
while accumulator >= 1.0 {
    tick_count += 1;
    accumulator -= TICK_RATE;
}
```

This caused the system to wait for a full second of accumulated time before processing a tick, making the simulation run 10x slower than intended.

### The Fix

```rust
// CORRECT - processes a tick every 0.1 seconds
while accumulator >= TICK_RATE {
    tick_count += 1;
    accumulator -= TICK_RATE;
}
```

## Issue 2: Time Remainder Loss

The more subtle but equally important issue was how leftover time was handled after processing ticks.

### The Problem with Reset

Initially, after processing ticks, the accumulator was reset to zero:

```rust
// WRONG - throws away leftover time
while accumulator >= TICK_RATE {
    tick_count += 1;
    accumulator = 0.0;  // Lost time!
}
```

This approach discards any fractional time that didn't make up a full tick. While each individual loss is small (typically 0-0.099 seconds), these losses compound over time.

### Why This Matters

Consider what happens over multiple frames:

**With Reset (Loses Time):**
- Frame 1: 0.12s accumulated → 1 tick processed, 0.02s lost
- Frame 2: 0.11s accumulated → 1 tick processed, 0.01s lost  
- Frame 3: 0.13s accumulated → 1 tick processed, 0.03s lost
- **Total:** 0.06s lost in just 3 frames

Over minutes or hours of simulation, this drift becomes significant:
- After 100 frames with average 0.02s loss: 2 seconds lost
- After 1000 frames: 20 seconds lost
- The simulation gradually falls behind real-time

### The Solution: Subtraction

The correct approach subtracts only the consumed time:

```rust
// CORRECT - keeps the remainder
while accumulator >= TICK_RATE {
    tick_count += 1;
    accumulator -= TICK_RATE;  // Preserves leftover time
}
```

**With Subtraction (Preserves Time):**
- Frame 1: 0.12s → process 1 tick, keep 0.02s remainder
- Frame 2: 0.02s + 0.11s = 0.13s → process 1 tick, keep 0.03s
- Frame 3: 0.03s + 0.13s = 0.16s → process 1 tick, keep 0.06s
- **Total:** No time lost, perfect accuracy

## Mathematical Proof

Let's prove why subtraction maintains accuracy:

Given:
- `TICK_RATE = 0.1` seconds per tick
- `dt` = time elapsed this frame
- `accumulator` = total unprocessed time

**Reset Method:**
```
ticks_processed = floor(accumulator / TICK_RATE)
time_consumed = ticks_processed * TICK_RATE
time_lost = accumulator - time_consumed
accumulator = 0  // Discards time_lost
```

**Subtraction Method:**
```
ticks_processed = floor(accumulator / TICK_RATE)
time_consumed = ticks_processed * TICK_RATE
accumulator = accumulator - time_consumed  // Preserves remainder
```

The subtraction method ensures:
`accumulator_after = accumulator_before % TICK_RATE`

This remainder is always in the range [0, TICK_RATE), meaning no time is ever lost.

## Real-World Impact

This fix is crucial for:

1. **Deterministic Simulation:** Ensures the simulation produces the same results given the same inputs
2. **Multiplayer Synchronization:** Prevents clients from drifting out of sync
3. **Replay Systems:** Allows accurate replay of recorded games
4. **Long-Running Simulations:** Prevents time drift in simulations running for hours or days
5. **Frame Rate Independence:** Maintains consistent simulation speed regardless of rendering performance

## Implementation in world_sim_simple

The complete, correct implementation in `/world_sim_simple/src/main.rs`:

```rust
const TICK_RATE: f32 = 0.1; // 10 ticks per second

// In the update loop:
let current_time = time.elapsed_seconds();
let delta = current_time - last_time;
last_time = current_time;

accumulator += delta;

// Process all accumulated ticks
while accumulator >= TICK_RATE {
    tick_count += 1;
    accumulator -= TICK_RATE;  // Subtract, don't reset!
    
    // Process tick...
}
```

## Lessons Learned

1. **Always use the defined constant** - Don't hardcode values like `1.0` when `TICK_RATE` exists
2. **Preserve temporal precision** - Small time losses compound into large errors
3. **Test with debug output** - This issue was caught by adding tick timing logs
4. **Consider edge cases** - What happens to leftover time is a critical design decision

## Testing the Fix

To verify the fix works correctly:

```bash
# Run with debug logging
RUST_LOG=debug cargo run --manifest-path world_sim_simple/Cargo.toml

# Look for tick timing in output
# Should see ~10 ticks per second, not 1
```

## References

- Original issue discovered: September 2025
- Fixed in commit: `ad22a5c fix: Fix tick system to run at 10 TPS instead of 1 TPS`
- Related documentation: [Tick-Based Architecture](/TICK_BASED_ARCHITECTURE.md)

## Prevention

To prevent similar issues:

1. Add unit tests for tick accumulators
2. Include timing assertions in integration tests  
3. Monitor tick rate in production with metrics
4. Document timing assumptions clearly
5. Use constants consistently throughout codebase

This fix demonstrates how subtle timing bugs can have major impacts on simulation accuracy. Always handle time with care!