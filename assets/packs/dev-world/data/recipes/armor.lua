-- Armor Crafting Recipes
-- Recipes for creating protective gear

-- Helmet
register_recipe {
    id = "helmet",
    name = "Helmet",
    description = "Forge protective headgear from iron",
    category = "armor",

    requirements = {
        {item = "iron_ingot", count = 3, consume = true},
    },

    outputs = {
        {item = "helmet", count = 1},
    },

    crafting = {
        time = 8.0,
        station = "anvil",
        skill_required = {smithing = 2},
    },
}

-- Chestplate
register_recipe {
    id = "chestplate",
    name = "Chestplate",
    description = "Forge body armor covering the torso",
    category = "armor",

    requirements = {
        {item = "iron_ingot", count = 6, consume = true},
    },

    outputs = {
        {item = "chestplate", count = 1},
    },

    crafting = {
        time = 12.0,
        station = "anvil",
        skill_required = {smithing = 3},
    },
}

-- Leggings
register_recipe {
    id = "leggings",
    name = "Leggings",
    description = "Forge protective leg armor",
    category = "armor",

    requirements = {
        {item = "iron_ingot", count = 4, consume = true},
    },

    outputs = {
        {item = "leggings", count = 1},
    },

    crafting = {
        time = 10.0,
        station = "anvil",
        skill_required = {smithing = 2},
    },
}

-- Boots
register_recipe {
    id = "boots",
    name = "Boots",
    description = "Craft sturdy footwear for protection",
    category = "armor",

    requirements = {
        {item = "leather", count = 3, consume = true},
        {item = "iron_ingot", count = 1, consume = true},
    },

    outputs = {
        {item = "boots", count = 1},
    },

    crafting = {
        time = 6.0,
        station = "workbench",
        skill_required = {crafting = 2},
    },
}

-- Shield
register_recipe {
    id = "shield",
    name = "Shield",
    description = "Craft a wooden shield reinforced with iron",
    category = "armor",

    requirements = {
        {item = "wood", count = 4, consume = true},
        {item = "iron_ingot", count = 2, consume = true},
    },

    outputs = {
        {item = "shield", count = 1},
    },

    crafting = {
        time = 7.0,
        station = "workbench",
        skill_required = {crafting = 2},
    },
}