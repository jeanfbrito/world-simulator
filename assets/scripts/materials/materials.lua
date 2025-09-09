-- Material Properties System (Dwarf Fortress Inspired)
-- Defines physical, mechanical, and thermal properties of all materials

-- Material categories
local categories = {
    METAL = "metal",
    STONE = "stone",
    WOOD = "wood",
    ORGANIC = "organic",
    LIQUID = "liquid",
    GAS = "gas",
    GLASS = "glass",
    CERAMIC = "ceramic",
    CLOTH = "cloth",
    LEATHER = "leather",
    GEM = "gem"
}

-- Material states
local states = {
    SOLID = "solid",
    LIQUID = "liquid",
    GAS = "gas",
    POWDER = "powder",
    PASTE = "paste"
}

-- Temperature constants (Kelvin)
local ABSOLUTE_ZERO = 0
local WATER_FREEZE = 273
local ROOM_TEMP = 293
local WATER_BOIL = 373

-- Main material definitions
materials = {
    -- METALS
    iron = {
        id = "iron",
        name = "Iron",
        category = categories.METAL,
        
        -- Physical properties
        density = 7850,  -- kg/m³
        melting_point = 1811,
        boiling_point = 3134,
        specific_heat = 449,  -- J/(kg·K)
        thermal_conductivity = 80.4,  -- W/(m·K)
        
        -- Mechanical properties (in kPa)
        yield_strength = 50000,
        tensile_strength = 80000,
        fracture_toughness = 100000,
        impact_resistance = 25000,
        shear_resistance = 30000,
        compressive_strength = 100000,
        
        -- Material behavior
        elasticity = 0.3,  -- Young's modulus ratio
        hardness = 4.0,  -- Mohs scale
        toughness = 0.7,
        brittleness = 0.2,
        
        -- Combat properties
        edge_retention = 0.7,
        edge_sharpness = 1.0,
        blunt_force = 1.0,
        armor_penetration = 0.8,
        
        -- Crafting properties
        malleability = 0.8,  -- How easy to work
        weldable = true,
        forgeable = true,
        castable = true,
        
        -- Value and rarity
        value_multiplier = 1.0,
        rarity = 0.3,  -- Common
        
        -- Reactions
        oxidizes = true,  -- Rusts
        corrosion_resistance = 0.3,
        magnetic = true
    },
    
    steel = {
        id = "steel",
        name = "Steel",
        category = categories.METAL,
        
        -- Improved over iron
        density = 7850,
        melting_point = 1643,
        boiling_point = 3000,
        
        -- Superior mechanical properties
        yield_strength = 250000,
        tensile_strength = 400000,
        fracture_toughness = 200000,
        impact_resistance = 50000,
        
        -- Better combat properties
        edge_retention = 0.9,
        edge_sharpness = 1.3,
        armor_penetration = 1.0,
        
        -- Crafting
        requires_skill = 5,  -- Minimum skill to work with
        malleability = 0.6,
        
        value_multiplier = 3.0,
        rarity = 0.1,  -- Rare (must be made)
        
        -- Alloy components
        alloy_of = {"iron", "carbon"},
        alloy_ratios = {0.98, 0.02}
    },
    
    copper = {
        id = "copper",
        name = "Copper",
        category = categories.METAL,
        
        density = 8960,
        melting_point = 1358,
        boiling_point = 2835,
        
        -- Softer than iron
        yield_strength = 30000,
        tensile_strength = 40000,
        
        -- Excellent conductivity
        thermal_conductivity = 401,
        electrical_conductivity = 0.596,  -- Relative to silver
        
        -- Easy to work
        malleability = 0.95,
        
        -- Antimicrobial
        special_properties = {
            antimicrobial = true,
            decorative = true
        },
        
        value_multiplier = 2.0,
        rarity = 0.4
    },
    
    bronze = {
        id = "bronze",
        name = "Bronze",
        category = categories.METAL,
        
        -- Copper-tin alloy
        alloy_of = {"copper", "tin"},
        alloy_ratios = {0.88, 0.12},
        
        density = 8800,
        melting_point = 1085,
        
        -- Better than copper, worse than iron
        yield_strength = 40000,
        tensile_strength = 60000,
        
        -- Good for weapons before iron
        edge_sharpness = 0.8,
        
        corrosion_resistance = 0.8,
        
        value_multiplier = 2.5,
        rarity = 0.2
    },
    
    silver = {
        id = "silver",
        name = "Silver",
        category = categories.METAL,
        
        density = 10490,
        melting_point = 1235,
        boiling_point = 2435,
        
        -- Soft precious metal
        yield_strength = 20000,
        
        -- Best conductivity
        thermal_conductivity = 429,
        electrical_conductivity = 1.0,  -- Reference
        
        -- Special properties
        special_properties = {
            antimicrobial = true,
            tarnishes = true,
            precious = true,
            decorative = true
        },
        
        value_multiplier = 10.0,
        rarity = 0.05
    },
    
    gold = {
        id = "gold",
        name = "Gold",
        category = categories.METAL,
        
        density = 19320,  -- Very heavy
        melting_point = 1337,
        boiling_point = 3243,
        
        -- Very soft
        yield_strength = 10000,
        
        -- Never corrodes
        corrosion_resistance = 1.0,
        
        -- Extremely malleable
        malleability = 1.0,
        
        special_properties = {
            precious = true,
            never_tarnishes = true,
            decorative = true,
            currency = true
        },
        
        value_multiplier = 40.0,
        rarity = 0.01
    },
    
    adamantine = {
        id = "adamantine",
        name = "Adamantine",
        category = categories.METAL,
        
        -- Legendary material (Dwarf Fortress reference)
        density = 200,  -- Impossibly light
        melting_point = 25000,  -- Nearly unmeltable
        
        -- Incredible strength
        yield_strength = 5000000,
        tensile_strength = 10000000,
        fracture_toughness = 10000000,
        
        -- Perfect for weapons
        edge_retention = 1.0,
        edge_sharpness = 5.0,
        armor_penetration = 3.0,
        
        -- Special properties
        special_properties = {
            legendary = true,
            never_dulls = true,
            deep_material = true,
            demon_bait = true  -- Mining it is dangerous
        },
        
        value_multiplier = 1000.0,
        rarity = 0.0001,
        
        -- Cannot be worked normally
        requires_skill = 10,
        forgeable = false,  -- Must use special methods
        
        extraction_danger = 0.9  -- 90% chance of !FUN!
    },
    
    -- STONES
    granite = {
        id = "granite",
        name = "Granite",
        category = categories.STONE,
        
        density = 2750,
        melting_point = 1473,
        
        -- Strong stone
        compressive_strength = 200000,
        tensile_strength = 5000,  -- Weak in tension
        
        -- Construction properties
        construction_value = 1.0,
        blocks_per_boulder = 4,
        
        special_properties = {
            igneous = true,
            magma_safe = true
        },
        
        value_multiplier = 1.0,
        rarity = 0.5
    },
    
    limestone = {
        id = "limestone",
        name = "Limestone",
        category = categories.STONE,
        
        density = 2500,
        melting_point = 1339,
        
        compressive_strength = 150000,
        
        -- Flux stone for steel
        special_properties = {
            sedimentary = true,
            flux = true,  -- Used in steel production
            calcium_carbonate = true
        },
        
        -- Reactions
        reactions = {
            heat = "quicklime",  -- CaO
            acid = "dissolves"
        },
        
        value_multiplier = 1.5,  -- Valuable for steel
        rarity = 0.4
    },
    
    obsidian = {
        id = "obsidian",
        name = "Obsidian",
        category = categories.STONE,
        
        density = 2350,
        melting_point = 1473,
        
        -- Volcanic glass
        hardness = 5.5,
        brittleness = 0.9,  -- Very brittle
        
        -- Incredibly sharp
        edge_sharpness = 3.0,  -- Sharper than steel
        edge_retention = 0.2,  -- But fragile
        
        special_properties = {
            volcanic = true,
            glass_like = true,
            magma_created = true
        },
        
        value_multiplier = 3.0,
        rarity = 0.2
    },
    
    -- ORGANIC MATERIALS
    wood_oak = {
        id = "wood_oak",
        name = "Oak Wood",
        category = categories.WOOD,
        
        density = 750,
        ignition_point = 573,
        
        -- Mechanical
        tensile_strength = 10000,
        compressive_strength = 5000,
        flexibility = 0.7,
        
        -- Wood properties
        hardwood = true,
        grain_quality = 0.8,
        workability = 0.7,
        
        -- Environmental
        flammable = true,
        organic = true,
        renewable = true,
        growth_time = 20,  -- Years
        
        special_properties = {
            rot_resistance = 0.7,
            insect_resistance = 0.6
        },
        
        value_multiplier = 1.5,
        rarity = 0.4
    },
    
    wood_pine = {
        id = "wood_pine",
        name = "Pine Wood",
        category = categories.WOOD,
        
        density = 550,
        ignition_point = 550,
        
        -- Softer than oak
        tensile_strength = 8000,
        compressive_strength = 4000,
        
        -- Softwood properties
        hardwood = false,
        workability = 0.9,  -- Easy to work
        resin_content = 0.3,
        
        flammable = true,
        burn_rate = 1.2,  -- Burns faster
        
        value_multiplier = 1.0,
        rarity = 0.6
    },
    
    leather = {
        id = "leather",
        name = "Leather",
        category = categories.LEATHER,
        
        density = 860,
        
        -- Flexible material
        tensile_strength = 20000,
        tear_resistance = 5000,
        flexibility = 0.9,
        
        -- Protection
        cut_resistance = 0.6,
        puncture_resistance = 0.4,
        
        -- Crafting
        workability = 0.8,
        dyeable = true,
        
        special_properties = {
            water_resistant = true,
            breathable = true,
            organic = true
        },
        
        value_multiplier = 2.0,
        rarity = 0.3
    },
    
    -- LIQUIDS
    water = {
        id = "water",
        name = "Water",
        category = categories.LIQUID,
        state = states.LIQUID,
        
        density = 1000,
        freezing_point = 273,
        boiling_point = 373,
        specific_heat = 4186,
        
        -- Fluid properties
        viscosity = 0.001,  -- Pa·s
        surface_tension = 0.072,
        
        -- State changes
        state_transitions = {
            {temp = 273, new_state = "ice"},
            {temp = 373, new_state = "steam"}
        },
        
        special_properties = {
            drinkable = true,
            solvent = true,
            essential = true
        },
        
        value_multiplier = 0.1,
        rarity = 0.8  -- Common on surface
    },
    
    magma = {
        id = "magma",
        name = "Magma",
        category = categories.LIQUID,
        state = states.LIQUID,
        
        density = 2800,
        temperature = 1473,  -- Always hot
        cooling_point = 1273,  -- Becomes obsidian
        
        viscosity = 100,  -- Very thick
        
        -- Dangerous properties
        damage_per_second = 50,
        ignites_materials = true,
        melts_materials = true,
        
        special_properties = {
            provides_heat = true,
            provides_light = true,
            infinite_fuel = true,
            dangerous = true
        },
        
        -- Cannot be stored normally
        containable = false,
        pumpable = true,
        
        value_multiplier = 0,  -- Cannot trade
        rarity = 0.1  -- Deep underground
    }
}

-- Helper functions
function get_material(id)
    return materials[id]
end

function get_material_state(material_id, temperature)
    local mat = materials[material_id]
    if not mat then return nil end
    
    -- Check state transitions
    if mat.state_transitions then
        for _, transition in ipairs(mat.state_transitions) do
            if temperature < transition.temp then
                return transition.new_state
            end
        end
    end
    
    -- Default state logic
    if mat.melting_point and temperature > mat.melting_point then
        if mat.boiling_point and temperature > mat.boiling_point then
            return states.GAS
        end
        return states.LIQUID
    end
    
    return mat.state or states.SOLID
end

function calculate_alloy_properties(components, ratios)
    local alloy = {
        density = 0,
        melting_point = 0,
        yield_strength = 0
    }
    
    -- Weighted average of properties
    for i, component_id in ipairs(components) do
        local mat = materials[component_id]
        local ratio = ratios[i]
        
        alloy.density = alloy.density + (mat.density * ratio)
        alloy.melting_point = alloy.melting_point + (mat.melting_point * ratio)
        alloy.yield_strength = alloy.yield_strength + (mat.yield_strength * ratio)
    end
    
    -- Alloys are often stronger than their components
    alloy.yield_strength = alloy.yield_strength * 1.2
    
    return alloy
end

function check_material_reaction(material_id, temperature, environment)
    local mat = materials[material_id]
    if not mat then return nil end
    
    local reactions = {}
    
    -- Temperature reactions
    if mat.ignition_point and temperature > mat.ignition_point then
        table.insert(reactions, "burning")
    end
    
    if mat.melting_point and temperature > mat.melting_point then
        table.insert(reactions, "melting")
    end
    
    -- Environmental reactions
    if environment.water and mat.oxidizes then
        table.insert(reactions, "oxidizing")
    end
    
    if environment.acid and mat.reactions and mat.reactions.acid then
        table.insert(reactions, mat.reactions.acid)
    end
    
    return reactions
end

function calculate_damage(weapon_material, armor_material, attack_type)
    local weapon = materials[weapon_material]
    local armor = materials[armor_material]
    
    if not weapon or not armor then return 0 end
    
    local damage = 0
    
    if attack_type == "slash" then
        damage = weapon.edge_sharpness * (weapon.yield_strength / armor.yield_strength)
        damage = damage * (1 - armor.cut_resistance)
        
    elseif attack_type == "pierce" then
        damage = weapon.armor_penetration * (weapon.hardness / armor.hardness)
        damage = damage * (1 - armor.puncture_resistance)
        
    elseif attack_type == "blunt" then
        damage = weapon.blunt_force * (weapon.density / armor.density)
        damage = damage * armor.brittleness  -- Brittle armor breaks
    end
    
    return damage
end

-- Initialize on load
function on_init()
    print("Loading " .. table_length(materials) .. " material definitions")
    
    -- Validate materials
    for id, mat in pairs(materials) do
        if mat.id ~= id then
            print("Warning: Material ID mismatch for " .. id)
        end
        
        -- Check alloys
        if mat.alloy_of then
            for _, component in ipairs(mat.alloy_of) do
                if not materials[component] then
                    print("Warning: Alloy component " .. component .. " not found for " .. id)
                end
            end
        end
    end
end

function table_length(t)
    local count = 0
    for _ in pairs(t) do count = count + 1 end
    return count
end