-- Storage Building Definitions
-- Buildings for storing resources and items

-- Storage Building
register_entity {
    id = "storage",
    name = "Storage",
    type = "building",
    description = "A small storage building for basic resource storage",

    properties = {
        health = 100.0,
        max_health = 100.0,
        size = {x = 1, y = 1},
    },

    building = {
        building_type = "storage",
        size = "small",
        construction_time = 10.0,

        -- Building requirements (moved from recipe system)
        requirements = {
            {item = "wood", count = 20},
            {item = "stone", count = 10},
        },

        -- Storage capacity
        storage = {
            capacity = 500,
            allowed_types = {"all"},
            efficiency = 1.0,
        },

        -- Production/Processing (none for storage)
        production = nil,

        -- Worker requirements
        workers = {
            max_workers = 0,
            required_skills = {},
        },

        -- Building functionality
        functions = {"storage"},
    },

    spawn = {
        initial_count = 1, -- Spawn one stockpile at startup
        spawn_area = {
            min_x = 31,
            max_x = 33,
            min_y = 31,
            max_y = 33,
        },
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "storage",
        animation_set = "building",
        color = "#8B4513",
    },

    tags = {"building", "storage", "small"},
}

-- Warehouse
register_entity {
    id = "warehouse",
    name = "Warehouse",
    type = "building",
    description = "A large warehouse for bulk resource storage",

    properties = {
        health = 300.0,
        max_health = 300.0,
        size = {x = 3, y = 3},
    },

    building = {
        building_type = "warehouse",
        size = "large",
        construction_time = 30.0,

        requirements = {
            {item = "wood", count = 50},
            {item = "stone", count = 30},
            {item = "iron_ingot", count = 10},
        },

        storage = {
            capacity = 2000,
            allowed_types = {"all"},
            efficiency = 1.2,
        },

        production = nil,

        workers = {
            max_workers = 2,
            required_skills = {logistics = 1},
        },

        functions = {"storage", "bulk_storage"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "warehouse",
        animation_set = "building",
        color = "#654321",
    },

    tags = {"building", "storage", "large", "warehouse"},
}

-- Stockpile
register_entity {
    id = "stockpile",
    name = "Stockpile",
    type = "building",
    description = "An outdoor stockpile for raw materials",

    properties = {
        health = 50.0,
        max_health = 50.0,
        size = {x = 2, y = 2},
    },

    building = {
        building_type = "stockpile",
        size = "medium",
        construction_time = 20.0,

        requirements = {
            {item = "wood", count = 10},
            {item = "stone", count = 5},
        },

        storage = {
            capacity = 300,
            allowed_types = {"wood", "stone", "ore"},
            efficiency = 0.8,
            exposed = true, -- Exposed to weather
        },

        production = nil,

        workers = {
            max_workers = 0,
            required_skills = {},
        },

        functions = {"storage", "outdoor_storage"},
    },

    spawn = {
        initial_count = 1, -- Spawn one stockpile at startup
        spawn_area = {
            min_x = 31,
            max_x = 33,
            min_y = 31,
            max_y = 33,
        },
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "stockpile",
        animation_set = "building",
        color = "#A0522D",
    },

    tags = {"building", "storage", "outdoor", "medium"},
}

-- Granary
register_entity {
    id = "granary",
    name = "Granary",
    type = "building",
    description = "A specialized storage building for food preservation",

    properties = {
        health = 200.0,
        max_health = 200.0,
        size = {x = 2, y = 2},
    },

    building = {
        building_type = "granary",
        size = "medium",
        construction_time = 20.0,

        requirements = {
            {item = "wood", count = 15},
            {item = "stone", count = 10},
        },

        storage = {
            capacity = 400,
            allowed_types = {"food", "grain"},
            efficiency = 1.5,
            preservation = true, -- Preserves food from spoiling
        },

        production = nil,

        workers = {
            max_workers = 1,
            required_skills = {farming = 1},
        },

        functions = {"storage", "food_preservation"},
    },

    spawn = {
        initial_count = 1, -- Spawn one granary at startup
        spawn_area = {
            min_x = 34,
            max_x = 36,
            min_y = 31,
            max_y = 33,
        },
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "granary",
        animation_set = "building",
        color = "#D2691E",
    },

    tags = {"building", "storage", "food", "medium"},
}