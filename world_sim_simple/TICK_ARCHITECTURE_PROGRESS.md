# Tick-Based Architecture Migration Progress

## Overview
Converting world_sim_simple from frame-based (60 FPS) to tick-based (10 TPS) architecture for better performance at scale (1000+ units).

## Completed Phases ✅

### Phase 1: Core Tick-Based Architecture ✅
**Status**: COMPLETED (Commit: c7112af)
- [x] Created `simulation/tick_config.rs` with tick constants
- [x] Implemented TickAccumulator for fixed timestep
- [x] Added SimulationState resource with tick counter
- [x] Created integer-based counter system (0-100,000)
- [x] Migrated needs system to UnitNeedsV2 with tick-based updates
- [x] Separated simulation (10 TPS) from rendering (60 FPS)

### Phase 2: Grid-Based Movement System ✅
**Status**: COMPLETED (Commit: ab73bb1)
- [x] Created GridPosition component for discrete tile positions
- [x] Implemented VisualPosition for smooth interpolation
- [x] Added GridMovement with tick-based progress counters
- [x] Created movement systems that run on ticks
- [x] Added configurable MovementSpeed per unit type
- [x] Implemented MovementEffects (exhaustion, encumbrance)
- [x] Added TerrainType affecting movement costs

### Phase 3: Tick-Based Work Systems ✅
**Status**: COMPLETED (Commit: 9637889)
- [x] Created WorkProgress component with tick counters
- [x] Implemented WorkType enum (Gathering, Building, Crafting, etc.)
- [x] Added WorkSpeed component for configurable rates
- [x] Created WorkQueue for task management
- [x] Implemented tick_work_system for execution
- [x] Added auto_gather_system for resource collection
- [x] Integrated with ResourceNode components
- [x] Added skill progression system

## Remaining Phases 📋

### Phase 4: Chunk-Based World Management 🔲
**Status**: NOT STARTED
- [ ] 4.1 Implement chunk system (32x32 or 64x64 tiles)
- [ ] 4.2 Active chunk detection near players
- [ ] 4.3 Chunk loading/unloading system
- [ ] 4.4 Entity serialization per chunk
- [ ] 4.5 Chunk-based pathfinding cache

### Phase 5: Enhanced Economy & Resources 🔲
**Status**: IN PROGRESS
- [ ] 5.1 Tick-based resource regeneration (via Lua scripts)
- [ ] 5.2 Market/trading with integer prices (defined in pack data)
- [ ] 5.3 Supply/demand simulation (configurable in Lua)
- [x] 5.4 Resource storage buildings (Lua-defined, data-driven)
- [ ] 5.5 Production chains (recipe scripts in packs)

### Phase 6: Advanced Building System 🔲
**Status**: NOT STARTED  
**Note**: All building definitions must come from pack scripts!
- [ ] 6.1 Multi-stage construction with ticks (stages in Lua)
- [ ] 6.2 Building placement validation (rules in scripts)
- [ ] 6.3 Multi-tile building support (sizes in Lua data)
- [ ] 6.4 Building upgrades and requirements (tech tree in packs)
- [ ] 6.5 Maintenance/decay system (rates configurable in Lua)

### Phase 7: Advanced AI Systems 🔲
**Status**: NOT STARTED
- [ ] 7.1 Job assignment and scheduling
- [ ] 7.2 Priority-based task queuing
- [ ] 7.3 Group behaviors (formations, squads)
- [ ] 7.4 Threat response and defense AI
- [ ] 7.5 Idle behavior improvements

### Phase 8: Save/Load System 🔲
**Status**: NOT STARTED
- [ ] 8.1 Efficient world state serialization
- [ ] 8.2 Compressed save file format
- [ ] 8.3 Auto-save on tick intervals
- [ ] 8.4 Quick save/load functionality
- [ ] 8.5 Save file versioning

### Phase 9: Performance Optimization 🔲
**Status**: NOT STARTED
- [ ] 9.1 Spatial indexing for fast queries
- [ ] 9.2 Component pooling and recycling
- [ ] 9.3 Parallel system execution
- [ ] 9.4 LOD system for distant entities
- [ ] 9.5 Memory usage optimization

### Phase 10: Network/Multiplayer 🔲
**Status**: NOT STARTED
- [ ] 10.1 WebSocket state synchronization
- [ ] 10.2 Client prediction system
- [ ] 10.3 Server authoritative simulation
- [ ] 10.4 Delta compression for updates
- [ ] 10.5 Lag compensation

## Key Architecture Decisions

### Data-Driven Design (IMPORTANT!)
- **All game content must be defined in Lua scripts within packs**
- **NO hardcoded game content in Rust systems**
- Buildings, items, recipes, AI behaviors - all loaded from pack scripts
- Rust provides the engine, Lua provides the content
- Example: Storage buildings defined in `assets/packs/stronghold/scripts/buildings/`

### Integer Counters
- All progress uses integers (0-100,000) instead of floats
- Provides deterministic behavior across platforms
- Enables exact replay and networking

### Tick Rate: 10 TPS
- Simulation runs at fixed 10 ticks per second
- 6x performance improvement over 60 FPS updates
- Visual interpolation maintains smooth 60 FPS display

### Component Architecture
- Separation of simulation (GridPosition) and presentation (VisualPosition)
- Tick-based systems only run on simulation ticks
- Frame-based systems handle interpolation every frame

## Performance Metrics

### Current Capabilities
- Phase 1-3 complete: Can handle ~500 units smoothly
- Target: 5000+ units with all phases complete

### Bottlenecks Identified
- Pathfinding needs optimization (Phase 4 chunks will help)
- GOAP planning could use caching (Phase 7)
- Rendering needs LOD system (Phase 9)

## Next Steps

**Recommended**: Start Phase 4 (Chunks) or Phase 5 (Economy)
- Chunks will help with scalability for large worlds
- Economy builds on existing work system

## Development Notes

### Testing Commands
```bash
# Run with debug output
RUST_LOG=debug cargo run -p world_sim_simple

# Monitor performance
RUST_LOG=info cargo run -p world_sim_simple | grep FPS

# Test with many units
# Edit main.rs to spawn more peasants
```

### Key Files Modified
- `src/simulation/` - Core tick systems
- `src/components/` - New tick-based components
- `src/systems/` - Tick-based system implementations
- `src/main.rs` - Integration points

## Contributors
- Migration started: 2024-12-10
- Architecture design based on Factorio/Dwarf Fortress patterns
- Using Bevy 0.16 ECS for implementation