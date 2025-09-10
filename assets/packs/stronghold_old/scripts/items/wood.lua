-- Stronghold-style Wood: Simpler than Dwarf Fortress version

return {
    id = "wood",
    name = "Wood",
    category = "resource",
    stack_size = 250,  -- Larger stacks for RTS style
    
    -- Simplified properties
    cost = 0,  -- Free from trees
    sell_price = 2,  -- Gold per unit
    
    -- Simple harvesting
    harvest_from = "tree",
    harvest_rate = 5,  -- Units per minute
    requires_woodcutter = true,
    
    -- Usage
    used_for = {
        "wooden_wall",
        "archer_tower", 
        "spear",
        "bow"
    },
    
    description = "Essential building material"
}