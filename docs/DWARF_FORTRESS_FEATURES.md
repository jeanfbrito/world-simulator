# Dwarf Fortress-Inspired Features for World Simulator

## Current Status vs Dwarf Fortress

### ✅ Already Implemented
- Basic workshops (Sawmill, Bakery, Workshop)
- Simple recipes with inputs/outputs
- Building construction and maintenance
- Item quality tiers
- Worker skills affecting production
- Seasonal effects

### 🎯 Key Features to Add from Dwarf Fortress

## 1. Workshop Tier System
Dwarf Fortress uses a 4-tier system:
- **Tier 0**: Raw materials (wood, stone, ore, plants, animals)
- **Tier 1**: Basic processing (Carpenter, Mason, Butcher, Smelter)
- **Tier 2**: Secondary processing (Tanner, Loom, Kitchen, Forge)
- **Tier 3**: Advanced crafting (Clothier, Metalsmith, Soap Maker)

### Implementation in Lua:
```lua
workshop_tiers = {
    -- Tier 1: Raw material processing
    carpenter = {
        tier = 1,
        inputs = {"wood"},
        outputs = {"planks", "furniture", "barrels", "bins"},
        labor = "carpentry"
    },
    butcher = {
        tier = 1,
        inputs = {"animal_corpse"},
        outputs = {"meat", "bones", "hide", "fat"},
        labor = "butchery"
    },
    smelter = {
        tier = 1,
        inputs = {"ore", "fuel"},
        outputs = {"metal_bars"},
        labor = "furnace_operating"
    },
    
    -- Tier 2: Processing tier 1 outputs
    tanner = {
        tier = 2,
        inputs = {"hide"},
        outputs = {"leather"},
        labor = "tanning"
    },
    forge = {
        tier = 2,
        inputs = {"metal_bars", "fuel"},
        outputs = {"tools", "weapons", "armor"},
        labor = "metalsmithing"
    },
    kitchen = {
        tier = 2,
        inputs = {"meat", "plants", "flour"},
        outputs = {"prepared_meals"},
        labor = "cooking"
    },
    
    -- Tier 3: Advanced products
    clothier = {
        tier = 3,
        inputs = {"cloth", "leather", "thread"},
        outputs = {"clothing", "bags", "ropes"},
        labor = "clothesmaking"
    }
}
```

## 2. Material Properties System

### Physical Properties:
```lua
materials = {
    iron = {
        density = 7.85,
        melting_point = 1811,  -- Kelvin
        boiling_point = 3134,
        thermal_conductivity = 80.4,
        specific_heat = 449,
        
        -- Mechanical properties
        yield_strength = 50000,  -- Pressure units
        fracture_strength = 80000,
        impact_resistance = 25000,
        shear_resistance = 30000,
        
        -- Combat properties
        edge_multiplier = 1.0,
        blunt_multiplier = 1.0,
        
        -- States at different temperatures
        states = {
            {temp = 0, state = "solid"},
            {temp = 1811, state = "liquid"},
            {temp = 3134, state = "gas"}
        }
    },
    
    wood = {
        density = 0.5,
        ignition_point = 573,
        burn_rate = 0.5,
        
        -- Mechanical
        yield_strength = 10000,
        fracture_strength = 15000,
        flexibility = 0.8,
        
        -- Special properties
        flammable = true,
        organic = true,
        renewable = true
    },
    
    adamantine = {  -- Legendary material
        density = 0.2,
        melting_point = 25000,  -- Nearly indestructible
        
        -- Exceptional properties
        yield_strength = 500000,
        fracture_strength = 1000000,
        edge_multiplier = 5.0,
        
        -- Special
        legendary = true,
        deep_material = true,  -- Only found deep underground
        extraction_danger = 0.9  -- Risk of unleashing demons
    }
}
```

## 3. Complex Reaction System

### Reaction Definition:
```lua
reactions = {
    -- Basic smelting
    smelt_iron = {
        name = "Smelt Iron Ore",
        building = "smelter",
        reagents = {
            {item = "iron_ore", amount = 1},
            {item = "fuel", amount = 1}
        },
        products = {
            {item = "iron_bar", amount = 1, probability = 1.0},
            {item = "slag", amount = 1, probability = 0.3}
        },
        skill = "furnace_operating",
        duration = 20,
        temperature_required = 1200
    },
    
    -- Alloy creation
    make_steel = {
        name = "Create Steel",
        building = "smelter",
        reagents = {
            {item = "iron_bar", amount = 1},
            {item = "pig_iron", amount = 1},
            {item = "flux_stone", amount = 1},
            {item = "fuel", amount = 2}
        },
        products = {
            {item = "steel_bar", amount = 1, probability = 1.0}
        },
        skill = "furnace_operating",
        skill_level_required = 5,
        duration = 40
    },
    
    -- Complex food preparation
    lavish_meal = {
        name = "Prepare Lavish Meal",
        building = "kitchen",
        reagents = {
            {item = "meat", amount = 1, material_class = "edible"},
            {item = "plant", amount = 2, material_class = "edible"},
            {item = "seasoning", amount = 1}
        },
        products = {
            {item = "lavish_meal", amount = 1, quality_from_skill = true}
        },
        skill = "cooking",
        duration = 15,
        happiness_bonus = 20  -- Eating gives mood boost
    },
    
    -- Automatic reactions
    tan_hide = {
        name = "Tan Hide",
        building = "tanner",
        automatic = true,  -- Queues automatically when hide available
        reagents = {
            {item = "hide", amount = 1}
        },
        products = {
            {item = "leather", amount = 1}
        },
        duration = 10
    }
}
```

## 4. Labor and Skill System

### Detailed Labor Types:
```lua
labors = {
    mining = {
        tools_required = {"pickaxe"},
        experience_sources = {"dig_tile", "mine_ore"},
        skill_effects = {speed = 1.5, quality = 1.2}
    },
    carpentry = {
        tools_required = {"saw", "hammer"},
        workshops = {"carpenter"},
        products_affected = {"furniture", "barrels", "bins"}
    },
    brewing = {
        workshops = {"still"},
        products_affected = {"beer", "wine", "rum"},
        quality_matters = true  -- Higher skill = better booze
    },
    military = {
        combat_skills = {"sword", "axe", "hammer", "dodge", "armor"},
        training_required = true,
        experience_from_combat = true
    }
}
```

## 5. Stockpile and Hauling System

### Advanced Storage:
```lua
stockpiles = {
    types = {
        food = {
            categories = {"meat", "fish", "plant", "cheese", "eggs"},
            requires_barrel = true,
            spoilage_modifier = 0.5  -- Reduces spoilage
        },
        wood = {
            categories = {"logs", "branches"},
            outdoor_allowed = true
        },
        stone = {
            categories = {"economic", "construction", "ore"},
            weight_limit = 1000
        },
        finished_goods = {
            categories = {"crafts", "clothing", "armor", "weapons"},
            requires_bin = true,
            quality_sorting = true
        }
    },
    
    settings = {
        take_from_anywhere = false,
        give_to_workshops = true,
        wheelbarrow_required = false,  -- For heavy items
        max_bins = 10,
        max_barrels = 10
    }
}
```

## 6. Z-Level System (Vertical Layers)

### Multi-level Construction:
```lua
z_levels = {
    surface = {
        level = 0,
        features = {"trees", "plants", "water", "soil"}
    },
    underground = {
        {level = -1, type = "soil", minerals = {"clay", "sand"}},
        {level = -5, type = "stone", minerals = {"limestone", "granite"}},
        {level = -10, type = "metal_ore", minerals = {"iron", "copper"}},
        {level = -20, type = "rare", minerals = {"gold", "gems"}},
        {level = -50, type = "deep", minerals = {"adamantine"}, dangers = {"demons"}}
    },
    
    digging_rules = {
        ramps = true,  -- Can create slopes
        stairs = true,
        channels = true,  -- Vertical shafts
        cave_ins = true  -- Unsupported ceilings collapse
    }
}
```

## 7. Fluid Dynamics

### Water and Magma:
```lua
fluids = {
    water = {
        levels = 7,  -- 1-7 depth per tile
        flow_rate = 1.0,
        evaporation_rate = 0.01,
        freezing_point = 273,
        
        uses = {"drinking", "irrigation", "power", "moat"},
        dangers = {"drowning", "flooding"}
    },
    
    magma = {
        levels = 7,
        flow_rate = 0.5,  -- Flows slower than water
        temperature = 1500,
        
        uses = {"smelting", "forging", "trap", "obsidian_farming"},
        dangers = {"burning", "melting", "fire"}
    },
    
    pressure = {
        enabled = true,
        pump_power = 100,  -- Units of pressure per pump
        dangerous_threshold = 500
    }
}
```

## 8. Military and Combat

### Detailed Combat:
```lua
combat = {
    -- Body parts system
    body_parts = {
        head = {vital = true, armor_slot = "helm"},
        torso = {vital = true, armor_slot = "breastplate"},
        arms = {vital = false, armor_slot = "gauntlets"},
        legs = {vital = false, armor_slot = "greaves"}
    },
    
    -- Weapon types
    weapons = {
        sword = {
            attack_types = {"slash", "stab"},
            skill = "swordsman",
            material_quality_matters = true
        },
        hammer = {
            attack_types = {"bash"},
            skill = "hammerman",
            armor_penetration = 1.5
        },
        crossbow = {
            attack_types = {"shoot"},
            skill = "marksman",
            requires_ammo = "bolts",
            range = 20
        }
    },
    
    -- Squad system
    squads = {
        size = 10,
        positions = {"militia_commander", "militia_captain", "soldier"},
        training_schedule = {
            minimum_soldiers = 2,
            months_active = 3,
            months_training = 3
        }
    }
}
```

## 9. Mood and Strange Moods

### Legendary Artifacts:
```lua
moods = {
    types = {
        fey = {
            probability = 0.001,
            requirements = {"workshop", "materials"},
            result = "artifact",
            skill_gain = 20
        },
        possessed = {
            probability = 0.0005,
            requirements = {"any_workshop", "specific_material"},
            result = "artifact",
            madness_on_failure = true
        },
        secretive = {
            probability = 0.0008,
            hidden_requirements = true,
            result = "legendary_artifact"
        }
    },
    
    artifact_properties = {
        value_multiplier = 100,
        indestructible = true,
        mood_bonus = 50,
        room_value = 1000
    }
}
```

## 10. Nobles and Administration

### Management System:
```lua
nobles = {
    positions = {
        expedition_leader = {
            requirements = {population = 1},
            responsibilities = {"trading", "diplomacy"},
            demands = {"office", "dining_room"}
        },
        mayor = {
            requirements = {population = 50},
            elected = true,
            term_length = 365,
            demands = {"office", "quarters", "dining_room"}
        },
        baron = {
            requirements = {wealth = 100000, population = 80},
            appointed_by = "mountainhome",
            demands = {"throne_room", "bedroom", "dining_room", "tomb"}
        }
    },
    
    mandates = {
        -- Nobles make demands
        export_prohibition = {item_type = "specific"},
        production_order = {amount = 3, deadline = 60}
    }
}
```

## Priority Implementation Order:
1. **Material Properties** - Foundation for everything
2. **Workshop Tiers** - Production chains
3. **Complex Reactions** - Crafting depth
4. **Labor/Skills** - Worker specialization
5. **Z-Levels** - Vertical gameplay
6. **Fluids** - Water/magma dynamics
7. **Military** - Combat system
8. **Moods** - Legendary items
9. **Nobles** - Management layer
10. **Advanced AI** - Pathfinding, priorities

This would transform the simulator into a truly deep, Dwarf Fortress-inspired system!