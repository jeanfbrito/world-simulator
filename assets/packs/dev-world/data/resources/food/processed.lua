-- Processed Food Resources
-- Food items that require processing

-- Bread
register_resource {
    id = "bread",
    name = "Bread",
    description = "Baked wheat loaf, staple food",
    category = "food",

    properties = {
        weight = 0.3,
        stack_size = 20,
        base_value = 10,
    },
}

-- Fish (from fishing)
register_resource {
    id = "fish",
    name = "Fish",
    description = "Fresh fish caught from water bodies",
    category = "food",

    properties = {
        weight = 0.5,
        stack_size = 20,
        base_value = 5,
    },
}

-- Meat (from hunting)
register_resource {
    id = "meat",
    name = "Meat",
    description = "Raw meat from hunted animals",
    category = "food",

    properties = {
        weight = 0.5,
        stack_size = 20,
        base_value = 8,
    },
}