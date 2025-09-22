-- Ore Resource Definitions
-- Various mineable ores found underground

-- Iron Ore
register_resource {
    id = "iron_ore",
    name = "Iron Ore",
    description = "Raw iron ore that can be smelted into ingots",
    category = "raw_material",

    properties = {
        weight = 2.0,
        stack_size = 100,
        base_value = 8,
    },

    harvestable = {
        tool_required = "pickaxe",  -- Any pickaxe
        yield = {
            {
                item = "iron_ore",
                min = 1,
                max = 3,
            }
        },
    },

    spawn = {
        biomes = {"mountains", "caves", "hills"},
        frequency = 0.15,
        cluster_size = {
            min = 3,
            max = 8,
        },
        min_distance = 5.0,
    },
}

-- Copper Ore
register_resource {
    id = "copper_ore",
    name = "Copper Ore",
    description = "Greenish ore that can be smelted into copper",
    category = "raw_material",

    properties = {
        weight = 2.0,
        stack_size = 100,
        base_value = 6,
    },

    harvestable = {
        tool_required = "pickaxe",
        yield = {
            {
                item = "copper_ore",
                min = 1,
                max = 3,
            }
        },
    },

    spawn = {
        biomes = {"mountains", "caves", "hills"},
        frequency = 0.20,
        cluster_size = {
            min = 2,
            max = 6,
        },
        min_distance = 4.0,
    },
}

-- Gold Ore
register_resource {
    id = "gold_ore",
    name = "Gold Ore",
    description = "Precious golden ore, rare and valuable",
    category = "raw_material",

    properties = {
        weight = 2.0,
        stack_size = 100,
        base_value = 20,
    },

    harvestable = {
        tool_required = "iron_pickaxe",  -- Requires iron or better
        yield = {
            {
                item = "gold_ore",
                min = 1,
                max = 2,
            }
        },
    },

    spawn = {
        biomes = {"deep_caves", "mountains"},
        frequency = 0.05,  -- Very rare
        cluster_size = {
            min = 1,
            max = 3,
        },
        min_distance = 10.0,
    },
}

-- Coal
register_resource {
    id = "coal",
    name = "Coal",
    description = "Combustible black rock used for fuel",
    category = "energy",

    properties = {
        weight = 1.5,
        stack_size = 100,
        base_value = 5,
    },

    harvestable = {
        tool_required = "pickaxe",
        yield = {
            {
                item = "coal",
                min = 2,
                max = 4,
            }
        },
    },

    spawn = {
        biomes = {"mountains", "caves", "hills"},
        frequency = 0.25,
        cluster_size = {
            min = 3,
            max = 7,
        },
        min_distance = 3.0,
    },
}