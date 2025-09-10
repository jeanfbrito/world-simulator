-- Stockpile Area Script
-- Designated ground area for storing resources (like Stronghold 1)

local stockpile = {
    id = "stockpile",
    name = "Stockpile Area",
    category = "storage",
    type = "designation",  -- Not a building, but a designated area
    
    -- Placement properties (no construction needed)
    placement_cost = {
        gold = 0,  -- Free to designate
        time = 0   -- Instant designation
    },
    area_size = { width = 2, height = 2 },  -- 2x2 designated area
    
    -- Storage properties
    storage_capacity = 200,  -- Higher capacity for designated areas
    accepts_resources = {
        "wood",
        "stone", 
        "iron",
        "food",
        "coal",
        "tools",
        "weapons",
        "gold",
        "leather",
        "pitch",
        "ale",
        "bread",
        "meat",
        "apples",
        "cheese"
    },
    
    -- Area properties (not building)
    health = nil,           -- Can't be destroyed
    defense = 0,
    blocks_movement = false,  -- Can walk through stockpiles
    blocks_construction = true,  -- Can't build on stockpiles
    
    -- Workers automatically use stockpiles
    auto_managed = true,     -- Peasants automatically haul to/from
    worker_range = 10,       -- Distance peasants will travel to use this
    
    -- Visual properties
    sprite_base = "stockpile_ground",  -- Ground marking
    pile_sprites = {         -- Different sprites for resource piles
        wood = "wood_pile",
        stone = "stone_pile",
        iron = "iron_pile",
        food = "food_pile",
        gold = "gold_pile"
    },
    max_pile_height = 5,     -- Visual stacking limit
    
    -- Weather exposure (true Stronghold 1 behavior)
    outdoor_storage = true,
    decay_protection = false,
    
    -- Current storage and visual state
    current_storage = {},    -- { resource_type = amount }
    current_capacity_used = 0,
    pile_heights = {},       -- { resource_type = visual_height }
    pile_positions = {}      -- Where within area each pile is located
}

-- Called when stockpile area is designated
function stockpile:on_designated(world, x, y, designator)
    print("Stockpile area designated at (" .. x .. ", " .. y .. ") by " .. (designator.name or "lord"))
    
    -- Initialize storage and visual state
    self.current_storage = {}
    self.current_capacity_used = 0
    self.pile_heights = {}
    self.pile_positions = {}
    self.area_center = { x = x, y = y }
    
    -- Mark the area as designated stockpile
    if world.mark_area then
        world:mark_area(x, y, self.area_size.width, self.area_size.height, "stockpile")
    end
    
    return true
end

-- Called when someone tries to store items
function stockpile:store_item(item_type, amount, depositor)
    -- Check if we accept this resource type
    local accepts = false
    for _, accepted in ipairs(self.accepts_resources) do
        if accepted == item_type then
            accepts = true
            break
        end
    end
    
    if not accepts then
        print("Stockpile area doesn't accept " .. item_type)
        return false
    end
    
    -- Check capacity
    if self.current_capacity_used + amount > self.storage_capacity then
        local available_space = self.storage_capacity - self.current_capacity_used
        print("Stockpile area full! Can only store " .. available_space .. " more items")
        
        if available_space <= 0 then
            return false
        end
        
        -- Store what we can
        amount = available_space
    end
    
    -- Store the items
    local old_amount = self.current_storage[item_type] or 0
    self.current_storage[item_type] = old_amount + amount
    self.current_capacity_used = self.current_capacity_used + amount
    
    -- Update visual pile height (Stronghold 1 behavior)
    self:update_pile_visual(item_type)
    
    -- Choose pile position if new resource type
    if old_amount == 0 then
        self:assign_pile_position(item_type)
    end
    
    print("Stored " .. amount .. " " .. item_type .. " in stockpile area. Total " .. item_type .. ": " .. self.current_storage[item_type])
    print("Stockpile area usage: " .. self.current_capacity_used .. "/" .. self.storage_capacity)
    
    -- Log visual pile growth like in Stronghold 1
    local pile_height = self.pile_heights[item_type] or 1
    print("Pile of " .. item_type .. " now " .. pile_height .. " units high")
    
    return amount  -- Return amount actually stored
end

-- Called when someone tries to retrieve items
function stockpile:retrieve_item(item_type, amount, retriever)
    local available = self.current_storage[item_type] or 0
    
    if available <= 0 then
        print("No " .. item_type .. " available in stockpile area")
        return false
    end
    
    -- Take what's available or what was requested
    local taken = math.min(amount, available)
    
    self.current_storage[item_type] = available - taken
    self.current_capacity_used = self.current_capacity_used - taken
    
    -- Update visual pile height (shrink pile like in Stronghold 1)
    self:update_pile_visual(item_type)
    
    -- Remove pile position if completely empty
    if self.current_storage[item_type] <= 0 then
        self.pile_positions[item_type] = nil
        self.pile_heights[item_type] = nil
        print("Pile of " .. item_type .. " completely depleted")
    else
        local pile_height = self.pile_heights[item_type] or 1
        print("Pile of " .. item_type .. " now " .. pile_height .. " units high")
    end
    
    print("Retrieved " .. taken .. " " .. item_type .. " from stockpile area. Remaining: " .. self.current_storage[item_type])
    
    return taken
end

-- Get available amount of a resource
function stockpile:get_available(item_type)
    return self.current_storage[item_type] or 0
end

-- Get total available space
function stockpile:get_free_space()
    return self.storage_capacity - self.current_capacity_used
end

-- Visual pile management functions (Stronghold 1 behavior)
function stockpile:update_pile_visual(item_type)
    local amount = self.current_storage[item_type] or 0
    
    if amount <= 0 then
        self.pile_heights[item_type] = nil
        return
    end
    
    -- Calculate pile height based on amount (every 10 items = 1 height level)
    local height = math.min(math.ceil(amount / 10), self.max_pile_height)
    self.pile_heights[item_type] = height
end

function stockpile:assign_pile_position(item_type)
    -- Assign position within the stockpile area for this resource type
    local area_width = self.area_size.width
    local area_height = self.area_size.height
    local center = self.area_center
    
    -- Simple algorithm: spread piles across the area
    local pile_count = 0
    for _ in pairs(self.pile_positions) do
        pile_count = pile_count + 1
    end
    
    local positions = {
        {x = center.x - 1, y = center.y - 1},  -- Top-left
        {x = center.x + 1, y = center.y - 1},  -- Top-right
        {x = center.x - 1, y = center.y + 1},  -- Bottom-left
        {x = center.x + 1, y = center.y + 1},  -- Bottom-right
    }
    
    local pos_index = (pile_count % #positions) + 1
    self.pile_positions[item_type] = positions[pos_index]
    
    print("Assigned pile position for " .. item_type .. " at (" .. 
          positions[pos_index].x .. ", " .. positions[pos_index].y .. ")")
end

-- Auto-management: Peasants automatically haul to/from stockpiles within range
function stockpile:auto_collect_nearby(world)
    -- Look for nearby dropped items to collect (automatic behavior)
    if world.find_nearby_items then
        local nearby_items = world:find_nearby_items(self.area_center, self.worker_range)
        
        for _, item in ipairs(nearby_items) do
            if self:can_accept(item.type) then
                local stored = self:store_item(item.type, item.amount, nil)
                if stored and stored > 0 then
                    -- Remove the item from the world
                    world:remove_item(item)
                    print("Auto-collected " .. stored .. " " .. item.type .. " into stockpile area")
                end
            end
        end
    end
end

-- Check if we can accept this item type
function stockpile:can_accept(item_type)
    for _, accepted in ipairs(self.accepts_resources) do
        if accepted == item_type then
            return true
        end
    end
    return false
end

-- Called each update tick
function stockpile:on_update(world, dt)
    -- Auto-collect nearby items (Stronghold 1 behavior)
    if self.auto_managed then
        self:auto_collect_nearby(world)
    end
    
    -- Handle item decay for outdoor storage (true to Stronghold 1)
    if self.outdoor_storage and not self.decay_protection then
        for item_type, amount in pairs(self.current_storage) do
            if amount > 0 then
                -- Very slow decay for outdoor storage
                local decay_chance = 0.001 * dt  -- Very small chance per second
                if math.random() < decay_chance then
                    self.current_storage[item_type] = math.max(0, amount - 1)
                    self.current_capacity_used = self.current_capacity_used - 1
                    
                    -- Update pile visual when items decay
                    self:update_pile_visual(item_type)
                    
                    print("Stockpile area: 1 " .. item_type .. " decayed from weather")
                end
            end
        end
    end
    
    return true
end

-- Get stockpile status information
function stockpile:get_status()
    local total_items = 0
    local item_types = 0
    
    for item_type, amount in pairs(self.current_storage) do
        if amount > 0 then
            total_items = total_items + amount
            item_types = item_types + 1
        end
    end
    
    return {
        capacity_used = self.current_capacity_used,
        capacity_total = self.storage_capacity,
        free_space = self.storage_capacity - self.current_capacity_used,
        item_types_stored = item_types,
        total_items = total_items,
        storage_contents = self.current_storage,
        accepts = self.accepts_resources
    }
end

-- Called when stockpile area is removed/cancelled
function stockpile:on_designation_removed(world)
    -- Drop all stored items on the ground (scatter around the area)
    if world.spawn_item then
        local center = self.area_center
        local area_width = self.area_size.width
        local area_height = self.area_size.height
        
        for item_type, amount in pairs(self.current_storage) do
            if amount > 0 then
                -- Scatter items across the stockpile area
                local scatter_x = center.x + math.random(-area_width/2, area_width/2)
                local scatter_y = center.y + math.random(-area_height/2, area_height/2)
                
                world:spawn_item(item_type, amount, scatter_x, scatter_y)
                print("Scattered " .. amount .. " " .. item_type .. " from removed stockpile area")
            end
        end
    end
    
    -- Clear area designation
    if world.clear_area then
        world:clear_area(self.area_center.x, self.area_center.y, 
                         self.area_size.width, self.area_size.height, "stockpile")
    end
    
    print("Stockpile area designation removed, contents scattered")
    return true
end

-- Get visual information for rendering (Stronghold 1 pile rendering)
function stockpile:get_visual_info()
    local piles = {}
    
    for item_type, position in pairs(self.pile_positions) do
        local height = self.pile_heights[item_type] or 1
        local sprite = self.pile_sprites[item_type] or "generic_pile"
        local amount = self.current_storage[item_type] or 0
        
        if amount > 0 then
            table.insert(piles, {
                type = item_type,
                position = position,
                height = height,
                sprite = sprite,
                amount = amount
            })
        end
    end
    
    return {
        area_center = self.area_center,
        area_size = self.area_size,
        ground_sprite = self.sprite_base,
        piles = piles,
        total_items = self.current_capacity_used,
        total_capacity = self.storage_capacity
    }
end

return stockpile