-- Crafting and Special Tool Definitions
-- Tools for construction and specialized tasks

-- Hammer
register_item {
    id = "hammer",
    name = "Hammer",
    description = "A heavy tool for construction and smithing",
    category = "tool",

    properties = {
        weight = 2.0,
        stack_size = 1,
        value = 8,
        rarity = "common",
        tradeable = true,
    },

    tool = {
        type = "hammer",
        material = "iron",
        durability = 80.0,
        max_durability = 80.0,
        efficiency = 1.2,  -- Construction speed bonus
        repairable = true,
        repair_cost = {
            {item = "iron_ingot", count = 1},
            {item = "wood", count = 1},
        },
        -- Used for construction and repair
    },

    tags = {"tool", "hammer", "construction", "repairable"},
}

-- Saw
register_item {
    id = "saw",
    name = "Saw",
    description = "A toothed blade for cutting wood precisely",
    category = "tool",

    properties = {
        weight = 1.0,
        stack_size = 1,
        value = 12,
        rarity = "common",
        tradeable = true,
    },

    tool = {
        type = "saw",
        material = "iron",
        durability = 60.0,
        max_durability = 60.0,
        efficiency = 1.8,  -- Very efficient wood cutting
        repairable = true,
        repair_cost = {
            {item = "iron_ingot", count = 1},
            {item = "wood", count = 1},
        },
        -- Used for fine woodworking
    },

    tags = {"tool", "saw", "woodworking", "repairable"},
}

-- Fishing Rod
register_item {
    id = "fishing_rod",
    name = "Fishing Rod",
    description = "A flexible rod for catching fish",
    category = "tool",

    properties = {
        weight = 0.5,
        stack_size = 1,
        value = 5,
        rarity = "common",
        tradeable = true,
    },

    tool = {
        type = "fishing_rod",
        material = "wood",
        durability = 40.0,
        max_durability = 40.0,
        efficiency = 1.0,  -- Base fishing success rate
        repairable = true,
        repair_cost = {
            {item = "wood", count = 1},
            {item = "string", count = 1},  -- Assuming string resource exists
        },
        -- Used for fishing
    },

    tags = {"tool", "fishing_rod", "fishing", "repairable"},
}