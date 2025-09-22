-- Energy Fuel Resources
-- Combustible materials used for fuel

-- Firewood
register_resource {
    id = "firewood",
    name = "Firewood",
    description = "Dry wood pieces used for fires",
    category = "energy",

    properties = {
        weight = 0.6,
        stack_size = 40,
        base_value = 3,
    },
}

-- Charcoal
register_resource {
    id = "charcoal",
    name = "Charcoal",
    description = "Carbon-rich fuel made from burned wood",
    category = "energy",

    properties = {
        weight = 0.6,
        stack_size = 40,
        base_value = 8,
    },
}

-- Oil
register_resource {
    id = "oil",
    name = "Oil",
    description = "Liquid petroleum used for fuel and lubrication",
    category = "energy",

    properties = {
        weight = 0.4,
        stack_size = 40,
        base_value = 15,
    },
}