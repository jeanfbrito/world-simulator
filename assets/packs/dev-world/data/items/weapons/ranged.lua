-- Ranged Weapon Definitions
-- Weapons for attacking from distance

-- Bow
register_item {
    id = "bow",
    name = "Bow",
    description = "A flexible wooden bow for shooting arrows",
    category = "weapon",

    properties = {
        weight = 0.8,
        stack_size = 1,
        value = 15,
        rarity = "common",
        tradeable = true,
    },

    tags = {"weapon", "bow", "wooden", "ranged"},
}

-- Arrows
register_item {
    id = "arrows",
    name = "Arrows",
    description = "A bundle of arrows for use with bows",
    category = "ammunition",

    properties = {
        weight = 0.1,
        stack_size = 20,
        value = 1,
        rarity = "common",
        tradeable = true,
    },

    tags = {"ammunition", "arrows", "ranged"},
}