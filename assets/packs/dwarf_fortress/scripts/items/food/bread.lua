-- Bread: Freshly baked bread
local constants = require("items._constants")
local common = require("items._common")

local bread = {
    id = "bread",
    name = "Bread",
    category = constants.categories.FOOD,
    stack_size = 30,
    weight = 0.8,
    base_value = 6,
    
    -- Decay properties
    decay_rate = 0.02,
    decay_time = 150,
    
    -- Nutrition properties
    nutrition = 30,
    morale_bonus = 8,
    consumable = true,
    
    -- Quality affects nutrition
    quality_affects_nutrition = true,
    quality_affects_value = true,
    
    -- Crafting info
    crafted_at = "bakery",
    requires_skill = "baking",
    ingredients = {
        flour = 2,
        water = 1
    },
    requires_heat = true,  -- Needs oven
    
    description = "Freshly baked bread that provides substantial nutrition"
}

return common.validate_item(bread)