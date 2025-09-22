-- Production Building Definitions
-- Buildings that produce and process resources

-- Lumbermill
register_entity {
    id = "lumbermill",
    name = "Lumbermill",
    type = "building",
    description = "A mill for processing wood into planks",

    properties = {
        health = 200.0,
        max_health = 200.0,
        size = {x = 2, y = 2},
    },

    building = {
        building_type = "lumbermill",
        size = "medium",
        construction_time = 20.0,

        requirements = {
            {item = "wood", count = 30},
            {item = "stone", count = 20},
            {item = "iron_ingot", count = 5},
        },

        storage = {
            capacity = 200,
            allowed_types = {"wood", "plank"},
            efficiency = 1.0,
        },

        production = {
            -- Converts wood to planks
            input = {{item = "wood", count = 1}},
            output = {{item = "plank", count = 4}},
            production_time = 5.0,
            efficiency = 1.0,
            auto_produce = true,
        },

        workers = {
            max_workers = 2,
            required_skills = {woodcutting = 2},
        },

        functions = {"production", "wood_processing"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "lumbermill",
        animation_set = "building",
        color = "#8B4513",
    },

    tags = {"building", "production", "wood", "medium"},
}

-- Quarry
register_entity {
    id = "quarry",
    name = "Quarry",
    type = "building",
    description = "A mining operation for extracting stone",

    properties = {
        health = 200.0,
        max_health = 200.0,
        size = {x = 2, y = 2},
    },

    building = {
        building_type = "quarry",
        size = "medium",
        construction_time = 20.0,

        requirements = {
            {item = "wood", count = 20},
            {item = "stone", count = 40},
            {item = "iron_ingot", count = 10},
        },

        storage = {
            capacity = 300,
            allowed_types = {"stone"},
            efficiency = 1.0,
        },

        production = {
            -- Extracts stone from the ground
            input = {},
            output = {{item = "stone", count = 1}},
            production_time = 8.0,
            efficiency = 0.8,
            auto_produce = true,
            requires_resource_deposit = true,
        },

        workers = {
            max_workers = 3,
            required_skills = {mining = 2},
        },

        functions = {"production", "mining", "stone"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_stone_ground = true, -- Must be built on stone
    },

    visuals = {
        sprite = "quarry",
        animation_set = "building",
        color = "#696969",
    },

    tags = {"building", "production", "mining", "stone", "medium"},
}

-- Mine
register_entity {
    id = "mine",
    name = "Mine",
    type = "building",
    description = "A deep mine for extracting metal ores",

    properties = {
        health = 300.0,
        max_health = 300.0,
        size = {x = 3, y = 3},
    },

    building = {
        building_type = "mine",
        size = "large",
        construction_time = 30.0,

        requirements = {
            {item = "wood", count = 40},
            {item = "stone", count = 60},
            {item = "iron_ingot", count = 20},
        },

        storage = {
            capacity = 500,
            allowed_types = {"ore"},
            efficiency = 1.0,
        },

        production = {
            -- Extracts metal ores
            input = {},
            output = {{item = "iron_ore", count = 1}},
            production_time = 10.0,
            efficiency = 0.5,
            auto_produce = true,
            requires_resource_deposit = true,
        },

        workers = {
            max_workers = 5,
            required_skills = {mining = 3},
        },

        functions = {"production", "mining", "metal"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_ore_ground = true, -- Must be built on ore deposits
    },

    visuals = {
        sprite = "mine",
        animation_set = "building",
        color = "#2F4F4F",
    },

    tags = {"building", "production", "mining", "metal", "large"},
}

-- Farm
register_entity {
    id = "farm",
    name = "Farm",
    type = "building",
    description = "Agricultural land for growing crops",

    properties = {
        health = 200.0,
        max_health = 200.0,
        size = {x = 2, y = 2},
    },

    building = {
        building_type = "farm",
        size = "medium",
        construction_time = 20.0,

        requirements = {
            {item = "wood", count = 25},
            {item = "stone", count = 5},
        },

        storage = {
            capacity = 200,
            allowed_types = {"food", "grain"},
            efficiency = 1.0,
        },

        production = {
            -- Grows crops over time
            input = {},
            output = {{item = "wheat", count = 2}},
            production_time = 15.0,
            efficiency = 0.3,
            auto_produce = true,
            requires_fertile_ground = true,
            seasonal = true,
        },

        workers = {
            max_workers = 4,
            required_skills = {farming = 2},
        },

        functions = {"production", "farming", "food"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_fertile_ground = true, -- Must be built on fertile soil
    },

    visuals = {
        sprite = "farm",
        animation_set = "building",
        color = "#228B22",
    },

    tags = {"building", "production", "farming", "food", "medium"},
}