use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::components::{
    GridPosition, GridOccupant, SolidObstacle, OccupationSize,
    resource::ResourceNode, UnitTag
};
use crate::debug::{DebugSystem, DebugLevel};

/// Resource that tracks which entities occupy which grid cells
#[derive(Resource, Debug, Default)]
pub struct GridOccupationMap {
    /// Maps grid positions to the entities occupying them
    occupied_cells: HashMap<(u32, u32), HashSet<Entity>>,
    /// Maps entities to their occupied positions (for quick lookup)
    entity_positions: HashMap<Entity, Vec<(u32, u32)>>,
    /// Set of positions that are solid obstacles (can't be walked through)
    solid_obstacles: HashSet<(u32, u32)>,
}

impl GridOccupationMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an entity occupying a position
    pub fn occupy(&mut self, entity: Entity, position: &GridPosition, size: &OccupationSize, is_solid: bool) {
        // Get all positions this entity occupies based on its size
        let positions = size.get_occupied_positions(position);

        // Store the entity's positions
        let pos_tuples: Vec<(u32, u32)> = positions.iter()
            .map(|p| (p.x, p.y))
            .collect();

        // Remove any previous positions for this entity
        if let Some(old_positions) = self.entity_positions.get(&entity) {
            for old_pos in old_positions {
                if let Some(occupants) = self.occupied_cells.get_mut(old_pos) {
                    occupants.remove(&entity);
                    if occupants.is_empty() {
                        self.occupied_cells.remove(old_pos);
                        self.solid_obstacles.remove(old_pos);
                    }
                }
            }
        }

        // Add new positions
        for pos_tuple in &pos_tuples {
            self.occupied_cells
                .entry(*pos_tuple)
                .or_insert_with(HashSet::new)
                .insert(entity);

            if is_solid {
                self.solid_obstacles.insert(*pos_tuple);
            }
        }

        self.entity_positions.insert(entity, pos_tuples);
    }

    /// Release an entity's occupation
    pub fn release(&mut self, entity: Entity) {
        if let Some(positions) = self.entity_positions.remove(&entity) {
            for pos in positions {
                if let Some(occupants) = self.occupied_cells.get_mut(&pos) {
                    occupants.remove(&entity);
                    if occupants.is_empty() {
                        self.occupied_cells.remove(&pos);
                        self.solid_obstacles.remove(&pos);
                    }
                }
            }
        }
    }

    /// Check if a position is occupied by any entity
    pub fn is_occupied(&self, position: &GridPosition) -> bool {
        self.occupied_cells.contains_key(&(position.x, position.y))
    }

    /// Check if a position is a solid obstacle
    pub fn is_solid_obstacle(&self, position: &GridPosition) -> bool {
        self.solid_obstacles.contains(&(position.x, position.y))
    }

    /// Check if a position is walkable (not a solid obstacle)
    pub fn is_walkable(&self, position: &GridPosition) -> bool {
        !self.is_solid_obstacle(position)
    }

    /// Get entities at a specific position
    pub fn get_entities_at(&self, position: &GridPosition) -> Option<&HashSet<Entity>> {
        self.occupied_cells.get(&(position.x, position.y))
    }

    /// Check if a specific entity occupies a position
    pub fn is_occupied_by(&self, position: &GridPosition, entity: Entity) -> bool {
        if let Some(entities) = self.get_entities_at(position) {
            entities.contains(&entity)
        } else {
            false
        }
    }

    /// Get all solid obstacle positions for pathfinding
    pub fn get_obstacle_set(&self) -> HashSet<(i32, i32)> {
        self.solid_obstacles
            .iter()
            .map(|(x, y)| (*x as i32, *y as i32))
            .collect()
    }

    /// Find the nearest walkable position to a target
    pub fn find_nearest_walkable(&self, target: &GridPosition, max_distance: u32) -> Option<GridPosition> {
        // Check if target is already walkable
        if self.is_walkable(target) {
            return Some(target.clone());
        }

        // Search in expanding circles
        for distance in 1..=max_distance {
            for dx in -(distance as i32)..=(distance as i32) {
                for dy in -(distance as i32)..=(distance as i32) {
                    // Check if this is on the current distance ring
                    if dx.abs().max(dy.abs()) != distance as i32 {
                        continue;
                    }

                    let new_x = target.x as i32 + dx;
                    let new_y = target.y as i32 + dy;

                    if new_x >= 0 && new_y >= 0 {
                        let pos = GridPosition::new(new_x as u32, new_y as u32);
                        if self.is_walkable(&pos) {
                            return Some(pos);
                        }
                    }
                }
            }
        }

        None
    }
}

/// System that updates the grid occupation map when entities move
pub fn update_grid_occupation_system(
    mut occupation_map: ResMut<GridOccupationMap>,
    // Track units
    units_query: Query<
        (Entity, &GridPosition, Option<&OccupationSize>),
        (With<UnitTag>, Or<(Changed<GridPosition>, Added<GridPosition>)>)
    >,
    // Track resource nodes
    resources_query: Query<
        (Entity, &GridPosition, Option<&OccupationSize>),
        (With<ResourceNode>, Or<(Changed<GridPosition>, Added<GridPosition>)>)
    >,
    // Track removed entities
    mut removed_units: RemovedComponents<UnitTag>,
    mut removed_resources: RemovedComponents<ResourceNode>,
    debug: Res<DebugSystem>,
) {
    // Update units
    for (entity, position, size) in units_query.iter() {
        let size = size.cloned().unwrap_or_default();
        occupation_map.occupy(entity, position, &size, false); // Units are not solid obstacles

        debug.log(
            DebugLevel::Debug,
            "OCCUPATION",
            &format!("Unit {:?} occupying ({}, {})", entity, position.x, position.y)
        );
    }

    // Update resource nodes
    for (entity, position, size) in resources_query.iter() {
        let size = size.cloned().unwrap_or_default();
        occupation_map.occupy(entity, position, &size, true); // Resources are solid obstacles

        debug.log(
            DebugLevel::Debug,
            "OCCUPATION",
            &format!("Resource {:?} occupying ({}, {}) as solid obstacle", entity, position.x, position.y)
        );
    }

    // Clean up removed units
    for entity in removed_units.read() {
        occupation_map.release(entity);

        debug.log(
            DebugLevel::Debug,
            "OCCUPATION",
            &format!("Released occupation for removed unit {:?}", entity)
        );
    }

    // Clean up removed resources
    for entity in removed_resources.read() {
        occupation_map.release(entity);

        debug.log(
            DebugLevel::Debug,
            "OCCUPATION",
            &format!("Released occupation for removed resource {:?}", entity)
        );
    }
}

/// System to initialize occupation for existing entities on startup
pub fn init_grid_occupation_system(
    mut occupation_map: ResMut<GridOccupationMap>,
    units_query: Query<(Entity, &GridPosition, Option<&OccupationSize>), With<UnitTag>>,
    resources_query: Query<(Entity, &GridPosition, Option<&OccupationSize>), With<ResourceNode>>,
    debug: Res<DebugSystem>,
) {
    let mut unit_count = 0;
    let mut resource_count = 0;

    // Register all existing units
    for (entity, position, size) in units_query.iter() {
        let size = size.cloned().unwrap_or_default();
        occupation_map.occupy(entity, position, &size, false);
        unit_count += 1;
    }

    // Register all existing resources
    for (entity, position, size) in resources_query.iter() {
        let size = size.cloned().unwrap_or_default();
        occupation_map.occupy(entity, position, &size, true);
        resource_count += 1;
    }

    debug.log(
        DebugLevel::Info,
        "OCCUPATION_INIT",
        &format!("Initialized grid occupation with {} units and {} resources", unit_count, resource_count)
    );
}

/// Helper function to get adjacent walkable positions around a target
pub fn get_adjacent_walkable_positions(
    target: &GridPosition,
    occupation_map: &GridOccupationMap,
    world_map: &crate::WorldMap,
) -> Vec<GridPosition> {
    let mut walkable_positions = Vec::new();

    for adjacent in target.get_adjacent() {
        // Check bounds
        if adjacent.x >= 64 || adjacent.y >= 64 {
            continue;
        }

        // Check if terrain is walkable
        if !world_map.tiles[adjacent.y as usize][adjacent.x as usize].is_walkable() {
            continue;
        }

        // Check if not occupied by solid obstacle
        if !occupation_map.is_solid_obstacle(&adjacent) {
            walkable_positions.push(adjacent);
        }
    }

    walkable_positions
}