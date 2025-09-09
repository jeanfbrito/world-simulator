-- Wood Processing Recipes
-- Dynamic recipes for converting wood into various products

-- Initialize the recipe module
function on_init()
    print("Loading wood processing recipes...")
    register_recipes()
end

-- Register all wood-related recipes
function register_recipes()
    -- Basic wood to planks
    local wood_planks = create_recipe("wood_to_planks")
    wood_planks:set_name("Wood to Planks")
    wood_planks:add_input("Wood", 2)
    wood_planks:add_output("Planks", 3)
    wood_planks:set_duration(10)
    wood_planks:set_building("Sawmill")
    registry:add_recipe(wood_planks)
    
    -- Advanced wood processing with skill bonus
    local advanced_planks = create_recipe("advanced_wood_processing")
    advanced_planks:set_name("Advanced Wood Processing")
    advanced_planks:add_input("Wood", 3)
    advanced_planks:add_output("Planks", 5)
    advanced_planks:set_duration(15)
    advanced_planks:set_building("Sawmill")
    registry:add_recipe(advanced_planks)
    
    -- Efficient bulk processing
    local bulk_processing = create_recipe("bulk_wood_processing")
    bulk_processing:set_name("Bulk Wood Processing")
    bulk_processing:add_input("Wood", 10)
    bulk_processing:add_output("Planks", 18)
    bulk_processing:set_duration(40)
    bulk_processing:set_building("Sawmill")
    registry:add_recipe(bulk_processing)
end

-- Handle recipe completion with skill-based bonuses
function on_recipe_complete(recipe_id, worker, inventory)
    local skill = worker:get_skill_level()
    
    if recipe_id == "advanced_wood_processing" then
        -- Skilled workers get bonus output
        if skill > 5 then
            inventory:add_resource("Planks", 1)
            print("Skilled worker produced bonus planks!")
        end
        
        -- Random chance for rare wood
        if math.random() < 0.1 then
            inventory:add_resource("RareWood", 1)
            print("Found rare wood during processing!")
        end
        
    elseif recipe_id == "bulk_wood_processing" then
        -- Efficiency bonus for experienced workers
        if skill > 7 then
            inventory:add_resource("Planks", 2)
            print("Expert efficiency bonus!")
        end
    end
    
    -- Skill improvement
    worker:improve_skill("woodworking", 0.1)
end

-- Modify recipes based on tool quality
function on_recipe_start(recipe_id, worker)
    local tool_quality = worker:get_tool_quality("saw")
    
    if tool_quality > 0.8 then
        -- High quality tools work faster
        return { duration_modifier = 0.8 }
    elseif tool_quality < 0.3 then
        -- Poor tools work slower and may fail
        return { 
            duration_modifier = 1.5,
            failure_chance = 0.1
        }
    end
    
    return {}
end

-- Seasonal adjustments
function on_season_change(season)
    if season == "winter" then
        -- Wood is harder to work in cold
        registry:modify_recipe("wood_to_planks", {
            duration_modifier = 1.2
        })
    elseif season == "summer" then
        -- Optimal conditions
        registry:modify_recipe("wood_to_planks", {
            duration_modifier = 0.9,
            output_modifier = 1.1
        })
    end
end