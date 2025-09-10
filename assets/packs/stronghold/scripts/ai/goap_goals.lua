-- Goal-Oriented Action Planning Goals for Stronghold Pack
-- Defines goals that units will try to achieve through planning

return {
    survive = {
        priority = 100,
        description = "Basic survival needs",
        conditions = {
            is_hungry = { type = "Float", value = 0.0 },
            has_energy = { type = "Float", value = 0.5 },
            has_shelter = { type = "Bool", value = true }
        }
    },
    
    gather_resources = {
        priority = 50,
        description = "Collect resources for settlement",
        conditions = {
            settlement_wood = { type = "Int", value = 50 },
            settlement_stone = { type = "Int", value = 25 },
            settlement_food = { type = "Int", value = 30 }
        }
    },
    
    build_settlement = {
        priority = 60,
        description = "Construct essential buildings",
        conditions = {
            has_house = { type = "Bool", value = true },
            storage_available = { type = "Bool", value = true },
            workshop_available = { type = "Bool", value = true }
        }
    },
    
    improve_efficiency = {
        priority = 30,
        description = "Craft tools and improve workflow",
        conditions = {
            has_tool = { type = "Bool", value = true },
            work_efficiency = { type = "Float", value = 0.5 }
        }
    },
    
    maintain_morale = {
        priority = 40,
        description = "Keep spirits high",
        conditions = {
            morale = { type = "Float", value = 0.7 },
            has_entertainment = { type = "Bool", value = true }
        }
    },
    
    emergency_food = {
        priority = 90,
        description = "Find food when starving",
        active_when = { is_hungry = { type = "Float", value = 0.8 } },  -- Only active when very hungry
        conditions = {
            has_food = { type = "Int", value = 5 }
        }
    },
    
    emergency_rest = {
        priority = 95,
        description = "Rest when exhausted",
        active_when = { has_energy = { type = "Float", value = 0.2 } },  -- Only active when very tired
        conditions = {
            has_energy = { type = "Float", value = 0.8 }
        }
    }
}