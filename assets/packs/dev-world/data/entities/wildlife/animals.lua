-- Wildlife Animal Definitions
-- Animals that spawn in the wild and can be hunted

-- Deer
register_entity {
    id = "deer",
    name = "Deer",
    type = "wildlife",
    description = "A wild deer found in forests",

    properties = {
        health = 50.0,
        max_health = 50.0,
        size = {x = 1, y = 1},
    },

    wildlife = {
        behavior = "herbivore",
        movement_speed = 1.2,
        group_size = {min = 2, max = 6},
        spawn_rate = 0.3,
        flee_range = 8.0,

        -- Resource drops when hunted
        drops = {
            {item = "meat", count = {min = 2, max = 4}, chance = 0.9},
            {item = "hide", count = {min = 1, max = 2}, chance = 0.7},
        },

        -- Habitat preferences
        habitat = {
            biomes = {"forest", "meadow"},
            density = "medium",
            time_of_day = {"day"},
        },

        -- Interaction with units
        interaction = {
            hostile = false,
            flee_on_sight = true,
            hunt_reward = 10,
        },
    },

    spawn = {
        initial_count = 0, -- Spawn dynamically
        spawn_area = "wilderness",
        require_walkable = true,
        avoid_settlements = true,
    },

    visuals = {
        sprite = "deer",
        animation_set = "quadruped",
        color_variation = true,
    },

    tags = {"wildlife", "animal", "herbivore", "huntable"},
}

-- Wolf
register_entity {
    id = "wolf",
    name = "Wolf",
    type = "wildlife",
    description = "A wild wolf that hunts in packs",

    properties = {
        health = 80.0,
        max_health = 80.0,
        size = {x = 1, y = 1},
    },

    wildlife = {
        behavior = "carnivore",
        movement_speed = 1.5,
        group_size = {min = 3, max = 8},
        spawn_rate = 0.2,
        attack_range = 1.0,

        drops = {
            {item = "meat", count = {min = 3, max = 5}, chance = 0.8},
            {item = "hide", count = {min = 1, max = 2}, chance = 0.6},
            {item = "tooth", count = {min = 1, max = 3}, chance = 0.3},
        },

        habitat = {
            biomes = {"forest", "mountain"},
            density = "low",
            time_of_day = {"night", "dawn", "dusk"},
        },

        interaction = {
            hostile = true,
            flee_on_sight = false,
            hunt_reward = 20,
            attack_damage = 15,
        },
    },

    spawn = {
        initial_count = 0,
        spawn_area = "wilderness",
        require_walkable = true,
        avoid_settlements = true,
    },

    visuals = {
        sprite = "wolf",
        animation_set = "quadruped",
        color_variation = true,
    },

    tags = {"wildlife", "animal", "carnivore", "dangerous"},
}

-- Bear
register_entity {
    id = "bear",
    name = "Bear",
    type = "wildlife",
    description = "A large bear, dangerous when provoked",

    properties = {
        health = 150.0,
        max_health = 150.0,
        size = {x = 2, y = 2},
    },

    wildlife = {
        behavior = "omnivore",
        movement_speed = 1.0,
        group_size = {min = 1, max = 2},
        spawn_rate = 0.1,
        attack_range = 1.0,

        drops = {
            {item = "meat", count = {min = 8, max = 12}, chance = 0.9},
            {item = "hide", count = {min = 2, max = 4}, chance = 0.8},
            {item = "fat", count = {min = 2, max = 5}, chance = 0.5},
        },

        habitat = {
            biomes = {"forest", "mountain"},
            density = "very_low",
            time_of_day = {"day", "night"},
        },

        interaction = {
            hostile = true,
            flee_on_sight = false,
            hunt_reward = 50,
            attack_damage = 30,
            territorial = true,
        },
    },

    spawn = {
        initial_count = 0,
        spawn_area = "wilderness",
        require_walkable = true,
        avoid_settlements = true,
    },

    visuals = {
        sprite = "bear",
        animation_set = "quadruped",
        color_variation = true,
    },

    tags = {"wildlife", "animal", "carnivore", "dangerous", "large"},
}

-- Rabbit
register_entity {
    id = "rabbit",
    name = "Rabbit",
    type = "wildlife",
    description = "A small rabbit found in meadows",

    properties = {
        health = 20.0,
        max_health = 20.0,
        size = {x = 1, y = 1},
    },

    wildlife = {
        behavior = "herbivore",
        movement_speed = 2.0,
        group_size = {min = 4, max = 10},
        spawn_rate = 0.5,
        flee_range = 6.0,

        drops = {
            {item = "meat", count = {min = 1, max = 2}, chance = 0.7},
            {item = "hide", count = {min = 1, max = 1}, chance = 0.5},
        },

        habitat = {
            biomes = {"meadow", "forest"},
            density = "high",
            time_of_day = {"day", "dawn", "dusk"},
        },

        interaction = {
            hostile = false,
            flee_on_sight = true,
            hunt_reward = 5,
        },
    },

    spawn = {
        initial_count = 0,
        spawn_area = "wilderness",
        require_walkable = true,
        avoid_settlements = false, -- Rabbits can be found near settlements
    },

    visuals = {
        sprite = "rabbit",
        animation_set = "quadruped",
        color_variation = true,
    },

    tags = {"wildlife", "animal", "herbivore", "small"},
}

-- Fish
register_entity {
    id = "fish",
    name = "Fish",
    type = "wildlife",
    description = "Fish found in rivers and lakes",

    properties = {
        health = 10.0,
        max_health = 10.0,
        size = {x = 1, y = 1},
    },

    wildlife = {
        behavior = "aquatic",
        movement_speed = 0.8,
        group_size = {min = 5, max = 20},
        spawn_rate = 0.8,

        drops = {
            {item = "fish", count = {min = 1, max = 1}, chance = 1.0},
        },

        habitat = {
            biomes = {"water", "river", "lake"},
            density = "high",
            time_of_day = {"day", "night"},
        },

        interaction = {
            hostile = false,
            flee_on_sight = true,
            hunt_reward = 3,
            requires_fishing = true,
        },
    },

    spawn = {
        initial_count = 0,
        spawn_area = "water",
        require_water = true,
    },

    visuals = {
        sprite = "fish",
        animation_set = "aquatic",
        color_variation = true,
    },

    tags = {"wildlife", "animal", "aquatic", "small"},
}