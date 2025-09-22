-- Berry Bush Resource Definition
-- A harvestable plant that yields berries for food

register_resource {
    id = "berry_bush",
    name = "Berry Bush",
    description = "A wild bush that produces edible berries",
    category = "plant",

    -- Physical properties
    properties = {
        weight = 0.0,  -- Bushes aren't carried
        stack_size = 0,  -- Not an inventory item
        base_value = 0,  -- Not tradeable directly
    },

    -- Harvesting configuration
    harvestable = {
        tool_required = nil,  -- Can harvest by hand
        yield = {
            {
                item = "berries",
                min = 2,
                max = 5,
            }
        },
        respawn_time = 120.0,  -- 2 minutes to regrow
        growth_stages = 3,      -- Seedling -> Young -> Mature
        stage_time = 40.0,      -- 40 seconds per stage
        requires_water = false,  -- Simplified for now
    },

    -- World spawning rules
    spawn = {
        biomes = {"forest", "plains", "hills"},
        frequency = 0.3,  -- 30% chance in valid tiles
        cluster_size = {
            min = 2,
            max = 5,
        },
        min_distance = 2.0,  -- Minimum 2 tiles between bushes
    },

    -- Visual/Audio properties (for future use)
    visuals = {
        sprite = "berry_bush",
        color_variation = true,
        size_variation = {0.8, 1.2},
    },
}