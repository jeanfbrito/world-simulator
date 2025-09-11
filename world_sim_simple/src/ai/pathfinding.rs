use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{VecDeque, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathNode {
    pub position: Vec3,
    pub cost: f32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    nodes: VecDeque<PathNode>,
    current_index: usize,
    total_distance: f32,
}

impl Path {
    pub fn new(nodes: Vec<PathNode>) -> Self {
        let total_distance = nodes.windows(2)
            .map(|w| w[0].position.distance(w[1].position))
            .sum();
        
        info!("[PATH] Created path with {} nodes, distance: {:.1}", 
            nodes.len(), total_distance);
        
        Self {
            nodes: nodes.into(),
            current_index: 0,
            total_distance,
        }
    }
    
    // Tick-based movement for consistent simulation
    pub fn follow_tick(&mut self, transform: &mut Transform, tiles_per_tick: f32) -> bool {
        if self.current_index >= self.nodes.len() {
            return true; // Path complete
        }
        
        let current_node = &self.nodes[self.current_index];
        let to_target = current_node.position - transform.translation;
        let distance = to_target.length();
        
        // Move by fixed amount per tick
        let move_distance = tiles_per_tick * 10.0; // Convert tiles to world units (10.0 units per tile)
        
        if distance <= move_distance {
            // Reached current node, move to next
            transform.translation = current_node.position;
            self.current_index += 1;
            
            if self.current_index >= self.nodes.len() {
                info!("[PATH] Path following completed");
                return true;
            }
        } else {
            // Move towards current node
            let direction = to_target.normalize();
            transform.translation += direction * move_distance;
        }
        
        false
    }
    
    // Legacy delta-time based movement (kept for compatibility)
    pub fn follow(&mut self, delta_time: f32, transform: &mut Transform) -> bool {
        if self.current_index >= self.nodes.len() {
            return true; // Path complete
        }
        
        let current_node = &self.nodes[self.current_index];
        let direction = (current_node.position - transform.translation).normalize();
        let speed = 50.0; // Units per second
        
        transform.translation += direction * speed * delta_time;
        
        // Check if reached current node
        if transform.translation.distance(current_node.position) < 2.0 {
            self.current_index += 1;
            
            if self.current_index >= self.nodes.len() {
                info!("[PATH] Path following completed");
                return true;
            }
        }
        
        false
    }
    
    pub fn is_complete(&self) -> bool {
        self.current_index >= self.nodes.len()
    }
    
    pub fn remaining_distance(&self) -> f32 {
        if self.current_index >= self.nodes.len() {
            return 0.0;
        }
        
        self.nodes.iter()
            .skip(self.current_index)
            .zip(self.nodes.iter().skip(self.current_index + 1))
            .map(|(a, b)| a.position.distance(b.position))
            .sum()
    }
    
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}

// Simple A* pathfinding implementation
pub fn find_path(start: Vec3, goal: Vec3, obstacles: &HashSet<(i32, i32)>) -> Option<Path> {
    info!("[PATH] Finding path from {:?} to {:?}", start, goal);
    
    // Convert to grid coordinates
    let start_grid = world_to_grid(start);
    let goal_grid = world_to_grid(goal);
    
    // Simple case - if no obstacles, direct path
    if obstacles.is_empty() || !has_obstacles_between(start_grid, goal_grid, obstacles) {
        let nodes = vec![
            PathNode { position: start, cost: 0.0 },
            PathNode { position: goal, cost: start.distance(goal) },
        ];
        return Some(Path::new(nodes));
    }
    
    // A* pathfinding
    let path_grid = astar_pathfind(start_grid, goal_grid, obstacles)?;
    
    // Convert grid path back to world coordinates
    let nodes: Vec<PathNode> = path_grid
        .into_iter()
        .map(|(x, y)| PathNode {
            position: grid_to_world(x, y),
            cost: 0.0, // Cost calculated during A*
        })
        .collect();
    
    if nodes.is_empty() {
        None
    } else {
        Some(Path::new(nodes))
    }
}

fn world_to_grid(pos: Vec3) -> (i32, i32) {
    ((pos.x / 10.0) as i32, (pos.y / 10.0) as i32)
}

fn grid_to_world(x: i32, y: i32) -> Vec3 {
    Vec3::new(x as f32 * 10.0, y as f32 * 10.0, 0.0)
}

fn has_obstacles_between(start: (i32, i32), goal: (i32, i32), obstacles: &HashSet<(i32, i32)>) -> bool {
    // Simple line check for obstacles
    let dx = (goal.0 - start.0).abs();
    let dy = (goal.1 - start.1).abs();
    let steps = dx.max(dy);
    
    if steps == 0 {
        return false;
    }
    
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = (start.0 as f32 + (goal.0 - start.0) as f32 * t) as i32;
        let y = (start.1 as f32 + (goal.1 - start.1) as f32 * t) as i32;
        
        if obstacles.contains(&(x, y)) {
            return true;
        }
    }
    
    false
}

fn astar_pathfind(
    start: (i32, i32),
    goal: (i32, i32),
    obstacles: &HashSet<(i32, i32)>,
) -> Option<Vec<(i32, i32)>> {
    use std::collections::BinaryHeap;
    use std::cmp::Ordering;
    
    #[derive(Clone, Eq, PartialEq)]
    struct State {
        cost: i32,
        position: (i32, i32),
    }
    
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }
    
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    let mut frontier = BinaryHeap::new();
    let mut came_from = std::collections::HashMap::new();
    let mut cost_so_far = std::collections::HashMap::new();
    
    frontier.push(State { cost: 0, position: start });
    cost_so_far.insert(start, 0);
    
    // Neighbors: orthogonal first, then diagonal (inspired by bevy_entitiles)
    let orthogonal = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let diagonal = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
    
    while let Some(State { cost: _, position }) = frontier.pop() {
        if position == goal {
            // Reconstruct path
            let mut path = Vec::new();
            let mut current = goal;
            
            while current != start {
                path.push(current);
                current = came_from[&current];
            }
            path.push(start);
            path.reverse();
            
            info!("[PATH] A* found path with {} nodes", path.len());
            return Some(path);
        }
        
        let current_cost = cost_so_far[&position];
        
        // Check orthogonal neighbors first (cost: 10)
        for &(dx, dy) in &orthogonal {
            let next = (position.0 + dx, position.1 + dy);
            
            if obstacles.contains(&next) {
                continue;
            }
            
            let new_cost = current_cost + 10;
            
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + heuristic(next, goal);
                frontier.push(State { cost: priority, position: next });
                came_from.insert(next, position);
            }
        }
        
        // Check diagonal neighbors (cost: 14, approximating sqrt(2) * 10)
        for &(dx, dy) in &diagonal {
            let next = (position.0 + dx, position.1 + dy);
            
            if obstacles.contains(&next) {
                continue;
            }
            
            // Check if diagonal movement is blocked by adjacent tiles
            let blocked_x = obstacles.contains(&(position.0 + dx, position.1));
            let blocked_y = obstacles.contains(&(position.0, position.1 + dy));
            
            if blocked_x || blocked_y {
                continue; // Can't move diagonally if adjacent tiles are blocked
            }
            
            let new_cost = current_cost + 14; // Diagonal cost
            
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + heuristic(next, goal);
                frontier.push(State { cost: priority, position: next });
                came_from.insert(next, position);
            }
        }
    }
    
    info!("[PATH] No path found");
    None
}

fn heuristic(a: (i32, i32), b: (i32, i32)) -> i32 {
    // Manhattan distance * 10 for integer math
    manhattan_distance(a, b) * 10
}

// Utility function inspired by bevy_entitiles for cleaner distance calculations
fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn euclidean_distance_squared(a: (i32, i32), b: (i32, i32)) -> i32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}