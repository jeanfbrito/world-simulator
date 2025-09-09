-- Food Production Recipes
-- Dynamic recipes for food preparation and cooking

function on_init()
    print("Loading food production recipes...")
    register_recipes()
end

function register_recipes()
    -- Basic bread recipe
    local bread = create_recipe("wheat_to_bread")
    bread:set_name("Bake Bread")
    bread:add_input("Wheat", 3)
    bread:add_output("Food", 5)
    bread:set_duration(15)
    bread:set_building("Bakery")
    registry:add_recipe(bread)
    
    -- Advanced cooking with multiple ingredients
    local stew = create_recipe("hearty_stew")
    stew:set_name("Hearty Stew")
    stew:add_input("Food", 2)
    stew:add_input("Wheat", 1)
    stew:add_output("Food", 8)
    stew:set_duration(20)
    stew:set_building("Bakery")
    registry:add_recipe(stew)
    
    -- Preservation recipe
    local preserved = create_recipe("preserve_food")
    preserved:set_name("Preserve Food")
    preserved:add_input("Food", 5)
    preserved:add_output("PreservedFood", 4)
    preserved:set_duration(25)
    preserved:set_building("Bakery")
    registry:add_recipe(preserved)
end

-- Quality variations based on cook skill
function on_recipe_complete(recipe_id, worker, inventory)
    local cooking_skill = worker:get_skill("cooking") or 0
    
    if recipe_id == "wheat_to_bread" then
        -- Quality variations
        local quality = calculate_food_quality(cooking_skill)
        
        if quality > 0.9 then
            -- Excellent quality
            inventory:add_resource("Food", 1)
            worker:add_reputation("master_baker", 1)
            print("Exceptional bread baked!")
            
        elseif quality < 0.3 then
            -- Poor quality
            inventory:remove_resource("Food", 1)
            print("Some bread was burnt...")
        end
        
    elseif recipe_id == "hearty_stew" then
        -- Complex recipe benefits more from skill
        if cooking_skill > 6 then
            inventory:add_resource("Food", 2)
            
            -- Chance for morale boost
            local nearby_workers = world:get_workers_in_radius(worker:get_position(), 10)
            for _, w in ipairs(nearby_workers) do
                w:add_morale(5)
            end
            print("The delicious smell boosts morale!")
        end
    end
    
    -- Improve cooking skill
    worker:improve_skill("cooking", 0.15)
end

-- Calculate food quality based on skill
function calculate_food_quality(skill)
    local base_quality = 0.5
    local skill_bonus = skill * 0.05
    local random_factor = (math.random() - 0.5) * 0.2
    
    return math.min(1.0, math.max(0, base_quality + skill_bonus + random_factor))
end

-- Time of day affects cooking
function on_recipe_start(recipe_id, worker)
    local time_of_day = world:get_time_of_day()
    
    if time_of_day == "morning" then
        -- Fresh ingredients in morning
        return { quality_bonus = 0.1 }
        
    elseif time_of_day == "night" then
        -- Tired workers at night
        return { 
            duration_modifier = 1.2,
            quality_penalty = 0.1
        }
    end
    
    return {}
end

-- Special event recipes
function on_event(event_type, event_data)
    if event_type == "harvest_festival" then
        -- Special feast recipe during harvest
        local feast = create_recipe("harvest_feast")
        feast:set_name("Harvest Feast")
        feast:add_input("Food", 10)
        feast:add_input("Wheat", 5)
        feast:add_output("Feast", 1)
        feast:set_duration(30)
        feast:set_building("Bakery")
        
        -- Feast provides settlement-wide bonus
        feast:on_complete(function(worker, inventory)
            world:broadcast_effect("well_fed", 100) -- Duration in ticks
            world:add_settlement_morale(20)
            print("The harvest feast brings joy to all!")
        end)
        
        registry:add_temporary_recipe(feast, 200) -- Available for limited time
    end
end

-- Spoilage system
function on_tick()
    -- Check for food spoilage
    local inventories = world:get_all_inventories()
    
    for _, inv in ipairs(inventories) do
        local food_age = inv:get_resource_age("Food")
        
        if food_age > 100 then
            -- Food spoils over time
            local spoilage = math.floor(food_age / 100) * 0.1
            local spoiled = math.floor(inv:get_resource("Food") * spoilage)
            
            if spoiled > 0 then
                inv:remove_resource("Food", spoiled)
                print("Food spoiled: " .. spoiled)
            end
        end
    end
end