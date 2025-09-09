-- Gem: Precious stone
local constants = require("items._constants")
local common = require("items._common")

local gem = {
    id = "gem",
    name = "Precious Gem",
    category = constants.categories.RARE,
    stack_size = 5,
    weight = 0.1,
    base_value = 100,
    decay_rate = 0,
    
    -- Rare drop chance
    drop_chance = 0.01,  -- 1% chance when mining
    drop_from = {"gem_vein", "deep_mine"},
    requires_tool = "pickaxe",
    
    -- Gem varieties (randomly chosen on generation)
    varieties = {
        ruby = {color = "red", value_mult = 1.5},
        emerald = {color = "green", value_mult = 1.3},
        sapphire = {color = "blue", value_mult = 1.2},
        diamond = {color = "clear", value_mult = 2.0},
        amethyst = {color = "purple", value_mult = 1.0}
    },
    
    -- Special properties
    special_properties = {
        morale_bonus_when_owned = 10,
        trade_value_multiplier = 3.0,
        decorative = true,
        can_be_cut = true,  -- Can be processed by jeweler
        can_encrust = true  -- Can decorate items
    },
    
    -- Jeweling properties
    cuttable = true,
    cut_at = "jeweler",
    cut_skill_required = 5,
    cut_value_multiplier = 2.0,
    
    -- Trade properties
    always_valuable = true,
    currency_equivalent = true,
    
    description = "A sparkling precious stone highly valued by merchants"
}

return common.validate_item(gem)