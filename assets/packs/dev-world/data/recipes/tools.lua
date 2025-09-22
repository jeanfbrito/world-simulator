-- Tool Crafting Recipes
-- Recipes for creating various tools

-- Wooden Pickaxe
register_recipe {
    id = "wooden_pickaxe",
    name = "Wooden Pickaxe",
    description = "Craft a basic wooden pickaxe",
    category = "tools",

    requirements = {
        {item = "wood", count = 3, consume = true},
        {item = "stone", count = 2, consume = true},
    },

    outputs = {
        {item = "wooden_pickaxe", count = 1, chance = 1.0},
    },

    crafting = {
        time = 3.0,  -- 3 seconds to craft
        station = nil,  -- Can craft by hand
        skill_required = {},  -- No skill requirements
        unlock_condition = nil,  -- Always available
    },

    tags = {"tool", "basic", "starter"},
}

-- Stone Pickaxe
register_recipe {
    id = "stone_pickaxe",
    name = "Stone Pickaxe",
    description = "Craft a sturdy stone pickaxe",
    category = "tools",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "stone", count = 3, consume = true},
    },

    outputs = {
        {item = "stone_pickaxe", count = 1},
    },

    crafting = {
        time = 5.0,
        station = "workbench",  -- Requires a workbench
        skill_required = {crafting = 1},
    },
}

-- Iron Pickaxe
register_recipe {
    id = "iron_pickaxe",
    name = "Iron Pickaxe",
    description = "Forge a strong iron pickaxe",
    category = "tools",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "iron_ingot", count = 3, consume = true},
    },

    outputs = {
        {item = "iron_pickaxe", count = 1},
    },

    crafting = {
        time = 8.0,
        station = "anvil",  -- Requires an anvil
        skill_required = {smithing = 2},
    },
}

-- Wooden Axe
register_recipe {
    id = "wooden_axe",
    name = "Wooden Axe",
    description = "Craft a basic wooden axe for chopping trees",
    category = "tools",

    requirements = {
        {item = "wood", count = 3, consume = true},
        {item = "stone", count = 2, consume = true},
    },

    outputs = {
        {item = "wooden_axe", count = 1},
    },

    crafting = {
        time = 3.0,
        station = nil,  -- Can craft by hand
    },
}

-- Fishing Rod
register_recipe {
    id = "fishing_rod",
    name = "Fishing Rod",
    description = "Craft a simple fishing rod",
    category = "tools",

    requirements = {
        {item = "wood", count = 3, consume = true},
        -- Future: Add string/fiber requirement
    },

    outputs = {
        {item = "fishing_rod", count = 1},
    },

    crafting = {
        time = 4.0,
        station = "workbench",
    },
}