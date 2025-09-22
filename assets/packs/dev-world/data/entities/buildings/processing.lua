-- Processing Building Definitions
-- Buildings that process raw materials into finished goods

-- Smelter
register_entity {
    id = "smelter",
    name = "Smelter",
    type = "building",
    description = "A furnace for smelting metal ores into ingots",

    properties = {
        health = 300.0,
        max_health = 300.0,
        size = {x = 3, y = 3},
    },

    building = {
        building_type = "smelter",
        size = "large",
        construction_time = 30.0,

        requirements = {
            {item = "stone", count = 50},
            {item = "clay", count = 20},
            {item = "iron_ingot", count = 15},
        },

        storage = {
            capacity = 300,
            allowed_types = {"ore", "ingot", "coal"},
            efficiency = 1.0,
        },

        production = {
            -- Smelts iron ore into iron ingots
            input = {
                {item = "iron_ore", count = 2},
                {item = "coal", count = 1},
            },
            output = {{item = "iron_ingot", count = 1}},
            production_time = 10.0,
            efficiency = 0.4,
            auto_produce = true,
            requires_fuel = true,
        },

        workers = {
            max_workers = 3,
            required_skills = {smithing = 2},
        },

        functions = {"processing", "smelting", "metal"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "smelter",
        animation_set = "building",
        color = "#FF4500",
    },

    tags = {"building", "processing", "smelting", "metal", "large"},
}

-- Workshop
register_entity {
    id = "workshop",
    name = "Workshop",
    type = "building",
    description = "A workshop for crafting tools and items",

    properties = {
        health = 200.0,
        max_health = 200.0,
        size = {x = 2, y = 2},
    },

    building = {
        building_type = "workshop",
        size = "medium",
        construction_time = 20.0,

        requirements = {
            {item = "wood", count = 35},
            {item = "stone", count = 25},
            {item = "iron_ingot", count = 10},
        },

        storage = {
            capacity = 200,
            allowed_types = {"tool", "weapon", "armor", "material"},
            efficiency = 1.0,
        },

        production = {
            -- Can craft various items based on available recipes
            input = {},
            output = {},
            production_time = 0.0,
            efficiency = 1.0,
            auto_produce = false,
            uses_recipes = true,
        },

        workers = {
            max_workers = 2,
            required_skills = {crafting = 2, smithing = 1},
        },

        functions = {"processing", "crafting", "tool_making"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "workshop",
        animation_set = "building",
        color = "#DEB887",
    },

    tags = {"building", "processing", "crafting", "medium"},
}

-- Kitchen
register_entity {
    id = "kitchen",
    name = "Kitchen",
    type = "building",
    description = "A kitchen for preparing food and meals",

    properties = {
        health = 200.0,
        max_health = 200.0,
        size = {x = 2, y = 2},
    },

    building = {
        building_type = "kitchen",
        size = "medium",
        construction_time = 20.0,

        requirements = {
            {item = "wood", count = 20},
            {item = "stone", count = 15},
            {item = "clay", count = 10},
        },

        storage = {
            capacity = 150,
            allowed_types = {"food", "ingredient"},
            efficiency = 1.0,
        },

        production = {
            -- Cooks raw food into prepared meals
            input = {{item = "raw_food", count = 1}},
            output = {{item = "cooked_food", count = 1}},
            production_time = 8.0,
            efficiency = 0.8,
            auto_produce = true,
            uses_recipes = true,
        },

        workers = {
            max_workers = 2,
            required_skills = {cooking = 2},
        },

        functions = {"processing", "cooking", "food"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "kitchen",
        animation_set = "building",
        color = "#F4A460",
    },

    tags = {"building", "processing", "cooking", "food", "medium"},
}