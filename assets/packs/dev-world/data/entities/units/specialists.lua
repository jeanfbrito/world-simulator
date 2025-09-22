-- Specialist Unit Definitions
-- Units with specialized skills and abilities

-- Blacksmith
register_entity {
    id = "blacksmith",
    name = "Blacksmith",
    type = "unit",
    description = "A skilled craftsman specializing in metalworking",

    properties = {
        health = 110.0,
        max_health = 110.0,
        size = {x = 1, y = 1},
    },

    unit = {
        movement_speed = 0.9,
        energy = 100.0,
        max_energy = 100.0,

        needs = {
            hunger_decay = 0.1,
            energy_decay = 0.05,
            morale_decay = 0.03,
        },

        inventory = {
            slots = 12,
            starting_items = {
                {item = "hammer", count = 1},
                {item = "tongs", count = 1},
                {item = "iron_ingot", count = 5},
                {item = "bread", count = 3},
            },
        },

        behaviors = {
            "wander",
            "forge",
            "repair_tools",
            "craft_weapons",
            "craft_armor",
            "eat_food",
            "sleep",
        },

        work_speed = 1.5, -- Very efficient at smithing tasks

        skills = {
            smithing = 4,
            crafting = 3,
            mining = 2,
            repair = 3,
        },

        -- Crafting bonuses
        crafting = {
            quality_bonus = 0.3,
            speed_bonus = 0.5,
            durability_bonus = 0.4,
            unlock_advanced_recipes = true,
        },

        specialist = {
            profession = "blacksmith",
            workshop_required = "workshop",
            tool_efficiency = 1.3,
            resource_efficiency = 0.8, -- Uses less materials
        },
    },

    spawn = {
        initial_count = 1, -- Start with one blacksmith
        spawn_area = {
            min_x = 25,
            max_x = 35,
            min_y = 25,
            max_y = 35,
        },
        require_walkable = true,
    },

    visuals = {
        sprite = "blacksmith",
        animation_set = "humanoid",
        equipment = {"apron", "hammer"},
        color_variation = true,
    },

    tags = {"unit", "specialist", "craftsman", "human"},
}

-- Farmer
register_entity {
    id = "farmer",
    name = "Farmer",
    type = "unit",
    description = "A specialized worker for agricultural tasks",

    properties = {
        health = 100.0,
        max_health = 100.0,
        size = {x = 1, y = 1},
    },

    unit = {
        movement_speed = 1.0,
        energy = 105.0,
        max_energy = 105.0,

        needs = {
            hunger_decay = 0.12,
            energy_decay = 0.06,
            morale_decay = 0.04,
        },

        inventory = {
            slots = 15,
            starting_items = {
                {item = "shovel", count = 1},
                {item = "seeds", count = 10},
                {item = "bread", count = 4},
            },
        },

        behaviors = {
            "wander",
            "farm",
            "plant_crops",
            "harvest",
            "tend_animals",
            "eat_food",
            "sleep",
        },

        work_speed = 1.4, -- Very efficient at farming tasks

        skills = {
            farming = 4,
            animal_husbandry = 2,
            foraging = 2,
            cooking = 1,
        },

        farming = {
            crop_yield_bonus = 0.4,
            growth_speed_bonus = 0.3,
            plant_health_bonus = 0.5,
            harvest_efficiency = 0.8,
        },

        specialist = {
            profession = "farmer",
            preferred_workplace = "farm",
            tool_efficiency = 1.2,
            weather_resistance = 0.3,
        },
    },

    spawn = {
        initial_count = 2, -- Start with 2 farmers
        spawn_area = {
            min_x = 20,
            max_x = 40,
            min_y = 20,
            max_y = 40,
        },
        require_walkable = true,
        require_fertile_ground = true,
    },

    visuals = {
        sprite = "farmer",
        animation_set = "humanoid",
        equipment = {"hat", "tools"},
        color_variation = true,
    },

    tags = {"unit", "specialist", "farmer", "human"},
}

-- Merchant
register_entity {
    id = "merchant",
    name = "Merchant",
    type = "unit",
    description = "A trader who handles commerce and trade",

    properties = {
        health = 90.0,
        max_health = 90.0,
        size = {x = 1, y = 1},
    },

    unit = {
        movement_speed = 1.1,
        energy = 95.0,
        max_energy = 95.0,

        needs = {
            hunger_decay = 0.1,
            energy_decay = 0.05,
            morale_decay = 0.02,
        },

        inventory = {
            slots = 20, -- Large inventory for trading
            starting_items = {
                {item = "gold_coins", count = 50},
                {item = "assorted_goods", count = 10},
                {item = "bread", count = 3},
            },
        },

        behaviors = {
            "wander",
            "trade",
            "negotiate",
            "appraise_items",
            "manage_shop",
            "eat_food",
            "sleep",
        },

        work_speed = 1.0,

        skills = {
            trading = 4,
            appraisal = 3,
            diplomacy = 2,
            logistics = 2,
        },

        trading = {
            price_bonus = 0.2,
            trade_range_bonus = 0.5,
            reputation_influence = 0.3,
            unlock_special_trades = true,
        },

        specialist = {
            profession = "merchant",
            preferred_workplace = "marketplace",
            charisma = 1.3,
            negotiation_skill = 1.4,
        },
    },

    spawn = {
        initial_count = 1, -- Start with one merchant
        spawn_area = {
            min_x = 30,
            max_x = 34,
            min_y = 30,
            max_y = 34,
        },
        require_walkable = true,
    },

    visuals = {
        sprite = "merchant",
        animation_set = "humanoid",
        equipment = {"fine_clothes", "bag"},
        color_variation = true,
    },

    tags = {"unit", "specialist", "trader", "human"},
}