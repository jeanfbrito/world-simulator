# Asset Pack Design Notes

## File Organization Strategy

### Dwarf Fortress Pack - Modular Approach
- **One file per item/material**
- **Reason**: Complex material properties, temperature mechanics, reactions
- **Example**: `wood.lua` has density, melting point, tool bonuses, seasonal modifiers
- **Benefits**: Easier to add complex materials, better for modding

### Stronghold Pack - Grouped Approach  
- **Items grouped by type in single files**
- **Reason**: Simpler items focused on economic relationships
- **Example**: All resources in `resources.lua` with buy/sell prices
- **Benefits**: Easier to see economic balance, matches game's focus

### When to Use Each Approach

**Use Modular (one file per item) when:**
- Items have complex properties (temperature, density, reactions)
- Items interact with multiple systems
- Modding support is important
- Items frequently change independently

**Use Grouped (items in category files) when:**
- Items are simple economic units
- Focus is on relationships between items
- Game balance requires seeing all items together
- Items rarely change independently

## Current Implementation
- **Dwarf Fortress**: Modular files in `/items/raw_materials/`, `/items/processed/`, etc.
- **Stronghold**: Grouped files - `resources.lua`, `buildings.lua`, `military.lua`
- **Both approaches are valid** - choose based on game complexity