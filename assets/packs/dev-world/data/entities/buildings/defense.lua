-- Defense Building Definitions
-- Buildings for protecting the settlement

-- Wall Section
register_entity {
    id = "wall_section",
    name = "Wall Section",
    type = "building",
    description = "A defensive wall section for fortification",

    properties = {
        health = 200.0,
        max_health = 200.0,
        size = {x = 1, y = 1},
    },

    building = {
        building_type = "wall_section",
        size = "small",
        construction_time = 10.0,

        requirements = {
            {item = "stone", count = 30},
        },

        storage = {
            capacity = 0,
            allowed_types = {},
            efficiency = 1.0,
        },

        production = nil,

        -- Defense properties
        defense = {
            armor_value = 50,
            blocks_movement = true,
            blocks_projectiles = true,
            provides_cover = true,
        },

        workers = {
            max_workers = 0,
            required_skills = {},
        },

        functions = {"defense", "barrier"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "wall_section",
        animation_set = "building",
        color = "#696969",
    },

    tags = {"building", "defense", "wall", "small"},
}

-- Tower
register_entity {
    id = "tower",
    name = "Tower",
    type = "building",
    description = "A defensive tower for archers and surveillance",

    properties = {
        health = 300.0,
        max_health = 300.0,
        size = {x = 3, y = 3},
    },

    building = {
        building_type = "tower",
        size = "large",
        construction_time = 30.0,

        requirements = {
            {item = "stone", count = 50},
            {item = "wood", count = 20},
        },

        storage = {
            capacity = 100,
            allowed_types = {"ammunition", "weapon"},
            efficiency = 1.0,
        },

        production = nil,

        -- Defense properties
        defense = {
            armor_value = 30,
            sight_range = 15,
            attack_range = 10,
            garrison_capacity = 4,
            provides_cover = true,
        },

        workers = {
            max_workers = 0,
            required_skills = {},
        },

        functions = {"defense", "surveillance", "garrison"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "tower",
        animation_set = "building",
        color = "#2F4F4F",
    },

    tags = {"building", "defense", "tower", "large"},
}

-- Gate
register_entity {
    id = "gate",
    name = "Gate",
    type = "building",
    description = "A fortified gatehouse for controlling access",

    properties = {
        health = 250.0,
        max_health = 250.0,
        size = {x = 2, y = 2},
    },

    building = {
        building_type = "gate",
        size = "medium",
        construction_time = 25.0,

        requirements = {
            {item = "stone", count = 40},
            {item = "iron_ingot", count = 20},
        },

        storage = {
            capacity = 50,
            allowed_types = {},
            efficiency = 1.0,
        },

        production = nil,

        -- Defense properties
        defense = {
            armor_value = 40,
            blocks_movement = true,
            controllable = true,
            provides_cover = true,
        },

        workers = {
            max_workers = 1,
            required_skills = {},
        },

        functions = {"defense", "access_control", "barrier"},
    },

    spawn = {
        initial_count = 0,
        require_walkable = true,
        require_flat_ground = true,
    },

    visuals = {
        sprite = "gate",
        animation_set = "building",
        color = "#8B4513",
    },

    tags = {"building", "defense", "gate", "medium"},
}