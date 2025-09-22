-- Berries Item
-- Consumable food item harvested from berry bushes

register_item {
    id = "berries",
    name = "Berries",
    description = "A handful of fresh berries",
    category = "food",
    
    properties = {
        weight = 0.1,
        stack_size = 20,
        value = 2,
        rarity = "common",
        tradeable = true,
    },
    
    consumable = {
        effects = {
            { effect_type = "hunger", amount = 10.0 },
            { effect_type = "health", amount = 2.0 },
        },
        cooldown = 1.0,
        perishable = true,
        perish_time = 600.0,  -- 10 minutes
    },
    
    tags = { "food", "consumable", "perishable", "tradeable" }
}
