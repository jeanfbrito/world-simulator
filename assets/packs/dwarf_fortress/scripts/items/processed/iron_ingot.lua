-- Iron Ingot: Refined metal ready for crafting
local constants = require("items._constants")
local common = require("items._common")

local iron_ingot = {
    id = "iron_ingot",
    name = "Iron Ingot",
    category = constants.categories.PROCESSED,
    stack_size = 20,
    weight = 7,
    base_value = 15,
    decay_rate = 0,
    
    -- Quality affects value and crafting
    quality_affects_value = true,
    quality_affects_crafting = true,
    
    -- Smelting info
    smelted_from = "iron_ore",
    smelted_at = "smelter",
    requires_skill = "furnace_operating",
    requires_fuel = true,
    smelt_temperature = 1538,
    
    -- Crafting uses
    crafting_material = true,
    material_tier = 3,  -- Mid-tier material
    
    -- Can be alloyed
    alloyable = true,
    alloys_to = {
        steel = {with = "carbon", ratio = 0.98},
        pig_iron = {with = "carbon", ratio = 0.95}
    },
    
    description = "Refined iron ready for forging into tools and weapons"
}

return common.validate_item(iron_ingot)