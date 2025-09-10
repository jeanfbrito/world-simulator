/// Grid-based position system for tick-based movement
/// 
/// This module implements a grid-based positioning system where units
/// occupy discrete tiles and movement happens in tick-based steps.
/// Visual interpolation provides smooth movement for presentation.

use bevy::prelude::*;
use crate::simulation::*;

/// The authoritative grid position for simulation logic
/// This is the "true" position used for all game logic
#[derive(Component, Clone, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

impl GridPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    
    /// Convert from tile coordinates
    pub fn from_tile(x: usize, y: usize) -> Self {
        Self {
            x: x as u32,
            y: y as u32,
        }
    }
    
    /// Get as tuple for easy use
    pub fn as_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }
    
    /// Manhattan distance to another position
    pub fn distance_to(&self, other: &GridPosition) -> u32 {
        let dx = (self.x as i32 - other.x as i32).abs() as u32;
        let dy = (self.y as i32 - other.y as i32).abs() as u32;
        dx + dy
    }
    
    /// Check if adjacent (including diagonals)
    pub fn is_adjacent_to(&self, other: &GridPosition) -> bool {
        let dx = (self.x as i32 - other.x as i32).abs();
        let dy = (self.y as i32 - other.y as i32).abs();
        dx <= 1 && dy <= 1 && (dx + dy) > 0
    }
    
    /// Get all adjacent positions (8-directional)
    pub fn get_adjacent(&self) -> Vec<GridPosition> {
        let mut adjacent = Vec::with_capacity(8);
        
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let new_x = self.x as i32 + dx;
                let new_y = self.y as i32 + dy;
                
                if new_x >= 0 && new_y >= 0 {
                    adjacent.push(GridPosition::new(new_x as u32, new_y as u32));
                }
            }
        }
        
        adjacent
    }
    
    /// Move toward target by one tile
    pub fn step_toward(&self, target: &GridPosition) -> GridPosition {
        let dx = (target.x as i32 - self.x as i32).signum();
        let dy = (target.y as i32 - self.y as i32).signum();
        
        GridPosition::new(
            (self.x as i32 + dx).max(0) as u32,
            (self.y as i32 + dy).max(0) as u32,
        )
    }
}

/// Visual position for smooth interpolation between grid positions
/// This is used only for rendering/presentation
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct VisualPosition {
    /// Current interpolated position in world space
    pub current: Vec3,
    /// Target position in world space (where we're moving to)
    pub target: Vec3,
    /// Interpolation speed (tiles per second)
    pub speed: f32,
}

impl VisualPosition {
    pub fn new(grid_pos: &GridPosition, tile_size: f32) -> Self {
        let world_pos = grid_to_world(grid_pos, tile_size);
        Self {
            current: world_pos,
            target: world_pos,
            speed: 5.0, // Default 5 tiles per second visual movement
        }
    }
    
    /// Update target from grid position
    pub fn set_target(&mut self, grid_pos: &GridPosition, tile_size: f32) {
        self.target = grid_to_world(grid_pos, tile_size);
    }
    
    /// Interpolate toward target (called every frame)
    pub fn interpolate(&mut self, delta_time: f32) {
        let distance = self.current.distance(self.target);
        if distance > 0.01 {
            let move_distance = self.speed * delta_time;
            let t = (move_distance / distance).min(1.0);
            self.current = self.current.lerp(self.target, t);
        } else {
            self.current = self.target;
        }
    }
    
    /// Check if visual has caught up to target
    pub fn at_target(&self) -> bool {
        self.current.distance(self.target) < 0.01
    }
}

/// Movement state for tick-based movement
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct GridMovement {
    /// Target grid position we're moving toward
    pub target: Option<GridPosition>,
    /// Path to follow (list of grid positions)
    pub path: Vec<GridPosition>,
    /// Movement progress counter (0 to MAX_MOVE_PROGRESS)
    pub progress_counter: u32,
    /// Whether currently moving
    pub is_moving: bool,
    /// Movement speed modifier (1.0 = normal)
    pub speed_modifier: f32,
}

impl GridMovement {
    pub fn new() -> Self {
        Self {
            target: None,
            path: Vec::new(),
            progress_counter: 0,
            is_moving: false,
            speed_modifier: 1.0,
        }
    }
    
    /// Set a new movement target
    pub fn set_target(&mut self, target: GridPosition) {
        self.target = Some(target);
        self.is_moving = true;
        self.progress_counter = 0;
    }
    
    /// Set a path to follow
    pub fn set_path(&mut self, path: Vec<GridPosition>) {
        if !path.is_empty() {
            self.path = path;
            self.target = self.path.first().cloned();
            self.is_moving = true;
            self.progress_counter = 0;
        }
    }
    
    /// Clear movement
    pub fn stop(&mut self) {
        self.target = None;
        self.path.clear();
        self.is_moving = false;
        self.progress_counter = 0;
    }
    
    /// Update movement progress (called on ticks)
    pub fn tick_update(&mut self, current_pos: &mut GridPosition) -> bool {
        if !self.is_moving || self.target.is_none() {
            return false;
        }
        
        // Increment progress based on speed
        let progress_increment = (MOVE_PROGRESS_PER_TICK as f32 * self.speed_modifier) as u32;
        self.progress_counter += progress_increment;
        
        // Check if we've completed movement to next tile
        if self.progress_counter >= MAX_WORK_PROGRESS {
            self.progress_counter = 0;
            
            // Move to target
            if let Some(target) = &self.target {
                *current_pos = target.clone();
                
                // If following a path, get next target
                if !self.path.is_empty() {
                    self.path.remove(0);
                    if !self.path.is_empty() {
                        self.target = self.path.first().cloned();
                    } else {
                        // Path complete
                        self.stop();
                        return true; // Movement complete
                    }
                } else {
                    // Single target reached
                    self.stop();
                    return true; // Movement complete
                }
            }
        }
        
        false // Still moving
    }
    
    /// Get movement progress as a float (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        self.progress_counter as f32 / MAX_WORK_PROGRESS as f32
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convert grid position to world space
pub fn grid_to_world(grid_pos: &GridPosition, tile_size: f32) -> Vec3 {
    Vec3::new(
        grid_pos.x as f32 * tile_size,
        grid_pos.y as f32 * tile_size,
        0.0,
    )
}

/// Convert world position to grid position
pub fn world_to_grid(world_pos: Vec3, tile_size: f32) -> GridPosition {
    GridPosition::new(
        (world_pos.x / tile_size).floor().max(0.0) as u32,
        (world_pos.y / tile_size).floor().max(0.0) as u32,
    )
}

// ============================================================================
// MIGRATION SYSTEM
// ============================================================================

/// System to migrate from old position components to grid-based
pub fn migrate_positions_system(
    mut commands: Commands,
    query: Query<(Entity, &crate::TileEntity), Without<GridPosition>>,
) {
    for (entity, tile_entity) in query.iter() {
        // Create grid position from tile entity
        let grid_pos = GridPosition::from_tile(tile_entity.x, tile_entity.y);
        
        // Add grid components
        commands.entity(entity).insert((
            grid_pos.clone(),
            VisualPosition::new(&grid_pos, 10.0), // Using default tile size
            GridMovement::new(),
        ));
        
        println!("Migrated entity {:?} to grid-based position ({}, {})", 
            entity, grid_pos.x, grid_pos.y);
    }
}