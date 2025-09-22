-- Residential Building Definitions
-- Buildings for housing population and military

-- House
register_entity {
    id = "house",
    name = "House",
    type = "building",
    description = "A small house for housing citizens",

    properties = {
        health = 100.0,
        max_health = 100.0,
        size = {x = 1, y = 1},
    },

    building = {
        building_type = "house",
        size = "small",
        construction_time = 10.0,

        requirements = {
            {item = "wood", count = 15},
            {item = "stone", count = 10},
        },

        storage = {
            capacity = 50,
            allowed_types = {"food", "tool"},
            efficiency = 1.0,
        },

        production = nil,

        -- Residential properties
        residential = {
            population_capacity = 4,
            comfort_level = 1.0,
            health_bonus = 0.1,
        },

        workers = {
            max_workers = 0,
            required_skills = {},
        },

        functions = {"housing", "residential"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "house",
        animation_set = "building",
        color = "#CD853F",
    },

    tags = {"building", "residential", "housing", "small"},
}

-- Barracks
register_entity {
    id = "barracks",
    name = "Barracks",
    type = "building",
    description = "Military barracks for training and housing soldiers",

    properties = {
        health = 300.0,
        max_health = 300.0,
        size = {x = 3, y = 3},
    },

    building = {
        building_type = "barracks",
        size = "large",
        construction_time = 30.0,

        requirements = {
            {item = "wood", count = 30},
            {item = "stone", count = 40},
            {item = "iron_ingot", count = 10},
        },

        storage = {
            capacity = 200,
            allowed_types = {"weapon", "armor", "military"},
            efficiency = 1.0,
        },

        production = {
            -- Trains military units
            input = {
                {item = "food", count = 10},
                {item = "weapon", count = 1},
            },
            output = {{item = "soldier", count = 1}},
            production_time = 30.0,
            efficiency = 0.5,
            auto_produce = false,
        },

        -- Military properties
        military = {
            training_capacity = 10,
            training_speed = 1.0,
            defense_bonus = 0.2,
        },

        workers = {
            max_workers = 2,
            required_skills = {military = 2},
        },

        functions = {"housing", "military", "training"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "barracks",
        animation_set = "building",
        color = "#8B0000",
    },

    tags = {"building", "residential", "military", "large"},
}