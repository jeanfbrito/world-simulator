-- Axe: Tool for chopping wood
local constants = require("items._constants")
local common = require("items._common")

local axe = {
    id = "axe",
    name = "Axe",
    category = constants.categories.TOOL,
    stack_size = 1,  -- Tools don't stack
    weight = 4,
    base_value = 25,
    decay_rate = 0,
    
    -- Tool properties
    tool_type = "axe",
    durability = 150,
    durability_loss_per_use = 0.5,
    
    -- Harvest bonuses
    harvest_bonus = {
        wood = 1.5,
        rare_wood = 1.3
    },
    
    -- Quality affects bonuses
    quality_affects_bonus = true,
    quality_affects_durability = true,
    quality_affects_value = true,
    
    -- Crafting info
    crafted_at = {"forge", "carpenter"},
    requires_skill = "toolmaking",
    materials = {
        metal_bar = 1,
        wood = 1
    },
    
    -- Can be made from different materials
    material_variants = {
        copper = {durability = 100, bonus = 1.2},
        bronze = {durability = 130, bonus = 1.4},
        iron = {durability = 150, bonus = 1.5},
        steel = {durability = 200, bonus = 1.8}
    },
    
    description = "Essential tool for efficient wood harvesting"
}

return common.validate_item(axe)