# Phase 1: Core Architecture - Detailed Implementation Guide

## Overview
Phase 1 transforms sim_simple from a monolithic structure to a modular, component-based architecture. This foundation enables all future enhancements.

## Prerequisites
- Working sim_simple that compiles and runs
- Terminal debug system configured
- Clean git working directory

## Step 1.1: Add Component Registry (30 minutes)

### Goal
Replace hardcoded structs with flexible components that can be added/removed at runtime.

### Files to Create/Modify
```
world_sim_simple/src/
├── components/
│   ├── mod.rs         [NEW]
│   ├── position.rs    [NEW]
│   └── health.rs      [NEW]
└── main.rs            [MODIFY]
```

### Implementation

#### 1. Create `src/components/mod.rs`
```rust
use bevy::prelude::*;
use colored::*;

pub mod position;
pub mod health;

pub use position::PositionComponent;
pub use health::HealthComponent;

/// Plugin to register all components
pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        println!("{}", "[COMPONENTS] Registering component systems...".green());
        
        app.register_type::<PositionComponent>()
           .register_type::<HealthComponent>();
           
        println!("{}", "[COMPONENTS] ✓ Position component registered".green());
        println!("{}", "[COMPONENTS] ✓ Health component registered".green());
    }
}
```

#### 2. Create `src/components/position.rs`
```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,  // For future 3D support
}

impl PositionComponent {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, z: 0.0 }
    }
    
    pub fn from_tile(tile_x: usize, tile_y: usize) -> Self {
        Self {
            x: tile_x as f32,
            y: tile_y as f32,
            z: 0.0,
        }
    }
    
    pub fn to_tile(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
    
    pub fn distance_to(&self, other: &PositionComponent) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
```

#### 3. Create `src/components/health.rs`
```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::debug::{DebugSystem, DebugLevel};

#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct HealthComponent {
    pub current: f32,
    pub maximum: f32,
    pub regeneration_rate: f32,
}

impl HealthComponent {
    pub fn new(maximum: f32) -> Self {
        Self {
            current: maximum,
            maximum,
            regeneration_rate: 0.1,
        }
    }
    
    pub fn damage(&mut self, amount: f32, entity_name: &str, debug: &DebugSystem) {
        let old_health = self.current;
        self.current = (self.current - amount).max(0.0);
        
        debug.log(
            DebugLevel::Info,
            "HEALTH",
            &format!("{}: {} → {} (-{} damage)", 
                entity_name, old_health, self.current, amount)
        );
        
        if self.is_dead() {
            debug.log(
                DebugLevel::Warn,
                "HEALTH",
                &format!("{} has died!", entity_name)
            );
        }
    }
    
    pub fn heal(&mut self, amount: f32, entity_name: &str, debug: &DebugSystem) {
        let old_health = self.current;
        self.current = (self.current + amount).min(self.maximum);
        
        debug.log(
            DebugLevel::Info,
            "HEALTH",
            &format!("{}: {} → {} (+{} healing)", 
                entity_name, old_health, self.current, amount)
        );
    }
    
    pub fn percentage(&self) -> f32 {
        (self.current / self.maximum) * 100.0
    }
    
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
    
    pub fn is_full(&self) -> bool {
        self.current >= self.maximum
    }
}
```

#### 4. Update `src/main.rs`
```rust
// Add to imports
mod components;
use components::{ComponentsPlugin, PositionComponent, HealthComponent};

// In main(), add the plugin:
.add_plugins(ComponentsPlugin)

// Update Worker spawn in setup() function:
// REPLACE the Worker spawn section with:
for i in 0..5 {
    let x = rng.gen_range(20..44);
    let y = rng.gen_range(20..44);
    
    if world_map.tiles[y][x].is_walkable() {
        let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
        let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
        
        let worker_entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.75, 0.0),
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.6, TILE_SIZE * 0.6)),
                    ..default()
                },
                transform: Transform::from_xyz(world_x, world_y, 1.0),
                ..default()
            },
            Worker {
                name: format!("Worker {}", i + 1),
                health: 100.0,
                energy: 100.0,
            },
            PositionComponent::from_tile(x, y),
            HealthComponent::new(100.0),
            TileEntity { x, y },
        )).id();
        
        // Log worker creation with components
        println!("{}", format!("[SPAWN] Worker {} at ({}, {}) with Position and Health components", 
            i + 1, x, y).green());
    }
}
```

### Debug Validation

#### Expected Terminal Output
```bash
$ RUST_LOG=debug cargo run

[COMPONENTS] Registering component systems...
[COMPONENTS] ✓ Position component registered
[COMPONENTS] ✓ Health component registered
[SPAWN] Worker 1 at (23, 31) with Position and Health components
[SPAWN] Worker 2 at (35, 28) with Position and Health components
[SPAWN] Worker 3 at (30, 33) with Position and Health components
[SPAWN] Worker 4 at (25, 25) with Position and Health components
[SPAWN] Worker 5 at (40, 30) with Position and Health components
```

#### Success Criteria
- ✅ All component registration messages appear in green
- ✅ Worker spawn messages show component attachment
- ✅ No compilation errors or warnings
- ✅ Game still runs and workers appear
- ✅ Workers still move as before

### Common Issues & Solutions

**Issue**: "cannot find module components"
**Solution**: Ensure `mod components;` is added to main.rs

**Issue**: Components not showing in debug
**Solution**: Check that ComponentsPlugin is added before setup system

**Issue**: Workers don't spawn
**Solution**: Verify the spawn code includes all required components

---

## Step 1.2: Add Basic Plugin System (45 minutes)

### Goal
Create a plugin architecture to organize code into logical modules.

### Files to Create/Modify
```
world_sim_simple/src/
├── plugin.rs          [NEW]
├── plugins/
│   ├── mod.rs        [NEW]
│   ├── world.rs      [NEW]
│   └── simulation.rs [NEW]
└── main.rs           [MODIFY]
```

### Implementation

#### 1. Create `src/plugin.rs`
```rust
use bevy::prelude::*;
use colored::*;
use std::collections::HashMap;

/// Base trait for all simulation plugins
pub trait SimulationPlugin {
    /// Plugin name for debug output
    fn name(&self) -> &str;
    
    /// Initialize the plugin
    fn build(&self, app: &mut App);
}

/// Manages all simulation plugins
#[derive(Resource)]
pub struct PluginManager {
    plugins: HashMap<String, bool>,
    load_order: Vec<String>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            load_order: Vec::new(),
        }
    }
    
    pub fn register(&mut self, name: &str) {
        if !self.plugins.contains_key(name) {
            self.plugins.insert(name.to_string(), true);
            self.load_order.push(name.to_string());
            println!("{}", format!("[PLUGIN] ✓ {} registered", name).green());
        }
    }
    
    pub fn is_loaded(&self, name: &str) -> bool {
        self.plugins.get(name).copied().unwrap_or(false)
    }
    
    pub fn list_plugins(&self) {
        println!("{}", "[PLUGIN] Loaded plugins:".cyan().bold());
        for name in &self.load_order {
            let status = if self.plugins[name] { "✓" } else { "✗" };
            println!("  {} {}", status.green(), name);
        }
    }
}

/// System to track plugin initialization
pub fn plugin_init_system(mut manager: ResMut<PluginManager>) {
    manager.list_plugins();
}
```

#### 2. Create `src/plugins/mod.rs`
```rust
pub mod world;
pub mod simulation;

pub use world::WorldPlugin;
pub use simulation::SimulationPlugin;
```

#### 3. Create `src/plugins/world.rs`
```rust
use bevy::prelude::*;
use colored::*;
use crate::{WorldMap, TileEntity, TILE_SIZE, MAP_SIZE};
use crate::plugin::PluginManager;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        println!("{}", "[WORLD] Initializing world plugin...".cyan());
        
        app.add_systems(Startup, init_world_plugin)
           .add_systems(Update, world_update_system);
    }
}

fn init_world_plugin(mut manager: ResMut<PluginManager>) {
    manager.register("WorldPlugin");
    println!("{}", "[WORLD] World systems initialized".green());
}

fn world_update_system(
    world_map: Res<WorldMap>,
    time: Res<Time>,
) {
    // Future world update logic
    // For now, just track that the system runs
    static mut LAST_LOG: f32 = 0.0;
    unsafe {
        LAST_LOG += time.delta_seconds();
        if LAST_LOG > 5.0 {
            println!("{}", "[WORLD] World update tick".dimmed());
            LAST_LOG = 0.0;
        }
    }
}
```

#### 4. Create `src/plugins/simulation.rs`
```rust
use bevy::prelude::*;
use colored::*;
use crate::SimulationState;
use crate::plugin::PluginManager;
use crate::debug::{DebugSystem, DebugLevel};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        println!("{}", "[SIMULATION] Initializing simulation plugin...".cyan());
        
        app.add_systems(Startup, init_simulation_plugin)
           .add_systems(Update, (
               simulation_tick_system,
               simulation_stats_system,
           ));
    }
}

fn init_simulation_plugin(mut manager: ResMut<PluginManager>) {
    manager.register("SimulationPlugin");
    println!("{}", "[SIMULATION] Simulation systems initialized".green());
}

fn simulation_tick_system(
    mut sim_state: ResMut<SimulationState>,
    time: Res<Time>,
    debug: Res<DebugSystem>,
) {
    if !sim_state.running {
        return;
    }
    
    sim_state.accumulated_time += time.delta_seconds() * sim_state.speed;
    
    if sim_state.accumulated_time >= 1.0 {
        sim_state.accumulated_time = 0.0;
        let old_tick = sim_state.tick;
        sim_state.tick += 1;
        
        // Log every 10 ticks
        if sim_state.tick % 10 == 0 {
            debug.log(
                DebugLevel::Debug,
                "SIMULATION",
                &format!("Tick {} → {}", old_tick, sim_state.tick)
            );
        }
    }
}

fn simulation_stats_system(
    sim_state: Res<SimulationState>,
    debug: Res<DebugSystem>,
    time: Res<Time>,
) {
    static mut LAST_STATS: f32 = 0.0;
    unsafe {
        LAST_STATS += time.delta_seconds();
        if LAST_STATS > 10.0 {  // Every 10 seconds
            debug.log(
                DebugLevel::Info,
                "STATS",
                &format!("Tick: {}, Speed: {:.1}x, Running: {}", 
                    sim_state.tick, sim_state.speed, sim_state.running)
            );
            LAST_STATS = 0.0;
        }
    }
}
```

#### 5. Update `src/main.rs`
```rust
// Add imports
mod plugin;
mod plugins;
use plugin::{PluginManager, plugin_init_system};
use plugins::{WorldPlugin, SimulationPlugin as SimPlugin};

// In main(), add:
.init_resource::<PluginManager>()
.add_plugins(WorldPlugin)
.add_plugins(SimPlugin)
.add_systems(PostStartup, plugin_init_system)
```

### Debug Validation

#### Expected Terminal Output
```bash
$ RUST_LOG=debug cargo run

[WORLD] Initializing world plugin...
[SIMULATION] Initializing simulation plugin...
[COMPONENTS] Registering component systems...
[COMPONENTS] ✓ Position component registered
[COMPONENTS] ✓ Health component registered
[WORLD] World systems initialized
[PLUGIN] ✓ WorldPlugin registered
[SIMULATION] Simulation systems initialized
[PLUGIN] ✓ SimulationPlugin registered
[PLUGIN] Loaded plugins:
  ✓ WorldPlugin
  ✓ SimulationPlugin
[SPAWN] Worker 1 at (23, 31) with Position and Health components
...
[WORLD] World update tick    # After 5 seconds
[STATS] Tick: 0, Speed: 1.0x, Running: false    # After 10 seconds
```

---

## Step 1.3: Refactor Entity System (45 minutes)

### Goal
Replace the Worker struct with pure components for maximum flexibility.

### Files to Create/Modify
```
world_sim_simple/src/
├── components/
│   ├── mod.rs         [MODIFY]
│   ├── name.rs        [NEW]
│   ├── worker.rs      [NEW]
│   └── energy.rs      [NEW]
└── main.rs            [MODIFY]
```

### Implementation

#### 1. Create `src/components/name.rs`
```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct NameComponent {
    pub name: String,
    pub display_name: String,
}

impl NameComponent {
    pub fn new(name: impl Into<String>) -> Self {
        let n = name.into();
        Self {
            display_name: n.clone(),
            name: n,
        }
    }
    
    pub fn with_title(name: impl Into<String>, title: impl Into<String>) -> Self {
        let n = name.into();
        let t = title.into();
        Self {
            name: n.clone(),
            display_name: format!("{} {}", t, n),
        }
    }
}
```

#### 2. Create `src/components/energy.rs`
```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::debug::{DebugSystem, DebugLevel};

#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct EnergyComponent {
    pub current: f32,
    pub maximum: f32,
    pub drain_rate: f32,
    pub recover_rate: f32,
}

impl EnergyComponent {
    pub fn new(maximum: f32) -> Self {
        Self {
            current: maximum,
            maximum,
            drain_rate: 0.5,  // Per second when working
            recover_rate: 1.0, // Per second when resting
        }
    }
    
    pub fn consume(&mut self, amount: f32, entity_name: &str, debug: &DebugSystem) -> bool {
        if self.current >= amount {
            self.current -= amount;
            debug.log(
                DebugLevel::Debug,
                "ENERGY",
                &format!("{}: Energy {} (-{})", entity_name, self.current, amount)
            );
            true
        } else {
            debug.log(
                DebugLevel::Warn,
                "ENERGY",
                &format!("{}: Insufficient energy ({}/{})", entity_name, self.current, amount)
            );
            false
        }
    }
    
    pub fn is_exhausted(&self) -> bool {
        self.current < self.maximum * 0.1
    }
    
    pub fn percentage(&self) -> f32 {
        (self.current / self.maximum) * 100.0
    }
}
```

#### 3. Create `src/components/worker.rs`
```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component for worker entities
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct WorkerTag;

/// Worker-specific data
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct WorkerStats {
    pub work_speed: f32,
    pub carry_capacity: f32,
    pub experience: u32,
}

impl Default for WorkerStats {
    fn default() -> Self {
        Self {
            work_speed: 1.0,
            carry_capacity: 10.0,
            experience: 0,
        }
    }
}
```

#### 4. Update `src/components/mod.rs`
```rust
pub mod position;
pub mod health;
pub mod name;
pub mod energy;
pub mod worker;

pub use position::PositionComponent;
pub use health::HealthComponent;
pub use name::NameComponent;
pub use energy::EnergyComponent;
pub use worker::{WorkerTag, WorkerStats};

// Update ComponentsPlugin
impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        println!("{}", "[COMPONENTS] Registering component systems...".green());
        
        app.register_type::<PositionComponent>()
           .register_type::<HealthComponent>()
           .register_type::<NameComponent>()
           .register_type::<EnergyComponent>()
           .register_type::<WorkerTag>()
           .register_type::<WorkerStats>();
           
        println!("{}", "[COMPONENTS] ✓ All components registered".green());
    }
}
```

#### 5. Update `src/main.rs` to remove Worker struct
```rust
// REMOVE the Worker struct definition entirely
// Remove: struct Worker { name, health, energy }

// Update imports
use components::{
    ComponentsPlugin, PositionComponent, HealthComponent,
    NameComponent, EnergyComponent, WorkerTag, WorkerStats
};

// Update the worker spawn in setup():
for i in 0..5 {
    let x = rng.gen_range(20..44);
    let y = rng.gen_range(20..44);
    
    if world_map.tiles[y][x].is_walkable() {
        let world_x = (x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
        let world_y = (y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
        
        let worker_name = format!("Worker {}", i + 1);
        
        let worker_entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.75, 0.0),
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.6, TILE_SIZE * 0.6)),
                    ..default()
                },
                transform: Transform::from_xyz(world_x, world_y, 1.0),
                ..default()
            },
            // Components instead of Worker struct
            WorkerTag,
            WorkerStats::default(),
            NameComponent::new(worker_name.clone()),
            PositionComponent::from_tile(x, y),
            HealthComponent::new(100.0),
            EnergyComponent::new(100.0),
            TileEntity { x, y },
        )).id();
        
        println!("{}", format!("[SPAWN] {} at ({}, {}) [Entity: {:?}]", 
            worker_name, x, y, worker_entity).green());
    }
}

// Update movement system to use components
fn simulation_system(
    time: Res<Time>,
    mut sim_state: ResMut<SimulationState>,
    mut workers: Query<(
        &mut Transform, 
        &mut TileEntity,
        &NameComponent,
        &mut PositionComponent,
        &mut EnergyComponent,
    ), With<WorkerTag>>,
    world_map: Res<WorldMap>,
    debug: Res<DebugSystem>,
) {
    if !sim_state.running {
        return;
    }
    
    sim_state.accumulated_time += time.delta_seconds() * sim_state.speed;
    
    if sim_state.accumulated_time >= 1.0 {
        sim_state.accumulated_time = 0.0;
        sim_state.tick += 1;
        
        let mut rng = rand::thread_rng();
        for (mut transform, mut tile_entity, name, mut position, mut energy) in workers.iter_mut() {
            // Drain energy when moving
            if energy.current > 0.0 && rng.gen_bool(0.3) {
                let dx = rng.gen_range(-1..=1);
                let dy = rng.gen_range(-1..=1);
                
                let new_x = (tile_entity.x as i32 + dx).max(0).min(MAP_SIZE as i32 - 1) as usize;
                let new_y = (tile_entity.y as i32 + dy).max(0).min(MAP_SIZE as i32 - 1) as usize;
                
                if world_map.tiles[new_y][new_x].is_walkable() {
                    // Update all position data
                    tile_entity.x = new_x;
                    tile_entity.y = new_y;
                    position.x = new_x as f32;
                    position.y = new_y as f32;
                    
                    let world_x = (new_x as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
                    let world_y = (new_y as f32 - MAP_SIZE as f32 / 2.0) * TILE_SIZE;
                    transform.translation.x = world_x;
                    transform.translation.y = world_y;
                    
                    // Consume energy for movement
                    energy.consume(1.0, &name.name, &debug);
                    
                    // Log movement occasionally
                    if rng.gen_bool(0.1) {
                        debug.log(
                            DebugLevel::Debug,
                            "MOVEMENT",
                            &format!("{} moved to ({}, {})", name.name, new_x, new_y)
                        );
                    }
                }
            }
        }
    }
}
```

### Debug Validation

#### Expected Terminal Output
```bash
$ RUST_LOG=debug cargo run

[COMPONENTS] Registering component systems...
[COMPONENTS] ✓ All components registered
[SPAWN] Worker 1 at (23, 31) [Entity: 12v0]
[SPAWN] Worker 2 at (35, 28) [Entity: 13v0]
[SPAWN] Worker 3 at (30, 33) [Entity: 14v0]
[SPAWN] Worker 4 at (25, 25) [Entity: 15v0]
[SPAWN] Worker 5 at (40, 30) [Entity: 16v0]
...
[0.125] [MOVEMENT] DEBUG: Worker 1 moved to (23, 32)
[0.125] [ENERGY] DEBUG: Worker 1: Energy 99 (-1)
[0.250] [MOVEMENT] DEBUG: Worker 3 moved to (31, 33)
[0.250] [ENERGY] DEBUG: Worker 3: Energy 99 (-1)
```

### Testing Checklist

- [ ] Run with `RUST_LOG=debug cargo run`
- [ ] Verify component registration messages
- [ ] Confirm workers spawn with entity IDs
- [ ] Watch workers move (F1 for stats)
- [ ] See energy consumption in debug
- [ ] Check no compilation warnings
- [ ] Verify UI still shows worker info

---

## Phase 1 Completion Criteria

### Must Have (Before Moving to Phase 2)
1. ✅ Components replace hardcoded structs
2. ✅ Plugin system organizes code
3. ✅ Workers use component composition
4. ✅ Debug messages confirm all systems work
5. ✅ No regression in existing features

### Debug Validation Summary
```bash
# Final validation for Phase 1
$ RUST_LOG=info cargo run

# Expected output:
[COMPONENTS] ✓ All components registered
[PLUGIN] ✓ WorldPlugin registered
[PLUGIN] ✓ SimulationPlugin registered
[SPAWN] Worker 1-5 spawned with components
[STATS] Tick: X, Speed: 1.0x, Running: true
# Workers move around map
# Energy depletes with movement
# All systems functioning
```

### Performance Check
- FPS should remain > 30
- Memory usage stable
- No error messages in terminal

### Git Commit
```bash
# After all validation passes:
git add -A
git commit -m "Phase 1 Complete: Component-based architecture with plugin system"
```

## Troubleshooting Guide

### Common Issues

#### Issue: "use of undeclared type Worker"
**Cause**: Worker struct removed but still referenced
**Fix**: Replace all Worker references with component queries

#### Issue: Workers don't move
**Cause**: Movement system not updated for components
**Fix**: Ensure simulation_system uses correct component query

#### Issue: No debug output
**Cause**: Debug system not receiving events
**Fix**: Check DebugSystem resource is available

#### Issue: Compilation errors after removing Worker
**Cause**: UI system still references Worker fields
**Fix**: Update UI to query components instead

### Debug Commands to Test
```bash
# Increase verbosity to see all component operations
RUST_LOG=trace cargo run

# Run for specific module only
RUST_LOG=world_sim_simple::components=debug cargo run

# Check for memory leaks
RUST_LOG=debug cargo run 2>&1 | grep -i leak
```

## Next Steps (Phase 2 Preview)

After Phase 1 validation completes:
1. **Chunk System** - Divide world into 16x16 chunks
2. **Dynamic Loading** - Load/unload chunks based on view
3. **Spatial Optimization** - Improve performance with chunks

Phase 2 builds on the component architecture, so Phase 1 must be solid!

## Summary

Phase 1 transforms sim_simple's architecture in 3 careful steps:
1. **Components** provide flexibility
2. **Plugins** organize features  
3. **Refactoring** proves the system works

Each step validated through terminal debug output before proceeding.

Total time: ~2 hours
Result: Modular, extensible architecture ready for advanced features