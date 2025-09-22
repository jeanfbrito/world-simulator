-- Military Unit Definitions
-- Combat units for defense and warfare

-- Soldier
register_entity {
    id = "soldier",
    name = "Soldier",
    type = "unit",
    description = "A trained military unit for combat",

    properties = {
        health = 150.0,
        max_health = 150.0,
        size = {x = 1, y = 1},
    },

    unit = {
        movement_speed = 1.2,
        energy = 120.0,
        max_energy = 120.0,

        needs = {
            hunger_decay = 0.12,
            energy_decay = 0.06,
            morale_decay = 0.05,
        },

        inventory = {
            slots = 8,
            starting_items = {
                {item = "iron_sword", count = 1},
                {item = "bread", count = 3},
            },
        },

        behaviors = {
            "patrol",
            "guard",
            "attack",
            "defend",
            "follow_orders",
            "formation_movement",
        },

        work_speed = 0.8, -- Less efficient at civilian tasks

        skills = {
            combat = 3,
            defense = 2,
            strength = 2,
            mining = 1,
            woodcutting = 1,
        },

        -- Combat-specific properties
        combat = {
            attack_power = 25,
            defense_value = 20,
            attack_range = 1.0,
            attack_speed = 1.0,
            accuracy = 0.8,
            morale = 1.0,
        },

        -- Military-specific properties
        military = {
            rank = "private",
            experience = 0,
            training_level = 2,
            equipment_quality = 1.0,
        },
    },

    spawn = {
        initial_count = 0, -- Trained at barracks
        spawn_area = "military",
        require_walkable = true,
    },

    visuals = {
        sprite = "soldier",
        animation_set = "humanoid",
        equipment = {"armor", "weapon"},
        color_variation = true,
    },

    tags = {"unit", "military", "combat", "human"},
}

-- Archer
register_entity {
    id = "archer",
    name = "Archer",
    type = "unit",
    description = "A ranged combat unit specializing in bows",

    properties = {
        health = 120.0,
        max_health = 120.0,
        size = {x = 1, y = 1},
    },

    unit = {
        movement_speed = 1.3,
        energy = 110.0,
        max_energy = 110.0,

        needs = {
            hunger_decay = 0.11,
            energy_decay = 0.05,
            morale_decay = 0.04,
        },

        inventory = {
            slots = 10,
            starting_items = {
                {item = "bow", count = 1},
                {item = "arrows", count = 20},
                {item = "dagger", count = 1},
                {item = "bread", count = 2},
            },
        },

        behaviors = {
            "patrol",
            "guard",
            "ranged_attack",
            "defend",
            "follow_orders",
            "take_cover",
        },

        work_speed = 0.9,

        skills = {
            ranged_combat = 4,
            accuracy = 3,
            stealth = 2,
            mining = 1,
        },

        combat = {
            attack_power = 20,
            defense_value = 12,
            attack_range = 8.0,
            attack_speed = 1.2,
            accuracy = 0.85,
            morale = 0.9,
        },

        military = {
            rank = "private",
            experience = 0,
            training_level = 2,
            equipment_quality = 0.9,
        },
    },

    spawn = {
        initial_count = 0,
        spawn_area = "military",
        require_walkable = true,
    },

    visuals = {
        sprite = "archer",
        animation_set = "humanoid",
        equipment = {"bow", "light_armor"},
        color_variation = true,
    },

    tags = {"unit", "military", "ranged", "human"},
}

-- Knight
register_entity {
    id = "knight",
    name = "Knight",
    type = "unit",
    description = "A heavily armored elite military unit",

    properties = {
        health = 250.0,
        max_health = 250.0,
        size = {x = 1, y = 1},
    },

    unit = {
        movement_speed = 1.0, -- Slower due to heavy armor
        energy = 150.0,
        max_energy = 150.0,

        needs = {
            hunger_decay = 0.15,
            energy_decay = 0.08,
            morale_decay = 0.03,
        },

        inventory = {
            slots = 6,
            starting_items = {
                {item = "iron_sword", count = 1},
                {item = "shield", count = 1},
                {item = "helmet", count = 1},
                {item = "chestplate", count = 1},
                {item = "bread", count = 5},
            },
        },

        behaviors = {
            "patrol",
            "guard",
            "charge",
            "defend",
            "follow_orders",
            "formation_breakthrough",
        },

        work_speed = 0.6, -- Very inefficient at civilian tasks

        skills = {
            combat = 5,
            defense = 4,
            strength = 4,
            leadership = 2,
        },

        combat = {
            attack_power = 40,
            defense_value = 35,
            attack_range = 1.0,
            attack_speed = 0.8,
            accuracy = 0.9,
            morale = 1.2,
            charge_bonus = 1.5,
        },

        military = {
            rank = "knight",
            experience = 50,
            training_level = 4,
            equipment_quality = 1.5,
        },
    },

    spawn = {
        initial_count = 0,
        spawn_area = "military",
        require_walkable = true,
    },

    visuals = {
        sprite = "knight",
        animation_set = "humanoid",
        equipment = {"heavy_armor", "sword", "shield"},
        color_variation = true,
    },

    tags = {"unit", "military", "elite", "heavy", "human"},
}