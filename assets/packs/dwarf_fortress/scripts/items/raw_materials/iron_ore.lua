-- Iron Ore: Raw metal for tools and weapons
local constants = require("items._constants")
local common = require("items._common")

local iron_ore = {
    id = "iron_ore",
    name = "Iron Ore",
    category = constants.categories.RAW_MATERIAL,
    stack_size = 30,
    weight = 8,
    base_value = 5,
    decay_rate = 0,
    
    -- Harvesting properties
    harvestable_from = {"iron_deposit", "mine"},
    harvest_time = 20,
    harvest_yield = {min = 1, max = 2},
    tool_required = "pickaxe",
    tool_bonus = {
        pickaxe = 1.0,
        advanced_pickaxe = 1.8
    },
    
    -- Seasonal modifiers
    seasons = {
        spring = 1.0,
        summer = 1.0,
        autumn = 1.0,
        winter = 0.5  -- Much harder in winter
    },
    
    -- Processing info
    smelts_to = "iron_ingot",
    smelt_temperature = 1538,
    
    description = "Raw metal ore that must be smelted into ingots"
}

return common.validate_item(iron_ore)