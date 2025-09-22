-- Building Construction Recipes
-- Recipes for constructing buildings and structures

-- Basic Structures

-- Campfire
register_recipe {
    id = "build_campfire",
    name = "Build Campfire",
    description = "Construct a basic campfire for cooking and warmth",
    category = "building",

    requirements = {
        {item = "stone", count = 4, consume = true},
        {item = "firewood", count = 2, consume = true},
    },

    outputs = {
        {item = "campfire", count = 1},
    },

    crafting = {
        time = 2.0,
        station = nil, -- Can build anywhere
        skill_required = {construction = 1},
    },
}

-- Workbench
register_recipe {
    id = "build_workbench",
    name = "Build Workbench",
    description = "Construct a workbench for crafting basic items",
    category = "building",

    requirements = {
        {item = "wood", count = 4, consume = true},
    },

    outputs = {
        {item = "workbench", count = 1},
    },

    crafting = {
        time = 4.0,
        station = nil,
        skill_required = {construction = 1},
    },
}

-- Processing Structures

-- Furnace
register_recipe {
    id = "build_furnace",
    name = "Build Furnace",
    description = "Construct a furnace for smelting metals",
    category = "building",

    requirements = {
        {item = "stone", count = 8, consume = true},
        {item = "brick", count = 4, consume = true},
    },

    outputs = {
        {item = "furnace", count = 1},
    },

    crafting = {
        time = 10.0,
        station = "workbench",
        skill_required = {construction = 2, smithing = 1},
    },
}

-- Kiln
register_recipe {
    id = "build_kiln",
    name = "Build Kiln",
    description = "Construct a kiln for firing pottery and charcoal",
    category = "building",

    requirements = {
        {item = "brick", count = 6, consume = true},
        {item = "stone", count = 4, consume = true},
    },

    outputs = {
        {item = "kiln", count = 1},
    },

    crafting = {
        time = 8.0,
        station = "workbench",
        skill_required = {construction = 2, crafting = 1},
    },
}

-- Anvil
register_recipe {
    id = "build_anvil",
    name = "Build Anvil",
    description = "Construct an anvil for advanced metalworking",
    category = "building",

    requirements = {
        {item = "iron_ingot", count = 5, consume = true},
        {item = "stone", count = 2, consume = true},
    },

    outputs = {
        {item = "anvil", count = 1},
    },

    crafting = {
        time = 6.0,
        station = "workbench",
        skill_required = {construction = 2, smithing = 1},
    },
}

-- Food Processing Structures

-- Mill
register_recipe {
    id = "build_mill",
    name = "Build Mill",
    description = "Construct a mill for grinding grain into flour",
    category = "building",

    requirements = {
        {item = "wood", count = 10, consume = true},
        {item = "stone", count = 5, consume = true},
    },

    outputs = {
        {item = "mill", count = 1},
    },

    crafting = {
        time = 12.0,
        station = "workbench",
        skill_required = {construction = 2},
    },
}

-- Kitchen
register_recipe {
    id = "build_kitchen",
    name = "Build Kitchen",
    description = "Construct a kitchen for advanced food preparation",
    category = "building",

    requirements = {
        {item = "wood", count = 8, consume = true},
        {item = "stone", count = 4, consume = true},
        {item = "iron_ingot", count = 2, consume = true},
    },

    outputs = {
        {item = "kitchen", count = 1},
    },

    crafting = {
        time = 10.0,
        station = "workbench",
        skill_required = {construction = 2},
    },
}

-- Advanced Structures

-- Sawmill
register_recipe {
    id = "build_sawmill",
    name = "Build Sawmill",
    description = "Construct a sawmill for efficient wood processing",
    category = "building",

    requirements = {
        {item = "wood", count = 12, consume = true},
        {item = "iron_ingot", count = 3, consume = true},
    },

    outputs = {
        {item = "sawmill", count = 1},
    },

    crafting = {
        time = 15.0,
        station = "workbench",
        skill_required = {construction = 3},
    },
}

-- Storage Structures

-- Storage Chest
register_recipe {
    id = "build_chest",
    name = "Build Storage Chest",
    description = "Construct a chest for storing items",
    category = "building",

    requirements = {
        {item = "wood", count = 6, consume = true},
        {item = "iron_ingot", count = 1, consume = true},
    },

    outputs = {
        {item = "storage_chest", count = 1},
    },

    crafting = {
        time = 5.0,
        station = "workbench",
        skill_required = {construction = 1},
    },
}

-- Silo (for grain storage)
register_recipe {
    id = "build_silo",
    name = "Build Silo",
    description = "Construct a silo for bulk grain storage",
    category = "building",

    requirements = {
        {item = "wood", count = 15, consume = true},
        {item = "stone", count = 10, consume = true},
    },

    outputs = {
        {item = "silo", count = 1},
    },

    crafting = {
        time = 20.0,
        station = "workbench",
        skill_required = {construction = 2},
    },
}