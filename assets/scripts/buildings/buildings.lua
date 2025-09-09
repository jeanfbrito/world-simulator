-- Building Definitions
-- Defines all constructable buildings with their properties and requirements

-- Building categories
local categories = {
    HOUSING = "housing",
    PRODUCTION = "production",
    STORAGE = "storage",
    MILITARY = "military",
    INFRASTRUCTURE = "infrastructure",
    DECORATION = "decoration"
}

-- Building tiers for progression
local tiers = {
    PRIMITIVE = 1,
    BASIC = 2,
    ADVANCED = 3,
    MASTER = 4
}

-- Main building definitions
buildings = {
    -- Housing Buildings
    house = {
        id = "house",
        name = "House",
        category = categories.HOUSING,
        tier = tiers.BASIC,
        size = {width = 2, height = 2},
        
        -- Construction requirements
        cost = {
            wood = 10,
            stone = 5
        },
        build_time = 30,
        workers_required = 2,
        
        -- Building properties
        max_health = 100,
        fire_resistance = 0.3,
        decay_rate = 0.001,  -- Needs maintenance
        
        -- Functionality
        population_capacity = 4,
        warmth_provided = 20,
        storage_capacity = 20,  -- Small personal storage
        
        -- Bonuses
        bonuses = {
            morale = 5,
            rest_quality = 1.2
        },
        
        -- Upgrade path
        upgrades_to = "large_house",
        upgrade_cost = {
            wood = 5,
            stone = 3,
            tools = 1
        },
        
        -- Requirements
        requirements = {
            tech = nil,  -- No tech required
            nearby = nil,  -- Can build anywhere
            terrain = {"grass", "dirt", "stone"}
        },
        
        -- Maintenance
        maintenance = {
            resources = {wood = 1},
            interval = 100,  -- Every 100 ticks
            penalty_if_not_maintained = {
                health_loss = 5,
                morale_penalty = 2
            }
        },
        
        description = "Basic shelter for your population"
    },
    
    large_house = {
        id = "large_house",
        name = "Large House",
        category = categories.HOUSING,
        tier = tiers.ADVANCED,
        size = {width = 3, height = 3},
        
        cost = {
            wood = 15,
            stone = 10,
            planks = 5
        },
        build_time = 45,
        workers_required = 3,
        
        max_health = 150,
        fire_resistance = 0.4,
        decay_rate = 0.0008,
        
        population_capacity = 8,
        warmth_provided = 30,
        storage_capacity = 40,
        
        bonuses = {
            morale = 8,
            rest_quality = 1.5,
            prestige = 2
        },
        
        upgrades_to = "mansion",
        requirements = {
            tech = "advanced_construction",
            nearby = {"road"},  -- Needs road access
            terrain = {"grass", "dirt", "stone"}
        },
        
        maintenance = {
            resources = {wood = 1, stone = 1},
            interval = 150,
            penalty_if_not_maintained = {
                health_loss = 3,
                morale_penalty = 3,
                prestige_loss = 1
            }
        },
        
        description = "Comfortable housing for multiple families"
    },
    
    -- Storage Buildings
    stockpile = {
        id = "stockpile",
        name = "Stockpile",
        category = categories.STORAGE,
        tier = tiers.PRIMITIVE,
        size = {width = 3, height = 3},
        
        cost = {
            wood = 5
        },
        build_time = 15,
        workers_required = 1,
        
        max_health = 50,
        fire_resistance = 0.1,
        decay_rate = 0.002,
        
        storage_capacity = 200,
        storage_types = {"all"},  -- Accepts all resources
        preservation_bonus = 0.9,  -- 10% slower decay
        
        bonuses = {
            organization = 1.1  -- 10% faster resource access
        },
        
        upgrades_to = "warehouse",
        requirements = {
            tech = nil,
            terrain = {"grass", "dirt", "stone", "sand"}
        },
        
        description = "Open storage area for resources"
    },
    
    warehouse = {
        id = "warehouse",
        name = "Warehouse",
        category = categories.STORAGE,
        tier = tiers.ADVANCED,
        size = {width = 4, height = 4},
        
        cost = {
            wood = 20,
            stone = 15,
            planks = 10,
            tools = 2
        },
        build_time = 60,
        workers_required = 4,
        
        max_health = 200,
        fire_resistance = 0.5,
        decay_rate = 0.0005,
        
        storage_capacity = 1000,
        storage_types = {"all"},
        preservation_bonus = 0.7,  -- 30% slower decay
        
        bonuses = {
            organization = 1.3,
            trade_efficiency = 1.2
        },
        
        special_features = {
            auto_sort = true,
            quality_preservation = true,
            theft_protection = 0.8
        },
        
        requirements = {
            tech = "logistics",
            nearby = {"road", "market"},
            terrain = {"grass", "dirt", "stone"}
        },
        
        description = "Advanced storage with preservation"
    },
    
    -- Production Buildings
    sawmill = {
        id = "sawmill",
        name = "Sawmill",
        category = categories.PRODUCTION,
        tier = tiers.BASIC,
        size = {width = 3, height = 3},
        
        cost = {
            wood = 15,
            stone = 10,
            tools = 1
        },
        build_time = 40,
        workers_required = 2,
        
        max_health = 150,
        fire_resistance = 0.2,
        decay_rate = 0.001,
        
        production_enabled = true,
        worker_slots = 3,
        storage_capacity = 100,
        storage_types = {"wood", "planks"},
        
        production_bonus = {
            wood_processing = 1.5,
            quality_chance = 0.2
        },
        
        recipes_enabled = {
            "wood_to_planks",
            "advanced_wood_processing",
            "bulk_wood_processing"
        },
        
        upgrades_to = "lumber_mill",
        requirements = {
            tech = nil,
            nearby = {"tree", "forest"},  -- Near wood source
            terrain = {"grass", "dirt"}
        },
        
        maintenance = {
            resources = {tools = 1},
            interval = 200,
            penalty_if_not_maintained = {
                production_penalty = 0.5,
                breakdown_chance = 0.1
            }
        },
        
        description = "Processes wood into planks"
    },
    
    farm = {
        id = "farm",
        name = "Farm",
        category = categories.PRODUCTION,
        tier = tiers.BASIC,
        size = {width = 5, height = 5},
        
        cost = {
            wood = 8,
            tools = 2
        },
        build_time = 25,
        workers_required = 1,
        
        max_health = 75,
        fire_resistance = 0.1,
        decay_rate = 0.0015,
        
        production_enabled = true,
        worker_slots = 4,
        storage_capacity = 150,
        storage_types = {"wheat", "food", "seeds"},
        
        production_bonus = {
            farming = 1.0,
            seasonal_modifier = true
        },
        
        special_features = {
            irrigation_bonus = 0,  -- Can be improved
            fertility = 1.0,  -- Depletes over time
            crop_rotation = false  -- Can be researched
        },
        
        seasonal_production = {
            spring = {planting = true, harvest = false, growth = 1.2},
            summer = {planting = false, harvest = false, growth = 1.5},
            autumn = {planting = false, harvest = true, growth = 0.8},
            winter = {planting = false, harvest = false, growth = 0}
        },
        
        upgrades_to = "advanced_farm",
        requirements = {
            tech = "agriculture",
            nearby = {"water", "well"},  -- Needs water source
            terrain = {"grass", "fertile_soil"}
        },
        
        description = "Grows food for your settlement"
    },
    
    bakery = {
        id = "bakery",
        name = "Bakery",
        category = categories.PRODUCTION,
        tier = tiers.BASIC,
        size = {width = 2, height = 2},
        
        cost = {
            wood = 10,
            stone = 6,
            tools = 1
        },
        build_time = 30,
        workers_required = 2,
        
        max_health = 90,
        fire_resistance = 0.3,
        decay_rate = 0.001,
        
        production_enabled = true,
        worker_slots = 2,
        storage_capacity = 60,
        storage_types = {"wheat", "food", "bread"},
        
        production_bonus = {
            food_quality = 1.3,
            morale_from_food = 1.5
        },
        
        recipes_enabled = {
            "wheat_to_bread",
            "hearty_stew",
            "preserve_food"
        },
        
        special_features = {
            aroma_bonus = {  -- Affects nearby buildings
                range = 5,
                morale = 2
            }
        },
        
        requirements = {
            tech = nil,
            nearby = {"house", "market"},  -- Near customers
            terrain = {"grass", "dirt", "stone"}
        },
        
        description = "Bakes bread and prepares food"
    },
    
    workshop = {
        id = "workshop",
        name = "Workshop",
        category = categories.PRODUCTION,
        tier = tiers.BASIC,
        size = {width = 3, height = 3},
        
        cost = {
            wood = 12,
            stone = 8,
            tools = 2
        },
        build_time = 35,
        workers_required = 2,
        
        max_health = 100,
        fire_resistance = 0.4,
        decay_rate = 0.0008,
        
        production_enabled = true,
        worker_slots = 3,
        storage_capacity = 80,
        storage_types = {"wood", "stone", "tools", "iron"},
        
        production_bonus = {
            tool_quality = 1.2,
            repair_speed = 1.5
        },
        
        recipes_enabled = {
            "make_tools",
            "make_axe",
            "make_pickaxe",
            "repair_tools"
        },
        
        special_features = {
            tool_maintenance = true,  -- Auto-repairs tools
            quality_crafting = 0.3  -- 30% chance for better quality
        },
        
        upgrades_to = "smithy",
        requirements = {
            tech = "basic_crafting",
            terrain = {"grass", "dirt", "stone"}
        },
        
        description = "Creates and repairs tools"
    },
    
    -- Infrastructure
    road = {
        id = "road",
        name = "Road",
        category = categories.INFRASTRUCTURE,
        tier = tiers.PRIMITIVE,
        size = {width = 1, height = 1},
        
        cost = {
            stone = 2
        },
        build_time = 5,
        workers_required = 1,
        
        max_health = 200,
        fire_resistance = 1.0,  -- Can't burn
        decay_rate = 0.0001,
        
        special_features = {
            movement_speed = 1.5,  -- 50% faster movement
            cart_access = true,
            maintenance_free_duration = 500
        },
        
        upgrades_to = "paved_road",
        requirements = {
            tech = nil,
            terrain = {"grass", "dirt", "sand"}
        },
        
        description = "Speeds up movement between buildings"
    },
    
    well = {
        id = "well",
        name = "Well",
        category = categories.INFRASTRUCTURE,
        tier = tiers.BASIC,
        size = {width = 1, height = 1},
        
        cost = {
            stone = 10,
            wood = 5
        },
        build_time = 20,
        workers_required = 2,
        
        max_health = 150,
        fire_resistance = 0.8,
        decay_rate = 0.0005,
        
        special_features = {
            water_supply = 100,  -- Units per day
            water_quality = 1.0,
            irrigation_range = 10,
            fire_fighting_bonus = 2.0
        },
        
        bonuses = {
            health = 1.1,
            farm_yield = 1.2  -- If near farms
        },
        
        upgrades_to = "deep_well",
        requirements = {
            tech = nil,
            terrain = {"grass", "dirt"},
            special = "water_table"  -- Needs underground water
        },
        
        description = "Provides clean water to settlement"
    },
    
    -- Military Buildings
    watchtower = {
        id = "watchtower",
        name = "Watchtower",
        category = categories.MILITARY,
        tier = tiers.BASIC,
        size = {width = 2, height = 2},
        
        cost = {
            wood = 15,
            stone = 10,
            tools = 1
        },
        build_time = 40,
        workers_required = 3,
        
        max_health = 200,
        fire_resistance = 0.5,
        decay_rate = 0.0008,
        
        special_features = {
            vision_range = 20,
            attack_range = 15,
            garrison_slots = 2,
            early_warning = true
        },
        
        bonuses = {
            defense = 1.5,
            morale = 3  -- People feel safer
        },
        
        upgrades_to = "guard_tower",
        requirements = {
            tech = "military_basics",
            terrain = {"grass", "dirt", "stone", "hill"}
        },
        
        description = "Provides early warning and defense"
    }
}

-- Helper functions
function get_building(id)
    return buildings[id]
end

function calculate_build_time(building_id, worker_skill, num_workers)
    local building = buildings[building_id]
    if not building then return 0 end
    
    local base_time = building.build_time
    
    -- Worker skill reduces time (average skill of all workers)
    local skill_mult = 1.0 - (worker_skill * 0.05)  -- 5% per skill level
    
    -- Multiple workers reduce time, but with diminishing returns
    local worker_mult = 1.0 / (1.0 + math.log(num_workers))
    
    return math.floor(base_time * skill_mult * worker_mult)
end

function check_requirements(building_id, location, available_tech)
    local building = buildings[building_id]
    if not building then return false, "Unknown building" end
    
    -- Check tech requirements
    if building.requirements.tech and not available_tech[building.requirements.tech] then
        return false, "Missing technology: " .. building.requirements.tech
    end
    
    -- Check terrain
    if building.requirements.terrain then
        local terrain_ok = false
        for _, allowed_terrain in ipairs(building.requirements.terrain) do
            if location.terrain == allowed_terrain then
                terrain_ok = true
                break
            end
        end
        if not terrain_ok then
            return false, "Invalid terrain for building"
        end
    end
    
    -- Check nearby requirements
    if building.requirements.nearby then
        for _, required_nearby in ipairs(building.requirements.nearby) do
            if not location:has_nearby(required_nearby, 10) then
                return false, "Must be built near: " .. required_nearby
            end
        end
    end
    
    return true, "Can build"
end

function apply_building_bonuses(building_id, settlement)
    local building = buildings[building_id]
    if not building or not building.bonuses then return end
    
    for stat, value in pairs(building.bonuses) do
        settlement:modify_stat(stat, value)
    end
    
    -- Apply special features
    if building.special_features then
        for feature, value in pairs(building.special_features) do
            settlement:add_feature(feature, value)
        end
    end
end

function calculate_maintenance_cost(building_id, age_ticks)
    local building = buildings[building_id]
    if not building or not building.maintenance then return {} end
    
    -- Check if maintenance is due
    if age_ticks % building.maintenance.interval ~= 0 then
        return {}
    end
    
    -- Age increases maintenance cost
    local age_mult = 1.0 + (age_ticks / 1000) * 0.1
    
    local costs = {}
    for resource, amount in pairs(building.maintenance.resources) do
        costs[resource] = math.ceil(amount * age_mult)
    end
    
    return costs
end

-- Seasonal effects on buildings
function apply_seasonal_effects(season)
    for id, building in pairs(buildings) do
        if building.seasonal_production and building.seasonal_production[season] then
            -- Apply seasonal modifiers to farms and similar buildings
            local seasonal = building.seasonal_production[season]
            building.current_production_mod = seasonal
        end
        
        -- Winter increases decay for exposed buildings
        if season == "winter" then
            if building.category == categories.HOUSING then
                building.current_decay_rate = building.decay_rate * 1.5
            end
        end
    end
end

-- Initialize buildings on script load
function on_init()
    print("Loading " .. table_length(buildings) .. " building definitions")
    
    -- Validate all buildings
    for id, building in pairs(buildings) do
        if building.id ~= id then
            print("Warning: Building ID mismatch for " .. id)
        end
    end
end

function table_length(t)
    local count = 0
    for _ in pairs(t) do count = count + 1 end
    return count
end