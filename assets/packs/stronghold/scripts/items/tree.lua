-- Tree Item Script
-- Natural resource node that provides wood

local tree = {
    id = "tree",
    name = "Tree", 
    category = "resource_node",
    type = "natural_resource",
    
    -- Visual representation
    sprite = "tree",
    size = 1,  -- 1x1 tile
    
    -- Resource properties
    resource_type = "wood",
    resource_amount = 10,  -- Wood available in this tree
    max_resource_amount = 10,
    
    -- Harvesting properties
    harvest_tool_required = "axe",
    harvest_skill_required = "woodcutting",
    harvest_time = 3.0,  -- Seconds to harvest
    harvest_yield = 2,   -- Wood per harvest action
    
    -- Tree properties
    regrowth_time = 60,  -- Seconds to regrow if not fully harvested
    can_regrow = true,
    fully_harvested_becomes = nil,  -- Tree disappears when fully harvested
    
    -- Blocking properties
    blocks_movement = true,
    blocks_construction = true,
    
    -- Environment
    provides_shelter = false,
    flammable = true
}

-- Called when tree is spawned/placed
function tree:on_spawn(world, x, y)
    print("Tree spawned at (" .. x .. ", " .. y .. ")")
    
    -- Random variation in resource amount
    local variation = math.random(-2, 3)
    self.resource_amount = math.max(1, self.max_resource_amount + variation)
    
    return true
end

-- Called when someone starts harvesting
function tree:on_harvest_started(harvester, world)
    print("Harvesting started on tree by " .. (harvester.name or "unknown"))
    
    -- Check if harvester has required tool
    if self.harvest_tool_required and harvester.inventory then
        if not harvester.inventory:has_item(self.harvest_tool_required) then
            print("Harvester lacks required tool: " .. self.harvest_tool_required)
            return false
        end
    end
    
    return true
end

-- Called when harvest action completes
function tree:on_harvested(harvester, world)
    if self.resource_amount <= 0 then
        print("Tree already fully harvested")
        return false
    end
    
    -- Calculate yield based on harvester skill
    local base_yield = self.harvest_yield
    local actual_yield = base_yield
    
    if harvester.skills and harvester.skills.woodcutting then
        local skill_bonus = math.floor(harvester.skills.woodcutting / 3)
        actual_yield = base_yield + skill_bonus
    end
    
    -- Don't yield more than what's available
    actual_yield = math.min(actual_yield, self.resource_amount)
    
    -- Update tree resource
    self.resource_amount = self.resource_amount - actual_yield
    
    print("Tree harvested for " .. actual_yield .. " wood. Remaining: " .. self.resource_amount)
    
    -- Give wood to harvester
    if harvester.inventory then
        harvester.inventory:add_item("wood", actual_yield)
    elseif world.add_resource then
        -- Drop wood on ground
        world:add_resource("wood", actual_yield, harvester.x, harvester.y)
    end
    
    -- Check if tree is fully harvested
    if self.resource_amount <= 0 then
        print("Tree fully harvested - will be removed")
        return "destroy"  -- Signal that tree should be removed
    end
    
    return true
end

-- Called each update tick
function tree:on_update(world, dt)
    -- Handle regrowth if not fully harvested
    if self.can_regrow and self.resource_amount < self.max_resource_amount then
        self.regrowth_timer = (self.regrowth_timer or 0) + dt
        
        if self.regrowth_timer >= self.regrowth_time then
            self.resource_amount = math.min(self.max_resource_amount, self.resource_amount + 1)
            self.regrowth_timer = 0
            print("Tree regrew some wood. Now has: " .. self.resource_amount)
        end
    end
    
    return true
end

-- Called when fire reaches this tree
function tree:on_fire(world)
    if self.flammable then
        print("Tree caught fire!")
        -- Tree burns down and might spread fire
        return "destroy"
    end
    return true
end

-- Get current harvest information
function tree:get_harvest_info()
    return {
        resource_type = self.resource_type,
        available_amount = self.resource_amount,
        max_amount = self.max_resource_amount,
        tool_required = self.harvest_tool_required,
        estimated_yield = self.harvest_yield,
        harvest_time = self.harvest_time
    }
end

return tree