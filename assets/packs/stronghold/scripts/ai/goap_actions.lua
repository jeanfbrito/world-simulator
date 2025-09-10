-- GOAP Actions for Stronghold Pack
-- Defines all actions available for planning with preconditions and effects

return {
    -- Movement Actions
    move_to_resource = {
        cost = 1.0,
        description = "Move to nearest resource location",
        preconditions = {
            has_energy = { type = "Float", value = 0.2 }  -- Need at least 20% energy
        },
        effects = {
            at_resource = { type = "Bool", value = true },
            at_storage = { type = "Bool", value = false },
            at_home = { type = "Bool", value = false }
        }
    },
    
    move_to_storage = {
        cost = 1.0,
        description = "Move to storage building",
        preconditions = {
            has_energy = { type = "Float", value = 0.2 }
        },
        effects = {
            at_storage = { type = "Bool", value = true },
            at_resource = { type = "Bool", value = false },
            at_home = { type = "Bool", value = false }
        }
    },
    
    move_to_home = {
        cost = 1.0,
        description = "Move to home/shelter",
        preconditions = {
            has_energy = { type = "Float", value = 0.1 }  -- Can go home even when very tired
        },
        effects = {
            at_home = { type = "Bool", value = true },
            at_storage = { type = "Bool", value = false },
            at_resource = { type = "Bool", value = false }
        }
    },
    
    -- Resource Gathering Actions
    gather_wood = {
        cost = 3.0,
        description = "Chop wood from trees",
        preconditions = {
            at_resource = { type = "Bool", value = true },
            has_energy = { type = "Float", value = 0.3 },
            inventory_full = { type = "Bool", value = false }
        },
        effects = {
            has_wood = { type = "Int", value = 5 },
            has_energy = { type = "Float", value = -0.2 }  -- Costs energy
        }
    },
    
    gather_stone = {
        cost = 4.0,
        description = "Mine stone from rocks",
        preconditions = {
            at_resource = { type = "Bool", value = true },
            has_energy = { type = "Float", value = 0.4 },
            inventory_full = { type = "Bool", value = false }
        },
        effects = {
            has_stone = { type = "Int", value = 3 },
            has_energy = { type = "Float", value = -0.25 }
        }
    },
    
    gather_berries = {
        cost = 2.0,
        description = "Gather food from berry bushes",
        preconditions = {
            at_resource = { type = "Bool", value = true },
            has_energy = { type = "Float", value = 0.2 },
            inventory_full = { type = "Bool", value = false }
        },
        effects = {
            has_food = { type = "Int", value = 5 },
            has_energy = { type = "Float", value = -0.1 }
        }
    },
    
    -- Storage Actions
    store_resources = {
        cost = 1.0,
        description = "Store resources in stockpile",
        preconditions = {
            at_storage = { type = "Bool", value = true }
        },
        effects = {
            inventory_full = { type = "Bool", value = false },
            settlement_wood = { type = "Int", value = 5 },
            settlement_stone = { type = "Int", value = 3 }
        }
    },
    
    -- Consumption Actions
    eat_food = {
        cost = 0.5,
        description = "Eat food to reduce hunger",
        preconditions = {
            has_food = { type = "Int", value = 1 },
            is_hungry = { type = "Float", value = 0.3 }  -- Only eat when moderately hungry
        },
        effects = {
            is_hungry = { type = "Float", value = 0.0 },
            has_energy = { type = "Float", value = 0.2 },  -- Food gives energy boost
            has_food = { type = "Int", value = -1 }
        }
    },
    
    rest = {
        cost = 0.1,
        description = "Rest to restore energy",
        preconditions = {
            has_energy = { type = "Float", value = 0.5 }  -- Can rest when below 50% energy
        },
        effects = {
            has_energy = { type = "Float", value = 1.0 },
            is_working = { type = "Bool", value = false }
        }
    },
    
    sleep = {
        cost = 0.1,
        description = "Sleep at home to fully restore",
        preconditions = {
            at_home = { type = "Bool", value = true },
            has_energy = { type = "Float", value = 0.3 }
        },
        effects = {
            has_energy = { type = "Float", value = 1.0 },
            is_hungry = { type = "Float", value = -0.1 },  -- Slight hunger reduction
            morale = { type = "Float", value = 0.1 }  -- Morale boost from good sleep
        }
    },
    
    -- Building Actions
    build_house = {
        cost = 10.0,
        description = "Construct a basic house",
        preconditions = {
            has_wood = { type = "Int", value = 10 },
            has_stone = { type = "Int", value = 5 },
            has_energy = { type = "Float", value = 0.5 },
            at_building_site = { type = "Bool", value = true }
        },
        effects = {
            has_house = { type = "Bool", value = true },
            has_wood = { type = "Int", value = -10 },
            has_stone = { type = "Int", value = -5 },
            has_energy = { type = "Float", value = -0.3 },
            morale = { type = "Float", value = 0.2 }  -- Happy to have shelter
        }
    },
    
    build_stockpile = {
        cost = 5.0,
        description = "Construct storage area",
        preconditions = {
            has_wood = { type = "Int", value = 5 },
            has_energy = { type = "Float", value = 0.4 },
            at_building_site = { type = "Bool", value = true }
        },
        effects = {
            storage_available = { type = "Bool", value = true },
            has_wood = { type = "Int", value = -5 },
            has_energy = { type = "Float", value = -0.2 }
        }
    },
    
    -- Crafting Actions
    craft_tool = {
        cost = 3.0,
        description = "Craft basic tools",
        preconditions = {
            has_wood = { type = "Int", value = 2 },
            has_stone = { type = "Int", value = 1 },
            at_workshop = { type = "Bool", value = true }
        },
        effects = {
            has_tool = { type = "Bool", value = true },
            has_wood = { type = "Int", value = -2 },
            has_stone = { type = "Int", value = -1 },
            work_efficiency = { type = "Float", value = 0.2 }  -- Tools improve efficiency
        }
    }
}