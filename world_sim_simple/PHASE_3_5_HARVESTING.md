# Phase 3.5: Resource Harvesting Integration

## Overview
Critical phase to connect the growth system with work system so peasants can actually survive by harvesting and eating resources.

## Problem Statement
After implementing the new growth system (Phase 5.1), we have a mismatch:
- Old system: ResourceNode with `ResourceType::Food`
- New system: GrowingResource with proper types (berries ripen, trees grow, etc.)
- Work system: Doesn't deplete resources when harvesting
- Eating system: Looks for `ResourceType::Berries` but gets `ResourceType::Food`
- **Result**: Peasants starve because they can't harvest berries!

## Tasks

### 3.5.1 Fix Resource Type System ✅
**Priority**: CRITICAL
- [ ] Add `ResourceType::Berries` to resource_types.rs
- [ ] Update berry bushes to produce Berries, not Food
- [ ] Ensure eating system consumes Berries
- [ ] Update inventory to handle new resource types

### 3.5.2 Connect Work to Resource Depletion
**Priority**: CRITICAL  
- [ ] When gathering work completes, reduce resource node amount
- [ ] Handle both ResourceNode (old) and GrowingResource (new)
- [ ] Prevent harvesting from depleted resources
- [ ] Update resource visual/state when depleted

### 3.5.3 Integrate GrowingResource with Work System
**Priority**: HIGH
- [ ] Work system checks `harvestable_amount` not just `current_amount`
- [ ] Different work times for different growth stages
- [ ] Can't harvest saplings (trees) or unripe fruit
- [ ] Harvesting triggers proper depletion behavior

### 3.5.4 GOAP Integration
**Priority**: HIGH
- [ ] Fix harvest_resource action preconditions
- [ ] Add "near_berry_bush" and "near_ripe_fruit" world states
- [ ] Update pathfinding to find harvestable resources
- [ ] Prioritize food gathering when hungry

### 3.5.5 Testing & Validation
**Priority**: MEDIUM
- [ ] Spawn peasants near berry bushes
- [ ] Verify full loop: hungry → find berries → harvest → eat → survive
- [ ] Test tree cutting and regrowth
- [ ] Ensure resources properly deplete and regenerate

## Implementation Order

1. **First**: Fix ResourceType (berries vs food) - peasants need to eat!
2. **Second**: Connect work completion to resource depletion
3. **Third**: Integrate new GrowingResource properly
4. **Fourth**: Fix GOAP so AI actually harvests
5. **Finally**: Test the full survival loop

## Success Criteria

✅ Peasants can harvest berries from bushes
✅ Harvested berries appear in inventory
✅ Eating berries reduces hunger
✅ Resources deplete when harvested
✅ Resources grow/ripen/regrow over time
✅ Peasants survive indefinitely with proper food access

## Code Locations

- `src/resources/resource_types.rs` - Add Berries type
- `src/systems/work.rs` - Fix work completion
- `src/components/growth.rs` - GrowingResource integration
- `src/ai/goap_actions.rs` - Fix harvest preconditions
- `src/systems/needs_update_v2.rs` - Eating system

## Testing Commands

```bash
# Run with debug to see harvesting
RUST_LOG=debug cargo run -p world_sim_simple

# Watch for key messages
grep -E "gathered|harvest|eat|hunger|starv" 

# Monitor resource depletion
grep -E "depleted|empty|regrow|ripen"
```

## Notes

This phase emerged because:
1. Phase 3 created work system but didn't fully implement harvesting
2. Phase 5.1 created new growth system with different resource types
3. The mismatch means peasants can't eat and will starve
4. This is a **critical blocker** for any further development

Without this, we're building an economy for dead peasants!