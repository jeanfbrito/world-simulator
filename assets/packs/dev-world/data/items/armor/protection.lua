-- Armor Definitions
-- Protective gear for combat and safety

-- Helmet
register_item {
    id = "helmet",
    name = "Helmet",
    description = "Protective headgear made from iron",
    category = "armor",

    properties = {
        weight = 2.0,
        stack_size = 1,
        value = 20,
        rarity = "uncommon",
        tradeable = true,
    },

    tags = {"armor", "helmet", "iron", "head"},
}

-- Chestplate
register_item {
    id = "chestplate",
    name = "Chestplate",
    description = "Body armor covering the torso",
    category = "armor",

    properties = {
        weight = 5.0,
        stack_size = 1,
        value = 40,
        rarity = "uncommon",
        tradeable = true,
    },

    tags = {"armor", "chestplate", "iron", "torso"},
}

-- Leggings
register_item {
    id = "leggings",
    name = "Leggings",
    description = "Protective leg armor",
    category = "armor",

    properties = {
        weight = 3.0,
        stack_size = 1,
        value = 25,
        rarity = "uncommon",
        tradeable = true,
    },

    tags = {"armor", "leggings", "iron", "legs"},
}

-- Boots
register_item {
    id = "boots",
    name = "Boots",
    description = "Sturdy footwear for protection",
    category = "armor",

    properties = {
        weight = 2.0,
        stack_size = 1,
        value = 15,
        rarity = "common",
        tradeable = true,
    },

    tags = {"armor", "boots", "leather", "feet"},
}

-- Shield
register_item {
    id = "shield",
    name = "Shield",
    description = "A wooden shield reinforced with iron",
    category = "armor",

    properties = {
        weight = 4.0,
        stack_size = 1,
        value = 18,
        rarity = "common",
        tradeable = true,
    },

    tags = {"armor", "shield", "wooden", "defense"},
}