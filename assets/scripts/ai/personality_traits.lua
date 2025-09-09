-- Personality Traits System
-- Defines worker personalities that affect behavior and performance

-- Available personality traits
local traits = {
    -- Work ethic traits
    lazy = {
        name = "Lazy",
        work_speed = 0.75,
        fatigue_rate = 0.85,
        rest_need = 1.3,
        skill_gain = 0.8
    },
    
    industrious = {
        name = "Industrious",
        work_speed = 1.3,
        fatigue_rate = 1.2,
        skill_gain = 1.25,
        reputation_gain = 1.2
    },
    
    perfectionist = {
        name = "Perfectionist",
        work_speed = 0.85,
        quality_bonus = 1.4,
        skill_gain = 1.35,
        stress_from_failure = 2.0
    },
    
    -- Social traits
    social = {
        name = "Social",
        morale_from_social = 1.5,
        work_alone_penalty = 0.7,
        teaching_ability = 1.3
    },
    
    loner = {
        name = "Loner",
        work_alone_bonus = 1.25,
        social_need = 0.5,
        crowd_stress = 1.5
    },
    
    leader = {
        name = "Natural Leader",
        nearby_worker_bonus = 1.15,
        morale_influence = 1.4,
        decision_speed = 1.2
    },
    
    -- Emotional traits
    optimist = {
        name = "Optimist",
        morale_decay = 0.7,
        stress_resistance = 1.3,
        inspiration_chance = 0.15
    },
    
    pessimist = {
        name = "Pessimist",
        morale_decay = 1.3,
        stress_resistance = 0.8,
        caution_bonus = 1.2
    },
    
    brave = {
        name = "Brave",
        fear_resistance = 1.5,
        danger_response = "aggressive",
        morale_in_crisis = 1.2
    },
    
    cautious = {
        name = "Cautious",
        accident_chance = 0.5,
        danger_detection = 1.4,
        work_speed_dangerous = 0.7
    }
}

-- Assign traits to new workers
function assign_personality(worker)
    local assigned_traits = {}
    
    -- Everyone gets 1-3 traits
    local num_traits = math.random(1, 3)
    
    -- Build weighted trait pool
    local trait_pool = {}
    for name, _ in pairs(traits) do
        table.insert(trait_pool, name)
    end
    
    -- Assign random traits
    for i = 1, num_traits do
        if #trait_pool > 0 then
            local index = math.random(1, #trait_pool)
            local trait_name = trait_pool[index]
            
            -- Check for incompatible traits
            if not has_conflict(assigned_traits, trait_name) then
                table.insert(assigned_traits, trait_name)
                apply_trait(worker, traits[trait_name])
                
                print("Assigned trait '" .. traits[trait_name].name .. 
                      "' to " .. worker:get_name())
            end
            
            -- Remove from pool to avoid duplicates
            table.remove(trait_pool, index)
        end
    end
    
    return assigned_traits
end

-- Check for trait conflicts
function has_conflict(current_traits, new_trait)
    local conflicts = {
        lazy = {"industrious"},
        industrious = {"lazy"},
        social = {"loner"},
        loner = {"social"},
        optimist = {"pessimist"},
        pessimist = {"optimist"},
        brave = {"cautious"},
        cautious = {"brave"}
    }
    
    local trait_conflicts = conflicts[new_trait] or {}
    
    for _, trait in ipairs(current_traits) do
        for _, conflict in ipairs(trait_conflicts) do
            if trait == conflict then
                return true
            end
        end
    end
    
    return false
end

-- Apply trait modifiers to worker
function apply_trait(worker, trait)
    for stat, value in pairs(trait) do
        if type(value) == "number" then
            worker:modify_stat(stat, value)
        elseif type(value) == "string" then
            worker:set_property(stat, value)
        end
    end
end

-- Calculate trait synergies for group work
function calculate_group_synergy(workers)
    local synergy = 1.0
    local traits_count = {}
    
    -- Count traits in group
    for _, worker in ipairs(workers) do
        local worker_traits = worker:get_traits()
        for _, trait in ipairs(worker_traits) do
            traits_count[trait] = (traits_count[trait] or 0) + 1
        end
    end
    
    -- Calculate synergies
    local group_size = #workers
    
    -- Leadership bonus
    if traits_count["leader"] and traits_count["leader"] > 0 then
        synergy = synergy + 0.1 * traits_count["leader"]
    end
    
    -- Too many leaders cause conflict
    if traits_count["leader"] and traits_count["leader"] > 2 then
        synergy = synergy - 0.05 * (traits_count["leader"] - 2)
    end
    
    -- Social workers work well together
    if traits_count["social"] then
        local social_ratio = traits_count["social"] / group_size
        synergy = synergy + social_ratio * 0.2
    end
    
    -- Loners reduce group efficiency
    if traits_count["loner"] then
        local loner_ratio = traits_count["loner"] / group_size
        synergy = synergy - loner_ratio * 0.15
    end
    
    -- Mix of optimists and pessimists is balanced
    local optimists = traits_count["optimist"] or 0
    local pessimists = traits_count["pessimist"] or 0
    if optimists > 0 and pessimists > 0 then
        synergy = synergy + 0.05 -- Balanced perspectives
    end
    
    return math.max(0.5, math.min(1.5, synergy))
end

-- Trait evolution over time
function evolve_traits(worker)
    local experience = worker:get_experience()
    local stress_level = worker:get_stress()
    local social_interactions = worker:get_social_count()
    
    -- Workers can develop new traits based on experiences
    if experience > 100 and not worker:has_trait("industrious") then
        if math.random() < 0.1 then
            worker:add_trait("industrious")
            print(worker:get_name() .. " has become industrious through experience!")
        end
    end
    
    -- High stress might make optimists become pessimists
    if stress_level > 80 and worker:has_trait("optimist") then
        if math.random() < 0.05 then
            worker:remove_trait("optimist")
            worker:add_trait("pessimist")
            print(worker:get_name() .. " has become pessimistic due to stress...")
        end
    end
    
    -- Isolated workers might become loners
    if social_interactions < 10 and not worker:has_trait("loner") then
        if math.random() < 0.02 then
            worker:add_trait("loner")
            print(worker:get_name() .. " has become a loner from isolation")
        end
    end
end

-- Daily trait effects
function apply_daily_trait_effects(worker)
    local worker_traits = worker:get_traits()
    
    for _, trait_name in ipairs(worker_traits) do
        local trait = traits[trait_name]
        
        -- Optimists recover morale faster
        if trait_name == "optimist" then
            worker:add_morale(2)
        end
        
        -- Social workers need interaction
        if trait_name == "social" then
            local nearby = world:get_workers_in_radius(worker:get_position(), 5)
            if #nearby < 2 then
                worker:add_stress(3)
            else
                worker:add_morale(1)
            end
        end
        
        -- Loners prefer solitude
        if trait_name == "loner" then
            local nearby = world:get_workers_in_radius(worker:get_position(), 5)
            if #nearby > 3 then
                worker:add_stress(2)
            else
                worker:add_morale(1)
            end
        end
        
        -- Leaders inspire nearby workers
        if trait_name == "leader" then
            local nearby = world:get_workers_in_radius(worker:get_position(), 10)
            for _, other in ipairs(nearby) do
                if other ~= worker then
                    other:add_morale(1)
                end
            end
        end
    end
end