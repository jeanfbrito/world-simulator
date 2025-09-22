use bevy::prelude::*;
use crate::components::GridPosition;

/// Marks an entity as occupying a grid cell
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct GridOccupant;

/// Marks an entity as a solid obstacle that blocks movement
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct SolidObstacle;

/// Marks an entity that can be walked through (like grass, flowers)
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct Walkable;

/// Defines the size of an entity's occupation footprint
#[derive(Component, Clone, Debug, Reflect)]
pub struct OccupationSize {
    pub width: u32,
    pub height: u32,
}

impl Default for OccupationSize {
    fn default() -> Self {
        Self { width: 1, height: 1 }
    }
}

impl OccupationSize {
    pub fn single() -> Self {
        Self { width: 1, height: 1 }
    }

    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Get all grid positions occupied by this entity
    pub fn get_occupied_positions(&self, base_pos: &GridPosition) -> Vec<GridPosition> {
        let mut positions = Vec::new();

        for x_offset in 0..self.width {
            for y_offset in 0..self.height {
                positions.push(GridPosition::new(
                    base_pos.x + x_offset,
                    base_pos.y + y_offset,
                ));
            }
        }

        positions
    }
}

/// Event fired when an entity's occupation changes
#[derive(Event, Clone, Debug)]
pub struct OccupationChangedEvent {
    pub entity: Entity,
    pub old_position: Option<GridPosition>,
    pub new_position: Option<GridPosition>,
}