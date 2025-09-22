-- Processing Recipes
-- Recipes for material processing and refinement

-- Smelting Recipes (require furnace)

-- Iron Ingot from Iron Ore
register_recipe {
    id = "smelt_iron",
    name = "Smelt Iron Ingot",
    description = "Smelt iron ore into iron ingots",
    category = "processing",

    requirements = {
        {item = "iron_ore", count = 2, consume = true},
        {item = "coal", count = 1, consume = true}, -- Fuel
    },

    outputs = {
        {item = "iron_ingot", count = 1},
    },

    crafting = {
        time = 10.0,
        station = "furnace",
        skill_required = {smithing = 1},
    },
}

-- Copper Ingot from Copper Ore
register_recipe {
    id = "smelt_copper",
    name = "Smelt Copper Ingot",
    description = "Smelt copper ore into copper ingots",
    category = "processing",

    requirements = {
        {item = "copper_ore", count = 2, consume = true},
        {item = "coal", count = 1, consume = true},
    },

    outputs = {
        {item = "copper_ingot", count = 1},
    },

    crafting = {
        time = 8.0,
        station = "furnace",
        skill_required = {smithing = 1},
    },
}

-- Gold Ingot from Gold Ore
register_recipe {
    id = "smelt_gold",
    name = "Smelt Gold Ingot",
    description = "Smelt gold ore into gold ingots",
    category = "processing",

    requirements = {
        {item = "gold_ore", count = 2, consume = true},
        {item = "coal", count = 1, consume = true},
    },

    outputs = {
        {item = "gold_ingot", count = 1},
    },

    crafting = {
        time = 12.0,
        station = "furnace",
        skill_required = {smithing = 2},
    },
}

-- Wood Processing Recipes

-- Planks from Wood
register_recipe {
    id = "make_planks",
    name = "Cut Wood Planks",
    description = "Process logs into wooden planks",
    category = "processing",

    requirements = {
        {item = "wood", count = 1, consume = true},
    },

    outputs = {
        {item = "plank", count = 4},
    },

    crafting = {
        time = 2.0,
        station = "sawmill", -- Or can use saw tool
        skill_required = {crafting = 1},
    },
}

-- Charcoal from Firewood
register_recipe {
    id = "make_charcoal",
    name = "Create Charcoal",
    description = "Convert firewood into charcoal through controlled burning",
    category = "processing",

    requirements = {
        {item = "firewood", count = 3, consume = true},
    },

    outputs = {
        {item = "charcoal", count = 1},
    },

    crafting = {
        time = 15.0,
        station = "kiln",
        skill_required = {crafting = 1},
    },
}

-- Material Processing Recipes

-- Glass from Sand
register_recipe {
    id = "make_glass",
    name = "Make Glass",
    description = "Melt sand into glass",
    category = "processing",

    requirements = {
        {item = "sand", count = 3, consume = true},
        {item = "coal", count = 1, consume = true},
    },

    outputs = {
        {item = "glass", count = 1},
    },

    crafting = {
        time = 8.0,
        station = "furnace",
        skill_required = {crafting = 2},
    },
}

-- Brick from Clay
register_recipe {
    id = "make_brick",
    name = "Make Brick",
    description = "Fire clay into bricks",
    category = "processing",

    requirements = {
        {item = "clay", count = 2, consume = true},
        {item = "coal", count = 1, consume = true},
    },

    outputs = {
        {item = "brick", count = 1},
    },

    crafting = {
        time = 10.0,
        station = "kiln",
        skill_required = {crafting = 2},
    },
}

-- Advanced Processing Recipes

-- Steel Ingot (requires iron + coal)
register_recipe {
    id = "make_steel",
    name = "Create Steel Ingot",
    description = "Create steel by combining iron and carbon",
    category = "processing",

    requirements = {
        {item = "iron_ingot", count = 2, consume = true},
        {item = "coal", count = 2, consume = true},
    },

    outputs = {
        {item = "steel_ingot", count = 1},
    },

    crafting = {
        time = 15.0,
        station = "furnace",
        skill_required = {smithing = 3},
        unlock_condition = {technology = "metallurgy"},
    },
}

-- Gem Cutting
register_recipe {
    id = "cut_gem",
    name = "Cut Gem",
    description = "Cut rough gems into finished gemstones",
    category = "processing",

    requirements = {
        {item = "rough_gem", count = 1, consume = true},
    },

    outputs = {
        {item = "gem", count = 1},
    },

    crafting = {
        time = 5.0,
        station = "jeweler_bench",
        skill_required = {jeweling = 2},
    },
}