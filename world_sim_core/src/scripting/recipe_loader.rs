//! Recipe script loader for dynamic recipe definitions

use bevy_ecs::prelude::*;
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_lua::prelude::*;
use crate::recipes::RecipeRegistry;
use super::{RecipeScriptEvent, RecipeEventType, ScriptEventData};

/// Component to mark entities with recipe scripts
#[derive(Component)]
pub struct RecipeScript {
    pub script_path: String,
    pub loaded: bool,
}

/// Command event to trigger script reloading
#[derive(Event)]
pub struct ReloadRecipeScriptsCommand;

/// System to load recipe scripts on command only
pub fn load_recipe_scripts(
    mut commands: Commands,
    mut reload_events: EventReader<ReloadRecipeScriptsCommand>,
    asset_server: Res<AssetServer>,
    existing_scripts: Query<Entity, With<RecipeScript>>,
) {
    // Only load when commanded
    if reload_events.is_empty() {
        return;
    }
    
    reload_events.clear();
    
    // Clear existing scripts first
    for entity in existing_scripts.iter() {
        commands.entity(entity).despawn();
    }
    
    // Load all recipe scripts from the assets directory
    let recipe_scripts = vec![
        "scripts/recipes/wood_processing.lua",
        "scripts/recipes/food_production.lua",
        "scripts/recipes/tool_crafting.lua",
        "scripts/recipes/building_materials.lua",
    ];
    
    for script_path in recipe_scripts {
        // Create an entity with the script component
        commands.spawn((
            RecipeScript {
                script_path: script_path.to_string(),
                loaded: false,
            },
            ScriptCollection::<LuaScript>::default(),
        ));
        
        tracing::info!("Loading recipe script: {}", script_path);
    }
}

/// System to process loaded recipe scripts
pub fn process_recipe_scripts(
    mut commands: Commands,
    mut scripts: Query<(Entity, &mut RecipeScript, &ScriptCollection<LuaScript>)>,
    mut recipe_registry: ResMut<RecipeRegistry>,
    mut events: EventWriter<RecipeScriptEvent>,
) {
    for (entity, mut script, collection) in scripts.iter_mut() {
        if !script.loaded && !collection.scripts.is_empty() {
            script.loaded = true;
            
            // Trigger script loaded event
            events.send(RecipeScriptEvent {
                recipe_id: script.script_path.clone(),
                event_type: RecipeEventType::Started,
                data: ScriptEventData::default(),
            });
            
            tracing::debug!("Recipe script loaded: {}", script.script_path);
        }
    }
}

/// Example Lua script template for recipes
pub const RECIPE_SCRIPT_TEMPLATE: &str = r#"
-- Recipe Definition Script
-- This script defines recipes that can be crafted in the game

-- Function called when the script is loaded
function on_init()
    print("Loading recipe script...")
    
    -- Register recipes with the game
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

-- Function called on recipe failure
function on_recipe_fail(recipe_id, worker, reason)
    print("Recipe failed: " .. recipe_id .. " - " .. reason)
    
    -- Return some materials on failure
    if reason == "interrupted" then
        -- Return 50% of materials
        return 0.5
    elseif reason == "lack_skill" then
        -- Return 25% of materials
        return 0.25
    end
    
    return 0
end

-- Seasonal recipe modifications
function on_season_change(season)
    if season == "winter" then
        -- Reduce efficiency in winter
        registry:modify_all_recipes({
            duration_modifier = 1.3,
            output_modifier = 0.9
        })
    elseif season == "summer" then
        -- Increase efficiency in summer
        registry:modify_all_recipes({
            duration_modifier = 0.9,
            output_modifier = 1.1
        })
    end
end

-- Special event recipes
function on_event(event_type, event_data)
    if event_type == "festival" then
        -- Unlock special festival recipes
        local festival_recipe = create_recipe("festival_feast")
        festival_recipe:set_name("Festival Feast")
        festival_recipe:add_input("Food", 5)
        festival_recipe:add_input("Wheat", 3)
        festival_recipe:add_output("Feast", 1)
        festival_recipe:set_duration(20)
        festival_recipe:set_building("Bakery")
        
        registry:add_temporary_recipe(festival_recipe, 100) -- Available for 100 ticks
    end
end
"#;

/// Create a recipe script file
pub fn create_recipe_script_file(path: &str, content: &str) -> std::io::Result<()> {
    use std::fs;
    use std::path::Path;
    
    let full_path = Path::new("assets").join(path);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(full_path, content)?;
    Ok(())
}