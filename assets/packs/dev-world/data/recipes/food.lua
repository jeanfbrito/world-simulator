-- Food Processing Recipes
-- Recipes for cooking and food preparation

-- Basic Cooking Recipes

-- Cook Fish
register_recipe {
    id = "cook_fish",
    name = "Cook Fish",
    description = "Cook raw fish over a fire",
    category = "food",

    requirements = {
        {item = "fish", count = 1, consume = true},
        {item = "firewood", count = 1, consume = true}, -- Fuel
    },

    outputs = {
        {item = "cooked_fish", count = 1},
    },

    crafting = {
        time = 3.0,
        station = "campfire",
        skill_required = {cooking = 1},
    },
}

-- Cook Meat
register_recipe {
    id = "cook_meat",
    name = "Cook Meat",
    description = "Cook raw meat over a fire",
    category = "food",

    requirements = {
        {item = "meat", count = 1, consume = true},
        {item = "firewood", count = 1, consume = true},
    },

    outputs = {
        {item = "cooked_meat", count = 1},
    },

    crafting = {
        time = 4.0,
        station = "campfire",
        skill_required = {cooking = 1},
    },
}

-- Bread Making

-- Make Dough
register_recipe {
    id = "make_dough",
    name = "Make Dough",
    description = "Mix flour and water to create dough",
    category = "food",

    requirements = {
        {item = "flour", count = 2, consume = true},
        {item = "water", count = 1, consume = true},
    },

    outputs = {
        {item = "dough", count = 1},
    },

    crafting = {
        time = 2.0,
        station = "kitchen",
        skill_required = {cooking = 1},
    },
}

-- Bake Bread
register_recipe {
    id = "bake_bread",
    name = "Bake Bread",
    description = "Bake dough into fresh bread",
    category = "food",

    requirements = {
        {item = "dough", count = 1, consume = true},
        {item = "firewood", count = 1, consume = true},
    },

    outputs = {
        {item = "bread", count = 1},
    },

    crafting = {
        time = 8.0,
        station = "oven",
        skill_required = {cooking = 2},
    },
}

-- Grain Processing

-- Mill Wheat into Flour
register_recipe {
    id = "mill_flour",
    name = "Mill Flour",
    description = "Grind wheat into flour",
    category = "food",

    requirements = {
        {item = "wheat", count = 2, consume = true},
    },

    outputs = {
        {item = "flour", count = 1},
    },

    crafting = {
        time = 2.0,
        station = "mill",
        skill_required = {cooking = 1},
    },
}

-- Mill Corn into Cornmeal
register_recipe {
    id = "mill_cornmeal",
    name = "Mill Cornmeal",
    description = "Grind corn into cornmeal",
    category = "food",

    requirements = {
        {item = "corn", count = 2, consume = true},
    },

    outputs = {
        {item = "cornmeal", count = 1},
    },

    crafting = {
        time = 2.0,
        station = "mill",
        skill_required = {cooking = 1},
    },
}

-- Advanced Food Recipes

-- Make Soup
register_recipe {
    id = "make_soup",
    name = "Make Soup",
    description = "Create a nutritious soup from vegetables and meat",
    category = "food",

    requirements = {
        {item = "cooked_meat", count = 1, consume = true},
        {item = "vegetables", count = 2, consume = true},
        {item = "water", count = 1, consume = true},
    },

    outputs = {
        {item = "soup", count = 2},
    },

    crafting = {
        time = 6.0,
        station = "kitchen",
        skill_required = {cooking = 2},
    },
}

-- Make Stew
register_recipe {
    id = "make_stew",
    name = "Make Stew",
    description = "Create a hearty stew with meat and vegetables",
    category = "food",

    requirements = {
        {item = "cooked_meat", count = 2, consume = true},
        {item = "vegetables", count = 3, consume = true},
        {item = "water", count = 1, consume = true},
    },

    outputs = {
        {item = "stew", count = 2},
    },

    crafting = {
        time = 10.0,
        station = "kitchen",
        skill_required = {cooking = 3},
    },
}

-- Baking Recipes

-- Bake Pie
register_recipe {
    id = "bake_pie",
    name = "Bake Pie",
    description = "Bake a delicious fruit pie",
    category = "food",

    requirements = {
        {item = "flour", count = 2, consume = true},
        {item = "fruit", count = 3, consume = true},
        {item = "water", count = 1, consume = true},
    },

    outputs = {
        {item = "pie", count = 1},
    },

    crafting = {
        time = 12.0,
        station = "oven",
        skill_required = {cooking = 3},
    },
}

-- Preserve Food (for long-term storage)
register_recipe {
    id = "preserve_food",
    name = "Preserve Food",
    description = "Preserve food for long-term storage",
    category = "food",

    requirements = {
        {item = "cooked_meat", count = 3, consume = true},
        {item = "salt", count = 1, consume = true},
    },

    outputs = {
        {item = "preserved_meat", count = 3},
    },

    crafting = {
        time = 15.0,
        station = "smokehouse",
        skill_required = {cooking = 2},
    },
}