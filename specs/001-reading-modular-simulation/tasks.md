# Tasks: Headless Modular Simulation Engine

**Input**: Design documents from `/specs/001-reading-modular-simulation/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/, quickstart.md

## Execution Summary

Building a headless Rust/Bevy simulation engine with pluggable visualizers. Core engine has ZERO rendering dependencies, uses ECS architecture, and communicates via events/commands.

**Tech Stack**: Rust, Bevy ECS (MinimalPlugins), serde
**Structure**: Workspace with core engine + optional visualizer crates
**Testing**: TDD with cargo test, property-based testing

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Phase 3.1: Setup & Project Structure

- [x] T001 Initialize Rust workspace at repository root with Cargo.toml
- [x] T002 Create world_sim_core crate with `cargo new world_sim_core --lib`
- [x] T003 [P] Create world_sim_interface crate with `cargo new world_sim_interface --lib`
- [x] T004 [P] Create world_sim_bevy_viz crate with `cargo new world_sim_bevy_viz --lib`
- [x] T005 [P] Create world_sim_terminal crate with `cargo new world_sim_terminal --lib`
- [x] T006 Configure workspace dependencies in root Cargo.toml
- [x] T007 [P] Setup rustfmt.toml and clippy.toml for linting
- [x] T008 [P] Create examples/ directory with placeholder files
- [x] T009 Add Bevy 0.14 dependency to world_sim_core (MinimalPlugins only!)
- [x] T010 [P] Add serde and serde_json to world_sim_interface

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3

### Contract Tests
- [x] T011 [P] Write failing test for EngineEvent serialization in world_sim_interface/tests/events_test.rs
- [x] T012 [P] Write failing test for EngineCommand serialization in world_sim_interface/tests/commands_test.rs
- [x] T013 [P] Write failing test for WorldSnapshot structure in world_sim_interface/tests/snapshot_test.rs
- [x] T014 [P] Write failing test for SimulationEngine trait in world_sim_core/tests/engine_test.rs
- [x] T015 [P] Write failing test for EngineObserver trait in world_sim_interface/tests/observer_test.rs

### Integration Tests
- [ ] T016 Write failing test for world generation in world_sim_core/tests/world_generation_test.rs
- [ ] T017 Write failing test for harvest system in world_sim_core/tests/harvest_system_test.rs
- [ ] T018 Write failing test for building construction in world_sim_core/tests/building_system_test.rs
- [ ] T019 Write failing test for recipe processing in world_sim_core/tests/recipe_system_test.rs
- [ ] T020 Write failing test for population management in world_sim_core/tests/population_test.rs
- [ ] T021 Write failing test for event emission in world_sim_core/tests/event_emission_test.rs
- [ ] T022 Write failing test for deterministic simulation in world_sim_core/tests/determinism_test.rs

## Phase 3.3: Core Implementation

### Interface Types (world_sim_interface)
- [x] T023 [P] Implement core types (EntityId, Position, Tick) in world_sim_interface/src/types.rs
- [x] T024 [P] Implement EngineEvent enum in world_sim_interface/src/events.rs
- [x] T025 [P] Implement EngineCommand enum in world_sim_interface/src/commands.rs
- [x] T026 [P] Implement EntityType, ResourceType, BuildingType enums in world_sim_interface/src/entities.rs
- [x] T027 [P] Implement WorldSnapshot and related types in world_sim_interface/src/state.rs
- [x] T028 [P] Implement EngineObserver trait in world_sim_interface/src/observer.rs
- [x] T029 [P] Implement CommandResult type in world_sim_interface/src/results.rs
- [x] T030 Create lib.rs re-exporting all public types in world_sim_interface/src/lib.rs

### ECS Components (world_sim_core)
- [ ] T031 [P] Implement PositionComponent in world_sim_core/src/components/position.rs
- [ ] T032 [P] Implement ResourceNodeComponent in world_sim_core/src/components/resources.rs
- [ ] T033 [P] Implement WorkerComponent in world_sim_core/src/components/workers.rs
- [ ] T034 [P] Implement BuildingComponent in world_sim_core/src/components/buildings.rs
- [ ] T035 [P] Implement InventoryComponent in world_sim_core/src/components/inventory.rs
- [ ] T036 [P] Implement TaskComponent in world_sim_core/src/components/tasks.rs
- [ ] T037 Create components module in world_sim_core/src/components/mod.rs

### Core Systems (world_sim_core)
- [ ] T038 Implement harvest_system in world_sim_core/src/systems/harvest_system.rs
- [ ] T039 Implement movement_system in world_sim_core/src/systems/movement_system.rs
- [ ] T040 Implement recipe_system in world_sim_core/src/systems/recipe_system.rs
- [ ] T041 Implement building_system in world_sim_core/src/systems/building_system.rs
- [ ] T042 Implement population_system in world_sim_core/src/systems/population_system.rs
- [ ] T043 Implement resource_regeneration_system in world_sim_core/src/systems/regeneration.rs
- [ ] T044 Create systems module in world_sim_core/src/systems/mod.rs

### Resources & Configuration (world_sim_core)
- [ ] T045 [P] Implement WorldState resource in world_sim_core/src/resources/world_state.rs
- [ ] T046 [P] Implement RecipeRegistry in world_sim_core/src/resources/recipes.rs
- [ ] T047 [P] Implement GameConfig in world_sim_core/src/resources/config.rs
- [ ] T048 [P] Create MVP recipes (house, sawmill, stockpile) in world_sim_core/src/data/recipes.json
- [ ] T049 [P] Create resource definitions in world_sim_core/src/data/resources.json
- [ ] T050 [P] Create building definitions in world_sim_core/src/data/buildings.json

### Event System (world_sim_core)
- [ ] T051 Implement EventQueue for collecting events in world_sim_core/src/events/queue.rs
- [ ] T052 Implement event emission in systems in world_sim_core/src/events/emitter.rs
- [ ] T053 Implement observer management in world_sim_core/src/events/observers.rs
- [ ] T054 Create events module in world_sim_core/src/events/mod.rs

### Engine API (world_sim_core)
- [ ] T055 Implement SimulationEngine struct in world_sim_core/src/engine.rs
- [ ] T056 Implement new_world method in world_sim_core/src/engine.rs
- [ ] T057 Implement execute_command method in world_sim_core/src/engine.rs
- [ ] T058 Implement tick method with deterministic ordering in world_sim_core/src/engine.rs
- [ ] T059 Implement snapshot method in world_sim_core/src/engine.rs
- [ ] T060 Implement observer management methods in world_sim_core/src/engine.rs

### World Generation (world_sim_core)
- [ ] T061 Implement basic world generation in world_sim_core/src/world/generator.rs
- [ ] T062 Implement resource node placement in world_sim_core/src/world/resources.rs
- [ ] T063 Implement starting worker spawn in world_sim_core/src/world/population.rs
- [ ] T064 Create world module in world_sim_core/src/world/mod.rs

### Core Plugin (world_sim_core)
- [ ] T065 Implement SimulationPlugin for Bevy in world_sim_core/src/plugin.rs
- [ ] T066 Wire up all systems in plugin in world_sim_core/src/plugin.rs
- [ ] T067 Create main lib.rs with public API in world_sim_core/src/lib.rs

## Phase 3.4: Visualizers

### Terminal Visualizer (world_sim_terminal)
- [ ] T068 [P] Implement ASCII grid renderer in world_sim_terminal/src/renderer.rs
- [ ] T069 [P] Implement EngineObserver for terminal in world_sim_terminal/src/observer.rs
- [ ] T070 [P] Implement keyboard input handler in world_sim_terminal/src/input.rs
- [ ] T071 [P] Create TerminalPlugin in world_sim_terminal/src/lib.rs

### Bevy 2D Visualizer (world_sim_bevy_viz)
- [ ] T072 [P] Implement sprite spawning system in world_sim_bevy_viz/src/sprites.rs
- [ ] T073 [P] Implement camera controller in world_sim_bevy_viz/src/camera.rs
- [ ] T074 [P] Implement UI overlay with egui in world_sim_bevy_viz/src/ui.rs
- [ ] T075 [P] Implement EngineObserver for Bevy in world_sim_bevy_viz/src/observer.rs
- [ ] T076 [P] Create VisualizationPlugin in world_sim_bevy_viz/src/lib.rs

## Phase 3.5: Examples & Integration

### Example Programs
- [ ] T077 Create headless.rs example running pure simulation
- [ ] T078 Create terminal.rs example with ASCII display
- [ ] T079 Create with_graphics.rs example with Bevy 2D
- [ ] T080 Create benchmark.rs for performance testing

### Save/Load System
- [ ] T081 Implement world serialization in world_sim_core/src/save/serializer.rs
- [ ] T082 Implement world deserialization in world_sim_core/src/save/deserializer.rs
- [ ] T083 Add save/load commands to engine API

## Phase 3.6: Testing & Documentation

### Performance Tests
- [ ] T084 [P] Write benchmark for 1,000 entities in benches/entities.rs
- [ ] T085 [P] Write benchmark for tick performance in benches/tick.rs
- [ ] T086 [P] Write memory usage test in tests/memory.rs

### Documentation
- [ ] T087 [P] Write rustdoc comments for all public APIs
- [ ] T088 [P] Create README.md with usage examples
- [ ] T089 [P] Document plugin development guide
- [ ] T090 [P] Create CONTRIBUTING.md with development workflow

## Phase 3.7: Polish & Optimization

### Optimizations
- [ ] T091 Implement spatial indexing (quadtree) for position queries
- [ ] T092 Add component pools for memory efficiency
- [ ] T093 Implement parallel system execution where safe
- [ ] T094 Add profiling instrumentation

### CI/CD
- [ ] T095 [P] Setup GitHub Actions for Rust tests
- [ ] T096 [P] Add clippy and fmt checks to CI
- [ ] T097 [P] Setup release workflow for crates.io
- [ ] T098 [P] Add benchmark regression tests

### Final Integration
- [ ] T099 Verify all tests pass with `cargo test --workspace`
- [ ] T100 Run full benchmark suite and document performance

## Execution Strategy

### Parallel Execution Groups

**Group 1 (Setup)**: T003, T004, T005, T007, T008, T010
```bash
# Can run simultaneously
cargo new world_sim_interface --lib &
cargo new world_sim_bevy_viz --lib &
cargo new world_sim_terminal --lib &
```

**Group 2 (Contract Tests)**: T011-T015
```bash
# All contract tests can run in parallel
cargo test --package world_sim_interface
```

**Group 3 (Interface Types)**: T023-T029
```bash
# All interface implementations in parallel
# Different files, no conflicts
```

**Group 4 (Components)**: T031-T036
```bash
# All component files are independent
```

**Group 5 (Data Files)**: T045-T050
```bash
# JSON data files and resources
```

### Dependencies
- Setup (T001-T010) → Tests (T011-T022) → Implementation (T023-T067)
- Core must complete before visualizers (T068-T076)
- All implementation before examples (T077-T080)
- Everything before final testing (T099-T100)

### Critical Path
1. Workspace setup (T001-T006)
2. Interface types (T023-T030)
3. Core engine (T055-T060)
4. Basic systems (T038-T044)
5. Integration test passes (T016-T022)

## Success Criteria
- [ ] All 100 tasks completed
- [ ] All tests passing (`cargo test --workspace`)
- [ ] Benchmarks show <1ms tick for 1,000 entities
- [ ] Examples run without graphics dependencies
- [ ] Documentation complete

## Notes
- Use `cargo watch -x test` for TDD workflow
- Commit after each passing test
- No DefaultPlugins in core - only MinimalPlugins!
- Event-driven architecture throughout
- Deterministic simulation required