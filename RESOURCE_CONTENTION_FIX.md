# Resource Contention Fix Plan

## Problem Analysis

Multiple peasants are claiming and moving to the same berry bush because:
1. **No exclusive locking**: Multiple `RESOURCE_CLAIM` events for the same position
2. **Claim system not checked**: GOAP planner doesn't consider existing claims
3. **Race conditions**: Multiple peasants evaluate resources simultaneously

### Evidence from Logs:
```
[4.014] Peasant 2 claimed berry bush at (27,27) [workers: 1/1]
[4.223] Peasant 3 claimed berry bush at (27,27) [workers: 1/1]  // DUPLICATE!
```

## Root Causes

### 1. ClaimedResource Component Issue
- Each peasant has their own `ClaimedResource` component
- No centralized resource tracking
- Claims are local to each entity, not globally coordinated

### 2. GOAP Planning Issue
- `find_closest_unclaimed_berry_bush` doesn't check global claims
- Multiple peasants plan paths to same resource simultaneously
- Plans are created before claims are established

### 3. System Ordering Issue
- Movement planning happens before claim validation
- No atomic claim-and-move operation
- Claims can be overwritten by subsequent claimants

## Solution Architecture

### Phase 1: Global Resource Registry
Create a singleton resource that tracks all claimed resources globally:

```rust
#[derive(Resource, Default)]
pub struct GlobalResourceClaims {
    // Map from resource entity to claimant entity
    claims: HashMap<Entity, Entity>,
    // Map from position to claimant (for position-based resources)
    position_claims: HashMap<(usize, usize), Entity>,
}
```

### Phase 2: Atomic Claim Operation
Implement exclusive claiming with try_claim pattern:

```rust
impl GlobalResourceClaims {
    pub fn try_claim_resource(&mut self, resource: Entity, claimant: Entity) -> bool {
        if self.claims.contains_key(&resource) {
            false // Already claimed
        } else {
            self.claims.insert(resource, claimant);
            true // Successfully claimed
        }
    }

    pub fn try_claim_position(&mut self, pos: (usize, usize), claimant: Entity) -> bool {
        if self.position_claims.contains_key(&pos) {
            false // Already claimed
        } else {
            self.position_claims.insert(pos, claimant);
            true // Successfully claimed
        }
    }

    pub fn release_claim(&mut self, resource: Entity) {
        self.claims.remove(&resource);
    }

    pub fn release_position_claim(&mut self, pos: (usize, usize)) {
        self.position_claims.remove(&pos);
    }

    pub fn is_claimed(&self, resource: Entity) -> bool {
        self.claims.contains_key(&resource)
    }

    pub fn is_position_claimed(&self, pos: (usize, usize)) -> bool {
        self.position_claims.contains_key(&pos)
    }
}
```

### Phase 3: Fix find_closest_unclaimed_berry_bush
Modify the function to check global claims:

```rust
pub fn find_closest_unclaimed_berry_bush(
    unit_pos: &GridPosition,
    bush_query: &Query<(Entity, &GridPosition, &BerryBushTag)>,
    global_claims: &GlobalResourceClaims,  // NEW
) -> Option<(Entity, GridPosition, u32)> {
    let mut closest = None;
    let mut min_distance = u32::MAX;

    for (entity, bush_pos, bush) in bush_query.iter() {
        // Check if bush has berries
        if bush.berries == 0 {
            continue;
        }

        // NEW: Check if bush is already claimed
        if global_claims.is_claimed(entity) ||
           global_claims.is_position_claimed((bush_pos.x, bush_pos.y)) {
            continue;
        }

        let distance = calculate_distance(unit_pos, bush_pos);
        if distance < min_distance {
            min_distance = distance;
            closest = Some((entity, bush_pos.clone(), distance));
        }
    }

    closest
}
```

### Phase 4: Modify handle_move_to_resource_action
Add atomic claim-and-move:

```rust
pub fn handle_move_to_resource_action(
    mut commands: Commands,
    mut action_query: Query<(Entity, &mut MoveToResourceAction, /* ... */)>,
    mut global_claims: ResMut<GlobalResourceClaims>,  // NEW
    // ... other params
) {
    for (entity, mut action, grid_pos, mut movement, /* ... */) in action_query.iter_mut() {
        if !action.started {
            // Find closest UNCLAIMED bush
            if let Some((bush_entity, bush_pos, distance)) =
                find_closest_unclaimed_berry_bush(&grid_pos, &berry_bush_query, &global_claims) {

                // Try to claim atomically
                if global_claims.try_claim_resource(bush_entity, entity) &&
                   global_claims.try_claim_position((bush_pos.x, bush_pos.y), entity) {

                    // SUCCESS: We have exclusive claim
                    debug.log(DebugLevel::Info, "EXCLUSIVE_CLAIM",
                        &format!("{} exclusively claimed bush at ({},{})",
                                name.0, bush_pos.x, bush_pos.y));

                    // Set movement target
                    movement.set_target(bush_pos.x, bush_pos.y);
                    action.started = true;
                    action.target_position = Some((bush_pos.x, bush_pos.y));
                    action.target_entity = Some(bush_entity);

                    // Update local claimed resource
                    claimed.resource_entity = Some(bush_entity);
                    claimed.resource_position = Some((bush_pos.x, bush_pos.y));
                } else {
                    // FAILED: Someone else claimed it, find another
                    debug.log(DebugLevel::Debug, "CLAIM_FAILED",
                        &format!("{} failed to claim bush at ({},{}) - already taken",
                                name.0, bush_pos.x, bush_pos.y));
                }
            }
        }

        // ... rest of movement handling
    }
}
```

### Phase 5: Add Claim Cleanup
Ensure claims are released when:
1. Gathering completes
2. Peasant dies
3. Action is cancelled
4. Timeout occurs

```rust
pub fn cleanup_stale_claims(
    mut global_claims: ResMut<GlobalResourceClaims>,
    query: Query<(Entity, &ClaimedResource), Without<GatherFoodAction>>,
) {
    for (entity, claimed) in query.iter() {
        if let Some(resource) = claimed.resource_entity {
            global_claims.release_claim(resource);
        }
        if let Some(pos) = claimed.resource_position {
            global_claims.release_position_claim(pos);
        }
    }
}
```

### Phase 6: Update GOAP Preconditions
Make GOAP aware of resource availability:

```rust
// In setup_dogoap_planners
let move_action = MoveToResourceAction::new()
    .add_precondition(NearBerryBush::is(0.0))
    .add_precondition(AvailableBerryBush::is(1.0))  // NEW: Check unclaimed bushes exist
    .add_effect(NearBerryBush::is(1.0));
```

## Implementation Order

1. **Add GlobalResourceClaims resource** (5 min)
2. **Update find_closest_unclaimed_berry_bush** (10 min)
3. **Modify handle_move_to_resource_action for atomic claims** (15 min)
4. **Add cleanup systems** (10 min)
5. **Test with multiple peasants** (10 min)
6. **Add timeout and error recovery** (10 min)

## Expected Outcome

After implementation:
- Each berry bush claimed by exactly ONE peasant
- No duplicate claims in logs
- Peasants naturally distribute to different bushes
- Failed claimants automatically find alternatives
- System handles contention gracefully

## Verification Metrics

Success indicators:
1. No duplicate `RESOURCE_CLAIM` logs for same position
2. Each bush entity appears in claims map at most once
3. Peasants spread out to different resources
4. Work system shows correct worker counts (1/1 per bush)
5. No peasants stuck trying to claim same resource