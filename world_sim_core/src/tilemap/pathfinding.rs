//! Pathfinding integration using bevy_entitiles built-in algorithms

use bevy::prelude::*;
use std::collections::HashMap;
#[cfg(feature = "algorithm")]
use bevy_entitiles::algorithm::pathfinding::*;
use world_sim_interface::Position;
use crate::components::{PositionComponent, MovementComponent};
use super::{world_grid::WorldGrid, PathRequestEvent, PathCallback};

/// Resource for managing pathfinding
#[derive(Resource, Default)]
pub struct PathfindingManager {
    /// Pending path requests
    pending: Vec<PathRequest>,
}

/// Internal path request structure
struct PathRequest {
    entity: Entity,
    start: Position,
    target: Position,
    callback: PathCallback,
}

/// Component for entities with active paths
#[derive(Component)]
pub struct PathComponent {
    pub path: Vec<Position>,
    pub current_index: usize,
}

impl PathComponent {
    pub fn new(path: Vec<Position>) -> Self {
        Self {
            path,
            current_index: 0,
        }
    }
    
    /// Get the next position in the path
    pub fn next_position(&self) -> Option<&Position> {
        self.path.get(self.current_index)
    }
    
    /// Advance to the next position
    pub fn advance(&mut self) {
        self.current_index += 1;
    }
    
    /// Check if path is complete
    pub fn is_complete(&self) -> bool {
        self.current_index >= self.path.len()
    }
}

/// System to process pathfinding requests
pub fn process_path_requests(
    mut events: EventReader<PathRequestEvent>,
    mut pathfinding: ResMut<PathfindingManager>,
    world_grid: Res<WorldGrid>,
    mut commands: Commands,
) {
    for event in events.read() {
        // Convert to internal request
        let request = PathRequest {
            entity: event.entity,
            start: event.start,
            target: event.target,
            callback: event.callback,
        };
        
        // Calculate path using bevy_entitiles pathfinding
        if let Some(path) = calculate_path(&world_grid, &request.start, &request.target) {
            // Add path component to entity
            commands.entity(request.entity)
                .insert(PathComponent::new(path));
            
            // Handle callback
            match request.callback {
                PathCallback::Movement => {
                    // Path will be used by movement system
                }
                PathCallback::AIPlanning => {
                    // Notify AI that path is ready
                    commands.entity(request.entity)
                        .insert(PathReady);
                }
            }
        } else {
            // No path found
            commands.entity(request.entity)
                .insert(NoPathFound);
        }
    }
}

/// Calculate path using A* algorithm
fn calculate_path(
    world_grid: &WorldGrid,
    start: &Position,
    target: &Position,
) -> Option<Vec<Position>> {
    // Simple A* implementation
    // In production, this would use bevy_entitiles' built-in pathfinding
    
    use std::collections::{BinaryHeap, HashMap};
    use std::cmp::Ordering;
    
    #[derive(Clone)]
    struct Node {
        position: Position,
        g_score: f32,
        f_score: f32,
    }
    
    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.f_score.eq(&other.f_score)
        }
    }
    
    impl Eq for Node {}
    
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            other.f_score.partial_cmp(&self.f_score)
                .unwrap_or(Ordering::Equal)
        }
    }
    
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    // Check if start and target are valid
    if !world_grid.is_walkable(start) || !world_grid.is_walkable(target) {
        return None;
    }
    
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut g_scores: HashMap<Position, f32> = HashMap::new();
    
    // Initialize start node
    g_scores.insert(*start, 0.0);
    open_set.push(Node {
        position: *start,
        g_score: 0.0,
        f_score: heuristic(start, target),
    });
    
    while let Some(current_node) = open_set.pop() {
        let current = current_node.position;
        
        // Found the target
        if current == *target {
            return Some(reconstruct_path(&came_from, current));
        }
        
        // Check neighbors
        for neighbor in get_neighbors(&current) {
            if !world_grid.in_bounds(&neighbor) || !world_grid.is_walkable(&neighbor) {
                continue;
            }
            
            let tentative_g = g_scores[&current] + world_grid.movement_cost(&neighbor);
            
            if tentative_g < *g_scores.get(&neighbor).unwrap_or(&f32::INFINITY) {
                came_from.insert(neighbor, current);
                g_scores.insert(neighbor, tentative_g);
                
                open_set.push(Node {
                    position: neighbor,
                    g_score: tentative_g,
                    f_score: tentative_g + heuristic(&neighbor, target),
                });
            }
        }
    }
    
    None
}

/// Get neighboring positions
fn get_neighbors(pos: &Position) -> Vec<Position> {
    vec![
        Position::new(pos.x - 1, pos.y),
        Position::new(pos.x + 1, pos.y),
        Position::new(pos.x, pos.y - 1),
        Position::new(pos.x, pos.y + 1),
        // Diagonal movement
        Position::new(pos.x - 1, pos.y - 1),
        Position::new(pos.x + 1, pos.y - 1),
        Position::new(pos.x - 1, pos.y + 1),
        Position::new(pos.x + 1, pos.y + 1),
    ]
}

/// Heuristic function for A*
fn heuristic(a: &Position, b: &Position) -> f32 {
    let dx = (a.x - b.x).abs() as f32;
    let dy = (a.y - b.y).abs() as f32;
    (dx * dx + dy * dy).sqrt()
}

/// Reconstruct path from came_from map
fn reconstruct_path(came_from: &HashMap<Position, Position>, mut current: Position) -> Vec<Position> {
    let mut path = vec![current];
    
    while let Some(&prev) = came_from.get(&current) {
        path.push(prev);
        current = prev;
    }
    
    path.reverse();
    path
}

/// Marker component for entities with ready paths
#[derive(Component)]
pub struct PathReady;

/// Marker component for entities with no valid path
#[derive(Component)]
pub struct NoPathFound;

/// System to follow paths for movement
pub fn follow_path_system(
    mut query: Query<(
        &mut PositionComponent,
        &mut MovementComponent,
        &mut PathComponent,
    )>,
    time: Res<Time>,
) {
    for (mut pos, mut movement, mut path) in query.iter_mut() {
        if path.is_complete() {
            continue;
        }
        
        if let Some(target) = path.next_position() {
            // Move towards next position in path
            let distance = pos.distance_to(&PositionComponent::at(*target));
            
            if distance < 0.1 {
                // Reached waypoint, advance to next
                path.advance();
            } else {
                // Move towards waypoint
                let speed = movement.speed * time.delta_secs();
                pos.move_towards(target, speed as i32);
            }
        }
    }
}