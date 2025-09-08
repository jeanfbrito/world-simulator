# Implementation Plan: Modular Simulation Engine

**Branch**: `001-reading-modular-simulation` | **Date**: 2025-09-08 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-reading-modular-simulation/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
4. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
5. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, or `GEMINI.md` for Gemini CLI).
6. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
7. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
8. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Building a pure headless simulation engine for medieval economy and fortress games with ECS architecture, supporting 1,000+ entities, recipe-based resource management, with pluggable visualization (2D, 3D, terminal, web). The engine has ZERO rendering dependencies and communicates through events/commands, allowing any frontend to connect.

## Technical Context
**Language/Version**: Rust with Bevy 0.14+ (following original spec)  
**Primary Dependencies**: Bevy ECS (no DefaultPlugins!), serde (serialization)  
**Storage**: In-memory ECS during play, SQLite/binary for save files only  
**Testing**: Rust built-in tests, cargo test, property testing  
**Target Platform**: Headless engine core, with pluggable frontends
**Project Type**: Pure simulation engine with event-driven API  
**Performance Goals**: Handle 10,000+ concurrent entities, <1ms tick time  
**Constraints**: <200MB memory usage, deterministic simulation  
**Scale/Scope**: Core engine with 10-20 resource types, modular systems

**Critical Architecture Points**:
- **NO rendering in core**: Use MinimalPlugins, not DefaultPlugins
- **Event-driven API**: All communication through events/commands
- **Plugin architecture**: Visualizers as separate crates
- **Any frontend**: Unity, Godot, Unreal, Bevy, Terminal, Web
- **Deterministic**: Same inputs = same outputs (for multiplayer)

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Simplicity**:
- Projects: 3 (backend, frontend, tests) ✓
- Using framework directly? YES - Express, Socket.io, React used directly
- Single data model? YES - shared entity definitions
- Avoiding patterns? YES - direct service calls, no unnecessary abstractions

**Architecture**:
- EVERY feature as library? YES - separate crates with clean APIs
- Libraries listed: 
  - world_sim_core - Headless simulation engine (NO rendering)
  - world_sim_interface - Shared types for events/commands
  - world_sim_bevy_viz - OPTIONAL Bevy 2D visualization
  - world_sim_terminal - OPTIONAL ASCII renderer
  - world_sim_network - OPTIONAL networking layer
  - world_sim_unity_bridge - OPTIONAL Unity FFI bindings
- CLI per library: Each crate exposes CLI for testing
- Library docs: Rust docs + integration examples

**Testing (NON-NEGOTIABLE)**:
- RED-GREEN-Refactor cycle enforced? YES - tests written first
- Git commits show tests before implementation? YES
- Order: Contract→Integration→E2E→Unit strictly followed? YES
- Real dependencies used? YES - PostgreSQL, Redis in tests
- Integration tests for: new libraries, contract changes, shared schemas? YES
- FORBIDDEN: Implementation before test, skipping RED phase ✓

**Observability**:
- Structured logging included? YES - Winston with structured JSON
- Frontend logs → backend? YES - unified logging stream
- Error context sufficient? YES - request IDs, entity states

**Versioning**:
- Version number assigned? 0.1.0 for MVP
- BUILD increments on every change? YES
- Breaking changes handled? N/A for initial release

## Project Structure

### Documentation (this feature)
```
specs/[###-feature]/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Workspace structure for decoupled engine
world-simulator/
├── Cargo.toml                    # Workspace root
├── world_sim_core/               # HEADLESS ENGINE
│   ├── src/
│   │   ├── components/          # ECS components
│   │   ├── systems/            # Game logic
│   │   ├── events/             # Event definitions
│   │   └── api/                # Public API
│   └── tests/
├── world_sim_interface/          # Shared types
│   └── src/
│       ├── events.rs
│       ├── commands.rs
│       └── state.rs
├── world_sim_bevy_viz/          # OPTIONAL: Bevy renderer
│   └── src/
├── world_sim_terminal/          # OPTIONAL: ASCII display
│   └── src/
├── world_sim_network/           # OPTIONAL: Networking
│   └── src/
├── world_sim_unity_bridge/      # OPTIONAL: Unity FFI
│   └── src/
└── examples/
    ├── headless.rs              # Pure simulation
    ├── with_graphics.rs         # With Bevy viz
    └── terminal.rs              # ASCII version
```

**Structure Decision**: Workspace with headless core + pluggable visualizers

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships
   - Validation rules from requirements
   - State transitions if applicable

2. **Generate API contracts** from functional requirements:
   - For each user action → endpoint
   - Use standard REST/GraphQL patterns
   - Output OpenAPI/GraphQL schema to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per endpoint
   - Assert request/response schemas
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each story → integration test scenario
   - Quickstart test = story validation steps

5. **Update agent file incrementally** (O(1) operation):
   - Run `/scripts/update-agent-context.sh [claude|gemini|copilot]` for your AI assistant
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each API endpoint → contract test task [P]
- Each WebSocket event → event test task [P]
- Each entity → model creation task [P] 
- Each user story → integration test task
- Implementation tasks to make tests pass

**Task Categories**:
1. **Setup Tasks** (1-5): Project initialization, dependencies, Docker setup
2. **Model Tasks** (6-15): Entity models, components, data structures [P]
3. **Contract Tests** (16-25): API and WebSocket contract tests [P]
4. **Core Systems** (26-35): ECS engine, resource system, recipe processor
5. **API Implementation** (36-45): REST endpoints, WebSocket handlers
6. **Frontend Tasks** (46-55): UI components, game canvas, state management
7. **Integration Tests** (56-60): End-to-end scenarios, multiplayer tests
8. **Performance Tests** (61-65): Load testing, memory profiling

**Ordering Strategy**:
- TDD order: Tests before implementation always
- Dependency order: Models → Systems → API → Frontend
- Mark [P] for parallel execution (independent files)
- Critical path: Setup → Models → Core Systems → Integration

**Estimated Output**: 60-65 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented (none required)

---
*Based on Constitution v2.1.1 - See `/memory/constitution.md`*