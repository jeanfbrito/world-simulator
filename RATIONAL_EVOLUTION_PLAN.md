# Rational Evolution Plan: world_sim_simple → world_sim_core Feature Parity

## Current State Analysis

### world_sim_simple (Working - 42 files)
**Already Implemented (Phases 1-9 Complete):**
- ✅ Component system (position, health, worker, etc.)
- ✅ Plugin architecture
- ✅ Chunk-based world (16x16 tiles per chunk)
- ✅ Basic AI with task system
- ✅ Resources and inventory
- ✅ Building system with construction
- ✅ Crafting with recipe registry
- ✅ Save/Load system with JSON serialization
- ✅ Performance optimizations (spatial indexing, parallel systems)
- ✅ WebSocket for frontend communication
- ✅ Debug system with terminal output

### world_sim_core (Complex - 55 files)
**Additional Features Not in sim_simple:**
1. **GOAP AI System** (dogoap/bevy_dogoap)
   - Goal-oriented action planning
   - State-based decision making
   - Action preconditions/effects
   
2. **Utility AI** (big-brain)
   - Scoring system for actions
   - Utility curves
   - Social and survival scorers
   
3. **Lua Scripting** (bevy_mod_scripting_lua)
   - Dynamic content loading
   - AI behavior scripts
   - Material/item/building/recipe loaders
   
4. **Advanced Tilemap** (bevy_entitiles)
   - Built-in pathfinding algorithms
   - Wave Function Collapse generation
   - LDTK level loading
   
5. **Squad Planning & Social Systems**
   - Group coordination
   - Social alerts and interactions
   - Squad-based tasks
   
6. **Level of Detail (LOD) System**
   - Dynamic simulation detail
   - Performance optimization for large worlds
   
7. **Advanced Parallel Processing**
   - SIMD optimizations
   - Task pools with bevy_tasks
   - Benchmark system

## Rational Evolution Order

### Phase 10: Advanced AI Foundation (GOAP)
**Rationale:** GOAP provides more intelligent agent behavior than simple task system
**Dependencies:** None - can layer on top of existing task system
**Effort:** 2-3 days

#### Step 10.1: GOAP State Components (2 hours)
- Port `components/goap_states.rs`
- Add state tracking to workers
- Debug validation with terminal

#### Step 10.2: GOAP Actions (3 hours)
- Port `components/goap_actions.rs`
- Define action preconditions/effects
- Integrate with existing task system

#### Step 10.3: GOAP Planner Integration (3 hours)
- Add dogoap dependency
- Port `systems/goap_systems.rs` (simplified)
- Test with debug commands

### Phase 11: Utility AI Layer
**Rationale:** Adds nuanced decision-making on top of GOAP
**Dependencies:** GOAP system (Phase 10)
**Effort:** 2 days

#### Step 11.1: Scorer System (2 hours)
- Port `ai/scorers/survival.rs`
- Port `ai/scorers/social.rs`
- Add scoring to worker decisions

#### Step 11.2: Utility Actions (3 hours)
- Port `ai/utility_actions.rs`
- Integrate with GOAP planner
- Debug output for score calculations

#### Step 11.3: Big Brain Integration (2 hours)
- Add big-brain dependency
- Connect scorers to actions
- Validate with behavior testing

### Phase 12: Squad & Social Systems
**Rationale:** Enables cooperative behaviors and emergent gameplay
**Dependencies:** GOAP + Utility AI (Phases 10-11)
**Effort:** 2 days

#### Step 12.1: Social Alerts (2 hours)
- Port `ai/social_alerts.rs`
- Add alert propagation
- Debug social interactions

#### Step 12.2: Squad Planning (3 hours)
- Port `ai/squad_planning.rs`
- Group task assignment
- Coordinate multiple workers

#### Step 12.3: Social Behaviors (2 hours)
- Add relationships/reputation
- Social need satisfaction
- Test group dynamics

### Phase 13: Advanced Tilemap Features
**Rationale:** Better world generation and pathfinding
**Dependencies:** None - enhances existing tilemap
**Effort:** 2-3 days

#### Step 13.1: Bevy Entitiles Integration (3 hours)
- Replace current tilemap with bevy_entitiles
- Maintain compatibility
- Test rendering performance

#### Step 13.2: Advanced Pathfinding (2 hours)
- Use built-in pathfinding algorithms
- Compare performance with current A*
- Debug path visualization

#### Step 13.3: Wave Function Collapse (3 hours)
- Add procedural generation
- Create tile rules
- Test world variety

### Phase 14: Scripting System (Optional)
**Rationale:** Enables modding and dynamic content
**Dependencies:** None - independent system
**Effort:** 3-4 days
**Note:** May skip if not needed - adds significant complexity

#### Step 14.1: Lua Runtime (4 hours)
- Add bevy_mod_scripting_lua
- Basic script execution
- Safety sandboxing

#### Step 14.2: Content Loaders (4 hours)
- Port material/item/building loaders
- Script-based recipes
- Validate loaded content

#### Step 14.3: AI Scripts (4 hours)
- Port `scripting/ai_scripts.rs`
- Behavior modification via Lua
- Debug script execution

### Phase 15: Performance Optimizations
**Rationale:** Scale to larger worlds
**Dependencies:** All core systems complete
**Effort:** 2 days

#### Step 15.1: LOD System (3 hours)
- Port `ai/lod_system.rs`
- Dynamic detail levels
- Performance monitoring

#### Step 15.2: Parallel Processing (3 hours)
- Port `ai/parallel_processing.rs`
- Task pool optimization
- Benchmark improvements

#### Step 15.3: SIMD Optimizations (2 hours)
- Add packed_simd_2 (optional)
- Vectorize hot paths
- Measure performance gains

## Implementation Priority

### Must Have (Phases 10-12)
**Timeline:** 6-7 days
1. GOAP AI - Core intelligence upgrade
2. Utility AI - Decision refinement  
3. Squad/Social - Emergent gameplay

### Should Have (Phase 13)
**Timeline:** 2-3 days
4. Advanced Tilemap - Better worlds

### Nice to Have (Phases 14-15)
**Timeline:** 5-6 days
5. Scripting - Modding support (complex)
6. Performance - Scaling optimizations

## Success Metrics

Each phase validated by:
1. **Compilation:** `cargo build` succeeds
2. **Debug Output:** Expected logs in terminal
3. **Functionality:** Feature works as intended
4. **Performance:** No FPS degradation
5. **Compatibility:** Existing features unbroken

## Risk Mitigation

1. **Complex Dependencies:** Start with GOAP (well-documented)
2. **Lua/Scripting:** Make optional - high complexity
3. **Performance:** Profile before optimizing
4. **Testing:** Debug-first validation at each step

## Next Steps

1. Begin Phase 10.1 - GOAP State Components
2. Follow debug-first workflow from CLAUDE.md
3. Commit after each working step
4. Re-evaluate plan after Phase 12

This evolution maintains sim_simple's stability while incrementally adding sim_core's advanced features in order of value and dependency.