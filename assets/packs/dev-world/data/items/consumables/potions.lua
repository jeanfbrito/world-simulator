-- Potion Definitions
-- Consumable items with temporary effects

-- Health Potion
register_item {
    id = "health_potion",
    name = "Health Potion",
    description = "A red liquid that restores health",
    category = "consumable",

    properties = {
        weight = 0.3,
        stack_size = 10,
        value = 25,
        rarity = "uncommon",
        tradeable = true,
    },

    consumable = {
        effects = {
            {
                effect_type = "health_restore",
                amount = 50.0,
            }
        },
        cooldown = 5.0,  -- 5 seconds between uses
    },

    tags = {"consumable", "potion", "health", "healing"},
}

-- Energy Potion
register_item {
    id = "energy_potion",
    name = "Energy Potion",
    description = "A blue liquid that restores energy",
    category = "consumable",

    properties = {
        weight = 0.3,
        stack_size = 10,
        value = 20,
        rarity = "uncommon",
        tradeable = true,
    },

    consumable = {
        effects = {
            {
                effect_type = "energy_restore",
                amount = 30.0,
            }
        },
        cooldown = 5.0,
    },

    tags = {"consumable", "potion", "energy", "restoration"},
}