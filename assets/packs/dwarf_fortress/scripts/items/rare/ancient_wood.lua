-- Ancient Wood: Wood from an ancient tree
local constants = require("items._constants")
local common = require("items._common")

local ancient_wood = {
    id = "ancient_wood",
    name = "Ancient Wood",
    category = constants.categories.RARE,
    stack_size = 10,
    weight = 3,
    base_value = 50,
    decay_rate = 0,
    
    -- Rare drop chance
    drop_chance = 0.05,  -- 5% chance when harvesting trees
    drop_from = {"ancient_tree", "elder_oak"},
    
    -- Special properties
    special_properties = {
        crafting_quality_bonus = 1.5,
        durability_bonus = 2.0,
        magical_affinity = true,
        never_rots = true
    },
    
    -- Enhanced crafting
    crafting_material = true,
    material_tier = 5,  -- High-tier material
    
    -- Items made from this get bonuses
    crafting_bonuses = {
        durability_multiplier = 2.0,
        value_multiplier = 3.0,
        enchantment_capacity = 3
    },
    
    -- Trade value
    high_demand = true,
    merchant_interest = 0.9,  -- 90% of merchants want this
    
    description = "Extremely rare wood from ancient trees with magical properties"
}

return common.validate_item(ancient_wood)