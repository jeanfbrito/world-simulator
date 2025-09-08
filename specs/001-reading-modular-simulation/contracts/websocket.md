# WebSocket Events Contract

## Connection Protocol

### Client → Server Events

#### `connection`
Initial connection handshake.
```typescript
{
  event: 'connection',
  data: {
    token: string;  // JWT auth token
    sessionId?: string;  // Rejoin existing session
  }
}
```

#### `join_world`
Join a specific world/game session.
```typescript
{
  event: 'join_world',
  data: {
    worldId: string;
    playerId: string;
  }
}
```

#### `command`
Execute a game command.
```typescript
{
  event: 'command',
  data: {
    id: string;  // Client-side command ID for tracking
    type: 'harvest' | 'build' | 'assign_task' | 'move_resources';
    targetEntity?: string;
    targetPosition?: { x: number; y: number };
    params?: any;
  }
}
```

#### `request_state`
Request current game state.
```typescript
{
  event: 'request_state',
  data: {
    area?: {
      x: number;
      y: number;
      width: number;
      height: number;
    };
    entityTypes?: string[];
  }
}
```

#### `ping`
Keepalive ping.
```typescript
{
  event: 'ping',
  data: {
    timestamp: number;
  }
}
```

### Server → Client Events

#### `connected`
Successful connection confirmation.
```typescript
{
  event: 'connected',
  data: {
    sessionId: string;
    playerId: string;
    serverTime: number;
  }
}
```

#### `world_joined`
Successfully joined world.
```typescript
{
  event: 'world_joined',
  data: {
    worldId: string;
    worldState: {
      gameTime: number;
      season: string;
      settings: object;
    };
    playerState: {
      settlements: Settlement[];
      resources: Map<string, number>;
    };
  }
}
```

#### `state_update`
Periodic state updates (delta compression).
```typescript
{
  event: 'state_update',
  data: {
    tick: number;
    entities: {
      created: Entity[];
      updated: { id: string; changes: Partial<Entity> }[];
      deleted: string[];
    };
    resources?: Map<string, number>;
    population?: PopulationStats;
  }
}
```

#### `command_result`
Response to client command.
```typescript
{
  event: 'command_result',
  data: {
    commandId: string;
    success: boolean;
    message?: string;
    data?: any;
  }
}
```

#### `entity_event`
Specific entity events.
```typescript
{
  event: 'entity_event',
  data: {
    entityId: string;
    type: 'resource_harvested' | 'building_completed' | 'worker_assigned';
    details: any;
    timestamp: number;
  }
}
```

#### `notification`
Game notifications.
```typescript
{
  event: 'notification',
  data: {
    id: string;
    type: 'info' | 'warning' | 'error' | 'success';
    title: string;
    message: string;
    timestamp: number;
  }
}
```

#### `player_joined`
Another player joined the session.
```typescript
{
  event: 'player_joined',
  data: {
    playerId: string;
    username: string;
    settlements: string[];
  }
}
```

#### `player_left`
Player left the session.
```typescript
{
  event: 'player_left',
  data: {
    playerId: string;
    username: string;
  }
}
```

#### `game_paused`
Game paused (cooperative mode).
```typescript
{
  event: 'game_paused',
  data: {
    pausedBy: string;
    reason: string;
  }
}
```

#### `game_resumed`
Game resumed.
```typescript
{
  event: 'game_resumed',
  data: {
    resumedBy: string;
  }
}
```

#### `pong`
Response to ping.
```typescript
{
  event: 'pong',
  data: {
    clientTimestamp: number;
    serverTimestamp: number;
  }
}
```

#### `error`
Error message.
```typescript
{
  event: 'error',
  data: {
    code: string;
    message: string;
    details?: any;
  }
}
```

#### `disconnect`
Server-initiated disconnect.
```typescript
{
  event: 'disconnect',
  data: {
    reason: string;
    reconnectable: boolean;
  }
}
```

## Binary Protocol

For efficiency, certain high-frequency updates use binary encoding:

### Binary State Update Format
```
[Header: 1 byte]
  - 0x01: Full update
  - 0x02: Delta update
  - 0x03: Area update

[Tick: 4 bytes] uint32

[Entity Count: 2 bytes] uint16

[Entities: variable]
  For each entity:
    [ID: 16 bytes] UUID
    [Type: 1 byte] enum
    [X: 2 bytes] int16
    [Y: 2 bytes] int16
    [Component Mask: 2 bytes] flags
    [Component Data: variable] based on mask
```

### Component Binary Encoding

#### Resource Node
```
[Resource Type: 2 bytes] uint16
[Current Quantity: 2 bytes] uint16
[Max Quantity: 2 bytes] uint16
[Harvestable: 1 byte] boolean
```

#### Worker
```
[Name Length: 1 byte] uint8
[Name: variable] UTF-8
[Happiness: 1 byte] uint8 (0-100)
[Efficiency: 1 byte] uint8 (0-100)
[Task ID: 16 bytes] UUID (0 if idle)
```

#### Building
```
[Building Type: 2 bytes] uint16
[Progress: 1 byte] uint8 (0-100)
[Operational: 1 byte] boolean
[Worker Count: 1 byte] uint8
```

## Connection Management

### Reconnection Protocol
1. Client disconnects (network issue)
2. Client reconnects with previous `sessionId`
3. Server validates session (<5 minutes old)
4. Server sends missed updates since disconnect
5. Client resynchronizes state

### Rate Limiting
- Commands: Max 10 per second per player
- State requests: Max 2 per second
- Ping: Max 1 per second
- Automatic throttling on violation

### Room Management
- Each world is a Socket.io room
- Players auto-join world room on `join_world`
- Broadcasts limited to room members
- Private messages for player-specific data

## Error Codes

| Code | Description |
|------|-------------|
| AUTH_001 | Invalid authentication token |
| AUTH_002 | Token expired |
| WORLD_001 | World not found |
| WORLD_002 | World full |
| CMD_001 | Invalid command |
| CMD_002 | Insufficient permissions |
| CMD_003 | Invalid target |
| RATE_001 | Rate limit exceeded |
| CONN_001 | Session expired |
| CONN_002 | Already connected |