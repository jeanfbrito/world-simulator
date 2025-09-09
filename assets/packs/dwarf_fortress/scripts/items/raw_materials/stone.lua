-- Stone: Sturdy building material
local constants = require("items._constants")
local common = require("items._common")

local stone = {
    id = "stone",
    name = "Stone",
    category = constants.categories.RAW_MATERIAL,
    stack_size = 50,
    weight = 5,
    base_value = 2,
    decay_rate = 0,
    
    -- Harvesting properties
    harvestable_from = {"stone_deposit", "quarry"},
    harvest_time = 15,
    harvest_yield = {min = 1, max = 3},
    tool_required = "pickaxe",
    tool_bonus = {
        pickaxe = 1.0,
        advanced_pickaxe = 1.5
    },
    
    -- Seasonal modifiers
    seasons = {
        spring = 1.0,
        summer = 1.0,
        autumn = 1.0,
        winter = 0.7  -- Harder to mine in winter
    },
    
    description = "Sturdy building material mined from deposits"
}

return common.validate_item(stone)