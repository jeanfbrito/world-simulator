-- Peasant Unit Script
-- Basic worker unit for the stronghold simulation
-- Based on stronghold_old/scripts/units/military.lua peasant definition

local peasant = {
    id = "peasant",
    name = "Peasant",
    category = "civilian",
    
    -- Cost
    gold_cost = 0,  -- Free, comes from population
    
    -- Stats
    health = 10,
    speed = 5,
    
    -- Work capabilities
    can_work = true,
    can_construct = true,
    can_repair = true,
    can_firefight = true,
    can_haul = true,
    
    -- Combat (when drafted)
    attack_damage = 2,
    defense = 0,
    morale = 2,
    
    -- Resources they can gather
    can_gather = {
        "wood",
        "stone", 
        "iron",
        "food"
    },
    
    -- Basic needs
    needs_food = true,
    needs_shelter = true
}

-- Called when peasant is spawned
function peasant:on_spawn(world, x, y)
    print("Peasant spawned at (" .. x .. ", " .. y .. ")")
    return true
end

-- Called each update tick
function peasant:on_update(world, dt)
    -- Basic survival needs
    -- This would be handled by the simulation system
    return true
end

-- Called when peasant performs work
function peasant:on_work_started(work_type, target)
    print("Peasant started work: " .. work_type)
    return true
end

function peasant:on_work_completed(work_type, target, result)
    print("Peasant completed work: " .. work_type .. " with result: " .. tostring(result))
    return true
end

-- Called when peasant is drafted for combat
function peasant:on_drafted()
    print("Peasant has been drafted!")
    -- Becomes a militia unit with basic combat stats
    self.category = "militia"
    self.morale = 1  -- Very low morale when drafted
    return true
end

-- Called when peasant returns from combat
function peasant:on_demobilized()
    print("Peasant returned from military service")
    self.category = "civilian"
    return true
end

return peasant