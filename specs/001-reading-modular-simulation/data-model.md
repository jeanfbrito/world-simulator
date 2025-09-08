# Data Model: Modular Simulation Engine

## Core Entities

### 1. World
Represents the game world container with spatial dimensions and metadata.

```typescript
interface World {
  id: string;
  name: string;
  width: number;
  height: number;
  seed: string;
  createdAt: Date;
  gameTime: number; // ticks since start
  season: 'spring' | 'summer' | 'autumn' | 'winter';
  settings: {
    resourceRegeneration: boolean;
    seasonalCycles: boolean;
    cooperativeMode: boolean;
  };
}
```

### 2. Entity (ECS Base)
Base entity with component composition for ECS architecture.

```typescript
interface Entity {
  id: string;
  worldId: string;
  type: EntityType;
  components: Map<ComponentType, Component>;
  createdAt: number; // game tick
  lastModified: number;
}

enum EntityType {
  RESOURCE_NODE = 'resource_node',
  WORKER = 'worker',
  BUILDING = 'building',
  RESOURCE_ITEM = 'resource_item',
}
```

### 3. Components

#### Position Component
```typescript
interface PositionComponent {
  x: number;
  y: number;
  z?: number; // optional for 3D
}
```

#### Resource Node Component
```typescript
interface ResourceNodeComponent {
  resourceType: string; // 'tree', 'stone', 'iron_ore', etc.
  currentQuantity: number;
  maxQuantity: number;
  regenerationRate: number; // units per tick
  harvestable: boolean;
  toolRequired?: string[];
  dropTable: DropTableEntry[];
}

interface DropTableEntry {
  resourceId: string;
  minQuantity: number;
  maxQuantity: number;
  probability: number; // 0-100
}
```

#### Worker Component
```typescript
interface WorkerComponent {
  name: string;
  happiness: number; // 0-100
  efficiency: number; // 0-1 multiplier
  currentTask?: Task;
  skills: Map<string, number>;
  needsFood: boolean;
  lastFedTick: number;
}
```

#### Inventory Component
```typescript
interface InventoryComponent {
  capacity: number;
  items: Map<string, number>; // resourceId -> quantity
  reserved: Map<string, number>; // reserved for recipes
}
```

#### Building Component
```typescript
interface BuildingComponent {
  buildingType: string;
  constructionProgress: number; // 0-100
  operational: boolean;
  capacity?: number;
  workers?: string[]; // worker entity IDs
  productionQueue?: ProductionOrder[];
}
```

### 4. Resource
Defines resource types and their properties.

```typescript
interface Resource {
  id: string;
  name: string;
  category: 'food' | 'material' | 'tool' | 'luxury';
  stackable: boolean;
  maxStack: number;
  weight?: number;
  spoilable?: boolean;
  spoilTime?: number; // ticks
  icon: string;
}
```

### 5. Recipe
Defines transformation rules for resources.

```typescript
interface Recipe {
  id: string;
  name: string;
  category: 'construction' | 'crafting' | 'cooking' | 'refining';
  inputs: RecipeInput[];
  outputs: RecipeOutput[];
  duration: number; // ticks
  requirements?: {
    buildings?: string[];
    tools?: string[];
    skills?: SkillRequirement[];
    research?: string[];
  };
}

interface RecipeInput {
  resourceId: string;
  quantity: number;
}

interface RecipeOutput {
  resourceId: string;
  quantity: number;
  probability?: number; // default 100
}

interface SkillRequirement {
  skillId: string;
  minLevel: number;
}
```

### 6. Task
Represents work assignments for workers.

```typescript
interface Task {
  id: string;
  type: TaskType;
  priority: number;
  assignedWorker?: string;
  targetEntity?: string;
  targetPosition?: Position;
  status: TaskStatus;
  createdAt: number;
  startedAt?: number;
  completedAt?: number;
  data?: any; // task-specific data
}

enum TaskType {
  HARVEST = 'harvest',
  CONSTRUCT = 'construct',
  CRAFT = 'craft',
  TRANSPORT = 'transport',
  IDLE = 'idle',
}

enum TaskStatus {
  PENDING = 'pending',
  IN_PROGRESS = 'in_progress',
  COMPLETED = 'completed',
  FAILED = 'failed',
  CANCELLED = 'cancelled',
}
```

### 7. Settlement
Represents a player's domain with aggregated resources and population.

```typescript
interface Settlement {
  id: string;
  worldId: string;
  playerId: string;
  name: string;
  position: Position;
  population: {
    total: number;
    workers: number;
    children: number;
    elderly: number;
  };
  happiness: number; // 0-100 average
  resources: Map<string, number>;
  buildings: string[]; // building entity IDs
  statistics: {
    foodProduced: number;
    foodConsumed: number;
    resourcesGathered: Map<string, number>;
    buildingsConstructed: number;
  };
}
```

### 8. Player
Represents a human player or AI controller.

```typescript
interface Player {
  id: string;
  username: string;
  email?: string;
  settlements: string[];
  permissions: Permission[];
  lastActive: Date;
  preferences: {
    autoSave: boolean;
    notificationsEnabled: boolean;
    uiScale: number;
  };
}

enum Permission {
  BUILD = 'build',
  HARVEST = 'harvest',
  ASSIGN_WORKERS = 'assign_workers',
  TRADE = 'trade',
  ADMIN = 'admin',
}
```

### 9. GameSession
Manages multiplayer game sessions.

```typescript
interface GameSession {
  id: string;
  worldId: string;
  hostPlayerId: string;
  players: string[];
  maxPlayers: number;
  status: 'lobby' | 'active' | 'paused' | 'ended';
  settings: {
    cooperativeMode: boolean;
    sharedResources: boolean;
    pauseOnDisconnect: boolean;
  };
  createdAt: Date;
  startedAt?: Date;
  endedAt?: Date;
}
```

### 10. Event
Game events for logging and triggering systems.

```typescript
interface GameEvent {
  id: string;
  worldId: string;
  type: EventType;
  timestamp: number; // game tick
  entityId?: string;
  playerId?: string;
  data: any;
}

enum EventType {
  RESOURCE_HARVESTED = 'resource_harvested',
  BUILDING_COMPLETED = 'building_completed',
  POPULATION_CHANGED = 'population_changed',
  TASK_COMPLETED = 'task_completed',
  SEASON_CHANGED = 'season_changed',
  TRADE_COMPLETED = 'trade_completed',
}
```

## Relationships

### Entity Relationships
- **World** ↔ **Entity**: One-to-many (world contains entities)
- **Settlement** ↔ **Building**: One-to-many (settlement owns buildings)
- **Settlement** ↔ **Worker**: One-to-many (settlement has population)
- **Worker** ↔ **Task**: One-to-one (worker assigned to task)
- **Building** ↔ **Worker**: Many-to-many (buildings employ workers)
- **Recipe** ↔ **Resource**: Many-to-many (recipes use/produce resources)

### Component Relationships
- Every **Entity** has a **PositionComponent**
- **Workers** have **InventoryComponent** for carrying
- **Buildings** may have **InventoryComponent** for storage
- **ResourceNodes** have **ResourceNodeComponent**

## State Transitions

### Worker States
```
IDLE → ASSIGNED → MOVING → WORKING → DELIVERING → IDLE
         ↓           ↓         ↓          ↓
      FAILED     BLOCKED   EXHAUSTED   COMPLETE
```

### Building Construction States
```
PLANNED → FOUNDATION → CONSTRUCTION → COMPLETED
    ↓          ↓             ↓
CANCELLED  INSUFFICIENT  DESTROYED
```

### Resource Node States
```
FULL → PARTIAL → DEPLETED → REGENERATING → FULL
                     ↓
                  DESTROYED
```

## Validation Rules

### Entity Validation
- All entities must have unique IDs
- Position must be within world bounds
- Entity type must match component composition

### Resource Validation
- Quantities must be non-negative
- Stack sizes cannot exceed maxStack
- Spoilable resources track expiration

### Recipe Validation
- Input resources must exist
- Output quantities must be positive
- Requirements must be satisfiable

### Task Validation
- Target entity must exist and be valid
- Worker skills must meet requirements
- Priority must be 1-10

## Indexes & Performance

### Database Indexes
```sql
-- Primary indexes
CREATE INDEX idx_entity_world ON entities(world_id);
CREATE INDEX idx_entity_type ON entities(type);
CREATE INDEX idx_entity_position ON entities(x, y);

-- Component indexes  
CREATE INDEX idx_component_entity ON components(entity_id);
CREATE INDEX idx_component_type ON components(type);

-- Task indexes
CREATE INDEX idx_task_worker ON tasks(assigned_worker);
CREATE INDEX idx_task_status ON tasks(status);

-- Event indexes
CREATE INDEX idx_event_world_time ON events(world_id, timestamp);
```

### Query Optimization
- Spatial queries use quadtree indexing
- Component queries use type filtering
- Batch entity updates per tick
- Cache frequently accessed recipes

## Data Migration Strategy

### Version Management
- Each entity includes schema version
- Migrations run on server startup
- Backward compatibility for save files

### Migration Path
```typescript
interface Migration {
  version: string;
  up: (data: any) => any;
  down: (data: any) => any;
  description: string;
}
```

## Security Considerations

### Data Validation
- Input sanitization for all user data
- Type checking at API boundaries
- Range validation for numeric values
- SQL injection prevention via parameterized queries

### Access Control
- Player permissions checked per action
- Settlement ownership validation
- Resource ownership tracking
- Admin-only debug commands