-- Melee Weapon Definitions
-- Close combat weapons for defense and hunting

-- Wooden Sword
register_item {
    id = "wooden_sword",
    name = "Wooden Sword",
    description = "A crude sword carved from hard wood",
    category = "weapon",

    properties = {
        weight = 1.0,
        stack_size = 1,
        value = 4,
        rarity = "common",
        tradeable = true,
    },

    -- Weapons would have combat properties (damage, speed, etc.)
    -- For now, basic structure

    tags = {"weapon", "sword", "wooden", "melee"},
}

-- Stone Sword
register_item {
    id = "stone_sword",
    name = "Stone Sword",
    description = "A sharpened stone blade mounted on wood",
    category = "weapon",

    properties = {
        weight = 1.5,
        stack_size = 1,
        value = 8,
        rarity = "common",
        tradeable = true,
    },

    tags = {"weapon", "sword", "stone", "melee"},
}

-- Iron Sword
register_item {
    id = "iron_sword",
    name = "Iron Sword",
    description = "A well-balanced sword forged from iron",
    category = "weapon",

    properties = {
        weight = 2.0,
        stack_size = 1,
        value = 35,
        rarity = "uncommon",
        tradeable = true,
    },

    tags = {"weapon", "sword", "iron", "melee"},
}

-- Spear
register_item {
    id = "spear",
    name = "Spear",
    description = "A long wooden shaft with a pointed tip",
    category = "weapon",

    properties = {
        weight = 1.8,
        stack_size = 1,
        value = 6,
        rarity = "common",
        tradeable = true,
    },

    tags = {"weapon", "spear", "wooden", "melee"},
}

-- Dagger
register_item {
    id = "dagger",
    name = "Dagger",
    description = "A small sharp blade for close combat",
    category = "weapon",

    properties = {
        weight = 0.5,
        stack_size = 1,
        value = 12,
        rarity = "common",
        tradeable = true,
    },

    tags = {"weapon", "dagger", "iron", "melee"},
}