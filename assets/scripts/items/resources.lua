-- Resource Item Definitions
-- Defines all harvestable and crafted resources in the game

-- Resource categories for organization
local categories = {
    RAW_MATERIAL = "raw_material",
    PROCESSED = "processed",
    FOOD = "food",
    RARE = "rare",
    TOOL = "tool",
    TRADE = "trade_good"
}

-- Quality tiers that affect value and effectiveness
local quality = {
    POOR = 0.7,
    NORMAL = 1.0,
    GOOD = 1.3,
    EXCELLENT = 1.6,
    MASTERWORK = 2.0
}

-- Main resource definitions
resources = {
    -- Raw Materials
    wood = {
        id = "wood",
        name = "Wood",
        category = categories.RAW_MATERIAL,
        stack_size = 100,
        weight = 2,
        base_value = 1,
        decay_rate = 0,  -- Doesn't decay
        harvestable_from = {"tree"},
        harvest_time = 10,
        harvest_yield = {min = 2, max = 5},
        tool_required = nil,  -- Can harvest by hand
        tool_bonus = {axe = 1.5},  -- But better with tools
        seasons = {
            spring = 1.0,
            summer = 1.2,
            autumn = 1.1,
            winter = 0.8
        },
        description = "Basic construction material"
    },
    
    stone = {
        id = "stone",
        name = "Stone",
        category = categories.RAW_MATERIAL,
        stack_size = 50,
        weight = 5,
        base_value = 2,
        decay_rate = 0,
        harvestable_from = {"stone_deposit", "quarry"},
        harvest_time = 15,
        harvest_yield = {min = 1, max = 3},
        tool_required = "pickaxe",
        tool_bonus = {pickaxe = 1.0, advanced_pickaxe = 1.5},
        seasons = {
            spring = 1.0,
            summer = 1.0,
            autumn = 1.0,
            winter = 0.7  -- Harder to mine in winter
        },
        description = "Sturdy building material"
    },
    
    iron_ore = {
        id = "iron_ore",
        name = "Iron Ore",
        category = categories.RAW_MATERIAL,
        stack_size = 30,
        weight = 8,
        base_value = 5,
        decay_rate = 0,
        harvestable_from = {"iron_deposit", "mine"},
        harvest_time = 20,
        harvest_yield = {min = 1, max = 2},
        tool_required = "pickaxe",
        tool_bonus = {pickaxe = 1.0, advanced_pickaxe = 1.8},
        seasons = {
            spring = 1.0,
            summer = 1.0,
            autumn = 1.0,
            winter = 0.5
        },
        description = "Raw metal for tools and weapons"
    },
    
    -- Food Resources
    berries = {
        id = "berries",
        name = "Berries",
        category = categories.FOOD,
        stack_size = 50,
        weight = 0.5,
        base_value = 2,
        decay_rate = 0.05,  -- Spoils over time
        decay_time = 50,  -- Ticks until spoiled
        nutrition = 10,
        morale_bonus = 2,
        harvestable_from = {"berry_bush"},
        harvest_time = 5,
        harvest_yield = {min = 3, max = 8},
        tool_required = nil,
        seasons = {
            spring = 0.5,
            summer = 1.5,  -- Peak season
            autumn = 1.0,
            winter = 0  -- No berries in winter
        },
        description = "Sweet wild berries"
    },
    
    wheat = {
        id = "wheat",
        name = "Wheat",
        category = categories.FOOD,
        stack_size = 100,
        weight = 1,
        base_value = 3,
        decay_rate = 0.01,
        decay_time = 200,
        harvestable_from = {"farm"},
        harvest_time = 8,
        harvest_yield = {min = 5, max = 15},
        tool_required = "scythe",
        tool_bonus = {scythe = 1.0, advanced_scythe = 1.5},
        seasons = {
            spring = 0,  -- Planting season
            summer = 0.5,
            autumn = 2.0,  -- Harvest season
            winter = 0
        },
        growth_time = 100,  -- Time to grow from planting
        description = "Golden grain for bread"
    },
    
    food = {
        id = "food",
        name = "Food",
        category = categories.FOOD,
        stack_size = 100,
        weight = 1,
        base_value = 4,
        decay_rate = 0.03,
        decay_time = 100,
        nutrition = 25,
        morale_bonus = 5,
        description = "Prepared food ready to eat"
    },
    
    -- Processed Materials
    planks = {
        id = "planks",
        name = "Planks",
        category = categories.PROCESSED,
        stack_size = 50,
        weight = 1.5,
        base_value = 3,
        decay_rate = 0,
        quality_affects_value = true,
        description = "Processed wood for construction"
    },
    
    iron_ingot = {
        id = "iron_ingot",
        name = "Iron Ingot",
        category = categories.PROCESSED,
        stack_size = 20,
        weight = 7,
        base_value = 15,
        decay_rate = 0,
        quality_affects_value = true,
        description = "Refined metal ready for crafting"
    },
    
    bread = {
        id = "bread",
        name = "Bread",
        category = categories.FOOD,
        stack_size = 30,
        weight = 0.8,
        base_value = 6,
        decay_rate = 0.02,
        decay_time = 150,
        nutrition = 30,
        morale_bonus = 8,
        quality_affects_nutrition = true,
        description = "Freshly baked bread"
    },
    
    -- Tools
    tools = {
        id = "tools",
        name = "Tools",
        category = categories.TOOL,
        stack_size = 10,
        weight = 3,
        base_value = 20,
        decay_rate = 0,
        durability = 100,
        durability_loss_per_use = 1,
        quality_affects_durability = true,
        description = "Basic tools for work"
    },
    
    axe = {
        id = "axe",
        name = "Axe",
        category = categories.TOOL,
        stack_size = 1,
        weight = 4,
        base_value = 25,
        decay_rate = 0,
        durability = 150,
        durability_loss_per_use = 0.5,
        harvest_bonus = {wood = 1.5},
        quality_affects_bonus = true,
        description = "Tool for chopping wood"
    },
    
    pickaxe = {
        id = "pickaxe",
        name = "Pickaxe",
        category = categories.TOOL,
        stack_size = 1,
        weight = 5,
        base_value = 30,
        decay_rate = 0,
        durability = 200,
        durability_loss_per_use = 0.8,
        harvest_bonus = {stone = 1.5, iron_ore = 1.5},
        quality_affects_bonus = true,
        description = "Tool for mining stone and ore"
    },
    
    -- Rare/Special Items
    rare_wood = {
        id = "rare_wood",
        name = "Ancient Wood",
        category = categories.RARE,
        stack_size = 10,
        weight = 3,
        base_value = 50,
        decay_rate = 0,
        drop_chance = 0.05,  -- 5% chance when harvesting trees
        special_properties = {
            crafting_quality_bonus = 1.5,
            durability_bonus = 2.0,
            magical_affinity = true
        },
        description = "Wood from an ancient tree"
    },
    
    gem = {
        id = "gem",
        name = "Precious Gem",
        category = categories.RARE,
        stack_size = 5,
        weight = 0.1,
        base_value = 100,
        decay_rate = 0,
        drop_chance = 0.01,  -- 1% chance when mining
        tradeable = true,
        special_properties = {
            morale_bonus_when_owned = 10,
            trade_value_multiplier = 3.0
        },
        description = "A sparkling precious stone"
    }
}

-- Helper functions for item management
function get_item(id)
    return resources[id]
end

function calculate_value(item_id, quality_tier, quantity)
    local item = resources[item_id]
    if not item then return 0 end
    
    local base = item.base_value * quantity
    
    if item.quality_affects_value then
        base = base * (quality_tier or quality.NORMAL)
    end
    
    return math.floor(base)
end

function calculate_decay(item_id, age_ticks)
    local item = resources[item_id]
    if not item or item.decay_rate == 0 then
        return 0
    end
    
    return math.min(1.0, (age_ticks / item.decay_time) * item.decay_rate)
end

function get_harvest_yield(item_id, tool_id, skill_level, season)
    local item = resources[item_id]
    if not item then return 0 end
    
    -- Base yield
    local min_yield = item.harvest_yield.min
    local max_yield = item.harvest_yield.max
    local yield = math.random(min_yield, max_yield)
    
    -- Tool bonus
    if tool_id and item.tool_bonus and item.tool_bonus[tool_id] then
        yield = yield * item.tool_bonus[tool_id]
    end
    
    -- Skill bonus (0-10 skill level)
    local skill_mult = 1.0 + (skill_level * 0.1)
    yield = yield * skill_mult
    
    -- Seasonal modifier
    if item.seasons and item.seasons[season] then
        yield = yield * item.seasons[season]
    end
    
    return math.floor(yield)
end

-- Special item interactions
function on_item_consumed(item_id, consumer)
    local item = resources[item_id]
    if not item then return end
    
    -- Apply nutrition
    if item.nutrition then
        consumer:add_nutrition(item.nutrition)
    end
    
    -- Apply morale bonus
    if item.morale_bonus then
        consumer:add_morale(item.morale_bonus)
    end
    
    -- Special effects
    if item.special_properties then
        for property, value in pairs(item.special_properties) do
            consumer:apply_effect(property, value)
        end
    end
end

-- Quality determination
function determine_quality(crafter_skill, recipe_difficulty, tool_quality)
    local base_chance = crafter_skill / 10  -- 0 to 1
    local tool_mult = tool_quality or 1.0
    local difficulty_penalty = math.max(0, recipe_difficulty - crafter_skill) * 0.1
    
    local roll = math.random() * tool_mult - difficulty_penalty
    
    if roll > 0.95 then
        return quality.MASTERWORK
    elseif roll > 0.85 then
        return quality.EXCELLENT
    elseif roll > 0.65 then
        return quality.GOOD
    elseif roll > 0.35 then
        return quality.NORMAL
    else
        return quality.POOR
    end
end

-- Trading value calculations
function calculate_trade_value(item_id, quantity, buyer_needs, seller_reputation)
    local item = resources[item_id]
    if not item then return 0 end
    
    local base = calculate_value(item_id, quality.NORMAL, quantity)
    
    -- Buyer need multiplier (0.5 to 2.0)
    local need_mult = buyer_needs[item.category] or 1.0
    
    -- Reputation bonus (0.8 to 1.5)
    local rep_mult = 0.8 + (seller_reputation / 100) * 0.7
    
    -- Rarity bonus
    if item.category == categories.RARE then
        base = base * 2.0
    end
    
    return math.floor(base * need_mult * rep_mult)
end

-- Initialize items on script load
function on_init()
    print("Loading " .. table_length(resources) .. " resource definitions")
    
    -- Validate all items
    for id, item in pairs(resources) do
        if item.id ~= id then
            print("Warning: Item ID mismatch for " .. id)
        end
    end
end

-- Utility function
function table_length(t)
    local count = 0
    for _ in pairs(t) do count = count + 1 end
    return count
end