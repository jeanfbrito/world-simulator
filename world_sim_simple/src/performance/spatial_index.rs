use crate::components::PositionComponent;
use crate::debug::{DebugLevel, DebugSystem};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

const CELL_SIZE: f32 = 32.0; // Size of each spatial hash cell

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellCoordinate {
    pub x: i32,
    pub y: i32,
}

impl CellCoordinate {
    pub fn from_position(pos: &PositionComponent) -> Self {
        Self {
            x: (pos.x / CELL_SIZE) as i32,
            y: (pos.y / CELL_SIZE) as i32,
        }
    }

    pub fn neighbors(&self) -> Vec<CellCoordinate> {
        let mut neighbors = Vec::with_capacity(9);
        for dx in -1..=1 {
            for dy in -1..=1 {
                neighbors.push(CellCoordinate {
                    x: self.x + dx,
                    y: self.y + dy,
                });
            }
        }
        neighbors
    }
}

#[derive(Resource, Default)]
pub struct SpatialIndex {
    cells: HashMap<CellCoordinate, HashSet<Entity>>,
    entity_cells: HashMap<Entity, CellCoordinate>,
    total_entities: usize,
    cells_occupied: usize,
}

impl SpatialIndex {
    pub fn new() -> Self {
        info!(
            "[SPATIAL] Spatial index initialized with cell size: {}",
            CELL_SIZE
        );
        Self::default()
    }

    pub fn insert(&mut self, entity: Entity, position: &PositionComponent) {
        let cell = CellCoordinate::from_position(position);

        // Remove from old cell if moved
        if let Some(old_cell) = self.entity_cells.get(&entity) {
            if *old_cell != cell {
                if let Some(entities) = self.cells.get_mut(old_cell) {
                    entities.remove(&entity);
                    if entities.is_empty() {
                        self.cells.remove(old_cell);
                        self.cells_occupied = self.cells_occupied.saturating_sub(1);
                    }
                }
            }
        } else {
            self.total_entities += 1;
        }

        // Add to new cell
        self.cells
            .entry(cell)
            .or_insert_with(|| {
                self.cells_occupied += 1;
                HashSet::new()
            })
            .insert(entity);

        self.entity_cells.insert(entity, cell);
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(cell) = self.entity_cells.remove(&entity) {
            if let Some(entities) = self.cells.get_mut(&cell) {
                entities.remove(&entity);
                if entities.is_empty() {
                    self.cells.remove(&cell);
                    self.cells_occupied = self.cells_occupied.saturating_sub(1);
                }
            }
            self.total_entities = self.total_entities.saturating_sub(1);
        }
    }

    pub fn get_neighbors(&self, position: &PositionComponent, radius: f32) -> Vec<Entity> {
        let center_cell = CellCoordinate::from_position(position);
        let cell_radius = (radius / CELL_SIZE).ceil() as i32;

        let mut neighbors = Vec::new();
        let radius_squared = radius * radius;

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                let cell = CellCoordinate {
                    x: center_cell.x + dx,
                    y: center_cell.y + dy,
                };

                if let Some(entities) = self.cells.get(&cell) {
                    for &entity in entities {
                        // Would need actual position check here for accuracy
                        neighbors.push(entity);
                    }
                }
            }
        }

        neighbors
    }

    pub fn get_entities_in_cell(&self, cell: &CellCoordinate) -> Vec<Entity> {
        self.cells
            .get(cell)
            .map(|entities| entities.iter().copied().collect())
            .unwrap_or_default()
    }

    pub fn get_stats(&self) -> (usize, usize) {
        (self.total_entities, self.cells_occupied)
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.entity_cells.clear();
        self.total_entities = 0;
        self.cells_occupied = 0;
        info!("[SPATIAL] Spatial index cleared");
    }
}

pub type SpatialHash = SpatialIndex;

pub fn spatial_index_update_system(
    mut spatial_index: ResMut<SpatialIndex>,
    query: Query<(Entity, &PositionComponent), Changed<PositionComponent>>,
    mut removed: RemovedComponents<PositionComponent>,
    debug: Res<DebugSystem>,
) {
    let mut updates = 0;

    // Update moved entities
    for (entity, position) in query.iter() {
        spatial_index.insert(entity, position);
        updates += 1;
    }

    // Remove deleted entities
    for entity in removed.read() {
        spatial_index.remove(entity);
        updates += 1;
    }

    if updates > 0 {
        let (total, cells) = spatial_index.get_stats();
        debug.log(
            DebugLevel::Debug,
            "SPATIAL",
            &format!(
                "Updated {} entities. Total: {} in {} cells",
                updates, total, cells
            ),
        );

        // Log only occasionally to avoid spam
        static mut FRAME_COUNT: u32 = 0;
        unsafe {
            FRAME_COUNT += 1;
            if FRAME_COUNT % 60 == 0 {
                info!(
                    "[SPATIAL] Spatial index: {} entities in {} cells",
                    total, cells
                );
            }
        }
    }
}
