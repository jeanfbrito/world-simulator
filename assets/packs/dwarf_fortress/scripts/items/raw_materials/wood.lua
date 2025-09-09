-- Wood: Basic construction material
local constants = require("items._constants")
local common = require("items._common")

local wood = {
    id = "wood",
    name = "Wood",
    category = constants.categories.RAW_MATERIAL,
    stack_size = 100,
    weight = 2,
    base_value = 1,
    decay_rate = 0,  -- Doesn't decay
    
    -- Harvesting properties
    harvestable_from = {"tree"},
    harvest_time = 10,
    harvest_yield = {min = 2, max = 5},
    tool_required = nil,  -- Can harvest by hand
    tool_bonus = {axe = 1.5},  -- But better with tools
    
    -- Seasonal modifiers
    seasons = {
        spring = 1.0,
        summer = 1.2,
        autumn = 1.1,
        winter = 0.8
    },
    
    description = "Basic construction material harvested from trees"
}

return common.validate_item(wood)