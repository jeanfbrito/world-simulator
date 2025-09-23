# Simulation Logging System

## Overview

This document describes how to run and debug the world simulator using the automated logging system.

## Quick Start

### Running the Simulation

```bash
# Start a new simulation with automatic logging
./run-simulation.sh
```

This will:
- Create a logs directory if it doesn't exist
- Stop any running simulations
- Start a new simulation with logs saved to `logs/simulation-YYYYMM-DD-HHMMSS.log`
- Show initial status and spawn information

### Monitoring Logs

```bash
# Follow logs in real-time
tail -f logs/simulation-*.log

# Search for specific information
grep -E '\[SPAWN\]' logs/simulation-*.log
grep -E 'peasant|entities|Found' logs/simulation-*.log
grep -E 'WARNING|ERROR' logs/simulation-*.log
```

## Log File Structure

### Log Naming Convention
- Format: `simulation-YYYYMMDD-HHMMSS.log`
- Location: `logs/` directory
- Example: `simulation-20250923-103155.log`

### Key Log Patterns

#### Spawn Information
```
[SPAWN] Starting entity spawning from pack definitions...
[SPAWN] Spawned 5 Peasant entities from pack definition
[SPAWN] Unit Peasant movement speed: 2 ticks/tile
[SPAWN] Unit Peasant work speed: 1.00x from pack definition
[SPAWN] Spawned 15 Tree entities from pack definition
[SPAWN] Spawned 25 Berry Bush entities from pack definition
```

#### Entity Status
```
🔍 IPC Debug: Found 5 entities (5 GOAP, 0 basic) and 46 resources
WARNING: No units found with UnitTag, GridPosition, GridMovement, and VisualPosition!
```

#### System Status
```
✅ Work system ACTIVE on tick 357
⚙️ Tick work system: 0/0 units working, tick 357
━━━ TICK 392 ━━━
🌍 World Resources:
   🌲 15 trees available
   🫐 25 berry bushes with fruit
   🌳 0 depleted berry bushes
```

## Common Issues and Solutions

### 1. "Unit has no unit properties"
**Symptom**: `[SPAWN] Unit Peasant has no unit properties`

**Cause**: The unit definition in Lua files is using outdated field names
**Solution**: Update Lua file to use `ticks_per_tile` instead of `movement_speed`

### 2. No entities found by IPC
**Symptom**: `Found 0 entities (0 GOAP, 0 basic) and 46 resources`

**Cause**: Units not getting UnitTag component or pack definition issues
**Solution**: Check unit spawning in logs and verify pack definitions

### 3. Movement not working
**Symptom**: Peasants spawn but don't move

**Cause**: Missing or incorrect movement speed configuration
**Solution**: Verify `ticks_per_tile` is set correctly in peasant.lua

## Manual Simulation Control

### Starting Manually
```bash
# Create logs directory
mkdir -p logs

# Run with custom log file
RUST_LOG=info cargo run --release --bin world_sim_simple -- --headless --ticks 1000 > logs/custom-log.log 2>&1 &
```

### Stopping Simulations
```bash
# Stop all headless simulations
pkill -f "world_sim_simple.*--headless"

# Stop specific PID
kill <PID>
```

### Checking Status
```bash
# Check running simulations
ps aux | grep "world_sim_simple.*--headless"

# Check log files
ls -la logs/
tail -20 logs/simulation-*.log
```

## Debug Commands

### Entity Spawn Debugging
```bash
# Check spawn status
grep -E "\[SPAWN\]" logs/simulation-*.log | head -20

# Check peasant specific logs
grep -E "Unit Peasant" logs/simulation-*.log

# Check total entities spawned
grep -E "Total entities spawned" logs/simulation-*.log
```

### Performance Monitoring
```bash
# Check tick performance
grep -E "TICK.*=" logs/simulation-*.log | tail -10

# Check system status
grep -E "✅.*ACTIVE" logs/simulation-*.log

# Check resource counts
grep -E "🌍 World Resources" logs/simulation-*.log | tail -5
```

### Error Detection
```bash
# Find warnings and errors
grep -E "WARNING|ERROR" logs/simulation-*.log

# Check for failed spawns
grep -E "Unknown unit type|has no unit properties" logs/simulation-*.log
```

## Configuration

### Simulation Parameters
- `--headless`: Run without GUI
- `--ticks N`: Run for N simulation ticks
- `RUST_LOG=info`: Enable info-level logging

### Default Settings
- **Ticks**: 1000 (in script)
- **Log Level**: info
- **Output**: Timestamped log files
- **Cleanup**: Stops previous simulations automatically

## Architecture Notes

### Entity Types
1. **GOAP Entities**: Old hardcoded peasants with full AI
2. **Basic Entities**: Pack-based entities with simple components
3. **Resources**: Trees, berry bushes, stone deposits

### Component Systems
- **Movement**: Uses `ticks_per_tile` for speed configuration
- **Energy**: GOAP-based energy system
- **Work**: Pack-based work speed configuration
- **AI**: Simplified behavior for basic survival

### File Locations
- **Pack Definitions**: `assets/packs/dev-world/data/entities/`
- **Spawn Logic**: `world_sim_simple/src/systems/entity_spawning.rs`
- **IPC Output**: `world_sim_simple/src/ipc_output.rs`
- **Viewer**: `web-viewer/viewer.html`

## Best Practices

1. **Always use the script**: `./run-simulation.sh` for consistent logging
2. **Check spawn logs first**: Verify entities are spawning correctly
3. **Monitor IPC output**: Ensure entities are being detected by the viewer
4. **Use grep for debugging**: Quick searches for specific patterns
5. **Keep logs organized**: Use timestamps to track different simulation runs
6. **Clean up old logs**: Remove old log files to save space