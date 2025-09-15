# Movement and Pathfinding System

The World Simulator uses a grid-based movement system with A* pathfinding for intelligent navigation across the tile-based world.

## 🗺️ Grid-Based World

### Coordinate System
```mermaid
graph LR
    subgraph "Grid Layout (32x32 default)"
        T00["(0,0)<br/>Top-Left"] --- T01["(1,0)"]
        T01 --- T02["(2,0)"]
        T02 --- T03["...(31,0)"]

        T00 --- T10["(0,1)"]
        T10 --- T20["(0,2)"]
        T20 --- T30["(0,31)<br/>Bottom-Left"]

        T03 --- T13["(31,1)"]
        T13 --- T23["(31,2)"]
        T23 --- T33["(31,31)<br/>Bottom-Right"]
    end

    style T00 fill:#ff9999
    style T33 fill:#99ff99
```

### Grid Components
```rust
// Position on the grid
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

// Movement state
pub struct GridMovement {
    pub path: Option<Vec<IVec2>>,     // Current path
    pub current_index: usize,          // Index in path
    pub is_moving: bool,               // Movement flag
    pub movement_progress: f32,        // Progress to next tile
    pub speed: f32,                    // Tiles per second
}
```

## 🚶 Movement Mechanics

### Movement Process
```mermaid
sequenceDiagram
    participant Unit
    participant Pathfinder
    participant Movement
    participant Grid
    participant Visual

    Unit->>Pathfinder: Request path to target
    Pathfinder->>Grid: Check walkable tiles
    Grid-->>Pathfinder: Return obstacles
    Pathfinder->>Pathfinder: Calculate A* path
    Pathfinder-->>Unit: Return path nodes

    loop Every Movement Tick
        Unit->>Movement: Update position
        Movement->>Movement: Progress += speed
        alt Progress >= 1.0
            Movement->>Grid: Move to next tile
            Grid->>Unit: Update GridPosition
            Movement->>Movement: Reset progress
        end
        Movement->>Visual: Interpolate position
    end
```

### Movement Speed
| Activity | Ticks per Tile | Real Time | Energy Cost |
|----------|---------------|-----------|-------------|
| **Normal Walk** | 3 ticks | 0.3 seconds | -0.05/tick |
| **Encumbered** | 5 ticks | 0.5 seconds | -0.08/tick |
| **Running** | 2 ticks | 0.2 seconds | -0.15/tick |
| **Exhausted** | 6 ticks | 0.6 seconds | -0.03/tick |

## 🧭 A* Pathfinding Algorithm

### How A* Works
```mermaid
graph TD
    Start[Start Node] --> Open[Add to Open List]
    Open --> Current[Select Lowest F-Score]
    Current --> Goal{Is Goal?}
    Goal -->|Yes| Path[Reconstruct Path]
    Goal -->|No| Neighbors[Check Neighbors]

    Neighbors --> Valid{Walkable?}
    Valid -->|No| Skip[Skip Tile]
    Valid -->|Yes| Calculate[Calculate Scores]

    Calculate --> GScore[G = Distance from Start]
    Calculate --> HScore[H = Heuristic to Goal]
    Calculate --> FScore[F = G + H]

    FScore --> AddOpen[Add to Open List]
    AddOpen --> Current

    style Start fill:#99ff99
    style Path fill:#99ff99
```

### Score Calculations
```rust
// G-Score: Actual distance from start
g_score = parent.g_score + distance_to_neighbor

// H-Score: Heuristic (Manhattan distance)
h_score = abs(current.x - goal.x) + abs(current.y - goal.y)

// F-Score: Total estimated cost
f_score = g_score + h_score
```

### Pathfinding Implementation
```rust
pub fn find_path(
    start: IVec2,
    goal: IVec2,
    grid: &Grid
) -> Option<Vec<IVec2>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();

    open_set.push(Node { pos: start, f_score: 0 });
    g_score.insert(start, 0);

    while let Some(current) = open_set.pop() {
        if current.pos == goal {
            return Some(reconstruct_path(came_from, current.pos));
        }

        for neighbor in get_neighbors(current.pos, grid) {
            let tentative_g = g_score[&current.pos] + 1;

            if tentative_g < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, current.pos);
                g_score.insert(neighbor, tentative_g);
                let f = tentative_g + heuristic(neighbor, goal);
                open_set.push(Node { pos: neighbor, f_score: f });
            }
        }
    }
    None
}
```

## 🎯 Movement Actions

### MoveToResourceAction
Navigates unit to a specific resource for gathering.

```mermaid
stateDiagram-v2
    [*] --> FindResource: Need Resource
    FindResource --> PathPlanning: Resource Found
    PathPlanning --> Moving: Path Calculated
    Moving --> CheckDistance: Each Tick
    CheckDistance --> Moving: Not Adjacent
    CheckDistance --> Arrived: Adjacent
    Arrived --> [*]: Ready to Gather
```

### Movement Execution
```rust
pub fn execute_movement_system(
    mut query: Query<(
        Entity,
        &mut GridPosition,
        &mut GridMovement,
        &mut Transform
    )>
) {
    for (entity, mut grid_pos, mut movement, mut transform) in query.iter_mut() {
        if !movement.is_moving || movement.path.is_none() {
            continue;
        }

        let path = movement.path.as_ref().unwrap();
        if movement.current_index >= path.len() {
            // Reached destination
            movement.is_moving = false;
            movement.path = None;
            continue;
        }

        // Progress toward next tile
        movement.movement_progress += MOVE_PROGRESS_PER_TICK;

        if movement.movement_progress >= MAX_WORK_PROGRESS {
            // Move to next tile
            let next_pos = path[movement.current_index];
            grid_pos.x = next_pos.x;
            grid_pos.y = next_pos.y;

            movement.current_index += 1;
            movement.movement_progress = 0.0;
        }
    }
}
```

## 🚧 Collision Detection

### Obstacle Types
```mermaid
graph TD
    Obstacles[Obstacles] --> Static[Static]
    Obstacles --> Dynamic[Dynamic]

    Static --> Walls[Walls]
    Static --> Mountains[Mountains]
    Static --> Water[Deep Water]

    Dynamic --> Units[Other Units]
    Dynamic --> Buildings[Structures]
    Dynamic --> WorkSites[Active Work]

    style Static fill:#ff9999
    style Dynamic fill:#9999ff
```

### Collision Handling
```rust
pub fn is_walkable(pos: IVec2, grid: &Grid) -> bool {
    // Check bounds
    if pos.x < 0 || pos.x >= GRID_SIZE as i32
    || pos.y < 0 || pos.y >= GRID_SIZE as i32 {
        return false;
    }

    // Check terrain
    if grid.get_terrain(pos) == Terrain::Mountain {
        return false;
    }

    // Check units (dynamic)
    if grid.has_unit_at(pos) {
        return false;  // Can't walk through units
    }

    // Check buildings
    if grid.has_building_at(pos) {
        return false;
    }

    true
}
```

## 🎨 Visual Interpolation

Movement happens discretely on ticks, but visuals interpolate smoothly at 60 FPS:

### Interpolation System
```mermaid
graph LR
    subgraph "Logic (10 TPS)"
        L1[Tile 5,5] -->|Instant| L2[Tile 6,5]
    end

    subgraph "Visual (60 FPS)"
        V1[5.0, 5.0] -->|Smooth| V2[5.16, 5.0]
        V2 --> V3[5.33, 5.0]
        V3 --> V4[5.50, 5.0]
        V4 --> V5[5.66, 5.0]
        V5 --> V6[5.83, 5.0]
        V6 --> V7[6.0, 5.0]
    end

    L1 -.->|Start| V1
    L2 -.->|End| V7
```

### Interpolation Code
```rust
pub fn interpolate_movement_system(
    mut query: Query<(&GridPosition, &mut Transform)>
) {
    for (grid_pos, mut transform) in query.iter_mut() {
        let target = grid_to_world(grid_pos);

        // Smooth interpolation (10% per frame)
        transform.translation = transform.translation.lerp(
            target,
            0.1
        );
    }
}
```

## 🔍 Path Visualization

### Debug Display
```
Unit Path:
Start: (5, 10)
Goal: (15, 20)
Path: [(5,10) → (6,10) → (7,11) → (8,12) → ... → (15,20)]
Length: 18 tiles
Est. Time: 5.4 seconds
Energy Cost: ~0.9
```

### Path States
| State | Description | Visual |
|-------|-------------|--------|
| **Planning** | Calculating path | Yellow dots |
| **Following** | Moving along path | Green line |
| **Blocked** | Path obstructed | Red X |
| **Recalculating** | Finding new path | Orange dots |
| **Arrived** | Reached destination | Green check |

## ⚡ Optimization Strategies

### Hierarchical Pathfinding
For long distances, use two-level pathfinding:
1. **Region Level**: Find path between regions
2. **Local Level**: Navigate within regions

### Path Caching
```rust
pub struct PathCache {
    cache: HashMap<(IVec2, IVec2), Vec<IVec2>>,
    max_age: u32,
}
```

### Dynamic Recalculation
Only recalculate when:
- Path becomes blocked
- Target moves significantly
- Better path becomes available

## 🐛 Common Issues

### Problem: Unit Stuck
**Cause**: Path blocked after calculation
**Solution**: Detect stuck state, recalculate path
```rust
if movement_progress == 0.0 && ticks_since_last_move > 10 {
    recalculate_path();
}
```

### Problem: Units Overlap
**Cause**: Multiple units pathfinding to same tile
**Solution**: Implement local avoidance
```rust
if next_tile_occupied {
    wait_or_find_alternate_path();
}
```

### Problem: Inefficient Paths
**Cause**: Not considering diagonal movement
**Solution**: Add diagonal pathfinding (if allowed)
```rust
neighbors.extend(&[
    pos + IVec2::new(1, 1),   // Diagonal
    pos + IVec2::new(-1, 1),
    pos + IVec2::new(1, -1),
    pos + IVec2::new(-1, -1),
]);
```

## 📊 Performance Metrics

### Pathfinding Performance
| Grid Size | Max Path Length | Calculation Time |
|-----------|----------------|------------------|
| 32x32 | 45 tiles | ~1ms |
| 64x64 | 90 tiles | ~5ms |
| 128x128 | 180 tiles | ~20ms |

### Movement Updates
- **Per Unit**: 0.1ms per tick
- **100 Units**: 10ms per tick
- **Interpolation**: 0.01ms per frame

## Next Steps

- Learn about [Work System](needs-system/work-system.md)
- Understand [Resource Gathering](needs-system/resource-gathering.md)
- Explore [Collision System](collision-avoidance.md)
- Read about [Grid World](architecture/grid-world.md)