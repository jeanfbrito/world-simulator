-- Wood Resource Script
-- Basic building material resource

local wood = {
    id = "wood",
    name = "Wood",
    category = "resource",
    type = "material",
    
    -- Visual representation
    sprite = "wood_log",
    stack_size = 50,  -- Max items per stack
    
    -- Resource properties
    material_type = "organic",
    weight = 1,  -- Weight per unit
    durability = 100,  -- For tools/items made from wood
    
    -- Economic properties
    base_value = 2,  -- Gold value per unit
    trade_good = true,
    
    -- Storage properties
    storage_category = "materials",
    requires_covered_storage = false,  -- Can be stored outdoors
    decay_rate = 0.01,  -- Very slow decay outdoors
    
    -- Usage properties
    fuel_value = 10,  -- Can be used as fuel
    construction_material = true,
    crafting_material = true,
    
    -- Fire properties
    flammable = true,
    burn_time = 30,  -- Seconds when used as fuel
    
    -- What can be crafted with wood
    crafting_recipes = {
        "wooden_wall",
        "wooden_door", 
        "wooden_floor",
        "wooden_stairs",
        "wooden_fence",
        "wooden_shield",
        "wooden_spear",
        "bow",
        "arrows",
        "campfire",
        "wooden_chest"
    }
}

-- Called when wood is spawned/dropped
function wood:on_spawn(world, x, y, amount)
    amount = amount or 1
    print("Wood spawned: " .. amount .. " units at (" .. x .. ", " .. y .. ")")
    
    self.current_stack_size = amount
    return true
end

-- Called when trying to stack with other wood
function wood:can_stack_with(other_item)
    if other_item.id ~= self.id then
        return false
    end
    
    local total = self.current_stack_size + other_item.current_stack_size
    return total <= self.stack_size
end

-- Called when wood is picked up
function wood:on_pickup(picker, world)
    print(picker.name .. " picked up " .. self.current_stack_size .. " wood")
    return true
end

-- Called when wood is used in construction
function wood:on_use_construction(construction_type, builder, world)
    print("Using wood for construction: " .. construction_type)
    
    local wood_required = self:get_construction_cost(construction_type)
    if self.current_stack_size >= wood_required then
        self.current_stack_size = self.current_stack_size - wood_required
        
        print("Used " .. wood_required .. " wood. Remaining: " .. self.current_stack_size)
        
        if self.current_stack_size <= 0 then
            return "consumed"  -- Item is used up
        end
        
        return true
    else
        print("Not enough wood! Need " .. wood_required .. ", have " .. self.current_stack_size)
        return false
    end
end

-- Called when wood is used as fuel
function wood:on_use_fuel(fire_source, world)
    print("Using wood as fuel")
    
    -- One piece of wood used for fuel
    self.current_stack_size = self.current_stack_size - 1
    
    if self.current_stack_size <= 0 then
        return "consumed", self.burn_time, self.fuel_value
    end
    
    return true, self.burn_time, self.fuel_value
end

-- Called when wood is used in crafting
function wood:on_use_crafting(recipe, crafter, world)
    print("Using wood for crafting: " .. recipe)
    
    local wood_needed = self:get_recipe_cost(recipe)
    if self.current_stack_size >= wood_needed then
        self.current_stack_size = self.current_stack_size - wood_needed
        
        if self.current_stack_size <= 0 then
            return "consumed"
        end
        
        return true
    end
    
    return false
end

-- Called each update tick (for decay, etc.)
function wood:on_update(world, dt)
    -- Handle outdoor decay if not in proper storage
    if not self.in_covered_storage and self.requires_covered_storage == false then
        self.decay_timer = (self.decay_timer or 0) + dt
        
        if self.decay_timer >= 86400 then  -- Once per game day
            if math.random() < self.decay_rate then
                self.current_stack_size = math.max(0, self.current_stack_size - 1)
                print("Wood stack decayed by 1. Remaining: " .. self.current_stack_size)
                
                if self.current_stack_size <= 0 then
                    return "destroy"
                end
            end
            self.decay_timer = 0
        end
    end
    
    return true
end

-- Get wood cost for construction types
function wood:get_construction_cost(construction_type)
    local costs = {
        wooden_wall = 3,
        wooden_door = 5,
        wooden_floor = 2,
        wooden_stairs = 4,
        wooden_fence = 2,
        campfire = 3,
        wooden_chest = 8
    }
    
    return costs[construction_type] or 1
end

-- Get wood cost for crafting recipes
function wood:get_recipe_cost(recipe)
    local costs = {
        wooden_shield = 5,
        wooden_spear = 3,
        bow = 4,
        arrows = 1  -- Per 10 arrows
    }
    
    return costs[recipe] or 2
end

-- Get current item information
function wood:get_info()
    return {
        name = self.name,
        amount = self.current_stack_size,
        max_stack = self.stack_size,
        weight_total = self.current_stack_size * self.weight,
        value_total = self.current_stack_size * self.base_value,
        material_type = self.material_type,
        uses = {
            construction = self.construction_material,
            crafting = self.crafting_material,
            fuel = self.fuel_value > 0
        }
    }
end

return wood