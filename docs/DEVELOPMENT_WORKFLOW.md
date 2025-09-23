# Development Workflow Guide

## Overview

This document describes the standard development workflow for the world simulator project, including logging, debugging, and common tasks.

## Quick Reference

### Essential Commands
```bash
# Run simulation with logging
./run-simulation.sh

# Check recent logs
tail -f logs/simulation-*.log

# Search for specific issues
grep -E '\[SPAWN\]|WARNING|ERROR' logs/simulation-*.log

# Stop all simulations
pkill -f "world_sim_simple.*--headless"

# Build and test
cargo build
cargo test
```

## Daily Workflow

### 1. Start Working
```bash
# Pull latest changes
git pull origin main

# Check current simulation status
ps aux | grep "world_sim_simple.*--headless"

# Stop any running simulations
pkill -f "world_sim_simple.*--headless"
```

### 2. Make Changes
```bash
# Edit files
vim assets/packs/dev-world/data/entities/units/peasant.lua
vim world_sim_simple/src/systems/entity_spawning.rs

# Test compilation
cargo check

# Run quick test
./run-simulation.sh
```

### 3. Debug Issues
```bash
# Check spawn logs
grep -E "\[SPAWN\]" logs/simulation-*.log | head -10

# Check entity detection
grep -E "Found.*entities" logs/simulation-*.log | tail -5

# Look for errors
grep -E "WARNING|ERROR" logs/simulation-*.log
```

### 4. Commit Changes
```bash
# Check git status
git status

# Review changes
git diff

# Add and commit
git add .
git commit -m "feat: Description of changes"

# Push if ready
git push origin main
```

## Common Development Tasks

### Adding New Unit Types

1. **Create Unit Definition**
```bash
# Edit or create unit file
vim assets/packs/dev-world/data/entities/units/new_unit.lua
```

2. **Update Spawning Logic**
```bash
# Check spawning system
vim world_sim_simple/src/systems/entity_spawning.rs
```

3. **Test**
```bash
./run-simulation.sh
grep -E "new_unit" logs/simulation-*.log
```

### Modifying Movement Speed

1. **Update Lua Definition**
```bash
# Edit peasant.lua
vim assets/packs/dev-world/data/entities/units/peasant.lua
```

2. **Change speed format**
```lua
-- Old format
unit = {
    movement_speed = 1.0,
}

-- New format
unit = {
    ticks_per_tile = 2,  -- 2 ticks per tile
}
```

3. **Verify**
```bash
./run-simulation.sh
grep -E "movement speed.*ticks/tile" logs/simulation-*.log
```

### Debugging Entity Spawning

1. **Check Pack Loading**
```bash
grep -E "\[PACK\].*entities" logs/simulation-*.log
```

2. **Check Spawn Results**
```bash
grep -E "\[SPAWN\].*Spawned.*entities" logs/simulation-*.log
```

3. **Check IPC Detection**
```bash
grep -E "Found.*entities" logs/simulation-*.log
```

### Debugging Movement Issues

1. **Check Movement Components**
```bash
grep -E "movement speed|ticks_per_tile" logs/simulation-*.log
```

2. **Check Grid Occupation**
```bash
grep -E "OCCUPATION" logs/simulation-*.log
```

3. **Check Pathfinding**
```bash
grep -E "PATH|movement" logs/simulation-*.log
```

## Logging Guidelines

### Log Levels
- **INFO**: General system status
- **DEBUG**: Detailed debugging information
- **WARNING**: Potential issues
- **ERROR**: Critical errors

### Key Log Patterns to Monitor

#### Success Patterns
```
[SPAWN] Unit Peasant movement speed: 2 ticks/tile
[SPAWN] Unit Peasant work speed: 1.00x from pack definition
🔍 IPC Debug: Found 5 entities (5 GOAP, 0 basic) and 46 resources
✅ Work system ACTIVE on tick 357
```

#### Error Patterns
```
[SPAWN] Unit Peasant has no unit properties
WARNING: No units found with UnitTag
ERROR: Failed to spawn entity
Unknown unit type: xyz
```

#### Performance Patterns
```
⚙️ Tick work system: 0/0 units working, tick 357
━━━ TICK 392 ━━━
🌍 World Resources:
   🌲 15 trees available
   🫐 25 berry bushes with fruit
```

## Testing Workflow

### Unit Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test spawn_system

# Run with output
cargo test -- --nocapture
```

### Integration Testing
```bash
# Run simulation for specific duration
./run-simulation.sh

# Check specific functionality
grep -E "gathering|eating|moving" logs/simulation-*.log
```

### Performance Testing
```bash
# Run with performance logging
RUST_LOG=debug cargo run --release --bin world_sim_simple -- --headless --ticks 1000 > logs/perf-test.log 2>&1 &

# Analyze performance
grep -E "TICK.*ms|performance" logs/perf-test.log
```

## Git Workflow

### Branch Strategy
- **main**: Production-ready code
- **feature/***: New features
- **fix/***: Bug fixes
- **refactor/***: Code restructuring

### Commit Guidelines
- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation
- **style**: Code formatting
- **refactor**: Code restructuring
- **test**: Testing
- **chore**: Maintenance

### Commit Message Format
```
type(scope): description

[optional body]

[optional footer]
```

Examples:
```
feat(spawning): Add new peasant unit type
fix(movement): Correct ticks_per_tile calculation
refactor(logging): Simplify log output format
docs(readme): Update development instructions
```

## Common Issues and Solutions

### Compilation Errors
```bash
# Check syntax
cargo check

# Fix imports
cargo clippy

# Update dependencies
cargo update
```

### Runtime Errors
```bash
# Check logs
grep -E "ERROR|panic" logs/simulation-*.log

# Run with debug
RUST_LOG=debug ./run-simulation.sh

# Check memory issues
cargo run --release --bin world_sim_simple -- --headless --ticks 10 2>&1 | grep -i error
```

### Performance Issues
```bash
# Check CPU usage
top | grep "world_sim"

# Check memory usage
ps aux | grep "world_sim" | awk '{print $4, $11}'

# Profile with perf
perf record --call-graph dwarf cargo run --release
```

## Tools and Utilities

### Essential Tools
- **run-simulation.sh**: Automated simulation runner
- **logs/**: Log file directory
- **grep**: Log searching
- **tail**: Real-time log monitoring
- **git**: Version control
- **cargo**: Build system

### Useful Aliases
```bash
# Add to ~/.bashrc or ~/.zshrc
alias sim-run='./run-simulation.sh'
alias sim-logs='tail -f logs/simulation-*.log'
alias sim-stop='pkill -f "world_sim_simple.*--headless"'
alias sim-status='ps aux | grep "world_sim_simple.*--headless"'
```

### IDE Configuration
- **VS Code**: Install Rust Analyzer
- **Vim/Neovim**: Use coc-rust or rust-tools.nvim
- **Debugger**: Use lldb or gdb for debugging

## Code Style Guidelines

### Rust Code
- Follow standard Rust formatting
- Use cargo fmt for formatting
- Use cargo clippy for linting
- Write documentation for public functions

### Lua Code
- Use 4-space indentation
- Comment complex logic
- Use table fields consistently
- Follow existing patterns

### Documentation
- Update README for user-facing changes
- Update docs/ for developer changes
- Include examples for complex features
- Document breaking changes

## Release Process

### Pre-Release Checklist
- [ ] All tests passing
- [ ] Documentation updated
- [ ] Performance acceptable
- [ ] No critical errors in logs
- [ ] Git history clean

### Release Steps
```bash
# Update version
vim Cargo.toml

# Tag release
git tag -a v1.0.0 -m "Version 1.0.0"

# Push to remote
git push origin main --tags

# Build release
cargo build --release
```

### Post-Release
- [ ] Update documentation
- [ ] Create release notes
- [ ] Monitor issues
- [ ] Plan next iteration