# Debug Plan: Fix Gathering Issue Definitively

## Current Situation
- ✅ Peasants move to berry bushes successfully
- ✅ Peasants detect they're near bushes (NearBerryBush = 1.0)
- ❌ GatherFoodAction not executing actual work
- ❌ Work system shows "0/5 units working" constantly
- ❌ Peasants stuck in claiming loop

## Root Cause Analysis

### Hypothesis 1: GatherFoodAction Not Being Spawned
**Check**: Is execute_goap_plans actually spawning GatherFoodAction?
```rust
// Add debug log in execute_goap_plans:
debug.log(DebugLevel::Info, "GOAP_EXEC",
    &format!("Current plan for {:?}: {:?}", entity, planner.current_plan));
```

### Hypothesis 2: GatherFoodAction Handler Not Working
**Check**: Is handle_gather_food_action being called?
```rust
// Add at start of handle_gather_food_action:
debug.log(DebugLevel::Info, "GATHER_CHECK",
    &format!("Checking gather for entity {:?}, near_bush: {}", entity, near_bush.0));
```

### Hypothesis 3: WorkProgress Component Missing
**Check**: Do peasants have WorkProgress component?
```rust
// The log shows WorkProgress is Option<&mut> - might be None
// Need to ensure WorkProgress exists before gathering
```

### Hypothesis 4: GOAP Plan Not Including GatherFoodAction
**Check**: What plan is GOAP creating?
```rust
// Log the full plan when created
debug.log(DebugLevel::Info, "GOAP_PLAN",
    &format!("Created plan: {:?}", planner.current_plan));
```

## Debug Implementation Strategy

### Step 1: Add Comprehensive Logging
Add debug logs at these critical points:

1. **In execute_goap_plans** - Log what actions are being spawned
2. **In handle_gather_food_action** - Log entry, conditions, and work start
3. **In setup_dogoap_planners** - Log initial plan creation
4. **In update_near_berry_bush** - Log state changes

### Step 2: Trace One Peasant's Journey
Pick Peasant 1 and trace:
1. Initial state (hungry, no food)
2. Plan creation (should be: MoveToResource → GatherFood → Eat)
3. Movement execution (✅ working)
4. Action transition (❌ failing here)
5. Gathering execution
6. Work completion
7. Eating

### Step 3: Check Component Registration
Verify all required components exist:
```bash
# Check for these components on peasants:
- Planner ✅
- GatherFoodAction (should appear after movement)
- WorkProgress (might be missing!)
- NearBerryBush ✅
- FoodCount
```

### Step 4: Fix Identification Points

#### Fix Point A: Plan Creation
The GOAP planner might not be creating the right sequence. Check:
```rust
// In setup_dogoap_planners, the goal is:
Goal::from_reqs(&[Satiety::is_more(30.0)])

// Actions defined:
- MoveToResourceAction (precond: !NearBush, effect: NearBush=1)
- GatherFoodAction (precond: NearBush=1, effect: FoodCount+3)
- EatAction (precond: FoodCount>0, effect: Satiety+20)
```

#### Fix Point B: Action Spawning
The execute_goap_plans might not recognize GatherFoodAction:
```rust
// Current matching uses string contains - might fail
match action_name.as_str() {
    name if name.contains("GatherFoodAction") => // Might not match
```

#### Fix Point C: Work System Integration
GatherFoodAction handler might not properly start work:
```rust
// Currently checks for WorkProgress component
// If None, tries to add it but might fail
```

## Definitive Fix Plan

### Phase 1: Enhanced Debugging (5 min)
1. Add detailed logging at all transition points
2. Log component presence/absence
3. Log plan contents explicitly

### Phase 2: Component Verification (5 min)
1. Ensure WorkProgress component is added to all peasants at spawn
2. Verify GatherFoodAction can be spawned
3. Check action name matching in execute_goap_plans

### Phase 3: Fix Implementation (10 min)
Based on debug findings:
1. Fix plan creation if needed
2. Fix action spawning if needed
3. Fix work system integration if needed

### Phase 4: Verification (5 min)
1. Run simulation with debug logs
2. Confirm peasants: Move → Gather → Get Food → Eat
3. Verify work system shows "5/5 units working" during gathering

## Quick Debug Commands

```bash
# Run with maximum debug info
RUST_LOG=trace cargo run -p world_sim_simple 2>&1 | grep -E "GOAP|GATHER|WORK|PLAN"

# Watch specific peasant
RUST_LOG=debug cargo run -p world_sim_simple 2>&1 | grep "Peasant 1"

# Monitor work system
RUST_LOG=info cargo run -p world_sim_simple 2>&1 | grep "units working"
```

## Expected Success Indicators

When fixed, we should see:
1. "GOAP_EXEC: Spawning GatherFoodAction"
2. "GATHER: Starting gathering work at (x,y)"
3. "⚙️ Tick work system: 5/5 units working"
4. "GATHER: Work complete, got 3 berries"
5. "GOAP_EXEC: Spawning EatAction"
6. "Worker ate food, hunger now: X"

## Most Likely Issue

Based on the symptoms, the most likely issue is:
**execute_goap_plans is not properly matching/spawning GatherFoodAction**

The plan probably contains it, but the string matching in execute_goap_plans might be failing to recognize it. The action name format might be different than expected.

## Immediate Action

1. Check exact action type names in plan
2. Fix string matching in execute_goap_plans
3. Ensure WorkProgress component exists on peasants
4. Test and verify gathering works