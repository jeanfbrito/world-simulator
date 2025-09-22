-- Berries Item Definition
-- Edible fruit harvested from berry bushes

register_item {
    id = "berries",
    name = "Berries",
    description = "Fresh wild berries, sweet and nutritious",
    category = "consumable",

    -- Core item properties
    properties = {
        weight = 0.2,
        stack_size = 20,
        value = 2,
        rarity = "common",
        tradeable = true,
    },

    -- Consumable properties
    consumable = {
        effects = {
            {
                type = "restore_hunger",
                amount = 10,
            },
            {
                type = "restore_health",
                amount = 2,
            },
        },
        cooldown = 0.5,  -- Can eat every 0.5 seconds
        perishable = true,
        perish_time = 300.0,  -- Spoils after 5 minutes
    },

    -- Tags for filtering/searching
    tags = {"food", "fruit", "perishable", "tradeable"},
}