use crate::simulation::*;
/// Grid-based position system for tick-based movement
///
/// This module implements a grid-based positioning system where units
/// occupy discrete tiles and movement happens in tick-based steps.
/// Visual interpolation provides smooth movement for presentation.
use bevy::prelude::*;
use std::collections::HashSet;

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
        let dx = (self.x as i32 - other.x as i32).unsigned_abs();
        let dy = (self.y as i32 - other.y as i32).unsigned_abs();
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
    
    /// Generate a simple path between two positions
    pub fn path_between(from: &GridPosition, to: &GridPosition) -> Vec<GridPosition> {
        let mut path = Vec::new();
        let mut current = from.clone();
        
        // Simple straight-line pathfinding
        while current != *to {
            current = current.step_toward(to);
            path.push(current.clone());
        }
        
        path
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
    /// Current index in the path (which node we're moving toward)
    pub current_path_index: usize,
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
            current_path_index: 0,
            progress_counter: 0,
            is_moving: false,
            speed_modifier: 1.0,
        }
    }

    /// Set a new movement target (generates path from current position)
    pub fn set_target(&mut self, target: GridPosition) {
        // Don't generate path here - we need current position which we don't have
        // This will be handled by set_target_from
        self.target = Some(target);
        self.is_moving = true;
        self.progress_counter = 0;
    }
    
    /// Set a new movement target with path generation from current position
    pub fn set_target_from(&mut self, current: &GridPosition, target: GridPosition) {
        // Generate simple path from current to target
        let path = GridPosition::path_between(current, &target);
        if !path.is_empty() {
            self.set_path(path);
        } else {
            // Same position, stop moving
            self.stop();
        }
    }
    
    /// Set a new movement target with A* pathfinding around obstacles
    pub fn set_target_from_with_pathfinding(&mut self, current: &GridPosition, target: GridPosition, obstacles: &HashSet<(i32, i32)>) {
        // Convert positions to Vec3 for pathfinding
        let start = Vec3::new(current.x as f32 * 10.0, current.y as f32 * 10.0, 0.0);
        let goal = Vec3::new(target.x as f32 * 10.0, target.y as f32 * 10.0, 0.0);
        
        // Use A* pathfinding
        if let Some(path) = crate::ai::pathfinding::find_path(start, goal, obstacles) {
            // Convert path waypoints to GridPositions
            let grid_path: Vec<GridPosition> = path.get_waypoints()
                .into_iter()
                .map(|(x, y)| GridPosition::new(x as u32, y as u32))
                .collect();
            
            if !grid_path.is_empty() {
                self.set_path(grid_path);
            }
        } else {
            // Fallback to simple pathfinding if A* fails
            self.set_target_from(current, target);
        }
    }

    /// Set a path to follow
    pub fn set_path(&mut self, path: Vec<GridPosition>) {
        if !path.is_empty() {
            self.path = path;
            self.current_path_index = 0;
            // Set target to first position in path
            self.target = self.path.get(0).cloned();
            self.is_moving = true;
            self.progress_counter = 0;
        }
    }

    /// Clear movement
    pub fn stop(&mut self) {
        self.target = None;
        self.path.clear();
        self.current_path_index = 0;
        self.is_moving = false;
        self.progress_counter = 0;
    }

    /// Update movement progress (called on ticks)
    /// Returns true if movement to a tile was completed
    pub fn tick_update(&mut self, current_pos: &mut GridPosition, ticks_per_tile: u32) -> bool {
        if !self.is_moving || self.target.is_none() {
            return false;
        }

        // Calculate progress increment based on movement speed
        // We need to ensure we reach exactly MAX_WORK_PROGRESS in the specified ticks
        // Use ceiling division to avoid off-by-one errors
        let progress_increment = (MAX_WORK_PROGRESS + ticks_per_tile - 1) / ticks_per_tile.max(1);
        self.progress_counter += progress_increment;

        // Check if we've completed movement to next tile
        if self.progress_counter >= MAX_WORK_PROGRESS {
            self.progress_counter = 0;

            // Move to target
            if let Some(target) = &self.target {
                *current_pos = target.clone();

                // If following a path, advance to next target
                if !self.path.is_empty() && self.current_path_index < self.path.len() {
                    // Move to next position in path
                    self.current_path_index += 1;
                    
                    if self.current_path_index < self.path.len() {
                        // Set next target
                        self.target = self.path.get(self.current_path_index).cloned();
                    } else {
                        // Path complete - we've reached the end
                        self.stop();
                        return true; // Movement complete
                    }
                } else {
                    // No path or reached end
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
    
    /// Get the current path (for debugging)
    pub fn get_path(&self) -> &[GridPosition] {
        &self.path
    }
    
    /// Get current position in path
    pub fn get_path_progress(&self) -> (usize, usize) {
        (self.current_path_index, self.path.len())
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

        println!(
            "Migrated entity {:?} to grid-based position ({}, {})",
            entity, grid_pos.x, grid_pos.y
        );
    }
}
