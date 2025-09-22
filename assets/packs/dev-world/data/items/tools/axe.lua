-- Axe Tool Definitions
-- Woodcutting tools for harvesting wood

-- Wooden Axe
register_item {
    id = "wooden_axe",
    name = "Wooden Axe",
    description = "A basic axe made from wood with a stone head",
    category = "tool",

    properties = {
        weight = 1.5,
        stack_size = 1,
        value = 4,
        rarity = "common",
        tradeable = true,
    },

    tool = {
        type = "axe",
        material = "wood",
        durability = 30.0,
        max_durability = 30.0,
        efficiency = 1.0,  -- Base chopping speed
        repairable = true,
        repair_cost = {
            {item = "wood", count = 2},
            {item = "stone", count = 1},
        },
        can_harvest = {"wood"},  -- Can harvest trees
    },

    tags = {"tool", "axe", "wooden", "repairable"},
}

-- Stone Axe
register_item {
    id = "stone_axe",
    name = "Stone Axe",
    description = "A sturdy axe with a sharpened stone head",
    category = "tool",

    properties = {
        weight = 2.0,
        stack_size = 1,
        value = 8,
        rarity = "common",
        tradeable = true,
    },

    tool = {
        type = "axe",
        material = "stone",
        durability = 60.0,
        max_durability = 60.0,
        efficiency = 1.5,  -- 50% faster than wood
        repairable = true,
        repair_cost = {
            {item = "stone", count = 2},
            {item = "wood", count = 1},
        },
        can_harvest = {"wood"},
    },

    tags = {"tool", "axe", "stone", "repairable"},
}

-- Iron Axe
register_item {
    id = "iron_axe",
    name = "Iron Axe",
    description = "A sharp axe forged from iron",
    category = "tool",

    properties = {
        weight = 2.5,
        stack_size = 1,
        value = 30,
        rarity = "uncommon",
        tradeable = true,
    },

    tool = {
        type = "axe",
        material = "iron",
        durability = 120.0,
        max_durability = 120.0,
        efficiency = 2.0,  -- 2x base speed
        repairable = true,
        repair_cost = {
            {item = "iron_ingot", count = 1},
            {item = "wood", count = 1},
        },
        can_harvest = {"wood"},
    },

    tags = {"tool", "axe", "iron", "repairable"},
}