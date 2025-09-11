-- Growth patterns for different resource types
-- Defines how resources grow, ripen, and regenerate

growth_patterns = {
    -- Trees grow through stages and regrow from stumps
    oak_tree = {
        type = "tree_growth",
        stages = {
            sapling = { duration_ticks = 3000, wood_yield = 0 },    -- 5 minutes
            young = { duration_ticks = 6000, wood_yield = 5 },      -- 10 minutes
            mature = { duration_ticks = 12000, wood_yield = 15 },   -- 20 minutes
            old = { duration_ticks = 18000, wood_yield = 20 },      -- 30 minutes
        },
        regrow_from_stump_ticks = 9000,  -- 15 minutes to regrow from stump
        seasonal_growth = true,
        winter_growth_rate = 0.2,  -- 20% growth in winter
    },
    
    pine_tree = {
        type = "tree_growth",
        stages = {
            sapling = { duration_ticks = 2400, wood_yield = 0 },    -- 4 minutes
            young = { duration_ticks = 4800, wood_yield = 4 },      -- 8 minutes
            mature = { duration_ticks = 9600, wood_yield = 12 },    -- 16 minutes
            old = { duration_ticks = 14400, wood_yield = 16 },     -- 24 minutes
        },
        regrow_from_stump_ticks = 7200,  -- 12 minutes to regrow
        seasonal_growth = true,
        winter_growth_rate = 0.5,  -- Pines handle winter better
    },
    
    -- Fruit bushes ripen berries over time
    berry_bush = {
        type = "fruit_ripening",
        max_fruit = 10,
        ripening_rate = 2,         -- 2 berries ripen at a time
        ripening_interval = 100,   -- Every 10 seconds
        start_with_fruit = 0,      -- Start empty for testing
        seasonal_production = true,
        peak_season = "summer",    -- Best production in summer
        off_season_rate = 0.3,     -- 30% production in winter
    },
    
    apple_tree = {
        type = "fruit_ripening",
        max_fruit = 20,
        ripening_rate = 3,
        ripening_interval = 200,   -- Every 20 seconds
        start_with_fruit = 5,
        seasonal_production = true,
        peak_season = "autumn",    -- Apples ripen in fall
        off_season_rate = 0.0,     -- No apples in winter/spring
    },
    
    -- Crops grow through defined stages
    wheat = {
        type = "crop_growth",
        stages = {
            planted = { duration_ticks = 300 },      -- 30 seconds
            sprouting = { duration_ticks = 600 },    -- 1 minute
            growing = { duration_ticks = 1200 },     -- 2 minutes
            flowering = { duration_ticks = 600 },    -- 1 minute
            ripe = { duration_ticks = 0 },           -- Stays ripe until harvested
        },
        yield_when_ripe = 5,
        requires_replanting = true,
        grows_in_seasons = { "spring", "summer", "autumn" },
    },
    
    -- Stone deposits with configurable regeneration
    stone_deposit = {
        type = "mineral_respawn",
        initial_amount = 50,
        regenerates = false,        -- By default, stone doesn't regenerate
        -- Server config can override to enable regeneration
        regeneration_chance = 0.0,  -- 0% chance by default
        check_interval = 6000,      -- Check every 10 minutes if enabled
    },
    
    iron_ore = {
        type = "mineral_respawn",
        initial_amount = 30,
        regenerates = false,
        regeneration_chance = 0.0,
        check_interval = 12000,     -- Check every 20 minutes if enabled
    },
    
    -- Magical crystals that slowly regenerate
    mana_crystal = {
        type = "mineral_respawn",
        initial_amount = 10,
        regenerates = true,         -- Magical crystals do regenerate
        regeneration_chance = 0.05, -- 5% chance every check
        check_interval = 3000,      -- Check every 5 minutes
    },
    
    -- Fish populations that reproduce
    fish_school = {
        type = "population_growth",
        max_population = 20,
        reproduction_rate = 0.1,    -- 10% growth per cycle
        reproduction_interval = 1200, -- Every 2 minutes
        min_population_for_reproduction = 2,
        overfishing_threshold = 5,  -- Below this, no reproduction
    },
    
    -- Renewable grass for grazing
    grass_pasture = {
        type = "simple_regeneration",
        max_amount = 100,
        regeneration_rate = 5,
        regeneration_interval = 50,  -- Every 5 seconds
        grazed_by = { "cow", "sheep", "horse" },
    },
}

-- Seasonal modifiers for growth
seasonal_modifiers = {
    spring = {
        tree_growth = 1.2,      -- 20% faster growth
        fruit_ripening = 1.0,
        crop_growth = 1.3,      -- 30% faster for crops
    },
    summer = {
        tree_growth = 1.0,
        fruit_ripening = 1.5,   -- 50% more fruit
        crop_growth = 1.0,
    },
    autumn = {
        tree_growth = 0.8,      -- Slower growth
        fruit_ripening = 1.2,   -- Harvest season
        crop_growth = 0.8,
    },
    winter = {
        tree_growth = 0.2,      -- Very slow growth
        fruit_ripening = 0.0,   -- No fruit in winter
        crop_growth = 0.0,      -- Crops don't grow
    },
}

-- Game configuration overrides
game_config = {
    -- Can be changed by server/player settings
    infinite_resources = false,     -- If true, all resources regenerate
    fast_growth = false,            -- If true, 5x growth speed
    realistic_seasons = true,       -- If true, apply seasonal effects
    resource_scarcity = 1.0,        -- Multiplier for resource amounts
}

return {
    growth_patterns = growth_patterns,
    seasonal_modifiers = seasonal_modifiers,
    game_config = game_config,
}