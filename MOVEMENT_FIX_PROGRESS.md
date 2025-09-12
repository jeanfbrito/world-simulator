# Movement System Fix Progress

## Problem Summary (After 3 Days)
- GOAP detects needs but doesn't create movement actions
- Movement system exists but nothing triggers it
- Peasants stuck in idle → looking around → idle loop

## Solution Strategy: Hybrid AI Architecture
1. **Big-Brain** - Handles execution (movement, eating, working)
2. **GOAP** - Handles strategy (what goals to pursue)
3. **Simple movement first** - Get something working immediately

## Phase 1: Simple Random Movement (COMPLETED)
- [x] Added UnitMind and rand imports to movement.rs
- [x] Add random movement trigger system (simple_random_movement_system)
- [x] Registered system in systems/mod.rs
- [x] Build and test that peasants actually move
- ✅ SUCCESS: Peasants now randomly wander! "Peasant reached destination" messages confirm movement

## Phase 2: Enable Big-Brain
- [ ] Uncomment big-brain plugin
- [ ] Add MoveToTarget action
- [ ] Add movement scorer
- [ ] Connect to GridMovement

## Phase 3: Simplify GOAP
- [ ] GOAP only sets high-level goals
- [ ] Big-brain executes the goals
- [ ] Test integrated system

## Code Changes Made:
1. `systems/movement.rs` - Added imports for UnitMind and rand