-- Berries: Sweet wild berries
local constants = require("items._constants")
local common = require("items._common")

local berries = {
    id = "berries",
    name = "Berries",
    category = constants.categories.FOOD,
    stack_size = 50,
    weight = 0.5,
    base_value = 2,
    
    -- Decay properties
    decay_rate = 0.05,  -- Spoils over time
    decay_time = 50,    -- Ticks until spoiled
    
    -- Nutrition properties
    nutrition = 10,
    morale_bonus = 2,
    consumable = true,
    
    -- Harvesting properties
    harvestable_from = {"berry_bush"},
    harvest_time = 5,
    harvest_yield = {min = 3, max = 8},
    tool_required = nil,  -- Hand-picked
    
    -- Seasonal modifiers (berries are very seasonal)
    seasons = {
        spring = 0.5,
        summer = 1.5,  -- Peak season
        autumn = 1.0,
        winter = 0     -- No berries in winter
    },
    
    description = "Sweet wild berries that provide quick nutrition"
}

return common.validate_item(berries)