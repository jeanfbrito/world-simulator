# World Simulator Development Guide

## Table of Contents

1. [Getting Started](#getting-started)
2. [Project Structure](#project-structure)
3. [Development Workflow](#development-workflow)
4. [Coding Standards](#coding-standards)
5. [Testing Guidelines](#testing-guidelines)
6. [Performance Considerations](#performance-considerations)
7. [Debugging Tips](#debugging-tips)
8. [Contributing Guidelines](#contributing-guidelines)
9. [Deployment and Releases](#deployment-and-releases)

## Getting Started

### Prerequisites

- **Rust**: Latest stable version (1.70+)
- **Cargo**: Rust package manager
- **Git**: Version control
- **Make**: Build automation (optional)
- **Docker**: Container support (optional)

### Setup Instructions

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/world-simulator.git
   cd world-simulator
   ```

2. **Install dependencies**
   ```bash
   # Install Rust dependencies
   cargo build

   # Install development tools
   cargo install cargo-watch cargo-nextest
   ```

3. **Set up development environment**
   ```bash
   # Create development configuration
   cp config/dev.example.lua config/dev.lua

   # Set up pre-commit hooks
   pre-commit install
   ```

### Building and Running

```bash
# Development build with debug symbols
cargo build

# Release build with optimizations
cargo build --release

# Run basic simulation
cargo run --example basic_simulation

# Run with specific configuration
cargo run -- --config config/dev.lua

# Watch mode for development
cargo watch -x run
```

## Project Structure

```
world-simulator/
├── src/                      # Core library source
│   ├── ecs/                 # Entity Component System
│   ├── simulation/          # Core simulation logic
│   ├── systems/             # Game systems
│   ├── components/          # ECS components
│   ├── ai/                  # AI systems
│   ├── economy/             # Economic simulation
│   ├── world/               # World generation
│   ├── networking/          # Network communication
│   ├── config/              # Configuration system
│   └── lib.rs               # Library entry point
├── world_sim_simple/        # Simple simulation binary
│   ├── src/                 # Binary-specific code
│   └── Cargo.toml           # Binary configuration
├── web-viewer/              # Web-based viewer
│   ├── static/              # Static web assets
│   └── src/                 # Web server code
├── tests/                   # Test suite
│   ├── integration/         # Integration tests
│   ├── performance/         # Performance tests
│   ├── e2e/                 # End-to-end tests
│   └── common/              # Test utilities
├── examples/                # Example applications
│   ├── basic_simulation/    # Basic simulation example
│   ├── ai_demo/             # AI demonstration
│   ├── custom_world/        # Custom world generation
│   ├── websocket_client/    # WebSocket client
│   ├── performance_test/    # Performance testing
│   └── modding_example/     # Modding system example
├── assets/                  # Game assets
│   └── packs/               # Entity and resource packs
├── docs/                    # Documentation
├── scripts/                 # Utility scripts
├── config/                  # Configuration files
├── .github/                 # GitHub workflows
└── target/                  # Build artifacts (generated)
```

## Development Workflow

### Branch Strategy

- `main`: Production-ready code
- `develop`: Integration branch for features
- `feature/*`: Individual feature branches
- `hotfix/*`: Emergency fixes
- `release/*`: Release preparation

### Feature Development

1. **Create feature branch**
   ```bash
   git checkout -b feature/new-feature develop
   ```

2. **Implement feature**
   ```bash
   # Write code following coding standards
   cargo check
   cargo test
   cargo clippy
   ```

3. **Test thoroughly**
   ```bash
   # Run all tests
   cargo nextest run

   # Run specific test
   cargo test integration::simulation

   # Performance test
   cargo test performance::scaling
   ```

4. **Commit changes**
   ```bash
   git add .
   git commit -m "feat: Add new feature

   - Implement core functionality
   - Add integration tests
   - Update documentation"
   ```

5. **Create pull request**
   ```bash
   git push origin feature/new-feature
   gh pr create --base develop --title "feat: Add new feature"
   ```

### Code Review Process

1. **Self-review checklist:**
   - [ ] Code follows style guidelines
   - [ ] All tests pass
   - [ ] No clippy warnings
   - [ ] Documentation updated
   - [ ] Performance impact considered

2. **Peer review focus:**
   - Algorithm efficiency
   - System design
   - Error handling
   - Test coverage
   - Security implications

### Continuous Integration

The project uses GitHub Actions for CI/CD:

- **On push to any branch**: Build, lint, and run tests
- **On pull request**: Full test suite including performance and e2e tests
- **On merge to develop**: Integration tests and coverage report
- **On merge to main**: Release build and deployment

## Coding Standards

### Rust Code Style

```rust
// Use rustfmt for formatting
cargo fmt

// Follow naming conventions
struct GameState;          // PascalCase for types
fn update_game() -> bool { // snake_case for functions
    let game_state = GameState::new(); // snake_case for variables
    game_state.is_running()
}

// Error handling
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimulationError {
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Entity not found: {id}")]
    EntityNotFound { id: u64 },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Documentation
/// Represents a game entity with unique identifier and components.
///
/// Entities are the basic building blocks of the simulation. Each entity
/// has a unique ID and can have multiple components attached to it.
///
/// # Examples
///
/// ```rust
/// let entity = Entity::new();
/// entity.add_component(Position { x: 0, y: 0 });
/// ```
pub struct Entity {
    id: u64,
    components: HashMap<ComponentTypeId, Box<dyn Component>>,
}
```

### Component System Guidelines

```rust
// Component definition
#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

// System implementation
pub struct MovementSystem {
    query: Query<(With<Position>, With<Velocity>)>,
}

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        for (entity, (position, velocity)) in self.query.iter(world) {
            position.x += velocity.x * dt;
            position.y += velocity.y * dt;
        }
    }
}
```

### Error Handling Patterns

```rust
// Result chaining
pub fn process_entity(id: u64) -> Result<ProcessedEntity, SimulationError> {
    let entity = get_entity(id)?;
    let components = get_entity_components(id)?;
    let processed = process_components(components)?;
    Ok(ProcessedEntity::new(entity, processed))
}

// Custom error types
#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("Component missing: {component_type}")]
    ComponentMissing { component_type: String },

    #[error("Invalid state: {reason}")]
    InvalidState { reason: String },

    #[error(transparent)]
    Internal(#[from] Box<dyn std::error::Error + Send + Sync>),
}

// Error context
use anyhow::{Context, Result};

pub fn load_world(path: &Path) -> Result<World> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read world file")?;

    toml::from_str(&content)
        .context("Failed to parse world configuration")?
}
```

## Testing Guidelines

### Test Organization

```rust
// Unit tests (inline in source files)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new();
        assert!(entity.is_alive());
    }
}

// Integration tests (tests/integration/)
mod integration {
    use super::*;

    #[tokio::test]
    async fn test_simulation_workflow() {
        let mut context = TestContext::new().await;
        let simulation = context.initialize_simulation(&TestConfig::default()).await.unwrap();

        // Test complete workflow
        for _ in 0..100 {
            simulation.tick().await.unwrap();
        }

        let state = simulation.get_state().await.unwrap();
        assert!(state.entities.len() > 0);
    }
}

// Performance tests (tests/performance/)
mod performance {
    use super::*;

    #[bench]
    fn benchmark_entity_updates(b: &mut test::Bencher) {
        let mut world = World::new();
        world.spawn_entities(1000);

        b.iter(|| {
            world.update_entities();
        });
    }
}
```

### Test Data Management

```rust
// Test fixtures
pub struct TestFixture {
    pub entities: Vec<Entity>,
    pub resources: Vec<Resource>,
    pub world_config: WorldConfig,
}

impl TestFixture {
    pub fn basic_scenario() -> Self {
        Self {
            entities: vec![Entity::test_peasant()],
            resources: vec![Resource::test_wood()],
            world_config: WorldConfig::small(),
        }
    }
}

// Mock objects
pub struct MockAI {
    pub actions: Vec<String>,
}

impl MockAI {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
}

impl AIComponent for MockAI {
    fn decide_action(&mut self, world: &World) -> Action {
        let action = self.actions.pop().unwrap_or_else(|| "idle".to_string());
        Action::new(action)
    }
}
```

## Performance Considerations

### Entity System Optimization

```rust
// Use archetypes for efficient queries
pub struct PhysicsSystem {
    query: ArchetypeQuery<(Position, Velocity, Mass)>,
}

// Batch operations
impl PhysicsSystem {
    fn update_positions(&mut self, world: &mut World, dt: f32) {
        let mut positions = self.query.get_component_mut::<Position>(world);
        let velocities = self.query.get_component::<Velocity>(world);

        // Parallel processing
        positions.par_iter_mut().zip(velocities.par_iter()).for_each(|(pos, vel)| {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
        });
    }
}
```

### Memory Management

```rust
// Use object pooling for frequently created/destroyed entities
pub struct EntityPool {
    available: Vec<Entity>,
    active: HashSet<u64>,
}

impl EntityPool {
    pub fn get_entity(&mut self) -> Entity {
        if let Some(entity) = self.available.pop() {
            self.active.insert(entity.id());
            entity
        } else {
            Entity::new()
        }
    }

    pub fn return_entity(&mut self, entity: Entity) {
        self.active.remove(&entity.id());
        entity.reset();
        self.available.push(entity);
    }
}
```

### Algorithm Efficiency

```rust
// Use appropriate data structures
pub struct SpatialHashGrid {
    cell_size: f32,
    grid: HashMap<(i32, i32), Vec<EntityId>>,
}

impl SpatialHashGrid {
    pub fn query_radius(&self, center: (f32, f32), radius: f32) -> Vec<EntityId> {
        let min_cell = self.world_to_grid((center.0 - radius, center.1 - radius));
        let max_cell = self.world_to_grid((center.0 + radius, center.1 + radius));

        let mut results = Vec::new();

        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                if let Some(entities) = self.grid.get(&(x, y)) {
                    results.extend(entities.iter().copied());
                }
            }
        }

        results
    }
}
```

## Debugging Tips

### Logging Setup

```rust
use tracing::{info, warn, error, debug};
use tracing_subscriber;

// Initialize logging
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

// Use logging in code
pub fn process_tick(&mut self) -> Result<(), SimulationError> {
    debug!("Processing tick {}", self.tick_count);

    let start_time = std::time::Instant::now();
    self.update_entities()?;

    let duration = start_time.elapsed();
    if duration > Duration::from_millis(100) {
        warn!("Tick processing took {:?} - longer than expected", duration);
    }

    info!("Completed tick {} in {:?}", self.tick_count, duration);
    Ok(())
}
```

### Debug Visualizations

```rust
// Entity visualization
#[derive(Debug)]
pub struct DebugEntity {
    pub id: u64,
    pub position: (f32, f32),
    pub components: Vec<String>,
    pub state: String,
}

impl Display for DebugEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity[{}] at {:?} [{}] ({})",
               self.id, self.position, self.components.join(", "), self.state)
    }
}

// World state dump
pub struct WorldDebugger;

impl WorldDebugger {
    pub fn dump_world(world: &World) -> String {
        let mut output = String::new();

        output.push_str(&format!("World Dump:\n"));
        output.push_str(&format!("  Entities: {}\n", world.entity_count()));
        output.push_str(&format!("  Resources: {}\n", world.resource_count()));
        output.push_str(&format!("  Tick: {}\n", world.current_tick()));

        output
    }
}
```

### Performance Profiling

```rust
use std::time::Instant;

#[derive(Default)]
pub struct PerformanceProfiler {
    measurements: HashMap<String, Vec<Duration>>,
}

impl PerformanceProfiler {
    pub fn start_measurement(&mut self, name: &str) -> Measurement {
        Measurement::new(name, self)
    }

    pub fn get_stats(&self, name: &str) -> Option<PerformanceStats> {
        let measurements = self.measurements.get(name)?;
        Some(PerformanceStats::calculate(measurements))
    }
}

pub struct Measurement<'a> {
    name: String,
    start_time: Instant,
    profiler: &'a mut PerformanceProfiler,
}

impl<'a> Measurement<'a> {
    fn new(name: &str, profiler: &'a mut PerformanceProfiler) -> Self {
        Self {
            name: name.to_string(),
            start_time: Instant::now(),
            profiler,
        }
    }
}

impl<'a> Drop for Measurement<'a> {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        self.profiler.measurements
            .entry(self.name.clone())
            .or_insert_with(Vec::new)
            .push(duration);
    }
}
```

## Contributing Guidelines

### Pull Request Process

1. **Fork the repository**
2. **Create feature branch** (`feature/amazing-feature`)
3. **Make changes** following coding standards
4. **Add tests** for new functionality
5. **Update documentation**
6. **Run full test suite**
7. **Submit pull request** with clear description

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Maintenance tasks

Examples:
```
feat(ai): add pathfinding system
- Implement A* pathfinding algorithm
- Add path caching for performance
- Update AI to use pathfinding

fix(simulation): fix entity position desync
- Position updates were not thread-safe
- Add mutex for position component access
- Fixes issue #123

docs(api): update entity component documentation
- Add examples for component usage
- Clarify lifecycle management
- Fix broken links in API reference
```

### Code Review Criteria

Reviewers will check for:

- **Functionality**: Does the code work as intended?
- **Performance**: Is the code efficient and scalable?
- **Reliability**: Is error handling comprehensive?
- **Maintainability**: Is the code clean and well-documented?
- **Test Coverage**: Are there sufficient tests?
- **Security**: Does the code follow security best practices?

## Deployment and Releases

### Release Process

1. **Update version numbers**
   ```bash
   # Update Cargo.toml versions
   cargo bump patch
   ```

2. **Update changelog**
   ```bash
   # Generate changelog
   git cliff --tag v1.0.0 > CHANGELOG.md
   ```

3. **Create release**
   ```bash
   # Create Git tag
   git tag -a v1.0.0 -m "Release version 1.0.0"
   git push origin v1.0.0

   # Create GitHub release
   gh release create v1.0.0 --title "Version 1.0.0" --notes-file CHANGELOG.md
   ```

4. **Deploy artifacts**
   ```bash
   # Build release binaries
   cargo build --release

   # Deploy to package registry
   cargo publish
   ```

### Deployment Targets

- **GitHub Releases**: Binaries for Windows, macOS, Linux
- **Docker Hub**: Containerized application
- **crates.io**: Rust library package
- **Web**: WebAssembly version for web viewer

### Monitoring and Maintenance

- **Error Tracking**: Sentry integration for error reporting
- **Performance Monitoring**: Application metrics and alerts
- **Health Checks**: Automated system health verification
- **Log Aggregation**: Centralized log collection and analysis

This development guide provides comprehensive information for contributing to the world simulator project. Following these guidelines will help maintain code quality and ensure smooth collaboration between contributors.