-- Peasant Unit Definition
-- Basic worker unit that performs various tasks

register_entity {
    id = "peasant",
    name = "Peasant",
    type = "unit",
    description = "A hardworking villager capable of basic tasks",

    -- Common entity properties
    properties = {
        health = 100.0,
        max_health = 100.0,
        size = {x = 1, y = 1},  -- Takes 1x1 tile
    },

    -- Unit-specific configuration
    unit = {
        movement_speed = 1.0,  -- Base movement speed
        energy = 100.0,
        max_energy = 100.0,

        -- Needs that decay over time
        needs = {
            hunger_decay = 0.1,  -- Loses 0.1 hunger per tick
            energy_decay = 0.05,  -- Loses 0.05 energy per tick
            thirst_decay = 0.08,  -- Future: Add water system
        },

        -- Inventory configuration
        inventory = {
            slots = 10,  -- Can carry 10 different item stacks
            starting_items = {
                -- Start with nothing - must forage to survive
            },
        },

        -- AI behaviors this unit can perform
        behaviors = {
            "wander",
            "forage",
            "gather_resources",
            "store_items",
            "eat_food",
            "sleep",
            "build",
            "craft",
        },

        -- Work efficiency
        work_speed = 1.0,  -- Base work speed multiplier

        -- Skills (for future skill system)
        skills = {
            mining = 1,
            woodcutting = 1,
            farming = 1,
            building = 1,
            crafting = 1,
        },
    },

    -- Spawning configuration
    spawn = {
        initial_count = 5,  -- Start with 5 peasants
        spawn_area = {
            min_x = 20,
            max_x = 44,
            min_y = 20,
            max_y = 44,
        },
        require_walkable = true,  -- Must spawn on walkable tiles
    },

    -- Visual properties (for future rendering)
    visuals = {
        sprite = "peasant",
        animation_set = "humanoid",
        color_variation = true,  -- Slight color variations
    },

    -- Tags for filtering and AI decisions
    tags = {"unit", "worker", "human", "civilian"},
}