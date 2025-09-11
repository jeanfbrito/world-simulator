-- Resource node definitions for Stronghold pack
-- Defines harvestable resources with tick-based regeneration

resource_nodes = {
    -- Berry bushes - fast regeneration, seasonal
    berry_bush = {
        id = "berry_bush",
        name = "Berry Bush",
        description = "Wild bush with edible berries",
        
        -- Resource properties
        resource_type = "berries",
        max_amount = 12,
        yield_amount = 3,  -- Berries per harvest
        
        -- Regeneration (tick-based, 10 TPS)
        regeneration_rate = 2,  -- Berries regenerated
        regeneration_interval = 100,  -- Every 10 seconds
        respawn_time_ticks = 200,  -- 20 seconds for full respawn
        
        -- Harvesting
        harvest_tool = nil,  -- Can harvest by hand
        tool_bonus = 1.5,  -- With basket
        
        -- Seasonal modifiers
        seasons = {
            spring = 0.8,  -- 80% yield
            summer = 1.2,  -- 120% yield - peak season
            autumn = 1.0,  -- 100% yield
            winter = 0.0,  -- No berries in winter
        },
        
        -- Visual
        sprite = "berry_bush",
        color = { r = 0.2, g = 0.5, b = 0.2 },
        ripe_color = { r = 0.5, g = 0.2, b = 0.5 },
    },
    
    -- Apple tree - slower regeneration, autumn harvest
    apple_tree = {
        id = "apple_tree",
        name = "Apple Tree",
        description = "Fruit tree bearing apples",
        
        resource_type = "apples",
        max_amount = 20,
        yield_amount = 5,
        
        -- Slower regeneration for fruit trees
        regeneration_rate = 3,
        regeneration_interval = 300,  -- Every 30 seconds
        respawn_time_ticks = 1200,  -- 2 minutes for full respawn
        
        harvest_tool = "ladder",  -- Better yield with ladder
        tool_bonus = 2.0,
        
        seasons = {
            spring = 0.0,  -- Blossoms only
            summer = 0.5,  -- Green apples
            autumn = 1.5,  -- Peak harvest
            winter = 0.0,  -- Dormant
        },
        
        sprite = "apple_tree",
        color = { r = 0.3, g = 0.4, b = 0.2 },
    },
    
    -- Oak tree - wood resource, very slow regeneration
    oak_tree = {
        id = "oak_tree",
        name = "Oak Tree",
        description = "Mature hardwood tree",
        
        resource_type = "wood",
        max_amount = 50,
        yield_amount = 10,
        
        -- Very slow regeneration for wood
        regeneration_rate = 5,
        regeneration_interval = 600,  -- Every minute
        respawn_time_ticks = 3000,  -- 5 minutes for full respawn
        
        harvest_tool = "axe",  -- Required
        tool_bonus = 3.0,
        
        -- Wood not affected by seasons
        seasons = {
            spring = 1.0,
            summer = 1.0,
            autumn = 1.0,
            winter = 0.9,  -- Slightly harder to cut
        },
        
        sprite = "oak_tree",
        color = { r = 0.4, g = 0.3, b = 0.2 },
    },
    
    -- Pine tree - faster growing softwood
    pine_tree = {
        id = "pine_tree",
        name = "Pine Tree",
        description = "Evergreen softwood tree",
        
        resource_type = "wood",
        max_amount = 30,
        yield_amount = 8,
        
        -- Faster than oak
        regeneration_rate = 4,
        regeneration_interval = 400,  -- Every 40 seconds
        respawn_time_ticks = 2000,  -- 3.3 minutes
        
        harvest_tool = "axe",
        tool_bonus = 2.5,
        
        seasons = {
            spring = 1.1,  -- Sap season
            summer = 1.0,
            autumn = 1.0,
            winter = 1.0,  -- Evergreen
        },
        
        sprite = "pine_tree",
        color = { r = 0.2, g = 0.3, b = 0.2 },
    },
    
    -- Stone deposit - very slow regeneration
    stone_deposit = {
        id = "stone_deposit",
        name = "Stone Deposit",
        description = "Surface stone outcropping",
        
        resource_type = "stone",
        max_amount = 100,
        yield_amount = 10,
        
        -- Stone regenerates very slowly
        regeneration_rate = 2,
        regeneration_interval = 500,  -- Every 50 seconds
        respawn_time_ticks = 6000,  -- 10 minutes
        
        harvest_tool = "pickaxe",  -- Required
        tool_bonus = 4.0,
        
        -- Not affected by seasons
        seasons = {
            spring = 1.0,
            summer = 1.0,
            autumn = 1.0,
            winter = 0.8,  -- Frozen ground
        },
        
        sprite = "stone_deposit",
        color = { r = 0.5, g = 0.5, b = 0.5 },
    },
    
    -- Iron ore vein - no regeneration until depleted
    iron_vein = {
        id = "iron_vein",
        name = "Iron Ore Vein",
        description = "Exposed iron ore deposit",
        
        resource_type = "iron_ore",
        max_amount = 50,
        yield_amount = 5,
        
        -- Only respawns when fully depleted
        regeneration_rate = 0,  -- No gradual regen
        regeneration_interval = 0,
        respawn_time_ticks = 12000,  -- 20 minutes
        
        harvest_tool = "pickaxe",
        tool_bonus = 5.0,
        
        seasons = {
            spring = 1.0,
            summer = 1.0,
            autumn = 1.0,
            winter = 0.7,
        },
        
        sprite = "iron_vein",
        color = { r = 0.4, g = 0.3, b = 0.3 },
    },
    
    -- Mushroom patch - spawns in damp areas
    mushroom_patch = {
        id = "mushroom_patch",
        name = "Mushroom Patch",
        description = "Edible mushrooms in damp soil",
        
        resource_type = "mushrooms",
        max_amount = 8,
        yield_amount = 2,
        
        -- Fast regeneration in right conditions
        regeneration_rate = 3,
        regeneration_interval = 150,  -- Every 15 seconds
        respawn_time_ticks = 300,  -- 30 seconds
        
        harvest_tool = nil,  -- By hand
        tool_bonus = 1.2,
        
        seasons = {
            spring = 1.2,  -- Wet season
            summer = 0.5,  -- Too dry
            autumn = 1.5,  -- Peak growth
            winter = 0.3,  -- Too cold
        },
        
        sprite = "mushroom_patch",
        color = { r = 0.6, g = 0.5, b = 0.4 },
    },
    
    -- Herb patch - medicinal plants
    herb_patch = {
        id = "herb_patch",
        name = "Wild Herbs",
        description = "Medicinal and culinary herbs",
        
        resource_type = "herbs",
        max_amount = 6,
        yield_amount = 2,
        
        -- Moderate regeneration
        regeneration_rate = 1,
        regeneration_interval = 200,  -- Every 20 seconds
        respawn_time_ticks = 400,  -- 40 seconds
        
        harvest_tool = "sickle",  -- Better yield
        tool_bonus = 2.0,
        
        seasons = {
            spring = 1.3,  -- Growing season
            summer = 1.0,
            autumn = 0.8,
            winter = 0.0,  -- Dormant
        },
        
        sprite = "herb_patch",
        color = { r = 0.3, g = 0.5, b = 0.3 },
    },
    
    -- Clay deposit - for pottery
    clay_deposit = {
        id = "clay_deposit",
        name = "Clay Deposit",
        description = "Riverbank clay suitable for pottery",
        
        resource_type = "clay",
        max_amount = 40,
        yield_amount = 8,
        
        -- Slow regeneration (geological)
        regeneration_rate = 1,
        regeneration_interval = 800,  -- Every 80 seconds
        respawn_time_ticks = 4000,  -- 6.7 minutes
        
        harvest_tool = "shovel",
        tool_bonus = 3.0,
        
        seasons = {
            spring = 1.1,  -- Softer from rain
            summer = 0.9,  -- Dried out
            autumn = 1.0,
            winter = 0.5,  -- Frozen
        },
        
        sprite = "clay_deposit",
        color = { r = 0.6, g = 0.4, b = 0.3 },
    },
    
    -- Fish spawning point - renewable
    fish_spawn = {
        id = "fish_spawn",
        name = "Fishing Spot",
        description = "Good spot for catching fish",
        
        resource_type = "fish",
        max_amount = 15,
        yield_amount = 1,  -- One fish at a time
        
        -- Fish repopulate regularly
        regeneration_rate = 2,
        regeneration_interval = 120,  -- Every 12 seconds
        respawn_time_ticks = 600,  -- 1 minute
        
        harvest_tool = "fishing_rod",  -- Required
        tool_bonus = 3.0,
        
        seasons = {
            spring = 1.2,  -- Spawning season
            summer = 1.0,
            autumn = 0.9,
            winter = 0.6,  -- Less active
        },
        
        sprite = "fish_spawn",
        color = { r = 0.2, g = 0.3, b = 0.5 },
    }
}

-- Function to get resource node by ID
function get_resource_node(id)
    return resource_nodes[id]
end

-- Function to calculate regeneration with seasonal modifier
function calculate_regeneration(node, season)
    local seasonal_mod = node.seasons[season] or 1.0
    return math.floor(node.regeneration_rate * seasonal_mod)
end

-- Function to check if tool provides bonus
function get_tool_bonus(node, tool)
    if node.harvest_tool == nil then
        return 1.0  -- No tool required
    elseif tool == node.harvest_tool then
        return node.tool_bonus
    else
        return 0.5  -- Wrong tool penalty
    end
end

-- Return the module
return {
    nodes = resource_nodes,
    get_node = get_resource_node,
    calculate_regen = calculate_regeneration,
    get_bonus = get_tool_bonus
}