-- Stronghold-style Stone: Focus on castle building

return {
    id = "stone",
    name = "Stone",
    category = "resource",
    stack_size = 250,
    
    -- Economic properties
    cost = 0,  -- Free from quarry
    sell_price = 3,  -- Gold per unit
    
    -- Simple extraction
    harvest_from = "quarry",
    harvest_rate = 3,  -- Units per minute
    requires_ox_tether = true,  -- Stronghold-specific
    
    -- Castle construction focus
    used_for = {
        "stone_wall",
        "gatehouse",
        "keep",
        "round_tower",
        "square_tower",
        "barracks",
        "cathedral"
    },
    
    -- No complex properties
    description = "Strong castle building material"
}