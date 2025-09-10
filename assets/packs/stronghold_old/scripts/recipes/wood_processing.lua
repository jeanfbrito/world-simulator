-- Wood Processing Recipe Script
-- This script defines wood-related recipes for the simulation

-- Function called when the script is loaded
function on_init()
    print("Loading wood processing recipe script...")
    register_recipes()
end

-- Register all recipes defined in this script
function register_recipes()
    -- Create a recipe for wood to planks
    local wood_planks = create_recipe("wood_to_planks")
    wood_planks:set_name("Wood to Planks")
    wood_planks:add_input("Wood", 2)
    wood_planks:add_output("Planks", 3)
    wood_planks:set_duration(10)
    wood_planks:set_building("Sawmill")
    
    -- Register the recipe
    registry:add_recipe(wood_planks)
    
    -- Create a dynamic recipe with skill-based output
    local advanced_planks = create_recipe("advanced_wood_processing")
    advanced_planks:set_name("Advanced Wood Processing")
    advanced_planks:add_input("Wood", 3)
    advanced_planks:add_output("Planks", 5)
    advanced_planks:set_duration(15)
    advanced_planks:set_building("Sawmill")
    
    -- Add a completion callback for bonus output
    advanced_planks:on_complete(function(worker, inventory)
        local skill = worker:get_skill_level()
        if skill > 5 then
            -- Skilled workers get bonus output
            inventory:add_resource("Planks", 1)
            print("Skilled worker produced bonus planks!")
        end
        
        -- Random chance for rare material
        if math.random() < 0.1 then
            inventory:add_resource("RareWood", 1)
            print("Found rare wood!")
        end
    end)
    
    registry:add_recipe(advanced_planks)
end

-- Function called when recipe is started
function on_recipe_start(recipe_id, worker)
    print("Worker " .. worker:get_name() .. " started recipe: " .. recipe_id)
    
    -- Apply worker-specific modifiers
    if worker:has_trait("careful") then
        return { duration_modifier = 1.2, quality_bonus = 0.1 }
    elseif worker:has_trait("hasty") then
        return { duration_modifier = 0.8, quality_penalty = 0.1 }
    end
    
    return {}
end

-- Function called when recipe is completed
function on_recipe_complete(recipe_id, worker, products)
    print("Recipe completed: " .. recipe_id)
    
    -- Check for special conditions
    local time_of_day = world:get_time_of_day()
    if time_of_day == "night" then
        -- Night shift penalty
        worker:add_fatigue(5)
    end
    
    -- Track statistics
    stats:increment("recipes_completed", 1)
    stats:increment("recipes_" .. recipe_id, 1)
end