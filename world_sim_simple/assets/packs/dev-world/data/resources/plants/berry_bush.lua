-- Berry Bush Resource
-- A harvestable wild plant that produces berries

register_resource {
    id = "berry_bush",
    name = "Berry Bush",
    description = "A wild bush that produces edible berries",
    category = "plant",
    
    properties = {
        weight = 0,      -- Can't be picked up
        stack_size = 0,  -- Can't be stored as-is
        base_value = 0,  -- No direct value
        quality = 1.0,   -- Standard quality
    },
    
    harvestable = {
        tool_required = nil,  -- Can be harvested by hand
        yield = {
            { item = "berries", min = 2, max = 5 }
        },
        respawn_time = 300.0,  -- 5 minutes to regrow
        growth_stages = 3,
        stage_time = 100.0,    -- Time per growth stage
        requires_water = false, -- Wild plant, self-sufficient
    },
    
    spawn = {
        biomes = { "forest", "meadow" },
        frequency = 0.8,
        cluster_size = { min = 1, max = 3 },
        min_distance = 2.0,
    },
    
    visuals = {
        sprite = "berry_bush",
        color_variation = true,
        size_variation = { 0.8, 1.0, 1.2 },
    }
}
