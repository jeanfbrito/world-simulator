# Big Brain Reactive System

Big Brain provides immediate, reactive responses to urgent situations, overriding GOAP planning when emergencies arise. It acts as the survival instinct layer of the AI.

## 🧠 Big Brain Overview

```mermaid
graph TD
    subgraph "Big Brain Process"
        Sensors[Sensor Input] --> Scorers[Score Evaluation]
        Scorers --> Priority[Priority Ranking]
        Priority --> Threshold{Above Threshold?}
        Threshold -->|Yes| Override[Override GOAP]
        Threshold -->|No| Continue[Continue GOAP]
        Override --> Action[Spawn Emergency Action]
    end

    subgraph "Scorers"
        Scorers --> Energy[Energy Scorer]
        Scorers --> Hunger[Hunger Scorer]
        Scorers --> Danger[Danger Scorer]
        Scorers --> Idle[Idle Scorer]
    end

    style Override fill:#ff9999
    style Action fill:#ffff99
```

## 🎯 Core Components

### Big Brain Structure
```rust
pub struct Thinker {
    pub scorers: Vec<Box<dyn Scorer>>,
    pub current_action: Option<ActionState>,
    pub threshold: f32,  // 0.5 default
}

pub struct ActionState {
    pub action: Box<dyn Action>,
    pub score: f32,
    pub started_tick: u32,
}
```

### Scorer System
```rust
pub trait Scorer {
    fn score(&self, entity: Entity, world: &World) -> f32;
    fn action(&self) -> Box<dyn Action>;
}
```

## 🔋 Energy Scorer

**Purpose**: Prevent energy exhaustion through emergency napping

### Score Calculation
```mermaid
graph LR
    E100[100% Energy] -->|Score: 0.0| None
    E50[50% Energy] -->|Score: 0.0| None
    E20[20% Energy] -->|Score: 0.7| Monitor
    E10[10% Energy] -->|Score: 0.9| Emergency
    E5[5% Energy] -->|Score: 0.95| Critical
    E0[0% Energy] -->|Score: 1.0| Force

    style E100 fill:#00ff00
    style E20 fill:#ffff00
    style E10 fill:#ff9900
    style E5 fill:#ff6600
    style E0 fill:#ff0000,color:#fff
```

### Implementation
```rust
pub fn energy_scorer_system(
    mut query: Query<(&mut Score, &Energy), With<EnergyScorer>>,
) {
    for (mut score, energy) in query.iter_mut() {
        let energy_percent = energy.0 / 100.0;

        let score_value = if energy_percent <= 0.05 {
            0.95 + (0.05 - energy_percent)  // 0.95-1.0
        } else if energy_percent <= 0.1 {
            0.9 + (0.1 - energy_percent) * 0.5  // 0.9-0.95
        } else if energy_percent <= 0.2 {
            0.7 + (0.2 - energy_percent) * 2.0  // 0.7-0.9
        } else {
            0.0  // No emergency
        };

        score.set(score_value as f32);
    }
}
```

### Energy Response Actions
```rust
RestQuickAction {
    triggers_at: Score > 0.7,
    spawns: NapAction,
    duration: 50 ticks,
    recovery: +80 energy,
}
```

## 🍽️ Hunger Scorer

**Purpose**: Force eating when critically hungry

### Score Calculation
```mermaid
graph LR
    S100[100% Satiety] -->|Score: 0.0| None
    S30[30% Satiety] -->|Score: 0.3| Low
    S20[20% Satiety] -->|Score: 0.6| High
    S10[10% Satiety] -->|Score: 0.85| Urgent
    S5[5% Satiety] -->|Score: 0.95| Critical

    style S100 fill:#00ff00
    style S30 fill:#ffff00
    style S10 fill:#ff9900
    style S5 fill:#ff0000,color:#fff
```

### Implementation
```rust
pub fn hunger_scorer_system(
    mut query: Query<(&mut Score, &Satiety, &FoodCount), With<HungerScorer>>,
) {
    for (mut score, satiety, food) in query.iter_mut() {
        let hunger_percent = 1.0 - (satiety.0 / 100.0);

        let base_score = if hunger_percent >= 0.95 {
            0.95  // Starving
        } else if hunger_percent >= 0.9 {
            0.85  // Very hungry
        } else if hunger_percent >= 0.8 {
            0.6   // Hungry
        } else if hunger_percent >= 0.7 {
            0.3   // Getting hungry
        } else {
            0.0   // Not hungry
        };

        // Boost score if we have food available
        let final_score = if food.0 > 0.0 && base_score > 0.3 {
            (base_score + 0.1).min(1.0)
        } else {
            base_score
        };

        score.set(final_score as f32);
    }
}
```

### Hunger Response Actions
```rust
PanicEatAction {
    triggers_at: Score > 0.85 AND FoodCount > 0,
    spawns: EatAction,
    immediate: true,
    restores: +20 satiety per food,
}
```

## ⚠️ Danger Scorer

**Purpose**: React to immediate threats (future implementation)

### Planned Score Calculation
```mermaid
graph TD
    Check[Check Threats] --> Distance{Distance to Threat}

    Distance -->|< 3 tiles| Immediate[Score: 0.95<br/>Immediate Danger]
    Distance -->|3-5 tiles| Close[Score: 0.7<br/>Close Threat]
    Distance -->|5-10 tiles| Aware[Score: 0.3<br/>Be Aware]
    Distance -->|> 10 tiles| Safe[Score: 0.0<br/>Safe]

    Immediate --> Flee[FleeAction]
    Close --> Defensive[DefensiveStance]
    Aware --> Cautious[CautiousMovement]

    style Immediate fill:#ff0000,color:#fff
    style Close fill:#ff9900
    style Aware fill:#ffff00
    style Safe fill:#99ff99
```

## 😴 Idle Scorer

**Purpose**: Provide default behavior when no urgent needs

### Score Calculation
```rust
pub fn idle_scorer_system(
    mut query: Query<(&mut Score, &LastActionTime), With<IdleScorer>>,
) {
    for (mut score, last_action) in query.iter_mut() {
        let idle_time = current_tick - last_action.0;

        let score_value = if idle_time > 100 {
            0.3  // Been idle too long
        } else if idle_time > 50 {
            0.1  // Starting to get bored
        } else {
            0.0  // Recently active
        };

        score.set(score_value);
    }
}
```

### Idle Actions
```rust
WanderAction {
    triggers_at: Score > 0.1,
    behavior: Explore nearby area,
    range: 10 tiles,
    searches_for: Resources,
}
```

## 🔄 Score Aggregation

### Priority System
```mermaid
graph TD
    All[All Scorers] --> Eval[Evaluate Scores]
    Eval --> Sort[Sort by Score]
    Sort --> Check{Highest > 0.5?}

    Check -->|Yes| Override[Override GOAP]
    Check -->|No| GOAP[Continue GOAP Plan]

    Override --> Select[Select Action]
    Select --> Execute[Execute Action]

    subgraph "Score Priority"
        P1[1. Energy: 0.95]
        P2[2. Danger: 0.90]
        P3[3. Hunger: 0.85]
        P4[4. Idle: 0.30]
    end

    style Override fill:#ff9999
```

### Action Selection
```rust
pub fn select_action_system(
    query: Query<(&Score, &ActionType), With<Scorer>>,
    mut commands: Commands,
) {
    let mut best_score = 0.0;
    let mut best_action = None;

    for (score, action_type) in query.iter() {
        if score.0 > best_score && score.0 > THRESHOLD {
            best_score = score.0;
            best_action = Some(action_type);
        }
    }

    if let Some(action) = best_action {
        // Override GOAP and spawn emergency action
        spawn_emergency_action(commands, action);
    }
}
```

## 🎮 GOAP Integration

### Override Mechanism
```mermaid
sequenceDiagram
    participant GOAP
    participant BigBrain
    participant Unit

    loop Every Tick
        GOAP->>Unit: Execute Plan
        BigBrain->>BigBrain: Calculate Scores

        alt Score > 0.5
            BigBrain->>GOAP: OVERRIDE
            BigBrain->>Unit: Emergency Action
            Note over Unit: Execute Emergency
            BigBrain->>GOAP: Release Control
        else Score <= 0.5
            Note over Unit: Continue GOAP Plan
        end
    end
```

### Handoff Protocol
```rust
pub fn handle_ai_handoff(
    entity: Entity,
    big_brain_score: f32,
    goap_plan: Option<&Plan>,
) {
    if big_brain_score > OVERRIDE_THRESHOLD {
        // Big Brain takes control
        pause_goap_plan(entity);
        execute_emergency_action(entity);
    } else if was_overriding(entity) {
        // Return control to GOAP
        resume_goap_plan(entity);
    }
}
```

## 📊 Score Visualization

### Real-Time Score Display
```
=== Big Brain Scores ===
Energy Scorer:  ████░░░░░░ 0.75 [ACTIVE]
Hunger Scorer:  ██░░░░░░░░ 0.20
Danger Scorer:  ░░░░░░░░░░ 0.00
Idle Scorer:    █░░░░░░░░░ 0.10

Current Action: RestQuickAction (NapAction)
Override Active: Yes
GOAP Suspended: Yes
```

## 🔧 Configuration

### Thresholds
```rust
pub const OVERRIDE_THRESHOLD: f32 = 0.5;  // When to override GOAP
pub const CRITICAL_THRESHOLD: f32 = 0.9;  // Immediate action
pub const RELEASE_THRESHOLD: f32 = 0.3;   // Return to GOAP
```

### Score Weights
```rust
pub struct ScorerWeights {
    energy: f32,    // 1.2 - Higher priority
    hunger: f32,    // 1.0 - Normal priority
    danger: f32,    // 1.5 - Highest priority
    idle: f32,      // 0.5 - Lower priority
}
```

## 🐛 Debugging Big Brain

### Common Issues

| Problem | Cause | Solution |
|---------|-------|----------|
| **Not triggering** | Score too low | Lower threshold |
| **Always overriding** | Score too high | Adjust calculation |
| **Wrong component** | Using old AIState | Use DOGOAP components |
| **Stuck in override** | Not releasing | Check release condition |

### Debug Commands
```rust
// Check current scores
Query<(&Score, &Name), With<Scorer>>

// Monitor overrides
Query<&BigBrainOverride>

// Track action spawning
[BIG_BRAIN] Spawning RestQuickAction at score 0.85
```

## 🎯 Design Philosophy

### Reactive Principles
1. **Immediate Response**: No planning delay
2. **Survival First**: Prioritize critical needs
3. **Simple Actions**: One action per emergency
4. **Quick Resolution**: Short duration actions
5. **Smooth Handoff**: Clean GOAP integration

### When Big Brain Activates
```mermaid
graph TD
    Situation[Situation] --> Critical{Is Critical?}

    Critical -->|Energy < 10%| BBYes[Big Brain]
    Critical -->|Satiety < 10%| BBYes
    Critical -->|Danger Close| BBYes
    Critical -->|Otherwise| GOAPNo[GOAP Handles]

    BBYes --> Quick[Quick Action]
    Quick --> Resolve[Resolve Emergency]
    Resolve --> Return[Return to GOAP]

    style BBYes fill:#ff9999
    style GOAPNo fill:#9999ff
```

## 📈 Performance

### Scorer Performance
- **Score Calculation**: ~0.01ms per scorer
- **Action Selection**: ~0.1ms per tick
- **Override Check**: ~0.001ms
- **Memory Usage**: Minimal (scores only)

## Next Steps

- Learn about [GOAP Planning](goap-planning.md)
- Understand [Action System](actions-and-tasks.md)
- Explore [AI Coordination](ai-coordination.md)
- Read about [Emergency Responses](emergency-system.md)