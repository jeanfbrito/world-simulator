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

-- Stone Axe
register_recipe {
    id = "stone_axe",
    name = "Stone Axe",
    description = "Craft a sturdy stone axe",
    category = "tools",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "stone", count = 3, consume = true},
    },

    outputs = {
        {item = "stone_axe", count = 1},
    },

    crafting = {
        time = 5.0,
        station = "workbench",
        skill_required = {crafting = 1},
    },
}

-- Iron Axe
register_recipe {
    id = "iron_axe",
    name = "Iron Axe",
    description = "Forge a strong iron axe",
    category = "tools",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "iron_ingot", count = 3, consume = true},
    },

    outputs = {
        {item = "iron_axe", count = 1},
    },

    crafting = {
        time = 8.0,
        station = "anvil",
        skill_required = {smithing = 2},
    },
}

-- Wooden Shovel
register_recipe {
    id = "wooden_shovel",
    name = "Wooden Shovel",
    description = "Craft a basic wooden shovel",
    category = "tools",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "stone", count = 1, consume = true},
    },

    outputs = {
        {item = "wooden_shovel", count = 1},
    },

    crafting = {
        time = 2.5,
        station = nil,
    },
}

-- Stone Shovel
register_recipe {
    id = "stone_shovel",
    name = "Stone Shovel",
    description = "Craft a sturdy stone shovel",
    category = "tools",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "stone", count = 2, consume = true},
    },

    outputs = {
        {item = "stone_shovel", count = 1},
    },

    crafting = {
        time = 4.0,
        station = "workbench",
        skill_required = {crafting = 1},
    },
}

-- Iron Shovel
register_recipe {
    id = "iron_shovel",
    name = "Iron Shovel",
    description = "Forge a strong iron shovel",
    category = "tools",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "iron_ingot", count = 2, consume = true},
    },

    outputs = {
        {item = "iron_shovel", count = 1},
    },

    crafting = {
        time = 6.0,
        station = "anvil",
        skill_required = {smithing = 2},
    },
}

-- Hammer
register_recipe {
    id = "hammer",
    name = "Hammer",
    description = "Forge a heavy hammer for construction and smithing",
    category = "tools",

    requirements = {
        {item = "wood", count = 1, consume = true},
        {item = "iron_ingot", count = 3, consume = true},
    },

    outputs = {
        {item = "hammer", count = 1},
    },

    crafting = {
        time = 7.0,
        station = "anvil",
        skill_required = {smithing = 2},
    },
}

-- Saw
register_recipe {
    id = "saw",
    name = "Saw",
    description = "Craft a toothed blade for cutting wood precisely",
    category = "tools",

    requirements = {
        {item = "wood", count = 1, consume = true},
        {item = "iron_ingot", count = 2, consume = true},
    },

    outputs = {
        {item = "saw", count = 1},
    },

    crafting = {
        time = 6.0,
        station = "workbench",
        skill_required = {crafting = 2},
    },
}