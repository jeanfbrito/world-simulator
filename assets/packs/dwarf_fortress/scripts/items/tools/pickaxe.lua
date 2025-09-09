-- Pickaxe: Tool for mining stone and ore
local constants = require("items._constants")
local common = require("items._common")

local pickaxe = {
    id = "pickaxe",
    name = "Pickaxe",
    category = constants.categories.TOOL,
    stack_size = 1,
    weight = 5,
    base_value = 30,
    decay_rate = 0,
    
    -- Tool properties
    tool_type = "pickaxe",
    durability = 200,
    durability_loss_per_use = 0.8,
    
    -- Harvest bonuses
    harvest_bonus = {
        stone = 1.5,
        iron_ore = 1.5,
        copper_ore = 1.5,
        gem = 1.2
    },
    
    -- Mining properties
    can_mine_levels = {
        copper = 1,  -- Can mine tier 1 materials
        bronze = 2,  -- Can mine tier 2 materials
        iron = 3,    -- Can mine tier 3 materials
        steel = 4    -- Can mine all materials
    },
    
    -- Quality affects bonuses
    quality_affects_bonus = true,
    quality_affects_durability = true,
    quality_affects_value = true,
    
    -- Crafting info
    crafted_at = "forge",
    requires_skill = "toolmaking",
    skill_required = 2,  -- Needs some skill
    materials = {
        metal_bar = 2,
        wood = 1
    },
    
    description = "Essential tool for mining stone and valuable ores"
}

return common.validate_item(pickaxe)