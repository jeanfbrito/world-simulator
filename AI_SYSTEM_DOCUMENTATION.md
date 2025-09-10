# AI System Documentation for World Simulator

## Overview

The World Simulator uses a hybrid AI architecture combining Goal-Oriented Action Planning (GOAP) for strategic decision-making and Utility AI for reactive behaviors. This document describes the complete AI system architecture, implementation details, and data-driven configuration.

## Architecture Philosophy

### Why Hybrid AI?

Our units need to handle both long-term goals and immediate reactions:
- **Strategic Layer (GOAP)**: "I need to build a house" → plans sequence of actions
- **Tactical Layer (Utility AI)**: "I'm starving!" → interrupts current plan to eat

This mirrors real behavior where creatures have goals but respond to immediate needs.

## Core Components

### 1. Consolidated State Management

Instead of 15+ separate components, we use 5 consolidated structures:

#### UnitNeeds (components/unit_state.rs)
```rust
pub struct UnitNeeds {
    pub hunger: f32,      // 0.0 = full, 1.0 = starving
    pub energy: f32,      // 0.0 = exhausted, 1.0 = full energy  
    pub morale: f32,      // 0.0 = demoralized, 1.0 = happy
    pub shelter: bool,    // Has a house/shelter
}
```

**Update Rates**:
- Hunger: +0.05/second (20 seconds from full to hungry)
- Energy: -0.03/second (33 seconds from full to tired)
- Morale: Affected by other needs

#### UnitInventory
```rust
pub struct UnitInventory {
    pub items: HashMap<ResourceType, u32>,
    pub max_weight: f32,
    pub current_weight: f32,
}
```

Replaces individual HasWood, HasStone, HasFood components with a flexible inventory system.

#### UnitLocation
```rust
pub struct UnitLocation {
    pub current_tile: (usize, usize),
    pub destination: Option<(usize, usize)>,
    pub at_building: Option<Entity>,
    pub location_type: LocationType,
}
```

Tracks position and context (at storage, at home, at resource, etc.)

#### UnitWorkState
```rust
pub struct UnitWorkState {
    pub is_working: bool,
    pub current_task: Option<String>,
    pub task_progress: f32,
    pub task_target: Option<Entity>,
}
```

Manages current activity and progress.

#### UnitOwnership
```rust
pub struct UnitOwnership {
    pub house: Option<Entity>,
    pub workplace: Option<Entity>,
    pub assigned_storage: Option<Entity>,
}
```

Tracks building relationships.

## GOAP System (Strategic Planning)

### How GOAP Works

1. **Current State**: Read from unit's components
2. **Goal State**: Desired outcome (e.g., "has_house = true")
3. **Actions**: Available operations with preconditions and effects
4. **Planning**: A* search finds action sequence from current to goal

### GOAP Actions Configuration

Actions are defined in `assets/packs/[pack_name]/scripts/ai/goap_actions.lua`:

```lua
-- Gather Wood Action
gather_wood = {
    cost = 2.0,
    description = "Chop wood from trees",
    preconditions = {
        has_energy = { type = "Float", value = 0.3 },     -- Need 30% energy
        at_resource = { type = "Bool", value = true },     -- Must be at tree
        inventory_full = { type = "Bool", value = false }  -- Need space
    },
    effects = {
        has_wood = { type = "Int", value = 5 }             -- Gain 5 wood
    }
}

-- Build House Action
build_house = {
    cost = 10.0,
    description = "Construct a basic house",
    preconditions = {
        has_wood = { type = "Int", value = 10 },        -- Need 10 wood
        has_stone = { type = "Int", value = 5 },        -- Need 5 stone
        at_building_site = { type = "Bool", value = true }  -- Must be at site
    },
    effects = {
        has_house = { type = "Bool", value = true },    -- Now has house
        has_wood = { type = "Int", value = -10 },       -- Consume wood
        has_stone = { type = "Int", value = -5 }        -- Consume stone
    }
}
```

### GOAP Goals Configuration

Goals are defined in `assets/packs/[pack_name]/scripts/ai/goap_goals.lua`:

```lua
survive = {
    priority = 100,
    description = "Basic survival needs",
    conditions = {
        is_hungry = { type = "Float", value = 0.0 },
        has_energy = { type = "Float", value = 0.5 },
        has_shelter = { type = "Bool", value = true }
    }
}

build_settlement = {
    priority = 50,
    description = "Construct essential buildings",
    conditions = {
        has_house = { type = "Bool", value = true },
        storage_built = { type = "Bool", value = true },
        workshop_available = { type = "Bool", value = true }
    }
}

gather_resources = {
    priority = 30,
    description = "Collect resources for settlement",
    conditions = {
        has_wood = { type = "Int", value = 20 },
        has_stone = { type = "Int", value = 10 },
        has_food = { type = "Int", value = 15 }
    }
}
```

### Planning Example

Given:
- Current: hungry, no food, at home
- Goal: not hungry

Planner generates:
1. Move to berry bush
2. Gather berries
3. Eat berries

## Utility AI System (Reactive Behaviors)

### How Utility AI Works

1. **Scorers**: Continuously evaluate need for actions (0.0 to 1.0)
2. **Threshold**: Actions trigger when score exceeds threshold
3. **Actions**: Execute with state machine (Requested → Executing → Complete)
4. **Interruption**: Higher priority actions can interrupt current action

### Utility Scorers Configuration

Scorers are defined in `assets/packs/[pack_name]/ai/utility_scorers.toml`:

```toml
[[scorers]]
name = "hunger_scorer"
component = "UnitNeeds"
field = "hunger"
curve = "linear"           # linear, quadratic, exponential, sigmoid
threshold = 0.6
priority = 90              # High priority for survival

[[scorers]]
name = "fatigue_scorer"
component = "UnitNeeds"
field = "energy"
curve = "inverse_linear"   # Low energy = high score
threshold = 0.3            # Trigger when energy < 30%
priority = 80

[[scorers]]
name = "work_scorer"
component = "UnitInventory"
evaluation = "custom"
script = "return inventory.is_empty() ? 0.8 : 0.2"
threshold = 0.5
priority = 40
```

### Utility Actions Configuration

Actions are defined in `assets/packs/[pack_name]/ai/utility_actions.toml`:

```toml
[[actions]]
name = "eat_food"
scorer = "hunger_scorer"
duration = 2.0              # Takes 2 seconds
animation = "eating"

[actions.requirements]
has_food = true

[actions.effects]
hunger = -0.5               # Reduces hunger by 50%
energy = 0.1                # Slight energy boost

[[actions]]
name = "sleep"
scorer = "fatigue_scorer"
duration = 10.0
animation = "sleeping"
interruptible = false       # Can't be interrupted

[actions.requirements]
at_home = true

[actions.effects]
energy = 1.0                # Full energy restore
```

## Behavior Trees (Complex Sequences)

For complex behaviors, we use behavior trees defined in `assets/packs/[pack_name]/ai/behaviors/`:

### Example: Morning Routine (morning_routine.bt)

```json
{
  "type": "sequence",
  "name": "morning_routine",
  "children": [
    {
      "type": "action",
      "name": "wake_up",
      "duration": 1.0
    },
    {
      "type": "selector",
      "children": [
        {
          "type": "sequence",
          "children": [
            { "type": "condition", "check": "has_food" },
            { "type": "action", "name": "eat_breakfast" }
          ]
        },
        {
          "type": "action",
          "name": "gather_berries",
          "max_attempts": 3
        }
      ]
    },
    {
      "type": "action",
      "name": "go_to_work"
    }
  ]
}
```

## Unit Types Configuration

Different unit types have different AI configurations in `assets/packs/[pack_name]/units/`:

### Peasant (peasant.toml)

```toml
[unit]
name = "Peasant"
category = "civilian"

[stats]
health = 10.0
speed = 5.0
carry_capacity = 50.0

[needs]
hunger_rate = 0.05          # Gets hungry faster
energy_rate = 0.03
morale_base = 0.7

[ai]
planner = "basic_goap"
goals = ["survive", "gather_resources", "build_shelter"]
scorers = ["hunger_scorer", "fatigue_scorer", "work_scorer"]
behaviors = ["morning_routine", "work_cycle", "evening_rest"]

[capabilities]
can_build = true
can_gather = true
can_fight = false
can_craft = ["basic_tools", "simple_structures"]
```

### Soldier (soldier.toml)

```toml
[unit]
name = "Soldier"
category = "military"

[stats]
health = 30.0
speed = 6.0
attack = 10.0
defense = 5.0

[needs]
hunger_rate = 0.07          # More hungry due to activity
energy_rate = 0.05          # Tires faster
morale_base = 0.8

[ai]
planner = "combat_goap"
goals = ["defend_settlement", "patrol", "train"]
scorers = ["threat_scorer", "duty_scorer", "hunger_scorer"]
behaviors = ["patrol_route", "combat_stance", "retreat"]

[capabilities]
can_build = false
can_gather = false
can_fight = true
combat_skills = ["melee", "ranged", "formation"]
```

## AI System Integration

### System Execution Order

1. **Need Update System** (every frame)
   - Updates hunger, energy, morale based on time
   
2. **Scorer System** (every frame)
   - Evaluates all utility scorers
   - Triggers high-priority actions
   
3. **GOAP Planning System** (every second)
   - Re-evaluates goals
   - Plans new action sequences if needed
   
4. **Action Execution System** (every frame)
   - Executes current action
   - Handles interruptions
   - Transitions between actions

### State Synchronization

The AI system maintains consistency between:
- Component states (source of truth)
- GOAP world state (for planning)
- Utility scorer cache (for performance)

Synchronization happens:
- Before GOAP planning
- After action completion
- On significant state changes

## Performance Optimization

### Hierarchical Planning

Units use different planning frequencies based on importance:
- **Heroes/Leaders**: Plan every 0.5 seconds
- **Regular Units**: Plan every 1-2 seconds
- **Background Units**: Plan every 5 seconds

### LOD (Level of Detail) AI

Based on distance from camera/player:
- **Close**: Full AI with all systems
- **Medium**: Simplified utility AI, cached GOAP plans
- **Far**: Basic state machines
- **Very Far**: Statistical simulation only

### Action Batching

Similar actions are batched for performance:
- Movement uses flow fields for groups
- Resource gathering shares pathfinding
- Combat uses formation AI

## Debugging AI

### Debug Commands

Enable AI debugging in console:
```
ai debug <unit_id>        # Show AI state for unit
ai goals <unit_id>        # Show current goals
ai plan <unit_id>         # Show current plan
ai scorers <unit_id>      # Show all scorer values
ai pause <unit_id>        # Pause AI for unit
```

### Debug Visualization

When debug mode is enabled:
- **Green**: Current action
- **Yellow**: Planned actions
- **Red**: Failed actions
- **Blue**: Movement path
- **Icons**: Show needs (hunger, energy, etc.)

### AI Metrics

Monitor AI performance:
- Planning time per unit
- Actions per second
- Plan success rate
- Average plan length
- Interruption frequency

## Extending the AI System

### Adding New Actions

1. Create action definition in `goap_actions.toml` or `utility_actions.toml`
2. Implement action handler in `ai/actions/[action_name].rs`
3. Register action in `ai/mod.rs`
4. Add to relevant unit types

### Adding New Scorers

1. Define scorer in `utility_scorers.toml`
2. Implement evaluation logic if custom
3. Add to unit AI configuration

### Adding New Behaviors

1. Create behavior tree in `behaviors/[name].bt`
2. Reference in unit configuration
3. Implement any custom nodes if needed

## Pack-Specific AI

Different packs can completely customize AI:

### Stronghold Pack
- Medieval peasants with basic needs
- Focus on settlement building
- Simple combat

### Dwarf Fortress Pack
- Complex mood systems
- Detailed crafting chains
- Elaborate social interactions

### Custom Pack
Create your own by:
1. Copy base pack structure
2. Modify AI configuration files
3. Override specific behaviors
4. Add pack-specific actions

## Best Practices

1. **Keep Actions Atomic**: One action = one clear task
2. **Preconditions Must Be Achievable**: Ensure there's always a path
3. **Balance Scorer Priorities**: Survival > Comfort > Productivity
4. **Test Interruptions**: Ensure units can handle action cancellation
5. **Profile Performance**: Monitor planning time in large populations
6. **Document Custom Behaviors**: Comment complex behavior trees
7. **Version AI Configs**: Track changes to AI configuration files

## Troubleshooting

### Unit Stuck in Planning
- Check for impossible preconditions
- Verify action costs aren't too high
- Ensure goals are achievable

### Units Ignore Threats
- Check threat scorer threshold
- Verify interruption priorities
- Ensure combat actions are available

### Poor Performance
- Reduce planning frequency
- Implement AI LOD
- Cache scorer evaluations
- Use action batching

## References

- **Dogoap Examples**: `~/Github/dogoap/crates/bevy_dogoap/examples/`
- **Big-Brain Examples**: `~/Github/big-brain/examples/`
- **Pack Configurations**: `assets/packs/[pack_name]/ai/`
- **Implementation**: `world_sim_simple/src/ai/`