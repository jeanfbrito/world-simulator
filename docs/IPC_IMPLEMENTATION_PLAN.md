# IPC Implementation Plan: World Simulator Communication Architecture

## Executive Summary

This document outlines the implementation of a high-performance Inter-Process Communication (IPC) system between the headless World Simulator and Sim Viewer using JSON over Stdout/Stdin pipes. This architecture enables clean separation between simulation logic and visualization while maintaining excellent performance.

## Architecture Overview

### Three-Tier Architecture

```
┌──────────────────┐    Stdout (JSON)    ┌──────────────────┐    WebSocket    ┌──────────────────┐
│  World Simulator│ ──────────────────> │  Sim Viewer     │ ──────────────> │  Web Viewer     │
│  (Headless)      │                    │  (Rust Bridge)   │                │  (HTML/JS)      │
└──────────────────┘                    └──────────────────┘                └──────────────────┘
```

### Component Responsibilities

1. **World Simulator**: Pure simulation logic, outputs state via IPC
2. **Sim Viewer**: IPC bridge + WebSocket server, manages web clients
3. **Web Viewer**: Pure web technologies, renders based on pack data

## Implementation Tasks

### Phase 1: Foundation & Protocol Design (3-4 days)

#### Task 1.1: Define IPC Message Specifications ✅ COMPLETED
- **Status**: Complete
- **Files Modified**: `world_sim_interface/src/ipc.rs`
- **Description**: Created comprehensive message schemas, versioning strategy, error handling

#### Task 1.2: Implement Simulator IPC Output
- **Status**: Pending
- **Estimated Time**: 1-2 days
- **Dependencies**: Task 1.1
- **Description**:
  - Remove WebSocket code from simulator
  - Create `ipc_output.rs` module in simulator
  - Implement JSON serialization for game state
  - Add buffered output for performance optimization
  - Create message batching for high-frequency updates

#### Task 1.3: Create Sim Viewer Project Structure
- **Status**: Pending
- **Estimated Time**: 1 day
- **Dependencies**: Task 1.1
- **Description**:
  - Initialize `sim_viewer` Rust crate
  - Set up Cargo.toml with dependencies (tokio, serde, etc.)
  - Create modular project structure
  - Implement basic IPC reader from stdin

### Phase 2: Core Communication System (3-4 days)

#### Task 2.1: Implement IPC Reader in Sim Viewer
- **Status**: Pending
- **Estimated Time**: 1-2 days
- **Dependencies**: Task 1.3
- **Description**:
  - Create `ipc_reader.rs` with async line reading
  - Implement JSON deserialization with error handling
  - Add message validation and filtering
  - Implement backpressure handling for fast simulator output
  - Add message buffering and reordering prevention

#### Task 2.2: Create WebSocket Server in Sim Viewer
- **Status**: Pending
- **Estimated Time**: 1-2 days
- **Dependencies**: Task 1.3
- **Description**:
  - Implement async WebSocket server using tokio-tungstenite
  - Handle client connections and disconnections
  - Implement message broadcasting to multiple clients
  - Add connection pooling and management
  - Create client authentication (if needed)

#### Task 2.3: Bridge IPC and WebSocket
- **Status**: Pending
- **Estimated Time**: 1 day
- **Dependencies**: Tasks 2.1, 2.2
- **Description**:
  - Create message routing system
  - Implement protocol translation (IPC → WebSocket)
  - Add state synchronization and consistency
  - Handle message transformation and filtering
  - Implement client-specific message filtering

### Phase 3: Pack Integration (2-3 days)

#### Task 3.1: Pack Data Transmission
- **Status**: Pending
- **Estimated Time**: 1-2 days
- **Dependencies**: Task 2.3
- **Description**:
  - Extend IPC protocol with pack definitions
  - Implement pack metadata serialization
  - Create visual registry transmission system
  - Add pack hot-reload notification system
  - Implement pack dependency resolution

#### Task 3.2: Web Viewer Pack Integration
- **Status**: Pending
- **Estimated Time**: 1 day
- **Dependencies**: Task 2.3
- **Description**:
  - Create pack loader in web viewer JavaScript
  - Implement dynamic visual definition loading
  - Create fallback rendering for undefined visuals
  - Add theme system integration
  - Implement pack hot-reload in web client

#### Task 3.3: Migrate Current Viewer
- **Status**: Pending
- **Estimated Time**: 1 day
- **Dependencies**: Tasks 3.1, 3.2
- **Description**:
  - Copy current `viewer.html` to Sim Viewer project
  - Remove hardcoded visual configurations
  - Integrate with pack-driven rendering system
  - Test compatibility with existing simulation

### Phase 4: Performance Optimization (1-2 days)

#### Task 4.1: Performance Optimization
- **Status**: Pending
- **Estimated Time**: 1-2 days
- **Dependencies**: All previous tasks
- **Description**:
  - Implement message batching and compression
  - Add differential state updates (only changed entities)
  - Create client-side interpolation for smooth animations
  - Implement bandwidth adaptive quality settings
  - Add memory management for large game states

### Phase 5: Advanced Features & Polish (2-3 days)

#### Task 5.1: Error Handling & Resilience
- **Status**: Pending
- **Estimated Time**: 1 day
- **Dependencies**: Task 4.1
- **Description**:
  - Implement connection recovery and reconnection
  - Add heartbeat/keepalive system
  - Create message acknowledgment system
  - Implement graceful degradation on failures
  - Add comprehensive logging and diagnostics

#### Task 5.2: Development Tools
- **Status**: Pending
- **Estimated Time**: 1 day
- **Dependencies**: Task 5.1
- **Description**:
  - Create IPC message debugger/sniffer
  - Implement protocol version compatibility checking
  - Add performance monitoring and metrics
  - Create message validation and testing tools
  - Develop documentation generation system

#### Task 5.3: Documentation & Examples
- **Status**: Pending
- **Estimated Time**: 1 day
- **Dependencies**: All previous tasks
- **Description**:
  - Write comprehensive IPC protocol documentation
  - Create example integrations and tutorials
  - Implement reference implementations
  - Add troubleshooting guide
  - Create performance benchmarks and optimization guide

## Technical Specifications

### Message Format

#### Core IPC Message
```json
{
  "version": 1,
  "timestamp": 1672531200,
  "seq_num": 123,
  "payload": {
    "type": "GameState",
    "data": {
      "tick": 456,
      "world_size": [64, 64],
      "entities": [...],
      "global_state": {...}
    }
  }
}
```

#### Visual Registry Format
```json
{
  "tiles": {
    "grass": {
      "name": "Grass",
      "color": "#3a5f3a",
      "emoji": "🌱",
      "blocks_movement": false
    }
  },
  "entities": {
    "peasant": {
      "name": "Peasant",
      "color": "#8B4513",
      "emoji": "👨‍🌾",
      "size": [1.0, 1.0],
      "animations": {...}
    }
  }
}
```

### Performance Requirements

- **Throughput**: 500+ state updates per second
- **Latency**: <5ms from simulator to web viewer
- **Memory**: <50MB baseline for Sim Viewer
- **CPU**: <10% utilization on modern hardware
- **Error Rate**: <0.1% of messages fail
- **Recovery Time**: <1s for connection failures

### File Structure

```
world-simulator/
├── world_sim_interface/           # Shared interface types
│   └── src/
│       └── ipc.rs                # ✅ IPC protocol definitions
├── world_sim_simple/              # Headless simulator
│   ├── src/
│   │   ├── ipc_output.rs         # ❌ To be created
│   │   └── main.rs               # ❌ To be modified
│   └── Cargo.toml
├── sim_viewer/                   # ❌ To be created
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs               # IPC + WebSocket bridge
│   │   ├── ipc_reader.rs         # IPC input handling
│   │   ├── websocket.rs          # WebSocket server
│   │   ├── pack_loader.rs        # Pack integration
│   │   └── state_manager.rs      # State management
│   └── web/                      # Web viewer files
│       ├── index.html            # Main viewer
│       ├── css/
│       └── js/
└── docs/
    └── IPC_IMPLEMENTATION_PLAN.md # ✅ This document
```

## Success Criteria

### Functional Requirements
- [ ] Simulator outputs game state via IPC without WebSocket dependency
- [ ] Sim Viewer receives IPC messages and broadcasts to web clients
- [ ] Web viewers connect and display real-time game state
- [ ] Pack definitions are transmitted and used for rendering
- [ ] Multiple web viewers can connect simultaneously
- [ ] System handles connection failures gracefully

### Performance Requirements
- [ ] Simulator performance is not degraded by IPC output
- [ ] Message latency remains below 5ms under normal load
- [ ] System supports 500+ state updates per second
- [ ] Memory usage remains under 50MB for Sim Viewer
- [ ] No message loss during normal operation

### Reliability Requirements
- [ ] Message sequence numbers prevent loss/reordering
- [ ] Connection recovery works automatically
- [ ] Version mismatch is handled gracefully
- [ ] Error messages provide clear diagnostic information
- [ ] System remains stable under high load

## Risk Mitigation

### Technical Risks
- **Performance Impact**: Implement batching and differential updates
- **Message Loss**: Add sequence numbers and acknowledgment system
- **Memory Issues**: Implement proper resource management and monitoring
- **Deadlocks**: Use async programming and timeouts
- **Protocol Evolution**: Implement version negotiation and compatibility

### Integration Risks
- **Breaking Changes**: Maintain backward compatibility where possible
- **Testing Complexity**: Implement comprehensive test coverage
- **Platform Issues**: Test on Windows, macOS, and Linux
- **Browser Compatibility**: Support modern browsers with fallbacks

## Timeline

**Total Estimated Time**: 12-16 days

- **Week 1**: Phase 1 - Foundation & Protocol Design (Tasks 1.1-1.3)
- **Week 2**: Phase 2 - Core Communication System (Tasks 2.1-2.3)
- **Week 3**: Phase 3 - Pack Integration (Tasks 3.1-3.3)
- **Week 4**: Phase 4 & 5 - Optimization & Polish (Tasks 4.1-5.3)

## Next Steps

1. **Immediate**: Start Task 1.2 - Implement Simulator IPC Output
2. **Parallel**: Begin setting up Sim Viewer project structure
3. **Testing**: Create test harness for IPC message validation
4. **Documentation**: Keep this document updated with progress

## Quality Assurance

### Testing Strategy
- **Unit Tests**: Message serialization/deserialization, IPC components
- **Integration Tests**: End-to-end message flow, client connection handling
- **Performance Tests**: Load testing, memory usage monitoring
- **Compatibility Tests**: Cross-platform, browser compatibility

### Code Quality
- **Documentation**: All public APIs documented
- **Error Handling**: Comprehensive error handling with user-friendly messages
- **Logging**: Structured logging for debugging and monitoring
- **Code Reviews**: All changes reviewed before merging

---

*This document will be updated as implementation progresses. Last updated: Current date*