-- Workshop Tier System (Dwarf Fortress Inspired)
-- Defines all workshops with their tiers, inputs, outputs, and labor requirements

-- Workshop tiers
local tiers = {
    RAW_PROCESSING = 1,    -- Process raw materials
    SECONDARY = 2,         -- Process tier 1 outputs  
    ADVANCED = 3,          -- Complex crafting
    LEGENDARY = 4          -- Master craftsman workshops
}

-- Labor types required by workshops
local labors = {
    MINING = "mining",
    WOODCUTTING = "woodcutting",
    CARPENTRY = "carpentry",
    MASONRY = "masonry",
    BUTCHERY = "butchery",
    TANNING = "tanning",
    BREWING = "brewing",
    COOKING = "cooking",
    FARMING = "farming",
    FISHING = "fishing",
    FURNACE_OPERATING = "furnace_operating",
    METALSMITHING = "metalsmithing",
    WEAPONSMITHING = "weaponsmithing",
    ARMORSMITHING = "armorsmithing",
    BLACKSMITHING = "blacksmithing",
    LEATHERWORKING = "leatherworking",
    CLOTHESMAKING = "clothesmaking",
    WEAVING = "weaving",
    DYEING = "dyeing",
    POTTERY = "pottery",
    GLAZING = "glazing",
    PRESSING = "pressing",
    BEEKEEPING = "beekeeping",
    CHEESEMAKING = "cheesemaking",
    MILKING = "milking",
    SHEARING = "shearing",
    SPINNING = "spinning",
    THRESHING = "threshing",
    MILLING = "milling",
    PLANT_PROCESSING = "plant_processing",
    MECHANICS = "mechanics",
    SIEGE_ENGINEERING = "siege_engineering",
    TRAP_MAKING = "trap_making",
    JEWELING = "jeweling",
    GEM_CUTTING = "gem_cutting",
    GEM_SETTING = "gem_setting",
    BONE_CARVING = "bone_carving",
    GLASSMAKING = "glassmaking",
    SCREW_PUMP_OPERATING = "screw_pump_operating",
    SIEGE_OPERATING = "siege_operating",
    CROSSBOW_MAKING = "crossbow_making",
    ALCHEMY = "alchemy",
    SOAP_MAKING = "soap_making",
    LYE_MAKING = "lye_making",
    POTASH_MAKING = "potash_making",
    PAPERMAKING = "papermaking",
    BOOKBINDING = "bookbinding"
}

-- Main workshop definitions
workshops = {
    -- TIER 1: Raw Material Processing
    carpenter = {
        id = "carpenter",
        name = "Carpenter's Workshop",
        tier = tiers.RAW_PROCESSING,
        size = {width = 3, height = 3},
        build_cost = {
            wood = 1
        },
        labor = labors.CARPENTRY,
        
        -- Reactions this workshop can perform
        reactions = {
            "make_bed",
            "make_door", 
            "make_table",
            "make_chair",
            "make_barrel",
            "make_bin",
            "make_bucket",
            "make_animal_trap",
            "make_cage",
            "make_minecart",
            "make_wheelbarrow",
            "make_training_weapon",
            "make_shield",
            "make_armor_stand",
            "make_weapon_rack",
            "make_cabinet",
            "make_coffin",
            "make_floodgate",
            "make_hatch_cover",
            "make_grate",
            "make_crutch",
            "make_splint",
            "make_pipe_section",
            "make_stepladder"
        },
        
        skill_affects_quality = true,
        skill_affects_speed = true,
        
        description = "Converts wood into furniture and wooden items"
    },
    
    mason = {
        id = "mason",
        name = "Mason's Workshop",
        tier = tiers.RAW_PROCESSING,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1
        },
        labor = labors.MASONRY,
        
        reactions = {
            "make_stone_door",
            "make_stone_table",
            "make_stone_chair",
            "make_stone_blocks",
            "make_stone_statue",
            "make_stone_slab",
            "make_stone_coffer",
            "make_stone_hatch_cover",
            "make_stone_grate",
            "make_stone_cabinet",
            "make_stone_coffin",
            "make_stone_weapon_rack",
            "make_stone_armor_stand",
            "make_millstone",
            "make_quern"
        },
        
        skill_affects_quality = true,
        description = "Shapes stone into furniture and blocks"
    },
    
    butcher = {
        id = "butcher",
        name = "Butcher's Shop",
        tier = tiers.RAW_PROCESSING,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            table = 1
        },
        labor = labors.BUTCHERY,
        
        reactions = {
            "butcher_animal",
            "extract_from_raw_fish",
            "prepare_raw_fish"
        },
        
        products = {
            meat = {min = 2, max = 15},
            bones = {min = 4, max = 12},
            hide = {min = 1, max = 3},
            fat = {min = 2, max = 8},
            skull = {min = 1, max = 1},
            hooves = {min = 0, max = 4}
        },
        
        automatic = true,  -- Processes corpses automatically
        description = "Processes animal corpses into meat and materials"
    },
    
    smelter = {
        id = "smelter",
        name = "Smelter",
        tier = tiers.RAW_PROCESSING,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            fire_safe_stone = 1  -- Requires magma-safe stone
        },
        labor = labors.FURNACE_OPERATING,
        
        reactions = {
            "smelt_iron",
            "smelt_copper",
            "smelt_tin",
            "smelt_silver",
            "smelt_gold",
            "smelt_lead",
            "smelt_zinc",
            "smelt_nickel",
            "smelt_platinum",
            "smelt_aluminum",
            "smelt_bismuth",
            "make_pig_iron",
            "make_steel",
            "make_bronze",
            "make_brass",
            "make_electrum",
            "make_billon",
            "make_pewter",
            "make_rose_gold",
            "make_bismuth_bronze",
            "make_adamantine_wafers"
        },
        
        fuel_required = true,  -- Needs charcoal, coal, or magma
        temperature_required = 1200,
        
        skill_affects_yield = true,
        description = "Smelts ore into metal bars and creates alloys"
    },
    
    still = {
        id = "still",
        name = "Still",  
        tier = tiers.RAW_PROCESSING,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            barrel = 1
        },
        labor = labors.BREWING,
        
        reactions = {
            "brew_beer",
            "brew_ale",
            "brew_wine",
            "brew_mead",
            "brew_rum",
            "brew_whip_wine",
            "brew_sunshine",
            "brew_swamp_whiskey",
            "brew_river_spirits",
            "brew_prickle_berry_wine",
            "brew_longland_beer",
            "brew_gutter_cruor",
            "extract_from_plants"
        },
        
        skill_affects_quality = true,
        products_provide_happiness = true,
        
        description = "Brews alcoholic beverages from plants"
    },
    
    -- TIER 2: Secondary Processing
    tanner = {
        id = "tanner",
        name = "Tanner's Shop",
        tier = tiers.SECONDARY,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            table = 1
        },
        labor = labors.TANNING,
        
        reactions = {
            "tan_hide"
        },
        
        inputs = {
            hide = 1
        },
        outputs = {
            leather = 1
        },
        
        automatic = true,  -- Tans hides automatically when available
        description = "Converts raw hides into leather"
    },
    
    forge = {
        id = "forge",
        name = "Metalsmith's Forge",
        tier = tiers.SECONDARY,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            anvil = 1
        },
        labor = labors.METALSMITHING,
        
        reactions = {
            "forge_battle_axe",
            "forge_war_hammer",
            "forge_short_sword",
            "forge_spear",
            "forge_mace",
            "forge_crossbow_bolts",
            "forge_pick",
            "forge_helmet",
            "forge_mail_shirt",
            "forge_breastplate",
            "forge_greaves",
            "forge_gauntlets",
            "forge_high_boots",
            "forge_shield",
            "forge_buckler",
            "forge_animal_trap",
            "forge_bucket",
            "forge_chain",
            "forge_flask",
            "forge_goblet",
            "forge_instrument",
            "forge_toy",
            "forge_cage",
            "forge_barrel",
            "forge_bin",
            "forge_pipe_section",
            "forge_anvil",
            "forge_mechanism",
            "forge_minecart",
            "forge_wheelbarrow",
            "stud_with_metal"
        },
        
        fuel_required = true,
        skill_affects_quality = true,
        
        -- Different metals can be used
        material_flexibility = true,
        
        description = "Forges metal items, weapons, and armor"
    },
    
    kitchen = {
        id = "kitchen",
        name = "Kitchen",
        tier = tiers.SECONDARY,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            table = 1,
            barrel = 1
        },
        labor = labors.COOKING,
        
        reactions = {
            "prepare_easy_meal",    -- 2 ingredients
            "prepare_fine_meal",    -- 3 ingredients
            "prepare_lavish_meal",  -- 4 ingredients
            "render_fat"
        },
        
        -- Combines multiple ingredients
        variable_ingredients = true,
        min_ingredients = 2,
        max_ingredients = 4,
        
        skill_affects_quality = true,
        quality_affects_value = true,
        quality_affects_happiness = true,
        
        description = "Prepares meals from ingredients"
    },
    
    loom = {
        id = "loom",
        name = "Loom",
        tier = tiers.SECONDARY,
        size = {width = 3, height = 3},
        build_cost = {
            wood = 1
        },
        labor = labors.WEAVING,
        
        reactions = {
            "weave_cloth",
            "weave_silk_cloth",
            "weave_plant_cloth",
            "collect_webs"
        },
        
        inputs = {
            thread = 1
        },
        outputs = {
            cloth = 1
        },
        
        automatic = true,
        skill_affects_quality = true,
        
        description = "Weaves thread into cloth"
    },
    
    -- TIER 3: Advanced Crafting
    clothier = {
        id = "clothier",
        name = "Clothier's Shop",
        tier = tiers.ADVANCED,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            table = 1,
            chair = 1
        },
        labor = labors.CLOTHESMAKING,
        
        reactions = {
            "make_shirt",
            "make_tunic",
            "make_toga",
            "make_vest",
            "make_dress",
            "make_robe",
            "make_coat",
            "make_cloak",
            "make_cape",
            "make_trousers",
            "make_loincloth",
            "make_thong",
            "make_skirt",
            "make_short_skirt",
            "make_long_skirt",
            "make_braies",
            "make_gloves",
            "make_mittens",
            "make_shoes",
            "make_socks",
            "make_sandals",
            "make_high_boots",
            "make_hood",
            "make_head_veil",
            "make_turban",
            "make_cap",
            "make_scarf",
            "make_mask",
            "make_bag",
            "make_backpack",
            "make_quiver",
            "make_rope"
        },
        
        materials = {"cloth", "silk", "leather", "yarn"},
        
        skill_affects_quality = true,
        quality_affects_value = true,
        
        can_use_dye = true,
        can_add_decorations = true,
        
        description = "Creates clothing and bags from cloth or leather"
    },
    
    jeweler = {
        id = "jeweler",
        name = "Jeweler's Workshop",
        tier = tiers.ADVANCED,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            table = 1,
            chair = 1
        },
        labor = labors.GEM_CUTTING,
        
        reactions = {
            "cut_gem",
            "cut_glass",
            "encrust_furniture",
            "encrust_ammo",
            "encrust_finished_goods",
            "make_totem",
            "make_gem_window"
        },
        
        skill_affects_quality = true,
        quality_multiplier = 10,  -- Gems are very valuable
        
        precision_required = true,
        
        description = "Cuts gems and decorates items"
    },
    
    soap_maker = {
        id = "soap_maker",
        name = "Soap Maker's Workshop",
        tier = tiers.ADVANCED,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            bucket = 1
        },
        labor = labors.SOAP_MAKING,
        
        reactions = {
            "make_soap"
        },
        
        inputs = {
            lye = 1,
            tallow = 1
        },
        outputs = {
            soap = 1
        },
        
        -- Soap prevents disease
        products_provide_hygiene = true,
        
        description = "Produces soap from lye and tallow"
    },
    
    siege_workshop = {
        id = "siege_workshop",
        name = "Siege Workshop",
        tier = tiers.ADVANCED,
        size = {width = 5, height = 5},
        build_cost = {
            wood = 3,
            mechanism = 3
        },
        labor = labors.SIEGE_ENGINEERING,
        
        reactions = {
            "build_catapult",
            "build_ballista",
            "make_ballista_arrow"
        },
        
        products_are_siege_engines = true,
        requires_engineer = true,
        
        description = "Constructs siege engines"
    },
    
    -- Special workshops
    ashery = {
        id = "ashery",
        name = "Ashery",
        tier = tiers.SECONDARY,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            bucket = 1,
            barrel = 1
        },
        labor = labors.LYE_MAKING,
        
        reactions = {
            "make_lye",
            "make_potash",
            "make_pearlash"
        },
        
        -- Chain production for soap and fertilizer
        part_of_chain = {"soap", "fertilizer", "glass"},
        
        description = "Processes ash into lye and potash"
    },
    
    quern = {
        id = "quern",
        name = "Quern",
        tier = tiers.RAW_PROCESSING,
        size = {width = 1, height = 1},
        build_cost = {
            stone = 1
        },
        labor = labors.MILLING,
        
        reactions = {
            "mill_wheat",
            "mill_cave_wheat",
            "mill_longland_grass",
            "mill_whip_vine",
            "grind_rock_nuts"
        },
        
        manual_power = true,  -- Requires worker present
        slow_speed = true,
        
        -- Can be upgraded to millstone
        upgrades_to = "millstone",
        
        description = "Manually grinds plants into flour and paste"
    },
    
    millstone = {
        id = "millstone",
        name = "Millstone",
        tier = tiers.RAW_PROCESSING,
        size = {width = 3, height = 3},
        build_cost = {
            stone = 1,
            mechanism = 1
        },
        labor = labors.MILLING,
        
        reactions = {
            -- Same as quern but faster
            "mill_wheat",
            "mill_cave_wheat",
            "mill_longland_grass",
            "mill_whip_vine",
            "grind_rock_nuts",
            "mill_sugar",
            "mill_dye"
        },
        
        requires_power = true,  -- Needs windmill or water wheel
        automatic = true,
        fast_speed = true,
        
        description = "Power-driven mill for grinding plants"
    },
    
    -- Legendary workshops (require legendary skill)
    magma_forge = {
        id = "magma_forge",
        name = "Magma Forge",
        tier = tiers.LEGENDARY,
        size = {width = 3, height = 3},
        build_cost = {
            fire_safe_stone = 8,
            anvil = 1
        },
        labor = labors.METALSMITHING,
        
        -- Same reactions as forge but no fuel needed
        inherits_from = "forge",
        
        requires_magma = true,
        no_fuel_required = true,
        
        bonus_quality = 1,  -- +1 quality level
        bonus_speed = 2.0,  -- 2x faster
        
        description = "Magma-powered forge for legendary smiths"
    },
    
    artifact_workshop = {
        id = "artifact_workshop",
        name = "Artifact Workshop",
        tier = tiers.LEGENDARY,
        size = {width = 5, height = 5},
        build_cost = {
            -- Built during strange mood
            special = true
        },
        
        -- Can only be built during strange mood
        mood_only = true,
        one_time_use = true,
        
        creates_artifact = true,
        
        description = "Temporary workshop for creating legendary artifacts"
    }
}

-- Helper functions
function get_workshop(id)
    return workshops[id]
end

function get_workshops_by_tier(tier)
    local result = {}
    for id, workshop in pairs(workshops) do
        if workshop.tier == tier then
            table.insert(result, workshop)
        end
    end
    return result
end

function get_workshops_by_labor(labor)
    local result = {}
    for id, workshop in pairs(workshops) do
        if workshop.labor == labor then
            table.insert(result, workshop)
        end
    end
    return result
end

function can_build_workshop(workshop_id, available_materials, skill_levels)
    local workshop = workshops[workshop_id]
    if not workshop then return false end
    
    -- Check materials
    for material, amount in pairs(workshop.build_cost) do
        if material ~= "special" then
            if not available_materials[material] or 
               available_materials[material] < amount then
                return false
            end
        end
    end
    
    -- Check if it's a mood workshop
    if workshop.mood_only then
        return false  -- Handled separately
    end
    
    -- Check skill requirements for legendary workshops
    if workshop.tier == tiers.LEGENDARY then
        local skill = skill_levels[workshop.labor] or 0
        if skill < 10 then  -- Legendary skill = 10
            return false
        end
    end
    
    return true
end

function get_workshop_efficiency(workshop_id, worker_skill, powered, quality_tools)
    local workshop = workshops[workshop_id]
    if not workshop then return 0 end
    
    local efficiency = 1.0
    
    -- Skill bonus
    if workshop.skill_affects_speed then
        efficiency = efficiency * (1.0 + worker_skill * 0.1)
    end
    
    -- Power bonus
    if workshop.requires_power and not powered then
        return 0  -- Can't work without power
    elseif powered and not workshop.manual_power then
        efficiency = efficiency * 1.5
    end
    
    -- Tool quality bonus
    if quality_tools then
        efficiency = efficiency * 1.2
    end
    
    -- Speed modifiers
    if workshop.slow_speed then
        efficiency = efficiency * 0.5
    elseif workshop.fast_speed then
        efficiency = efficiency * 2.0
    end
    
    return efficiency
end

-- Initialize on load
function on_init()
    print("Loading " .. table_length(workshops) .. " workshop definitions")
    
    -- Validate workshop inheritance
    for id, workshop in pairs(workshops) do
        if workshop.inherits_from then
            local parent = workshops[workshop.inherits_from]
            if parent then
                -- Copy reactions from parent
                workshop.reactions = workshop.reactions or parent.reactions
            end
        end
    end
end

function table_length(t)
    local count = 0
    for _ in pairs(t) do count = count + 1 end
    return count
end