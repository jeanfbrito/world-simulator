-- Shovel Tool Definitions
-- Digging tools for excavation and mining

-- Wooden Shovel
register_item {
    id = "wooden_shovel",
    name = "Wooden Shovel",
    description = "A basic shovel made from wood with a flat stone blade",
    category = "tool",

    properties = {
        weight = 1.2,
        stack_size = 1,
        value = 3,
        rarity = "common",
        tradeable = true,
    },

    tool = {
        type = "shovel",
        material = "wood",
        durability = 25.0,
        max_durability = 25.0,
        efficiency = 1.0,  -- Base digging speed
        repairable = true,
        repair_cost = {
            {item = "wood", count = 1},
            {item = "stone", count = 1},
        },
        can_harvest = {"sand", "clay"},  -- Good for soft materials
    },

    tags = {"tool", "shovel", "wooden", "repairable"},
}

-- Stone Shovel
register_item {
    id = "stone_shovel",
    name = "Stone Shovel",
    description = "A sturdy shovel with a carved stone blade",
    category = "tool",

    properties = {
        weight = 1.8,
        stack_size = 1,
        value = 6,
        rarity = "common",
        tradeable = true,
    },

    tool = {
        type = "shovel",
        material = "stone",
        durability = 50.0,
        max_durability = 50.0,
        efficiency = 1.5,  -- 50% faster than wood
        repairable = true,
        repair_cost = {
            {item = "stone", count = 2},
            {item = "wood", count = 1},
        },
        can_harvest = {"sand", "clay", "coal"},  -- Can also dig coal
    },

    tags = {"tool", "shovel", "stone", "repairable"},
}

-- Iron Shovel
register_item {
    id = "iron_shovel",
    name = "Iron Shovel",
    description = "A durable shovel forged from iron",
    category = "tool",

    properties = {
        weight = 2.2,
        stack_size = 1,
        value = 25,
        rarity = "uncommon",
        tradeable = true,
    },

    tool = {
        type = "shovel",
        material = "iron",
        durability = 100.0,
        max_durability = 100.0,
        efficiency = 2.0,  -- 2x base speed
        repairable = true,
        repair_cost = {
            {item = "iron_ingot", count = 1},
            {item = "wood", count = 1},
        },
        can_harvest = {"sand", "clay", "coal", "iron_ore"},  -- Can dig soft ores
    },

    tags = {"tool", "shovel", "iron", "repairable"},
}