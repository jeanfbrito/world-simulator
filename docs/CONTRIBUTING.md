# Contributing to World Simulator

Thank you for your interest in contributing to the World Simulator project! This document provides guidelines and information for contributors.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Development Setup](#development-setup)
3. [Code Style and Standards](#code-style-and-standards)
4. [Pull Request Process](#pull-request-process)
5. [Testing Guidelines](#testing-guidelines)
6. [Documentation](#documentation)
7. [Issue Reporting](#issue-reporting)
8. [Community Guidelines](#community-guidelines)

## Getting Started

### Prerequisites

- **Rust**: Latest stable version (1.70+)
- **Cargo**: Rust package manager
- **Git**: Version control
- **GitHub Account**: For pull requests and issues

### First Steps

1. **Fork the repository**
   ```bash
   # Fork on GitHub, then clone your fork
   git clone https://github.com/yourusername/world-simulator.git
   cd world-simulator
   ```

2. **Set up upstream remote**
   ```bash
   git remote add upstream https://github.com/original/world-simulator.git
   ```

3. **Install dependencies**
   ```bash
   cargo build
   cargo test
   ```

4. **Create development branch**
   ```bash
   git checkout -b feature/your-feature-name develop
   ```

## Development Setup

### Development Tools

Install recommended development tools:

```bash
# Code formatting
rustup component add rustfmt

# Linting
rustup component add clippy

# Testing
cargo install cargo-nextest

# Documentation generation
cargo install cargo-docs
```

### Pre-commit Hooks

Set up pre-commit hooks for code quality:

```bash
# Install pre-commit
pip install pre-commit

# Install hooks
pre-commit install

# Run hooks manually
pre-commit run --all-files
```

### IDE Setup

#### VS Code

Install these extensions:
- **rust-analyzer**: Rust language support
- **CodeLLDB**: Debugging support
- **Better TOML**: Configuration file support
- **Lua**: Lua configuration support

#### IntelliJ IDEA

- Install the Rust plugin
- Configure the project as a Cargo project

## Code Style and Standards

### Rust Code Style

The project follows standard Rust conventions:

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
}
```

### Documentation Standards

All public APIs must be documented:

```rust
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
///
/// # Panics
///
/// Panics if the entity ID exceeds the maximum allowed value.
///
/// # Errors
///
/// Returns an error if the entity cannot be created due to system limitations.
pub struct Entity {
    id: u64,
    components: HashMap<ComponentTypeId, Box<dyn Component>>,
}
```

### Performance Considerations

- Use appropriate data structures for the use case
- Minimize allocations in hot paths
- Consider cache efficiency when designing systems
- Use benchmarks to validate performance improvements

## Pull Request Process

### Branch Strategy

- `main`: Production-ready code
- `develop`: Integration branch
- `feature/*`: New features
- `bugfix/*`: Bug fixes
- `docs/*`: Documentation changes

### Commit Message Format

Follow conventional commits format:

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

```bash
feat(ai): add pathfinding system
- Implement A* pathfinding algorithm
- Add path caching for performance
- Update AI to use pathfinding

fix(simulation): fix entity position desync
- Position updates were not thread-safe
- Add mutex for position component access
- Fixes issue #123
```

### Pull Request Template

```markdown
## Summary
Brief description of the changes.

## Changes
- List of changes made
- Files modified
- New features added

## Testing
- How was this tested?
- What test cases were added?
- Performance impact assessment

## Related Issues
- Closes #123
- Related to #456

## Checklist
- [ ] Code follows style guidelines
- [ ] All tests pass
- [ ] Documentation updated
- [ ] Performance considered
- [ ] Breaking changes documented
```

### Pull Request Process

1. **Update your branch**
   ```bash
   git fetch upstream
   git rebase upstream/develop
   ```

2. **Run tests and checks**
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```

3. **Create pull request**
   ```bash
   git push origin feature/your-feature
   gh pr create --base develop --title "feat: Your feature"
   ```

4. **Address review feedback**
   - Respond to all review comments
   - Make requested changes
   - Keep PR history clean

## Testing Guidelines

### Test Structure

```
tests/
├── integration/     # Integration tests
├── performance/     # Performance tests
├── e2e/            # End-to-end tests
└── common/         # Test utilities
```

### Writing Tests

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new();
        assert!(entity.is_alive());
    }

    #[test]
    fn test_component_addition() {
        let mut entity = Entity::new();
        entity.add_component(Position { x: 0, y: 0 });
        assert!(entity.has_component::<Position>());
    }
}
```

#### Integration Tests

```rust
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
```

#### Performance Tests

```rust
#[bench]
fn benchmark_entity_spawning(b: &mut test::Bencher) {
    let mut context = TestContext::new().await;
    let simulation = context.initialize_simulation(&TestConfig::default()).await.unwrap();

    b.iter(|| {
        let entity = simulation.spawn_entity().unwrap();
        simulation.despawn_entity(entity.id());
    });
}
```

### Test Coverage

- Aim for 80%+ code coverage
- Test both success and error cases
- Include edge cases and boundary conditions
- Test concurrent access patterns

## Documentation

### API Documentation

- Document all public APIs with Rustdoc
- Include examples for complex functionality
- Document panics and error conditions
- Use markdown for formatting

### User Documentation

- Update README.md for user-facing changes
- Add/update examples for new features
- Document configuration options
- Include troubleshooting guides

### Architecture Documentation

- Update ARCHITECTURE.md for structural changes
- Document design decisions and trade-offs
- Include performance implications
- Provide migration guides for breaking changes

### Inline Documentation

```rust
/// Performance-critical path that updates entity positions.
///
/// This function is called every frame and must be optimized for speed.
/// It uses vectorized operations and cache-friendly data access patterns.
///
/// # Performance
///
/// - Time complexity: O(n) where n is number of entities
/// - Memory usage: No allocations, operates on existing data
/// - Cache efficiency: High - sequential access pattern
fn update_positions(&mut self, dt: f32) {
    // Implementation
}
```

## Issue Reporting

### Bug Reports

When reporting bugs, include:

1. **Environment**: OS, Rust version, simulator version
2. **Steps to reproduce**: Clear, minimal reproduction steps
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Error messages**: Full error output
6. **Additional context**: Any relevant logs or screenshots

### Feature Requests

For feature requests:

1. **Problem description**: What problem does this solve?
2. **Proposed solution**: How should this work?
3. **Alternatives considered**: Other approaches you've thought of
4. **Use cases**: Specific examples of when this would be useful
5. **Implementation suggestions**: Any ideas on how to implement

### Issue Template

```markdown
## Description
Brief description of the issue or feature request.

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen.

## Actual Behavior
What actually happens.

## Environment
- OS: [e.g. Ubuntu 20.04]
- Rust version: [e.g. 1.70.0]
- Simulator version: [e.g. v0.1.0]

## Additional Context
Any other relevant information, logs, or screenshots.
```

## Community Guidelines

### Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Participants are expected to:

- Be respectful and inclusive
- Focus on constructive feedback
- Assume good faith
- Welcome newcomers
- Be collaborative

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and discussions
- **Discord**: Real-time chat (when available)
- **Email**: Private concerns (maintainers@world-simulator.dev)

### Contributing Levels

#### First-Time Contributors

- Look for issues labeled "good first issue"
- Start with documentation improvements
- Help with bug triage
- Review pull requests

#### Regular Contributors

- Implement new features
- Refactor existing code
- Improve performance
- Mentor newcomers

#### Core Contributors

- Review and merge pull requests
- Make architectural decisions
- Manage releases
- Guide project direction

### Getting Help

- **Documentation**: Check the [docs](./docs/) directory
- **Examples**: Look at the [examples](./examples/) directory
- **Issues**: Search existing issues before creating new ones
- **Discussions**: Ask questions in GitHub Discussions

## Recognition and Attribution

### Contributors Hall of Fame

Significant contributors will be recognized in:

- README.md contributors section
- Release notes
- Documentation credits
- GitHub team membership

### License and CLA

By contributing, you agree that your contributions will be licensed under the project's license (MIT). For significant contributions, you may be asked to sign a Contributor License Agreement (CLA).

## Project Governance

### Maintainers

Current maintainers:
- [@maintainer1](https://github.com/maintainer1)
- [@maintainer2](https://github.com/maintainer2)

### Decision Making

- **Code changes**: Reviewed and merged by maintainers
- **Architecture changes**: Discussed with core contributors
- **Breaking changes**: Announced in advance with migration guides
- **Roadmap decisions**: Community discussion with maintainer approval

### Release Process

1. **Version bump**: Update version numbers
2. **Changelog**: Update release notes
3. **Testing**: Full test suite
4. **Documentation**: Update all documentation
5. **Release**: Create GitHub release and publish packages

Thank you for contributing to the World Simulator project! Your help makes this project better for everyone.