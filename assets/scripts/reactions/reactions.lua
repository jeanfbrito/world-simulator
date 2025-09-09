-- Complex Reaction System (Dwarf Fortress Inspired)
-- Defines all crafting reactions with inputs, outputs, conditions, and probabilities

-- Reaction categories
local categories = {
    SMELTING = "smelting",
    FORGING = "forging",
    CRAFTING = "crafting",
    COOKING = "cooking",
    BREWING = "brewing",
    PROCESSING = "processing",
    ALCHEMY = "alchemy",
    CONSTRUCTION = "construction"
}

-- Material classes for flexible reactions
local material_classes = {
    METAL = "metal",
    STONE = "stone",
    WOOD = "wood",
    BONE = "bone",
    LEATHER = "leather",
    CLOTH = "cloth",
    GLASS = "glass",
    GEM = "gem",
    EDIBLE_PLANT = "edible_plant",
    EDIBLE_MEAT = "edible_meat",
    EDIBLE_FISH = "edible_fish",
    BREWABLE = "brewable",
    MILLABLE = "millable",
    THREAD_PLANT = "thread_plant",
    MILK = "milk",
    CHEESE = "cheese",
    TALLOW = "tallow",
    OIL = "oil",
    FLUX = "flux",
    FUEL = "fuel",
    ASH = "ash",
    SAND = "sand"
}

-- Main reaction definitions
reactions = {
    -- SMELTING REACTIONS
    smelt_iron = {
        id = "smelt_iron",
        name = "Smelt Iron Ore",
        category = categories.SMELTING,
        building = "smelter",
        skill = "furnace_operating",
        
        reagents = {
            {item = "iron_ore", amount = 1, preserve = false},
            {material_class = material_classes.FUEL, amount = 1, preserve = false}
        },
        
        products = {
            {item = "iron_bar", amount = 1, probability = 1.0, quality_from_skill = true},
            {item = "slag", amount = 1, probability = 0.3}
        },
        
        temperature_required = 1538,  -- Celsius
        duration = 20,
        
        skill_modifiers = {
            speed = 0.1,     -- 10% faster per skill level
            yield = 0.05,    -- 5% more yield per skill level
            quality = 0.15   -- 15% quality bonus per skill level
        }
    },
    
    make_steel = {
        id = "make_steel",
        name = "Create Steel",
        category = categories.SMELTING,
        building = "smelter",
        skill = "furnace_operating",
        skill_required = 5,  -- Minimum skill level
        
        reagents = {
            {item = "iron_bar", amount = 1},
            {item = "pig_iron", amount = 1},
            {material_class = material_classes.FLUX, amount = 1},
            {material_class = material_classes.FUEL, amount = 2}
        },
        
        products = {
            {item = "steel_bar", amount = 2, probability = 1.0, quality_from_skill = true}
        },
        
        temperature_required = 1700,
        duration = 40,
        
        -- Steel quality depends heavily on skill
        critical_skill = true
    },
    
    make_bronze = {
        id = "make_bronze",
        name = "Create Bronze",
        category = categories.SMELTING,
        building = "smelter",
        skill = "furnace_operating",
        
        reagents = {
            {item = "copper_bar", amount = 3},
            {item = "tin_bar", amount = 1},
            {material_class = material_classes.FUEL, amount = 1}
        },
        
        products = {
            {item = "bronze_bar", amount = 4, probability = 1.0}
        },
        
        temperature_required = 1085,
        duration = 25
    },
    
    make_pig_iron = {
        id = "make_pig_iron",
        name = "Make Pig Iron",
        category = categories.SMELTING,
        building = "smelter",
        skill = "furnace_operating",
        
        reagents = {
            {item = "iron_bar", amount = 1},
            {material_class = material_classes.FLUX, amount = 1},
            {item = "coal", amount = 1, preserve = false}
        },
        
        products = {
            {item = "pig_iron", amount = 1, probability = 1.0}
        },
        
        -- Requires coal specifically, not just any fuel
        specific_fuel_required = "coal",
        temperature_required = 1500,
        duration = 30
    },
    
    -- FORGING REACTIONS
    forge_sword = {
        id = "forge_sword",
        name = "Forge Sword",
        category = categories.FORGING,
        building = "forge",
        skill = "weaponsmithing",
        
        reagents = {
            {material_class = material_classes.METAL, amount = 2, preserve_material = true}
        },
        
        products = {
            {item = "sword", amount = 1, inherit_material = true, quality_from_skill = true}
        },
        
        fuel_required = true,
        anvil_required = true,
        hammer_required = true,
        
        duration = 30,
        
        -- Different metals produce different quality
        material_modifiers = {
            copper = {damage = 0.6, durability = 0.7},
            bronze = {damage = 0.8, durability = 0.9},
            iron = {damage = 1.0, durability = 1.0},
            steel = {damage = 1.3, durability = 1.5},
            adamantine = {damage = 5.0, durability = 10.0}
        }
    },
    
    forge_breastplate = {
        id = "forge_breastplate",
        name = "Forge Breastplate",
        category = categories.FORGING,
        building = "forge",
        skill = "armorsmithing",
        
        reagents = {
            {material_class = material_classes.METAL, amount = 3, preserve_material = true}
        },
        
        products = {
            {item = "breastplate", amount = 1, inherit_material = true, quality_from_skill = true}
        },
        
        fuel_required = true,
        duration = 40,
        
        -- Heavier armors require more skill
        skill_required = 3
    },
    
    forge_mechanism = {
        id = "forge_mechanism",
        name = "Forge Mechanisms",
        category = categories.FORGING,
        building = "forge",
        skill = "mechanics",
        
        reagents = {
            {material_class = material_classes.METAL, amount = 1, preserve_material = true}
        },
        
        products = {
            {item = "mechanism", amount = 1, inherit_material = true, quality_from_skill = true}
        },
        
        -- Mechanisms are used in traps and machines
        precision_required = true,
        duration = 20
    },
    
    -- COOKING REACTIONS
    prepare_easy_meal = {
        id = "prepare_easy_meal",
        name = "Prepare Easy Meal",
        category = categories.COOKING,
        building = "kitchen",
        skill = "cooking",
        
        reagents = {
            {material_class = material_classes.EDIBLE_PLANT, amount = 1, any = true},
            {material_class = material_classes.EDIBLE_MEAT, amount = 1, any = true}
        },
        
        products = {
            {item = "prepared_meal", amount = 1, quality_from_skill = true, stack_size = 2}
        },
        
        duration = 10,
        
        -- Meal value is sum of ingredients plus skill bonus
        value_additive = true,
        
        -- Provides mood bonus based on quality
        mood_modifier = {
            poor = -2,
            normal = 0,
            good = 2,
            excellent = 5,
            masterwork = 10
        }
    },
    
    prepare_lavish_meal = {
        id = "prepare_lavish_meal",
        name = "Prepare Lavish Meal",
        category = categories.COOKING,
        building = "kitchen",
        skill = "cooking",
        skill_required = 5,
        
        reagents = {
            {material_class = material_classes.EDIBLE_MEAT, amount = 1},
            {material_class = material_classes.EDIBLE_PLANT, amount = 1},
            {material_class = material_classes.CHEESE, amount = 1},
            {item = "prepared_meal", amount = 1}  -- Uses another meal as ingredient
        },
        
        products = {
            {item = "lavish_meal", amount = 1, quality_from_skill = true, stack_size = 4}
        },
        
        duration = 20,
        value_multiplier = 4,
        
        -- High happiness bonus
        mood_modifier = {
            poor = 5,
            normal = 10,
            good = 15,
            excellent = 20,
            masterwork = 30
        }
    },
    
    render_fat = {
        id = "render_fat",
        name = "Render Fat",
        category = categories.COOKING,
        building = "kitchen",
        skill = "cooking",
        
        reagents = {
            {item = "fat", amount = 1}
        },
        
        products = {
            {item = "tallow", amount = 1, probability = 1.0}
        },
        
        duration = 5,
        automatic = true  -- Happens automatically when fat is available
    },
    
    -- BREWING REACTIONS
    brew_beer = {
        id = "brew_beer",
        name = "Brew Beer",
        category = categories.BREWING,
        building = "still",
        skill = "brewing",
        
        reagents = {
            {item = "wheat", amount = 1},
            {item = "barrel", amount = 1, preserve = true}  -- Barrel is preserved
        },
        
        products = {
            {item = "beer", amount = 5, quality_from_skill = true, container = "barrel"}
        },
        
        duration = 30,
        
        -- Alcohol prevents bad thoughts from lack of alcohol
        produces_alcohol = true,
        
        -- Different plants make different drinks
        plant_varieties = {
            wheat = "beer",
            barley = "ale",
            grapes = "wine",
            honey = "mead",
            sugar = "rum"
        }
    },
    
    -- PROCESSING REACTIONS
    tan_hide = {
        id = "tan_hide",
        name = "Tan Hide",
        category = categories.PROCESSING,
        building = "tanner",
        skill = "tanning",
        
        reagents = {
            {item = "hide", amount = 1}
        },
        
        products = {
            {item = "leather", amount = 1, quality_from_skill = true}
        },
        
        duration = 10,
        automatic = true,
        
        -- Different animals produce different leather quality
        source_modifiers = {
            cow = 1.0,
            pig = 0.8,
            elephant = 1.5,
            dragon = 3.0
        }
    },
    
    spin_thread = {
        id = "spin_thread",
        name = "Spin Thread",
        category = categories.PROCESSING,
        building = "farmer_workshop",
        skill = "spinning",
        
        reagents = {
            {material_class = material_classes.THREAD_PLANT, amount = 1}
        },
        
        products = {
            {item = "thread", amount = 1, quality_from_skill = true}
        },
        
        duration = 10,
        automatic = true
    },
    
    weave_cloth = {
        id = "weave_cloth",
        name = "Weave Cloth",
        category = categories.PROCESSING,
        building = "loom",
        skill = "weaving",
        
        reagents = {
            {item = "thread", amount = 1}
        },
        
        products = {
            {item = "cloth", amount = 1, quality_from_skill = true}
        },
        
        duration = 15,
        automatic = true,
        
        -- Can be dyed later
        can_be_dyed = true
    },
    
    mill_wheat = {
        id = "mill_wheat",
        name = "Mill Wheat",
        category = categories.PROCESSING,
        building = {"quern", "millstone"},  -- Can use either
        skill = "milling",
        
        reagents = {
            {item = "wheat", amount = 1}
        },
        
        products = {
            {item = "flour", amount = 1, probability = 1.0},
            {item = "wheat_seeds", amount = 1, probability = 0.1}
        },
        
        duration = 20,  -- Quern duration
        
        -- Millstone is faster
        building_speed = {
            quern = 1.0,
            millstone = 3.0
        }
    },
    
    -- ALCHEMY/SPECIAL REACTIONS
    make_soap = {
        id = "make_soap",
        name = "Make Soap",
        category = categories.ALCHEMY,
        building = "soap_maker",
        skill = "soap_making",
        
        reagents = {
            {item = "lye", amount = 1},
            {item = "tallow", amount = 1}
        },
        
        products = {
            {item = "soap", amount = 1, quality_from_skill = true}
        },
        
        duration = 15,
        
        -- Soap reduces disease
        hygiene_product = true
    },
    
    make_lye = {
        id = "make_lye",
        name = "Make Lye",
        category = categories.ALCHEMY,
        building = "ashery",
        skill = "lye_making",
        
        reagents = {
            {material_class = material_classes.ASH, amount = 1},
            {item = "bucket", amount = 1, preserve = true}
        },
        
        products = {
            {item = "lye", amount = 1, container = "bucket"}
        },
        
        duration = 10,
        
        -- Part of soap production chain
        chain_reaction = "soap"
    },
    
    make_potash = {
        id = "make_potash",
        name = "Make Potash",
        category = categories.ALCHEMY,
        building = "ashery",
        skill = "potash_making",
        
        reagents = {
            {item = "lye", amount = 1}
        },
        
        products = {
            {item = "potash", amount = 1}
        },
        
        duration = 15,
        
        -- Used for fertilizer and glass
        multi_purpose = true
    },
    
    make_glass = {
        id = "make_glass",
        name = "Make Glass",
        category = categories.CRAFTING,
        building = "glass_furnace",
        skill = "glassmaking",
        
        reagents = {
            {material_class = material_classes.SAND, amount = 1},
            {item = "potash", amount = 1, optional = true},  -- Optional, improves quality
            {material_class = material_classes.FUEL, amount = 1}
        },
        
        products = {
            {item = "raw_glass", amount = 1, quality_bonus_from_optional = true}
        },
        
        temperature_required = 1700,
        duration = 25,
        
        -- Can make green glass without potash, clear glass with it
        variants = {
            {without_optional = "green_glass"},
            {with_optional = "clear_glass"}
        }
    },
    
    -- LEGENDARY REACTIONS
    create_adamantine_wafer = {
        id = "create_adamantine_wafer",
        name = "Extract Adamantine Strands",
        category = categories.PROCESSING,
        building = "smelter",
        skill = "furnace_operating",
        skill_required = 10,  -- Legendary skill required
        
        reagents = {
            {item = "raw_adamantine", amount = 1}
        },
        
        products = {
            {item = "adamantine_wafer", amount = 1, probability = 0.5},  -- 50% chance
            {item = "adamantine_wafer", amount = 1, probability = 0.3},  -- Additional 30% chance
            {item = "adamantine_wafer", amount = 1, probability = 0.1}   -- Additional 10% chance
        },
        
        duration = 100,
        
        -- Extremely dangerous to mine
        danger_level = 10,
        
        -- Cannot be improved with skill (too delicate)
        fixed_output = true
    },
    
    -- CONSTRUCTION REACTIONS
    construct_bed = {
        id = "construct_bed",
        name = "Construct Bed",
        category = categories.CONSTRUCTION,
        building = "carpenter",
        skill = "carpentry",
        
        reagents = {
            {material_class = material_classes.WOOD, amount = 1, preserve_material = true}
        },
        
        products = {
            {item = "bed", amount = 1, inherit_material = true, quality_from_skill = true}
        },
        
        duration = 20,
        
        -- Beds provide rest quality based on material and quality
        rest_quality = {
            material_modifier = {
                pine = 0.8,
                oak = 1.0,
                mahogany = 1.2,
                feather_wood = 1.5
            },
            quality_modifier = {
                poor = 0.7,
                normal = 1.0,
                good = 1.3,
                excellent = 1.6,
                masterwork = 2.0
            }
        }
    },
    
    -- STRANGE MOOD REACTIONS
    artifact_reaction = {
        id = "artifact_reaction",
        name = "Create Artifact",
        category = categories.CRAFTING,
        building = "artifact_workshop",
        
        -- Requirements determined by mood
        reagents_determined_by_mood = true,
        
        products = {
            {item = "artifact", amount = 1, legendary_quality = true}
        },
        
        -- Cannot be repeated
        one_time_only = true,
        mood_only = true,
        
        -- Gives massive skill increase
        skill_gain = 10,
        
        -- Artifact value is extreme
        value_multiplier = 100
    }
}

-- Helper functions
function get_reaction(id)
    return reactions[id]
end

function can_perform_reaction(reaction_id, available_materials, worker_skill, building_type, temperature)
    local reaction = reactions[reaction_id]
    if not reaction then return false, "Unknown reaction" end
    
    -- Check building
    if type(reaction.building) == "table" then
        local valid = false
        for _, b in ipairs(reaction.building) do
            if b == building_type then
                valid = true
                break
            end
        end
        if not valid then return false, "Wrong building" end
    elseif reaction.building ~= building_type then
        return false, "Wrong building"
    end
    
    -- Check skill
    if reaction.skill_required and worker_skill < reaction.skill_required then
        return false, "Insufficient skill"
    end
    
    -- Check temperature
    if reaction.temperature_required and temperature < reaction.temperature_required then
        return false, "Insufficient temperature"
    end
    
    -- Check materials
    for _, reagent in ipairs(reaction.reagents) do
        if not reagent.optional then
            local found = false
            
            if reagent.item then
                -- Specific item
                if available_materials[reagent.item] and 
                   available_materials[reagent.item] >= reagent.amount then
                    found = true
                end
            elseif reagent.material_class then
                -- Any item of material class
                for mat, amount in pairs(available_materials) do
                    if get_material_class(mat) == reagent.material_class and
                       amount >= reagent.amount then
                        found = true
                        break
                    end
                end
            end
            
            if not found then
                return false, "Missing materials"
            end
        end
    end
    
    return true, "Can perform"
end

function calculate_reaction_products(reaction_id, worker_skill, material_quality, has_optional)
    local reaction = reactions[reaction_id]
    if not reaction then return {} end
    
    local products = {}
    
    for _, product in ipairs(reaction.products) do
        -- Check probability
        local roll = math.random()
        if roll <= product.probability then
            local item = {
                id = product.item,
                amount = product.amount or 1
            }
            
            -- Apply quality
            if product.quality_from_skill then
                item.quality = determine_quality(worker_skill, reaction.skill_required or 0)
            end
            
            -- Apply material inheritance
            if product.inherit_material then
                item.material = material_quality
            end
            
            -- Apply optional bonus
            if product.quality_bonus_from_optional and has_optional then
                item.quality = improve_quality(item.quality)
            end
            
            -- Apply stack size
            if product.stack_size then
                item.amount = product.stack_size
            end
            
            table.insert(products, item)
        end
    end
    
    return products
end

function determine_quality(skill, difficulty)
    local roll = math.random() + (skill - difficulty) * 0.1
    
    if roll < 0.2 then
        return "poor"
    elseif roll < 0.5 then
        return "normal"
    elseif roll < 0.75 then
        return "good"
    elseif roll < 0.95 then
        return "excellent"
    else
        return "masterwork"
    end
end

function improve_quality(current)
    local qualities = {"poor", "normal", "good", "excellent", "masterwork"}
    for i, q in ipairs(qualities) do
        if q == current and i < #qualities then
            return qualities[i + 1]
        end
    end
    return current
end

function get_material_class(item)
    -- This would look up the item's material class
    -- Implementation depends on item definitions
    return material_classes.METAL  -- Placeholder
end

function calculate_reaction_duration(reaction_id, worker_skill, building_efficiency)
    local reaction = reactions[reaction_id]
    if not reaction then return 0 end
    
    local duration = reaction.duration or 10
    
    -- Apply skill modifier
    if reaction.skill_modifiers and reaction.skill_modifiers.speed then
        duration = duration / (1 + worker_skill * reaction.skill_modifiers.speed)
    end
    
    -- Apply building efficiency
    duration = duration / building_efficiency
    
    return math.max(1, math.floor(duration))
end

-- Initialize
function on_init()
    print("Loading " .. table_length(reactions) .. " reaction definitions")
end

function table_length(t)
    local count = 0
    for _ in pairs(t) do count = count + 1 end
    return count
end