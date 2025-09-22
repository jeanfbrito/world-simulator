-- Peasant Unit
-- Basic worker unit capable of harvesting and building

register_entity {
    id = "peasant",
    name = "Peasant",
    entity_type = "unit",
    description = "A humble worker, the backbone of your settlement",
    
    properties = {
        health = 100.0,
        max_health = 100.0,
        size = { x = 1, y = 1 },
    },
    
    unit = {
        movement_speed = 50.0,
        energy = 100.0,
        max_energy = 100.0,
        needs = {
            hunger_decay = 0.1,
            energy_decay = 0.05,
        },
        inventory = {
            slots = 8,
            starting_items = {},
        },
        behaviors = { "gather", "build", "craft", "eat", "sleep" },
        work_speed = 1.0,
        skills = {
            harvesting = 1,
            building = 1,
            crafting = 1,
        },
    },
    
    spawn = {
        initial_count = 3,
        spawn_area = {
            min_x = 45,
            max_x = 55,
            min_y = 45,
            max_y = 55,
        },
        require_walkable = true,
    },
    
    visuals = {
        sprite = "peasant",
    },
    
    tags = { "unit", "worker", "peasant" }
}
