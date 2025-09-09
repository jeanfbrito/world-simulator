# Asset Pack Comparison

## Overview
The asset pack system allows completely different game experiences using the same simulation engine.

## Pack Differences

### Dwarf Fortress Pack
**Focus**: Deep simulation, complex systems

#### Item Example (Wood)
- **Properties**: 15+ fields (weight, decay, seasons, tool bonuses)
- **Harvesting**: Seasonal modifiers, skill effects, tool requirements
- **Quality**: 5 tiers affecting value
- **Systems**: Decay, stacking, trade value calculations

#### Features
- Z-levels (50+ layers)
- Temperature simulation
- Fluid dynamics
- Material properties (density, melting points)
- Moods and artifacts
- 40+ workshops
- 30+ materials
- Complex reaction chains

### Stronghold Pack  
**Focus**: Castle building, economic management

#### Item Example (Wood)
- **Properties**: 5 fields (cost, sell price, harvest rate)
- **Harvesting**: Simple rate-based
- **Quality**: Not tracked
- **Systems**: Gold-based economy

#### Features
- Flat maps with elevation
- No temperature simulation
- Simple resource chains
- Castle components focus
- Military formations
- ~15 workshops
- ~10 resources
- Direct production

## Key Differences

| Aspect | Dwarf Fortress | Stronghold |
|--------|---------------|------------|
| **Complexity** | Very High | Medium |
| **Resource Types** | 50+ | 10-15 |
| **Workshops** | 40+ with tiers | 10-15 simple |
| **Combat** | Individual limbs | Unit health |
| **Economy** | Barter + coins | Gold only |
| **Depth** | 50 Z-levels | Mostly flat |
| **Learning Curve** | Steep | Moderate |

## File Structure Comparison

### Dwarf Fortress Pack
```
items/
├── raw_materials/     # 15+ files
├── food/             # 20+ files  
├── tools/            # 10+ files
├── processed/        # 25+ files
├── rare/            # 10+ files
└── _common.lua      # Complex utilities
```

### Stronghold Pack
```
items/
├── resources.lua     # All resources (10 items)
├── military.lua      # Weapons/armor (15 items)
├── food.lua         # Simple food (5 items)
└── buildings.lua    # Castle parts
```

## Configuration Differences

### Dwarf Fortress `config.toml`
- 60+ configuration options
- Complex feature flags
- Detailed balance settings
- Multiple difficulty dimensions

### Stronghold `config.toml`
- 30+ configuration options
- Simple on/off features
- Focus on combat/economy balance
- Single difficulty slider

## Performance Impact

### Dwarf Fortress Pack
- **CPU**: High (temperature, fluids, complex AI)
- **Memory**: High (50 Z-levels, thousands of items)
- **Optimal for**: Enthusiasts, powerful PCs

### Stronghold Pack
- **CPU**: Medium (mainly pathfinding, combat)
- **Memory**: Low (single level, fewer items)
- **Optimal for**: Casual players, moderate PCs

## Development Time

### Adding New Content

**Dwarf Fortress Pack**:
- New item: ~100 lines (many properties)
- New workshop: ~200 lines (reactions, tiers)
- Testing: Complex interactions

**Stronghold Pack**:
- New item: ~25 lines (simple properties)
- New building: ~50 lines (cost, function)
- Testing: Straightforward

## Target Audiences

### Dwarf Fortress Pack
- Simulation enthusiasts
- Players who enjoy complexity
- Long play sessions
- Emergent storytelling fans

### Stronghold Pack
- RTS players
- Castle building fans
- Shorter play sessions
- Combat/economy focus

## Switching Between Packs

```bash
# Environment variable
export WORLD_SIM_PACK=stronghold

# Or in-game
/switch-pack stronghold
```

The engine automatically:
1. Loads new configuration
2. Reloads all scripts
3. Adjusts features
4. Updates UI (if applicable)