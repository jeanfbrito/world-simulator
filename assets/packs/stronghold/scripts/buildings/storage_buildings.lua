-- Storage building definitions for Stronghold pack
-- Defines various storage buildings with their properties

storage_buildings = {
    -- Basic stockpile - open air storage
    stockpile = {
        id = "stockpile",
        name = "Stockpile",
        description = "Open-air storage area for general goods",
        
        -- Size and capacity
        size = { width = 3, height = 3 },
        capacity_per_tile = 100,
        total_capacity = 900,  -- 3x3 * 100
        
        -- Storage properties
        storage_type = "general",
        allowed_resources = nil,  -- nil means all types allowed
        priority = 0,
        
        -- Visual properties
        sprite = "stockpile",
        color = { r = 0.6, g = 0.5, b = 0.3 },
        
        -- Requirements
        build_cost = {
            wood = 10,
            stone = 5
        },
        build_time_ticks = 50,
        requires_workers = false,
        
        -- Efficiency
        protection_level = 0.0,  -- No protection from elements
        decay_rate = 0.01,  -- Items decay slowly
    },
    
    -- Warehouse - enclosed storage with better protection
    warehouse = {
        id = "warehouse",
        name = "Warehouse",
        description = "Enclosed storage building with weather protection",
        
        -- Size and capacity
        size = { width = 4, height = 4 },
        capacity_per_tile = 150,
        total_capacity = 2400,  -- 4x4 * 150
        
        -- Storage properties
        storage_type = "general",
        allowed_resources = nil,
        priority = 1,  -- Higher priority than stockpile
        
        -- Visual properties
        sprite = "warehouse",
        color = { r = 0.4, g = 0.3, b = 0.2 },
        
        -- Requirements
        build_cost = {
            wood = 50,
            stone = 30,
            iron = 10
        },
        build_time_ticks = 200,
        requires_workers = true,
        required_workers = 2,
        
        -- Efficiency
        protection_level = 0.9,  -- 90% protection
        decay_rate = 0.001,  -- Very slow decay
        efficiency_multiplier = 1.5,  -- 50% more efficient than stockpile
    },
    
    -- Granary - specialized food storage
    granary = {
        id = "granary",
        name = "Granary",
        description = "Specialized storage for food and grain",
        
        -- Size and capacity
        size = { width = 3, height = 3 },
        capacity_per_tile = 200,
        total_capacity = 1800,  -- High capacity for food
        
        -- Storage properties
        storage_type = "specialized",
        allowed_resources = { "wheat", "berries", "meat", "bread", "vegetables" },
        priority = 2,  -- High priority for food
        
        -- Visual properties
        sprite = "granary",
        color = { r = 0.7, g = 0.6, b = 0.4 },
        
        -- Requirements
        build_cost = {
            wood = 30,
            stone = 40,
            thatch = 20
        },
        build_time_ticks = 150,
        requires_workers = true,
        required_workers = 1,
        
        -- Efficiency
        protection_level = 0.95,  -- Excellent protection for food
        decay_rate = 0.0005,  -- Very slow food spoilage
        preservation_bonus = 2.0,  -- Food lasts 2x longer
    },
    
    -- Stone depot - heavy materials storage
    stone_depot = {
        id = "stone_depot",
        name = "Stone Depot",
        description = "Open storage for stone and heavy materials",
        
        -- Size and capacity  
        size = { width = 2, height = 2 },
        capacity_per_tile = 300,  -- Heavy items
        total_capacity = 1200,
        
        -- Storage properties
        storage_type = "specialized",
        allowed_resources = { "stone", "iron_ore", "coal", "marble" },
        priority = 0,
        
        -- Visual properties
        sprite = "stone_depot",
        color = { r = 0.5, g = 0.5, b = 0.5 },
        
        -- Requirements
        build_cost = {
            wood = 5,
            stone = 10
        },
        build_time_ticks = 30,
        requires_workers = false,
        
        -- Efficiency
        protection_level = 0.0,  -- Stone doesn't need protection
        decay_rate = 0.0,  -- Stone doesn't decay
    },
    
    -- Armory - weapons and tools storage
    armory = {
        id = "armory",
        name = "Armory",
        description = "Secure storage for weapons and tools",
        
        -- Size and capacity
        size = { width = 3, height = 2 },
        capacity_per_tile = 50,  -- Fewer items but valuable
        total_capacity = 300,
        
        -- Storage properties
        storage_type = "specialized",
        allowed_resources = { "sword", "bow", "arrow", "armor", "tool", "pickaxe", "axe" },
        priority = 3,  -- Very high priority
        
        -- Visual properties
        sprite = "armory",
        color = { r = 0.3, g = 0.3, b = 0.4 },
        
        -- Requirements
        build_cost = {
            stone = 50,
            iron = 20,
            wood = 20
        },
        build_time_ticks = 250,
        requires_workers = true,
        required_workers = 1,
        
        -- Efficiency
        protection_level = 1.0,  -- Full protection
        decay_rate = 0.0,  -- No decay for metal items
        security_level = "high",  -- Prevents theft
    },
    
    -- Market stall - temporary storage for trade
    market_stall = {
        id = "market_stall", 
        name = "Market Stall",
        description = "Small storage for market goods",
        
        -- Size and capacity
        size = { width = 1, height = 1 },
        capacity_per_tile = 50,
        total_capacity = 50,
        
        -- Storage properties
        storage_type = "trade",
        allowed_resources = nil,  -- Any tradeable goods
        priority = -1,  -- Low priority
        
        -- Visual properties
        sprite = "market_stall",
        color = { r = 0.8, g = 0.7, b = 0.5 },
        
        -- Requirements
        build_cost = {
            wood = 5,
            cloth = 2
        },
        build_time_ticks = 20,
        requires_workers = true,
        required_workers = 1,
        
        -- Efficiency
        protection_level = 0.3,
        decay_rate = 0.005,
        allows_trading = true,
    }
}

-- Function to get storage building by ID
function get_storage_building(id)
    return storage_buildings[id]
end

-- Function to get all storage buildings
function get_all_storage_buildings()
    return storage_buildings
end

-- Function to calculate actual capacity with modifiers
function calculate_capacity(building, efficiency_modifier)
    local base_capacity = building.total_capacity
    if building.efficiency_multiplier then
        base_capacity = base_capacity * building.efficiency_multiplier
    end
    if efficiency_modifier then
        base_capacity = base_capacity * efficiency_modifier
    end
    return math.floor(base_capacity)
end

-- Function to check if resource can be stored
function can_store_resource(building, resource_type)
    if building.allowed_resources == nil then
        return true  -- General storage accepts all
    end
    
    for _, allowed in ipairs(building.allowed_resources) do
        if allowed == resource_type then
            return true
        end
    end
    
    return false
end

-- Return the module
return {
    buildings = storage_buildings,
    get_building = get_storage_building,
    get_all = get_all_storage_buildings,
    calculate_capacity = calculate_capacity,
    can_store = can_store_resource
}