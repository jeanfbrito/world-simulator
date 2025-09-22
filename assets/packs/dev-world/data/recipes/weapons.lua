-- Weapon Crafting Recipes
-- Recipes for creating combat weapons

-- Wooden Sword
register_recipe {
    id = "wooden_sword",
    name = "Wooden Sword",
    description = "Craft a crude sword from hard wood",
    category = "weapons",

    requirements = {
        {item = "wood", count = 3, consume = true},
        {item = "stone", count = 1, consume = true},
    },

    outputs = {
        {item = "wooden_sword", count = 1},
    },

    crafting = {
        time = 3.0,
        station = "workbench",
        skill_required = {crafting = 1},
    },
}

-- Stone Sword
register_recipe {
    id = "stone_sword",
    name = "Stone Sword",
    description = "Craft a sharpened stone blade mounted on wood",
    category = "weapons",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "stone", count = 4, consume = true},
    },

    outputs = {
        {item = "stone_sword", count = 1},
    },

    crafting = {
        time = 5.0,
        station = "workbench",
        skill_required = {crafting = 2},
    },
}

-- Iron Sword
register_recipe {
    id = "iron_sword",
    name = "Iron Sword",
    description = "Forge a well-balanced sword from iron",
    category = "weapons",

    requirements = {
        {item = "wood", count = 1, consume = true},
        {item = "iron_ingot", count = 4, consume = true},
    },

    outputs = {
        {item = "iron_sword", count = 1},
    },

    crafting = {
        time = 10.0,
        station = "anvil",
        skill_required = {smithing = 3},
    },
}

-- Spear
register_recipe {
    id = "spear",
    name = "Spear",
    description = "Craft a long wooden shaft with a pointed tip",
    category = "weapons",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "stone", count = 2, consume = true},
    },

    outputs = {
        {item = "spear", count = 1},
    },

    crafting = {
        time = 4.0,
        station = "workbench",
        skill_required = {crafting = 1},
    },
}

-- Dagger
register_recipe {
    id = "dagger",
    name = "Dagger",
    description = "Forge a small sharp blade for close combat",
    category = "weapons",

    requirements = {
        {item = "wood", count = 1, consume = true},
        {item = "iron_ingot", count = 2, consume = true},
    },

    outputs = {
        {item = "dagger", count = 1},
    },

    crafting = {
        time = 6.0,
        station = "anvil",
        skill_required = {smithing = 2},
    },
}

-- Bow
register_recipe {
    id = "bow",
    name = "Bow",
    description = "Craft a flexible wooden bow for shooting arrows",
    category = "weapons",

    requirements = {
        {item = "wood", count = 3, consume = true},
        -- Future: Add string requirement
    },

    outputs = {
        {item = "bow", count = 1},
    },

    crafting = {
        time = 5.0,
        station = "workbench",
        skill_required = {crafting = 2},
    },
}

-- Arrows
register_recipe {
    id = "arrows",
    name = "Arrows",
    description = "Craft a bundle of arrows for use with bows",
    category = "weapons",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "stone", count = 1, consume = true},
    },

    outputs = {
        {item = "arrows", count = 10},
    },

    crafting = {
        time = 3.0,
        station = "workbench",
        skill_required = {crafting = 1},
    },
}

-- Iron Arrows (upgraded version)
register_recipe {
    id = "iron_arrows",
    name = "Iron Arrows",
    description = "Craft superior arrows with iron tips",
    category = "weapons",

    requirements = {
        {item = "wood", count = 2, consume = true},
        {item = "iron_ingot", count = 1, consume = true},
    },

    outputs = {
        {item = "iron_arrows", count = 10},
    },

    crafting = {
        time = 4.0,
        station = "workbench",
        skill_required = {crafting = 2},
    },
}