-- Natural Resource Entity Definitions
-- Trees, berry bushes, and other harvestable natural resources

-- Tree
register_entity {
    id = "tree",
    name = "Tree",
    type = "resource",
    description = "A mature tree that can be harvested for wood",

    properties = {
        health = 100.0,
        max_health = 100.0,
        size = {x = 1, y = 1},
    },

    resource = {
        resource_type = "wood",
        harvestable = {
            yield = {min = 8, max = 12}, -- Wood yield per harvest
            regrowth_time = 300.0, -- Time to regrow after harvest
            max_harvests = 3, -- Can be harvested multiple times
        },
        growth = {
            stages = {"sapling", "young", "mature", "old"},
            current_stage = "mature",
            growth_rate = 0.1,
        },
    },

    spawn = {
        initial_count = 15, -- Spawn 15 trees at startup
        spawn_area = {
            min_x = 10,
            max_x = 54,
            min_y = 10,
            max_y = 54,
        },
        require_walkable = false, -- Can spawn on non-walkable terrain
        avoid_settlements = false,
    },

    visuals = {
        sprite = "tree",
        animation_set = "static",
        color = "#228B22",
    },

    tags = {"resource", "wood", "tree", "natural"},
}

-- Berry Bush
register_entity {
    id = "berry_bush",
    name = "Berry Bush",
    type = "resource",
    description = "A bush that produces edible berries",

    properties = {
        health = 30.0,
        max_health = 30.0,
        size = {x = 1, y = 1},
    },

    resource = {
        resource_type = "berry",
        harvestable = {
            yield = {min = 1, max = 4}, -- Berry yield per harvest
            regrowth_time = 50.0, -- Quick berry regrowth
            max_harvests = -1, -- Unlimited harvests
        },
        growth = {
            stages = {"seedling", "flowering", "fruiting", "dormant"},
            current_stage = "fruiting",
            growth_rate = 0.2,
        },
    },

    spawn = {
        initial_count = 25, -- Spawn 25 berry bushes at startup
        spawn_area = {
            min_x = 5,
            max_x = 59,
            min_y = 5,
            max_y = 59,
        },
        require_walkable = true,
        avoid_settlements = false,
    },

    visuals = {
        sprite = "berry_bush",
        animation_set = "static",
        color = "#8B4513",
    },

    tags = {"resource", "food", "berry", "bush", "natural"},
}

-- Berry Bush (Corner variant for clusters)
register_entity {
    id = "berry_bush_corner",
    name = "Berry Bush (Corner)",
    type = "resource",
    description = "A berry bush found in corner areas",

    properties = {
        health = 30.0,
        max_health = 30.0,
        size = {x = 1, y = 1},
    },

    resource = {
        resource_type = "berry",
        harvestable = {
            yield = {min = 1, max = 4},
            regrowth_time = 50.0,
            max_harvests = -1,
        },
        growth = {
            stages = {"seedling", "flowering", "fruiting", "dormant"},
            current_stage = "fruiting",
            growth_rate = 0.2,
        },
    },

    spawn = {
        initial_count = 12, -- Spawn 3 bushes in each of 4 corners
        spawn_area = {
            min_x = 5,
            max_x = 15,
            min_y = 5,
            max_y = 15,
        },
        require_walkable = true,
        avoid_settlements = false,
    },

    visuals = {
        sprite = "berry_bush",
        animation_set = "static",
        color = "#8B4513",
    },

    tags = {"resource", "food", "berry", "bush", "natural", "corner"},
}

-- Stone Deposit
register_entity {
    id = "stone_deposit",
    name = "Stone Deposit",
    type = "resource",
    description = "A deposit of stone that can be mined",

    properties = {
        health = 200.0,
        max_health = 200.0,
        size = {x = 1, y = 1},
    },

    resource = {
        resource_type = "stone",
        harvestable = {
            yield = {min = 5, max = 8}, -- Stone yield per harvest
            regrowth_time = 600.0, -- Very slow regrowth
            max_harvests = 10, -- Limited mining
        },
        growth = {
            stages = {"depleted", "partial", "full"},
            current_stage = "full",
            growth_rate = 0.05,
        },
    },

    spawn = {
        initial_count = 6, -- Spawn 6 stone deposits
        spawn_area = {
            min_x = 20,
            max_x = 44,
            min_y = 20,
            max_y = 44,
        },
        require_walkable = false,
        avoid_settlements = true,
    },

    visuals = {
        sprite = "stone_deposit",
        animation_set = "static",
        color = "#696969",
    },

    tags = {"resource", "stone", "mineral", "natural"},
}

-- Iron Ore Deposit
register_entity {
    id = "iron_ore_deposit",
    name = "Iron Ore Deposit",
    type = "resource",
    description = "A deposit of iron ore that can be mined",

    properties = {
        health = 150.0,
        max_health = 150.0,
        size = {x = 1, y = 1},
    },

    resource = {
        resource_type = "iron_ore",
        harvestable = {
            yield = {min = 3, max = 6}, -- Iron ore yield per harvest
            regrowth_time = 800.0, -- Very slow regrowth
            max_harvests = 8, -- Limited mining
        },
        growth = {
            stages = {"depleted", "partial", "full"},
            current_stage = "full",
            growth_rate = 0.03,
        },
    },

    spawn = {
        initial_count = 0, -- Don't spawn by default - rare resource
        spawn_area = {
            min_x = 10,
            max_x = 54,
            min_y = 10,
            max_y = 54,
        },
        require_walkable = false,
        avoid_settlements = true,
    },

    visuals = {
        sprite = "iron_ore",
        animation_set = "static",
        color = "#CD853F",
    },

    tags = {"resource", "iron_ore", "mineral", "natural", "rare"},
}