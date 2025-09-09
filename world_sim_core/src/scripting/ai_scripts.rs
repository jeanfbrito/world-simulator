//! AI behavior scripts for dynamic personality and decision making

use bevy_ecs::prelude::*;
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_lua::prelude::*;
use crate::components::WorkerComponent;
use crate::ai::AICoordinator;
use super::{AIScriptEvent, AIEventType, ScriptEventData};

/// Component to mark entities with AI behavior scripts
#[derive(Component)]
pub struct AIBehaviorScript {
    pub script_path: String,
    pub script_type: AIScriptType,
    pub loaded: bool,
}

#[derive(Debug, Clone)]
pub enum AIScriptType {
    Personality,  // Defines personality traits
    Goals,        // Modifies goal priorities
    Decisions,    // Custom decision logic
    Reactions,    // Event reactions
}

/// Command event to trigger AI script reloading
#[derive(Event)]
pub struct ReloadAIScriptsCommand;

/// System to load AI scripts on command
pub fn load_ai_scripts(
    mut commands: Commands,
    mut reload_events: EventReader<ReloadAIScriptsCommand>,
    asset_server: Res<AssetServer>,
    existing_scripts: Query<Entity, With<AIBehaviorScript>>,
) {
    // Only load when commanded
    if reload_events.is_empty() {
        return;
    }
    
    reload_events.clear();
    
    // Clear existing scripts
    for entity in existing_scripts.iter() {
        commands.entity(entity).despawn();
    }
    
    // Load AI behavior scripts
    let ai_scripts = vec![
        ("scripts/ai/personality_traits.lua", AIScriptType::Personality),
        ("scripts/ai/seasonal_goals.lua", AIScriptType::Goals),
        ("scripts/ai/smart_decisions.lua", AIScriptType::Decisions),
        ("scripts/ai/danger_reactions.lua", AIScriptType::Reactions),
    ];
    
    for (script_path, script_type) in ai_scripts {
        commands.spawn((
            AIBehaviorScript {
                script_path: script_path.to_string(),
                script_type,
                loaded: false,
            },
            ScriptCollection::<LuaScript>::default(),
        ));
        
        tracing::info!("Loading AI script: {}", script_path);
    }
}

/// System to process AI scripts
pub fn process_ai_scripts(
    mut scripts: Query<(Entity, &mut AIBehaviorScript, &ScriptCollection<LuaScript>)>,
    mut events: EventWriter<AIScriptEvent>,
) {
    for (entity, mut script, collection) in scripts.iter_mut() {
        if !script.loaded && !collection.scripts.is_empty() {
            script.loaded = true;
            
            // Trigger script loaded event
            events.send(AIScriptEvent {
                entity,
                event_type: AIEventType::BehaviorChange,
                data: ScriptEventData::default(),
            });
            
            tracing::debug!("AI script loaded: {}", script.script_path);
        }
    }
}

/// Apply personality traits from scripts to workers
pub fn apply_personality_scripts(
    workers: Query<(Entity, &WorkerComponent, &AICoordinator)>,
    scripts: Query<&AIBehaviorScript>,
    mut events: EventWriter<AIScriptEvent>,
) {
    // This system would execute personality trait scripts
    // and apply their effects to workers
}

/// Example Lua script templates for AI behaviors
pub mod templates {
    pub const PERSONALITY_TRAITS_SCRIPT: &str = r#"
-- Personality Traits Script
-- Defines various personality traits that affect worker behavior

-- Trait definitions
local traits = {
    lazy = {
        name = "Lazy",
        description = "Works slower but needs less supervision",
        modifiers = {
            work_speed = 0.8,
            fatigue_rate = 0.9,
            rest_need = 1.2,
            social_need = 0.7
        }
    },
    
    industrious = {
        name = "Industrious", 
        description = "Works faster but gets tired quicker",
        modifiers = {
            work_speed = 1.25,
            fatigue_rate = 1.15,
            skill_gain = 1.2,
            hunger_rate = 1.1
        }
    },
    
    social = {
        name = "Social",
        description = "Works better near others",
        modifiers = {
            work_speed = 1.0,
            morale_from_social = 2.0,
            work_alone_penalty = 0.7
        }
    },
    
    loner = {
        name = "Loner",
        description = "Prefers working alone",
        modifiers = {
            work_speed = 1.1,
            work_alone_bonus = 1.3,
            social_need = 0.5,
            crowd_penalty = 0.8
        }
    },
    
    perfectionist = {
        name = "Perfectionist",
        description = "Produces higher quality but works slower",
        modifiers = {
            work_speed = 0.85,
            quality_bonus = 1.5,
            skill_gain = 1.3,
            stress_from_failure = 2.0
        }
    }
}

-- Function to assign random trait to a worker
function assign_random_trait(worker)
    local trait_list = {"lazy", "industrious", "social", "loner", "perfectionist"}
    local index = math.random(1, #trait_list)
    local trait_name = trait_list[index]
    local trait = traits[trait_name]
    
    worker:add_trait(trait_name)
    
    -- Apply modifiers
    for stat, value in pairs(trait.modifiers) do
        worker:modify_stat(stat, value)
    end
    
    print("Assigned trait '" .. trait.name .. "' to " .. worker:get_name())
    return trait_name
end

-- Function to check trait compatibility for teamwork
function check_compatibility(worker1, worker2)
    local traits1 = worker1:get_traits()
    local traits2 = worker2:get_traits()
    
    -- Social workers work well together
    if traits1:has("social") and traits2:has("social") then
        return 1.5 -- 50% bonus
    end
    
    -- Loners don't work well with social workers
    if (traits1:has("loner") and traits2:has("social")) or
       (traits1:has("social") and traits2:has("loner")) then
        return 0.7 -- 30% penalty
    end
    
    -- Industrious workers inspire lazy ones slightly
    if traits1:has("industrious") and traits2:has("lazy") then
        return 1.1
    end
    
    return 1.0 -- Neutral
end

-- Apply trait effects based on situation
function on_work_start(worker, task)
    local traits = worker:get_traits()
    
    -- Perfectionist takes longer to start
    if traits:has("perfectionist") then
        task:add_prep_time(5)
    end
    
    -- Check if working alone or in group
    local nearby_workers = worker:get_nearby_workers(10)
    
    if #nearby_workers == 0 then
        if traits:has("loner") then
            worker:apply_temp_modifier("work_speed", 1.2)
        elseif traits:has("social") then
            worker:apply_temp_modifier("work_speed", 0.8)
        end
    else
        if traits:has("social") then
            local compatibility_sum = 0
            for _, other in ipairs(nearby_workers) do
                compatibility_sum = compatibility_sum + check_compatibility(worker, other)
            end
            local avg_compatibility = compatibility_sum / #nearby_workers
            worker:apply_temp_modifier("work_speed", avg_compatibility)
        end
    end
end
"#;

    pub const SEASONAL_GOALS_SCRIPT: &str = r#"
-- Seasonal Goals Script
-- Adjusts AI goal priorities based on season

local season_goals = {
    spring = {
        gather_food = 1.5,
        farming = 2.0,
        building = 1.2,
        wood_cutting = 0.8,
        socializing = 1.3
    },
    
    summer = {
        gather_food = 1.2,
        farming = 1.5,
        building = 1.5,
        wood_cutting = 0.7,
        socializing = 1.5
    },
    
    autumn = {
        gather_food = 2.0,  -- Prepare for winter
        farming = 1.8,      -- Last harvest
        building = 1.0,
        wood_cutting = 1.5, -- Stock up for winter
        socializing = 1.0
    },
    
    winter = {
        gather_food = 0.5,  -- Limited availability
        farming = 0.0,      -- Can't farm
        building = 0.5,     -- Too cold
        wood_cutting = 2.0, -- Need for heating
        socializing = 1.8,  -- Stay indoors together
        crafting = 1.5      -- Indoor activity
    }
}

-- Apply seasonal modifiers to worker goals
function apply_seasonal_goals(worker, season)
    local goals = season_goals[season]
    
    if not goals then
        print("Unknown season: " .. season)
        return
    end
    
    -- Update worker's goal priorities
    for goal, priority in pairs(goals) do
        worker:set_goal_priority(goal, priority)
    end
    
    -- Special winter behavior
    if season == "winter" then
        -- Reduce outdoor activities
        worker:set_preference("indoor_tasks", true)
        
        -- Increase need for warmth
        worker:add_need("warmth", 1.0)
        
        -- Group together for warmth
        if worker:get_temperature() < 10 then
            worker:set_goal_priority("seek_shelter", 3.0)
        end
    else
        worker:set_preference("indoor_tasks", false)
        worker:remove_need("warmth")
    end
    
    print("Applied " .. season .. " goals to " .. worker:get_name())
end

-- Adjust goals based on weather events
function on_weather_change(weather_type)
    if weather_type == "storm" then
        -- Everyone seeks shelter
        world:broadcast_goal_priority("seek_shelter", 5.0)
    elseif weather_type == "rain" then
        -- Reduce outdoor work
        world:modify_all_goals({
            outdoor_work = 0.5,
            indoor_work = 1.5
        })
    end
end
"#;

    pub const DANGER_REACTIONS_SCRIPT: &str = r#"
-- Danger Reactions Script
-- Defines how workers react to various dangers

-- Reaction strategies based on worker type
local reactions = {
    civilian = {
        wolf = "flee",
        fire = "flee_and_alert",
        enemy = "hide",
        disaster = "seek_shelter"
    },
    
    warrior = {
        wolf = "attack",
        fire = "fight_fire",
        enemy = "defend",
        disaster = "help_others"
    },
    
    leader = {
        wolf = "organize_defense",
        fire = "coordinate_response",
        enemy = "rally_troops",
        disaster = "evacuate_civilians"
    }
}

-- Process danger alert for a worker
function on_danger_alert(worker, danger_type, danger_location, danger_level)
    local worker_type = worker:get_type()
    local reaction_strategy = reactions[worker_type][danger_type]
    
    if not reaction_strategy then
        reaction_strategy = "flee" -- Default
    end
    
    -- Execute reaction based on strategy
    if reaction_strategy == "flee" then
        local safe_location = find_safe_location(worker, danger_location)
        worker:set_immediate_goal("move_to", safe_location)
        worker:set_movement_speed(1.5) -- Run faster
        
    elseif reaction_strategy == "flee_and_alert" then
        -- Flee while alerting others
        local safe_location = find_safe_location(worker, danger_location)
        worker:set_immediate_goal("move_to", safe_location)
        worker:broadcast_alert(danger_type, danger_location, 20) -- Alert radius
        
    elseif reaction_strategy == "attack" then
        if worker:has_weapon() then
            worker:set_immediate_goal("attack", danger_location)
        else
            -- Find weapon first
            worker:set_immediate_goal("find_weapon")
        end
        
    elseif reaction_strategy == "organize_defense" then
        -- Rally nearby warriors
        local warriors = worker:find_nearby_type("warrior", 30)
        for _, warrior in ipairs(warriors) do
            warrior:set_immediate_goal("defend_position", worker:get_position())
        end
        
    elseif reaction_strategy == "help_others" then
        -- Find and help vulnerable workers
        local civilians = worker:find_nearby_type("civilian", 20)
        local weakest = find_most_vulnerable(civilians)
        if weakest then
            worker:set_immediate_goal("escort", weakest)
        end
    end
    
    -- Apply fear/stress based on personality
    local traits = worker:get_traits()
    local fear_multiplier = 1.0
    
    if traits:has("brave") then
        fear_multiplier = 0.5
    elseif traits:has("coward") then
        fear_multiplier = 2.0
    end
    
    worker:add_stress(danger_level * fear_multiplier)
end

-- Find safe location away from danger
function find_safe_location(worker, danger_location)
    local current_pos = worker:get_position()
    
    -- Calculate direction away from danger
    local dx = current_pos.x - danger_location.x
    local dy = current_pos.y - danger_location.y
    
    -- Normalize and scale
    local distance = math.sqrt(dx*dx + dy*dy)
    if distance > 0 then
        dx = dx / distance * 20 -- Move 20 units away
        dy = dy / distance * 20
    end
    
    local safe_x = current_pos.x + dx
    local safe_y = current_pos.y + dy
    
    -- Find nearest building for shelter
    local shelter = world:find_nearest_building(safe_x, safe_y, "any")
    if shelter then
        return shelter:get_position()
    end
    
    return {x = safe_x, y = safe_y}
end

-- Find most vulnerable worker (children, elderly, injured)
function find_most_vulnerable(workers)
    local most_vulnerable = nil
    local max_vulnerability = 0
    
    for _, worker in ipairs(workers) do
        local vulnerability = 0
        
        if worker:get_age() < 16 then
            vulnerability = vulnerability + 3
        elseif worker:get_age() > 60 then
            vulnerability = vulnerability + 2
        end
        
        if worker:get_health() < 50 then
            vulnerability = vulnerability + 2
        end
        
        if worker:has_trait("disabled") then
            vulnerability = vulnerability + 3
        end
        
        if vulnerability > max_vulnerability then
            max_vulnerability = vulnerability
            most_vulnerable = worker
        end
    end
    
    return most_vulnerable
end
"#;
}