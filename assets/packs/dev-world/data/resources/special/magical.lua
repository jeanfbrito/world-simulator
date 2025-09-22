-- Special Magical Resources
-- Rare and valuable materials with magical properties

-- Gem
register_resource {
    id = "gem",
    name = "Gem",
    description = "Precious crystalline mineral",
    category = "special",

    properties = {
        weight = 0.3,
        stack_size = 10,
        base_value = 50,
    },

    spawn = {
        biomes = {"deep_caves", "mountains"},
        frequency = 0.02,  -- Very rare
        cluster_size = {min = 1, max = 2},
        min_distance = 15.0,
    },
}

-- Crystal
register_resource {
    id = "crystal",
    name = "Crystal",
    description = "Magical crystal with内在 energy",
    category = "special",

    properties = {
        weight = 0.3,
        stack_size = 10,
        base_value = 30,
    },

    spawn = {
        biomes = {"magic_forest", "deep_caves"},
        frequency = 0.03,
        cluster_size = {min = 1, max = 3},
        min_distance = 12.0,
    },
}

-- Magic Dust
register_resource {
    id = "magic_dust",
    name = "Magic Dust",
    description = "Finely ground magical powder",
    category = "special",

    properties = {
        weight = 0.1,
        stack_size = 10,
        base_value = 100,
    },
}