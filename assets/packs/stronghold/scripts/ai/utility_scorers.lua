-- Utility AI Scorers for Stronghold Pack
-- Evaluates needs and triggers reactive behaviors

return {
    hunger_scorer = {
        component = "UnitNeeds",
        field = "hunger",
        curve = "quadratic",          -- Urgency increases exponentially with hunger
        threshold = 0.6,
        priority = 90,
        description = "Triggers eating behavior when hungry"
    },
    
    fatigue_scorer = {
        component = "UnitNeeds",
        field = "energy",
        curve = "inverse_quadratic",  -- Low energy = high score (exponentially)
        threshold = 0.3,
        priority = 85,
        description = "Triggers rest/sleep when tired"
    },
    
    danger_scorer = {
        component = "ThreatDetection",
        field = "threat_level",
        curve = "linear",
        threshold = 0.5,
        priority = 100,              -- Highest priority for survival
        description = "Triggers flee/fight response"
    },
    
    work_scorer = {
        component = "UnitInventory",
        evaluation = "custom",
        script = [[
            if inventory:is_full() then return 0.0 end
            if needs.hunger > 0.7 then return 0.0 end
            if needs.energy < 0.3 then return 0.0 end
            return 0.7
        ]],
        threshold = 0.5,
        priority = 40,
        description = "Evaluates if unit should work"
    },
    
    social_scorer = {
        component = "UnitNeeds",
        field = "morale",
        curve = "inverse_linear",    -- Low morale = need social interaction
        threshold = 0.4,
        priority = 30,
        description = "Triggers social activities when morale is low"
    },
    
    storage_scorer = {
        component = "UnitInventory",
        evaluation = "custom",
        script = [[
            local weight_ratio = inventory.current_weight / inventory.max_weight
            if weight_ratio > 0.8 then return 0.9
            elseif weight_ratio > 0.6 then return 0.5
            else return 0.0
            end
        ]],
        threshold = 0.5,
        priority = 50,
        description = "Triggers storage behavior when inventory is full"
    },
    
    shelter_scorer = {
        component = "UnitOwnership",
        evaluation = "custom",
        script = [[
            if ownership:has_house() then return 0.0 end
            if needs.energy < 0.5 and not ownership:has_house() then return 0.8 end
            return 0.3
        ]],
        threshold = 0.3,
        priority = 60,
        description = "Triggers house building when homeless"
    },
    
    temperature_scorer = {
        component = "EnvironmentSensor",
        field = "temperature_discomfort",
        curve = "sigmoid",
        threshold = 0.6,
        priority = 70,
        description = "Seeks shelter in extreme temperatures"
    }
}