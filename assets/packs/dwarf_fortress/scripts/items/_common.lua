-- Common utility functions for item management

local M = {}
local constants = require("items._constants")

-- Calculate item value based on quality and quantity
function M.calculate_value(base_value, quality_tier, quantity, quality_affects_value)
    local value = base_value * (quantity or 1)
    
    if quality_affects_value then
        value = value * (quality_tier or constants.quality.NORMAL)
    end
    
    return math.floor(value)
end

-- Calculate decay for perishable items
function M.calculate_decay(decay_rate, decay_time, age_ticks)
    if not decay_rate or decay_rate == 0 then
        return 0
    end
    
    return math.min(1.0, (age_ticks / decay_time) * decay_rate)
end

-- Calculate harvest yield with modifiers
function M.get_harvest_yield(harvest_yield, tool_id, tool_bonus, skill_level, season, season_modifiers)
    if not harvest_yield then return 0 end
    
    -- Base yield
    local min_yield = harvest_yield.min or 1
    local max_yield = harvest_yield.max or 1
    local yield = math.random(min_yield, max_yield)
    
    -- Tool bonus
    if tool_id and tool_bonus and tool_bonus[tool_id] then
        yield = yield * tool_bonus[tool_id]
    end
    
    -- Skill bonus (0-10 skill level)
    local skill_mult = 1.0 + ((skill_level or 0) * 0.1)
    yield = yield * skill_mult
    
    -- Seasonal modifier
    if season_modifiers and season_modifiers[season] then
        yield = yield * season_modifiers[season]
    end
    
    return math.floor(yield)
end

-- Determine quality based on crafter skill
function M.determine_quality(crafter_skill, recipe_difficulty, tool_quality)
    local base_chance = (crafter_skill or 0) / 10  -- 0 to 1
    local tool_mult = tool_quality or 1.0
    local difficulty_penalty = math.max(0, (recipe_difficulty or 0) - (crafter_skill or 0)) * 0.1
    
    local roll = math.random() * tool_mult - difficulty_penalty
    
    if roll > 0.95 then
        return constants.quality.MASTERWORK
    elseif roll > 0.85 then
        return constants.quality.EXCELLENT
    elseif roll > 0.65 then
        return constants.quality.GOOD
    elseif roll > 0.35 then
        return constants.quality.NORMAL
    else
        return constants.quality.POOR
    end
end

-- Calculate trade value with market modifiers
function M.calculate_trade_value(base_value, quantity, category, buyer_needs, seller_reputation)
    local value = base_value * (quantity or 1)
    
    -- Buyer need multiplier (0.5 to 2.0)
    local need_mult = 1.0
    if buyer_needs and buyer_needs[category] then
        need_mult = buyer_needs[category]
    end
    
    -- Reputation bonus (0.8 to 1.5)
    local rep_mult = 0.8 + ((seller_reputation or 0) / 100) * 0.7
    
    -- Rarity bonus
    if category == constants.categories.RARE then
        value = value * 2.0
    end
    
    return math.floor(value * need_mult * rep_mult)
end

-- Validate item definition
function M.validate_item(item)
    if not item.id then
        error("Item missing required field: id")
    end
    if not item.name then
        error("Item " .. item.id .. " missing required field: name")
    end
    if not item.category then
        error("Item " .. item.id .. " missing required field: category")
    end
    
    -- Set defaults
    item.stack_size = item.stack_size or 1
    item.weight = item.weight or 1
    item.base_value = item.base_value or 1
    item.decay_rate = item.decay_rate or 0
    item.description = item.description or ""
    
    return item
end

return M