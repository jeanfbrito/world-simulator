# The Great Berry Bush Regeneration Fiasco: A Tale of Two Components

## Or: How I Learned to Stop Worrying and Love Component APIs

### The Problem (Cost: 7.5+ Hours of Debugging)

We spent over 7.5 hours debugging why berry bushes would never deplete and seemed to regenerate instantly. Like magical cornucopias of infinite berries, defying all laws of resource management. The symptoms were:
- Berry bushes always showed as "25 berry bushes with fruit, 0 depleted"
- Berries appeared to regenerate within 1-2 ticks instead of the configured 150 ticks
- Debug logs showed harvesting was happening but resources never depleted
- Peasants would freeze in place, seemingly stuck but actually in an infinite loop of instant berry regeneration

### The Investigation

We checked everything:
- Resource depletion logic ✓
- Regeneration timers ✓
- Component updates ✓
- System execution order ✓
- Save/load serialization ✓

Everything looked correct. The code was perfect. The logic was sound. Yet the berries were infinite.

### The Root Cause

After 7.5 hours of debugging, tracing through every system, adding debug logs everywhere, we discovered the issue:

```rust
// The component existed...
#[derive(Component)]
pub struct Harvestable {
    pub resource_type: ResourceType,
    pub amount: u32,
    pub max_amount: u32,
    pub regeneration_ticks: u32,
    pub ticks_since_depletion: u32,
}

// The methods existed...
impl Harvestable {
    pub fn harvest(&mut self, amount: u32) -> u32 {
        let harvested = self.amount.min(amount);
        self.amount -= harvested;
        harvested
    }
}

// But in the gathering system...
fn handle_gathering(
    harvestable: &Harvestable,  // <- Immutable reference!
    // ...
) {
    // We were reading the amount...
    let available = harvestable.amount;

    // But never actually calling harvest()!
    // The component was never modified!
}
```

### The Fix (5 Lines of Code)

```rust
fn handle_gathering(
    harvestable: &mut Harvestable,  // <- Make it mutable
    // ...
) {
    // Actually call the harvest method
    let harvested = harvestable.harvest(3);
    // Component is now properly modified
}
```

### The Lessons Learned

1. **Component APIs are only useful if you call them** - Having a perfect `harvest()` method means nothing if you never invoke it

2. **Immutable references in ECS are a code smell** - If you're querying a resource component as immutable, you're probably not modifying it

3. **The simplest bugs take the longest to find** - We looked everywhere except the most obvious place

4. **Debug logs can lie by omission** - We logged "Harvesting 3 berries!" but never logged whether the harvest actually succeeded

### The Silver Lining

The peasants enjoyed 7.5 hours of infinite berries. They were the happiest, most well-fed peasants in any simulation ever. Until we fixed it and they had to face the harsh reality of resource scarcity again.

### The Punchline

We blamed the peasants for being lazy and freezing in place. Turns out they were just drunk on the impossible bounty of infinite berries. The berries were just too good to be true - because they literally were.

---

*This documentation stands as a monument to the 7.5 hours lost to two missing characters: `&mut`*

🫐 Forever in our hearts, infinite berry bushes. 2025-2025.