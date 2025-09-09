# Terminal Debug System

The terminal-based debug system provides real-time debugging output directly in your terminal, improving development speed by:

1. **Immediate Feedback** - No need to switch to browser for debug info
2. **Colored Output** - Different log levels with color coding
3. **Interactive CLI** - Control debugging without recompiling
4. **Performance Monitoring** - Real-time FPS and frame stats

## Features

### Log Levels with Colors
- **ERROR** (Red, Bold) - Critical issues
- **WARN** (Yellow) - Important warnings  
- **INFO** (Green) - General information
- **DEBUG** (Blue) - Detailed debugging
- **TRACE** (Dimmed) - Verbose tracing

### Interactive Commands
Type commands directly in terminal while game runs:
- `verbosity <level>` - Change log detail (error/warn/info/debug/trace)
- `grid` or `g` - Toggle world grid display
- `agents` or `a` - Toggle agent list
- `stats` or `s` - Toggle performance stats
- `clear` or `c` - Clear debug buffer
- `pause` or `p` - Pause simulation
- `resume` or `r` - Resume simulation
- `step` or `n` - Step one frame

### Keyboard Shortcuts (In-Game)
- `F1` - Toggle stats overlay
- `F2` - Toggle grid visualization
- `F3` - Toggle agents display
- `F5` - Clear debug buffer

## Usage

Run with debug mode:
```bash
./run_debug.sh
```

Or manually with specific log level:
```bash
RUST_LOG=debug cargo run --manifest-path world_sim_simple/Cargo.toml
```

## Benefits vs HTML-Only Debug

1. **Speed** - No HTTP overhead or browser rendering
2. **Focus** - Stay in terminal during development
3. **Integration** - Works with existing terminal tools (grep, tee, etc.)
4. **Responsiveness** - Instant feedback on state changes
5. **Scriptability** - Pipe output to files or other tools

## Example Output

```
[0.125] [SIMULATION] INFO: Agent count changed: 0 -> 5
[0.250] [DEBUG] INFO: Verbosity set to Debug
[0.375] [WORKER] DEBUG: Worker 1 moved to (23, 31)
[0.500] [WORLD] INFO: Tile (15, 20) changed from Grass to Stone

=== STATS ===
  Time: 5.2s
  Frame: 312
  FPS: 60.0
```

The HTML visualization remains available for detailed visual debugging when needed.