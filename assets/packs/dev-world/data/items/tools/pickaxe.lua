-- Pickaxe Tool Definitions
-- Mining tools for harvesting stone and ores

-- Wooden Pickaxe
register_item {
    id = "wooden_pickaxe",
    name = "Wooden Pickaxe",
    description = "A basic pickaxe made from wood and stone",
    category = "tool",

    properties = {
        weight = 2.0,
        stack_size = 1,
        value = 6,
        rarity = "common",
        tradeable = true,
    },

    tool = {
        type = "pickaxe",
        material = "wood",
        durability = 50.0,
        max_durability = 50.0,
        efficiency = 1.0,  -- Base mining speed
        repairable = true,
        repair_cost = {
            {item = "wood", count = 1},
            {item = "stone", count = 1},
        },
        -- What this tool can harvest
        can_harvest = {"stone", "coal"},
    },

    tags = {"tool", "pickaxe", "wooden", "repairable"},
}

-- Stone Pickaxe
register_item {
    id = "stone_pickaxe",
    name = "Stone Pickaxe",
    description = "A sturdy pickaxe with a stone head",
    category = "tool",

    properties = {
        weight = 2.5,
        stack_size = 1,
        value = 10,
        rarity = "common",
        tradeable = true,
    },

    tool = {
        type = "pickaxe",
        material = "stone",
        durability = 100.0,
        max_durability = 100.0,
        efficiency = 1.5,  -- 50% faster than wood
        repairable = true,
        repair_cost = {
            {item = "stone", count = 2},
        },
        can_harvest = {"stone", "coal", "iron_ore", "copper_ore"},
    },

    tags = {"tool", "pickaxe", "stone", "repairable"},
}

-- Iron Pickaxe
register_item {
    id = "iron_pickaxe",
    name = "Iron Pickaxe",
    description = "A strong pickaxe forged from iron",
    category = "tool",

    properties = {
        weight = 3.0,
        stack_size = 1,
        value = 45,
        rarity = "uncommon",
        tradeable = true,
    },

    tool = {
        type = "pickaxe",
        material = "iron",
        durability = 200.0,
        max_durability = 200.0,
        efficiency = 2.0,  -- 2x base speed
        repairable = true,
        repair_cost = {
            {item = "iron_ingot", count = 1},
        },
        can_harvest = {"stone", "coal", "iron_ore", "copper_ore", "gold_ore"},
    },

    tags = {"tool", "pickaxe", "iron", "repairable"},
}