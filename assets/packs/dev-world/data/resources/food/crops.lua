-- Food Crop Resources
-- Agricultural products and basic foodstuffs

-- Wheat
register_resource {
    id = "wheat",
    name = "Wheat",
    description = "Golden grain used for making bread",
    category = "food",

    properties = {
        weight = 0.2,
        stack_size = 20,
        base_value = 3,
    },

    harvestable = {
        tool_required = "sickle",
        yield = {
            {
                item = "wheat",
                min = 2,
                max = 4,
            }
        },
        growth_stages = 3,
        stage_time = 30.0,  -- 30 seconds per growth stage
        respawn_time = 60.0,
        requires_water = true,
    },

    spawn = {
        biomes = {"plains", "farmland"},
        frequency = 0.4,
        cluster_size = {min = 4, max = 8},
    },
}

-- Corn
register_resource {
    id = "corn",
    name = "Corn",
    description = "Yellow kernels growing on tall stalks",
    category = "food",

    properties = {
        weight = 0.2,
        stack_size = 20,
        base_value = 3,
    },

    harvestable = {
        tool_required = "sickle",
        yield = {
            {
                item = "corn",
                min = 1,
                max = 3,
            }
        },
        growth_stages = 4,
        stage_time = 25.0,
        respawn_time = 50.0,
        requires_water = true,
    },

    spawn = {
        biomes = {"plains", "farmland"},
        frequency = 0.3,
        cluster_size = {min = 3, max = 6},
    },
}