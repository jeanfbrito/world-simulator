-- Basic World Simulation Configuration
-- This configuration provides a simple, easy-to-understand setup
-- for running the basic world simulation example.

pack = {
    name = "basic_simulation",
    description = "Basic simulation pack for example",
    version = "1.0.0",
    author = "World Simulator Examples",
    dependencies = {}
}

-- World configuration
world = {
    size = {
        width = 32,
        height = 32
    },
    terrain = "grassland",
    climate = "temperate",
    resources = {
        -- Basic resources scattered around the world
        {type = "wood", x = 8, y = 8, amount = 100},
        {type = "wood", x = 24, y = 8, amount = 100},
        {type = "wood", x = 8, y = 24, amount = 100},
        {type = "wood", x = 24, y = 24, amount = 100},

        {type = "stone", x = 12, y = 12, amount = 50},
        {type = "stone", x = 20, y = 20, amount = 50},

        {type = "food", x = 16, y = 8, amount = 80},
        {type = "food", x = 8, y = 16, amount = 80},
        {type = "food", x = 24, y = 16, amount = 80},
        {type = "food", x = 16, y = 24, amount = 80}
    },
    units = {
        -- Starting units positioned around the center
        {type = "peasant", x = 15, y = 15},
        {type = "peasant", x = 17, y = 15},
        {type = "peasant", x = 15, y = 17},
        {type = "peasant", x = 17, y = 17},
        {type = "peasant", x = 16, y = 16}
    }
}

-- Unit definitions
units = {
    peasant = {
        display_name = "Peasant",
        description = "Basic worker unit that can gather resources and build structures",
        health = 100,
        max_health = 100,
        hunger = 0,
        max_hunger = 100,
        energy = 100,
        max_energy = 100,
        movement_speed = 1.0,
        ticks_per_tile = 2,
        attack_damage = 5,
        attack_speed = 1.0,
        defense = 2,
        components = {"unit", "movable", "ai_controlled", "worker"},
        tags = {"civilian", "worker", "gatherer"},

        -- AI configuration
        ai = {
            personality = "balanced",
            needs = {
                hunger = 0.6,
                energy = 0.5,
                social = 0.3,
                safety = 0.4,
                purpose = 0.7
            }
        },

        -- Available actions
        actions = {
            "gather_wood",
            "gather_stone",
            "gather_food",
            "build_house",
            "move_to",
            "rest",
            "eat"
        }
    }
}

-- Resource definitions
resources = {
    wood = {
        display_name = "Wood",
        description = "Basic building material gathered from trees",
        resource_type = "wood",
        amount = 100,
        max_amount = 100,
        regeneration_rate = 0.05,
        harvest_tool = "axe",
        harvest_amount = 10
    },

    stone = {
        display_name = "Stone",
        description = "Solid building material for structures",
        resource_type = "stone",
        amount = 50,
        max_amount = 50,
        regeneration_rate = 0.02,
        harvest_tool = "pickaxe",
        harvest_amount = 5
    },

    food = {
        display_name = "Food",
        description = "Basic sustenance for units",
        resource_type = "food",
        amount = 80,
        max_amount = 80,
        regeneration_rate = 0.1,
        harvest_tool = "hands",
        harvest_amount = 15
    }
}

-- Building definitions
buildings = {
    house = {
        display_name = "House",
        description = "Basic shelter for units",
        health = 200,
        max_health = 200,
        size = {width = 2, height = 2},
        construction_time = 100,
        provides = {
            shelter = true,
            comfort = 0.5
        },
        required_resources = {
            wood = 20,
            stone = 10
        }
    }
}

-- Simulation settings
simulation = {
    tick_rate = 60,
    max_entities = 100,
    world_size = {width = 32, height = 32},
    day_length = 1000,
    season_length = 10000
}

-- AI settings
ai = {
    goap_enabled = true,
    utility_ai_enabled = true,
    planning_interval = 10,
    max_plan_depth = 5
}

-- Performance settings
performance = {
    spatial_indexing = true,
    multithreading = false,
    debug_mode = false,
    entity_lod = true
}

-- Logging settings
logging = {
    level = "info",
    file_output = true,
    console_output = true,
    format = "timestamp"
}