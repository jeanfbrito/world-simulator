# Research Findings: Modular Simulation Engine

## Executive Summary
Research conducted to resolve technical decisions for implementing a modular simulation engine supporting medieval economy gameplay with ECS architecture, real-time multiplayer, and multiple frontend support.

## Technology Stack Decisions

### Core Runtime: TypeScript/Node.js (MVP) → Rust/Bevy (Production)
**MVP Decision**: TypeScript with Node.js 20+ for initial backend
**Production Plan**: Migrate to Rust/Bevy after MVP validation

**Rationale for MVP Choice**: 
- Rapid prototyping to validate game mechanics quickly
- Easier debugging and iteration during development
- Larger pool of developers for initial team
- Can handle 1,000 entities for MVP requirements
- Faster time-to-market (6-8 weeks vs 16+ weeks)

**Migration Path to Rust/Bevy**:
- Keep same API contracts (REST/WebSocket)
- Port proven game logic after mechanics validated
- Gradual migration: TypeScript frontend → Rust backend
- Performance gains: 10x+ improvement expected
- Better memory safety and concurrency

**Why Not Rust/Bevy Initially**:
- Longer development time (2-3x for MVP)
- Steeper learning curve for team
- More complex debugging during rapid iteration
- Overkill for 1,000 entity MVP target

**Alternatives Considered**:
- Go: Good concurrency but limited game dev ecosystem
- Python: Too slow for real-time simulation
- C++: Too complex for rapid prototyping

### ECS Architecture: Custom TypeScript Implementation
**Decision**: Lightweight custom ECS using TypeScript classes and arrays
**Rationale**:
- Full control over performance optimizations
- Tailored to simulation requirements
- Avoid heavy game engine overhead
**Alternatives Considered**:
- bitecs: WebAssembly-based but complex integration
- ecsy: Abandoned project
- tick-knock: Too minimal for requirements

### Real-time Communication: Socket.io
**Decision**: Socket.io for WebSocket communication
**Rationale**:
- Automatic fallback mechanisms
- Room-based multiplayer built-in
- Excellent reconnection handling
- Binary data support for efficient updates
**Alternatives Considered**:
- Raw WebSockets: More work for reliability
- gRPC-web: Poor browser support
- GraphQL subscriptions: Overhead for real-time game data

### Data Persistence: In-Memory + SQLite for Saves
**Decision**: In-memory game state with SQLite for save files
**Rationale**:
- Game engines keep state in memory during play
- SQLite for save/load functionality only
- No database latency during gameplay
- Follows standard game engine patterns (Unity, Godot, Bevy)

**Better Approach for Game Engine**:
- **Runtime State**: Pure in-memory ECS (like Bevy)
- **Save System**: SQLite or binary files for persistence
- **Networking**: Direct memory synchronization via binary protocol
- **No Database During Play**: Database only for saves/loads

**Why PostgreSQL/Redis Are Wrong Here**:
- Network latency kills game performance
- Games need microsecond access, not millisecond
- ECS already provides efficient data structure
- Database abstraction unnecessary overhead
- Real game engines never use external DBs for runtime

**Alternatives Considered**:
- PostgreSQL: Too slow for real-time game state
- Redis: Still network overhead, unnecessary
- Binary files: Good for saves, used by most games
- Memory-mapped files: Good for large worlds

### Frontend Framework: React with Canvas
**Decision**: React for UI, HTML5 Canvas for game visualization
**Rationale**:
- React for UI controls and menus
- Canvas for efficient grid-based world rendering
- Pixi.js for sprite management if needed
**Alternatives Considered**:
- Pure Canvas: Difficult UI management
- Three.js: Overkill for 2D grid view
- Phaser: Too game-focused, want modular approach

## Performance Optimization Strategies

### Entity Management
- **Spatial indexing**: QuadTree for efficient area queries
- **Component arrays**: Structure-of-arrays for cache efficiency
- **Dirty flagging**: Only update changed entities
- **Object pooling**: Reuse entity objects to reduce GC

### Network Optimization
- **Delta compression**: Send only changed properties
- **Binary protocol**: MessagePack for efficient serialization
- **Batched updates**: Combine multiple changes per tick
- **Area of interest**: Only send relevant entity updates

### Database Optimization
- **Batch inserts**: PostgreSQL COPY for bulk operations
- **Connection pooling**: pg-pool with 20 connections
- **Prepared statements**: Cached query plans
- **Partial indexes**: For common query patterns

## Multiplayer Architecture

### Session Management
- **Authoritative server**: All game logic server-side
- **Client prediction**: Immediate visual feedback
- **Lag compensation**: Interpolation between states
- **Rollback on conflict**: Server state is truth

### Cooperative Play
- **Shared world state**: All players see same entities
- **Permission system**: Role-based action authorization
- **Conflict resolution**: Last-write-wins with timestamps
- **Synchronized commands**: Command queue with ordering

## Recipe System Design

### Data Structure
```typescript
interface Recipe {
  id: string;
  inputs: { resourceId: string; quantity: number }[];
  outputs: { resourceId: string; quantity: number }[];
  duration: number; // ticks
  requirements?: {
    buildings?: string[];
    research?: string[];
    workers?: { skill: string; level: number }[];
  };
}
```

### Processing Pipeline
1. **Validation**: Check all requirements met
2. **Reservation**: Lock required resources
3. **Processing**: Track progress over time
4. **Completion**: Generate outputs, release locks
5. **Events**: Emit completion notifications

## Testing Strategy

### Unit Tests
- **Components**: Individual ECS component logic
- **Systems**: System update functions in isolation
- **Recipes**: Recipe validation and calculation

### Integration Tests
- **API endpoints**: Contract testing with supertest
- **WebSocket events**: Socket.io-client for testing
- **Database operations**: Real PostgreSQL in Docker
- **Multi-client scenarios**: Concurrent connection tests

### E2E Tests
- **Playwright**: Full user workflows
- **Game scenarios**: Automated gameplay testing
- **Performance tests**: Load testing with k6
- **Multiplayer sync**: Multiple browser instances

## Development Workflow

### Local Development
```bash
# Backend
npm run dev:backend    # Nodemon with TypeScript
npm run test:watch     # Jest in watch mode

# Frontend  
npm run dev:frontend   # Vite dev server
npm run storybook      # Component development

# Full stack
docker-compose up      # PostgreSQL, Redis, both apps
```

### CI/CD Pipeline
- **Pre-commit**: ESLint, Prettier, type checking
- **GitHub Actions**: Test on push, deploy on merge
- **Docker images**: Multi-stage builds for production
- **Health checks**: /health endpoint monitoring

## Security Considerations

### Input Validation
- **Command validation**: Joi schemas for all inputs
- **Rate limiting**: Express-rate-limit per user
- **SQL injection**: Parameterized queries only
- **XSS prevention**: DOMPurify for user content

### Authentication
- **JWT tokens**: Short-lived access tokens
- **Refresh tokens**: Stored in httpOnly cookies
- **Session management**: Redis with TTL
- **Permission checks**: Middleware for all routes

## Monitoring & Observability

### Logging
- **Winston**: Structured JSON logging
- **Log levels**: ERROR, WARN, INFO, DEBUG
- **Request correlation**: UUID per request
- **Performance metrics**: Response time, DB queries

### Metrics
- **Prometheus**: Time-series metrics
- **Custom metrics**: Entity count, tick duration
- **Grafana dashboards**: Real-time monitoring
- **Alerts**: PagerDuty for critical issues

## Deployment Architecture

### Infrastructure
- **Container orchestration**: Kubernetes for scaling
- **Load balancing**: NGINX for WebSocket support
- **CDN**: CloudFlare for static assets
- **Database hosting**: Managed PostgreSQL (RDS/CloudSQL)

### Scaling Strategy
- **Horizontal scaling**: Multiple backend instances
- **Sticky sessions**: Required for WebSocket
- **Read replicas**: For database queries
- **Cache layer**: Redis for frequent reads

## Risk Mitigation

### Technical Risks
- **Performance degradation**: Profiling tools ready, optimization paths identified
- **State synchronization**: Comprehensive testing, rollback mechanisms
- **Database bottlenecks**: Query optimization, caching strategies
- **Memory leaks**: Heap profiling, automated restarts

### Project Risks
- **Scope creep**: Clear MVP boundaries defined
- **Technical debt**: Refactoring time allocated
- **Testing overhead**: Automated test generation
- **Documentation lag**: Inline documentation required

## Conclusions

The chosen technology stack provides:
1. **Rapid MVP development** with TypeScript/Node.js
2. **Proven scalability** with PostgreSQL/Redis
3. **Real-time multiplayer** via Socket.io
4. **Flexible architecture** with custom ECS
5. **Comprehensive testing** with Jest/Playwright

All technical decisions align with the requirements for a modular simulation engine supporting 1,000+ entities, real-time updates, and cooperative multiplayer gameplay.

## Next Steps
- Generate data models from entity specifications
- Create API contracts for client-server communication
- Define integration test scenarios
- Set up development environment