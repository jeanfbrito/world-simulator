-- Utility AI Actions for Stronghold Pack
-- Reactive behaviors triggered by scorers

return {
    eat_food = {
        scorer = "hunger_scorer",
        duration = 2.0,
        animation = "eating",
        interruptible = true,
        description = "Consume food to reduce hunger",
        
        requirements = {
            has_food = { type = "Int", min = 1 }
        },
        
        effects = {
            hunger = -0.5,
            energy = 0.1,
            morale = 0.05
        },
        
        sounds = {
            start = "eating_start.ogg",
            loop = "eating_loop.ogg",
            finish = "eating_end.ogg"
        }
    },
    
    sleep = {
        scorer = "fatigue_scorer",
        duration = 10.0,
        animation = "sleeping",
        interruptible = false,      -- Can't interrupt sleep
        description = "Sleep to restore energy",
        
        requirements = {
            at_home = true
        },
        
        effects = {
            energy = 1.0,
            hunger = 0.1,               -- Get hungrier while sleeping
            morale = 0.1
        },
        
        sounds = {
            loop = "sleeping.ogg"
        }
    },
    
    quick_rest = {
        scorer = "fatigue_scorer",
        duration = 3.0,
        animation = "sitting",
        interruptible = true,
        description = "Quick rest when no bed available",
        
        requirements = {
            at_home = false            -- Only when away from home
        },
        
        effects = {
            energy = 0.3               -- Partial energy restore
        }
    },
    
    flee_danger = {
        scorer = "danger_scorer",
        duration = 0.0,             -- Instant action
        animation = "running",
        interruptible = false,
        description = "Run away from threats",
        
        effects = {
            position = "safe_location",
            energy = -0.2,              -- Running costs energy
            morale = -0.1               -- Fear reduces morale
        },
        
        sounds = {
            start = "scream.ogg"
        }
    },
    
    store_items = {
        scorer = "storage_scorer",
        duration = 1.0,
        animation = "carrying",
        interruptible = true,
        description = "Store items in stockpile",
        
        requirements = {
            at_storage = true,
            inventory_not_empty = true
        },
        
        effects = {
            inventory_weight = 0.0
        },
        
        sounds = {
            finish = "items_drop.ogg"
        }
    },
    
    warm_by_fire = {
        scorer = "temperature_scorer",
        duration = 5.0,
        animation = "warming",
        interruptible = true,
        description = "Warm up by a fire",
        
        requirements = {
            near_fire = true,
            temperature = "cold"
        },
        
        effects = {
            temperature_comfort = 1.0,
            morale = 0.1
        }
    },
    
    socialize = {
        scorer = "social_scorer",
        duration = 4.0,
        animation = "talking",
        interruptible = true,
        description = "Talk with other units",
        
        requirements = {
            near_other_unit = true
        },
        
        effects = {
            morale = 0.3,
            social_need = 0.0
        },
        
        sounds = {
            loop = "conversation.ogg"
        }
    },
    
    panic = {
        scorer = "danger_scorer",
        duration = 1.0,
        animation = "panic",
        interruptible = false,
        description = "Panic when cornered",
        
        requirements = {
            threat_level = { type = "Float", min = 0.9 },
            escape_route = false
        },
        
        effects = {
            morale = -0.3,
            energy = -0.1
        },
        
        sounds = {
            loop = "panic_breathing.ogg"
        }
    }
}