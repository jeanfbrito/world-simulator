-- Planks: Processed wood for construction
local constants = require("items._constants")
local common = require("items._common")

local planks = {
    id = "planks",
    name = "Planks",
    category = constants.categories.PROCESSED,
    stack_size = 50,
    weight = 1.5,
    base_value = 3,
    decay_rate = 0,
    
    -- Quality affects value
    quality_affects_value = true,
    
    -- Crafting info
    crafted_at = "sawmill",
    requires_skill = "carpentry",
    ingredients = {
        wood = 1
    },
    output_amount = 4,  -- 1 wood makes 4 planks
    
    -- Construction uses
    construction_material = true,
    build_speed_modifier = 1.2,  -- Faster than raw wood
    
    description = "Processed wood planks ideal for construction"
}

return common.validate_item(planks)