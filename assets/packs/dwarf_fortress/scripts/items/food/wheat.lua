-- Wheat: Golden grain for bread
local constants = require("items._constants")
local common = require("items._common")

local wheat = {
    id = "wheat",
    name = "Wheat",
    category = constants.categories.FOOD,
    stack_size = 100,
    weight = 1,
    base_value = 3,
    
    -- Decay properties
    decay_rate = 0.01,  -- Stores well
    decay_time = 200,
    
    -- Harvesting properties
    harvestable_from = {"farm"},
    harvest_time = 8,
    harvest_yield = {min = 5, max = 15},
    tool_required = "scythe",
    tool_bonus = {
        scythe = 1.0,
        advanced_scythe = 1.5
    },
    
    -- Farming properties
    growth_time = 100,  -- Time to grow from planting
    requires_tilling = true,
    requires_water = true,
    
    -- Seasonal modifiers (wheat is a specific season crop)
    seasons = {
        spring = 0,    -- Planting season, no harvest
        summer = 0.5,  -- Early harvest possible
        autumn = 2.0,  -- Main harvest season
        winter = 0     -- No harvest
    },
    
    -- Processing info
    mills_to = "flour",
    brewable = true,
    brews_to = "beer",
    
    description = "Golden grain that can be milled into flour or brewed into beer"
}

return common.validate_item(wheat)