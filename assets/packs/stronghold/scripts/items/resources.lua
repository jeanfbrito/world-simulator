-- Stronghold Resources: Primary resources for castle building and economy

local resources = {}

-- PRIMARY RESOURCES
resources.wood = {
    id = "wood",
    name = "Wood",
    category = "primary_resource",
    stack_size = 250,
    
    -- Economy
    buy_price = 4,
    sell_price = 1,
    
    -- Harvesting
    source = "woodcutter",
    harvest_rate = 8,  -- per worker per minute
    
    -- Uses
    used_for = {
        "hovels", "woodcutter", "hunter", "dairy_farm",
        "apple_orchard", "wheat_farm", "hops_farm",
        "poleturner", "fletcher", "barracks",
        "bow", "crossbow", "spear", "pike"
    },
    
    description = "Essential for construction and weapon production"
}

resources.stone = {
    id = "stone",
    name = "Stone",
    category = "primary_resource",
    stack_size = 250,
    
    -- Economy
    buy_price = 10,
    sell_price = 2,
    
    -- Harvesting
    source = "quarry",
    harvest_rate = 6,  -- per quarry per minute (with ox)
    requires_ox = true,
    
    -- Uses
    used_for = {
        "walls", "towers", "gatehouse", "keep",
        "barracks", "armory", "granary", "stockpile",
        "church", "cathedral", "market"
    },
    
    description = "Required for all defensive structures"
}

resources.iron = {
    id = "iron",
    name = "Iron",
    category = "primary_resource",
    stack_size = 250,
    
    -- Economy
    buy_price = 20,
    sell_price = 4,
    
    -- Mining
    source = "iron_mine",
    harvest_rate = 4,  -- per mine per minute
    
    -- Uses
    used_for = {
        "sword", "mace", "metal_armor", "plate_armor"
    },
    
    description = "Essential for advanced weapons and armor"
}

resources.pitch = {
    id = "pitch",
    name = "Pitch",
    category = "primary_resource",
    stack_size = 250,
    
    -- Economy
    buy_price = 15,
    sell_price = 3,
    
    -- Collection
    source = "pitch_rig",
    harvest_rate = 3,  -- per rig per minute
    
    -- Uses
    used_for = {
        "pitch_ditch", "burning_oil", "brazier"
    },
    
    description = "Flammable substance for castle defense"
}

-- FOOD RESOURCES
resources.wheat = {
    id = "wheat",
    name = "Wheat",
    category = "raw_food",
    stack_size = 250,
    
    -- Economy
    buy_price = 8,
    sell_price = 1,
    
    -- Farming
    source = "wheat_farm",
    harvest_rate = 8,  -- per farm per harvest
    harvest_time = 60,  -- seconds between harvests
    
    -- Uses
    processes_to = "flour",
    
    description = "Raw grain for bread production"
}

resources.flour = {
    id = "flour",
    name = "Flour",
    category = "processed_food",
    stack_size = 250,
    
    -- Processing
    source = "mill",
    made_from = "wheat",
    conversion_rate = 1.0,  -- 1 wheat = 1 flour
    
    -- Uses
    processes_to = "bread",
    
    description = "Milled wheat ready for baking"
}

resources.hops = {
    id = "hops",
    name = "Hops",
    category = "raw_food",
    stack_size = 250,
    
    -- Economy
    buy_price = 8,
    sell_price = 1,
    
    -- Farming
    source = "hops_farm",
    harvest_rate = 6,
    harvest_time = 60,
    
    -- Uses
    processes_to = "ale",
    
    description = "Ingredient for brewing ale"
}

resources.apples = {
    id = "apples",
    name = "Apples",
    category = "food",
    stack_size = 250,
    
    -- Economy
    buy_price = 8,
    sell_price = 1,
    
    -- Farming
    source = "apple_orchard",
    harvest_rate = 5,
    harvest_time = 90,  -- slower than other farms
    
    -- Consumption
    food_value = 1,
    
    description = "Fresh fruit, ready to eat"
}

resources.cheese = {
    id = "cheese",
    name = "Cheese",
    category = "food",
    stack_size = 250,
    
    -- Economy
    buy_price = 10,
    sell_price = 2,
    
    -- Production
    source = "dairy_farm",
    production_rate = 3,
    requires_cows = true,
    
    -- Consumption
    food_value = 1,
    
    description = "Dairy product from cow farms"
}

resources.meat = {
    id = "meat",
    name = "Meat",
    category = "food",
    stack_size = 250,
    
    -- Economy
    buy_price = 12,
    sell_price = 2,
    
    -- Hunting
    source = "hunter",
    production_rate = 2,  -- slower than farms
    requires_deer = true,
    
    -- Consumption
    food_value = 1,
    
    description = "Hunted game meat"
}

resources.bread = {
    id = "bread",
    name = "Bread",
    category = "food",
    stack_size = 250,
    
    -- Economy
    buy_price = 16,
    sell_price = 3,
    
    -- Production
    source = "bakery",
    made_from = "flour",
    conversion_rate = 1.0,
    
    -- Consumption
    food_value = 1,
    
    description = "Most efficient food source"
}

resources.ale = {
    id = "ale",
    name = "Ale",
    category = "goods",
    stack_size = 250,
    
    -- Economy
    buy_price = 20,
    sell_price = 4,
    
    -- Production
    source = "brewery",
    made_from = "hops",
    conversion_rate = 1.0,
    
    -- Distribution
    consumed_at = "inn",
    popularity_bonus = 8,  -- max bonus when covered
    
    description = "Boosts popularity when distributed at inns"
}

-- WEAPONS
resources.bow = {
    id = "bow",
    name = "Bow",
    category = "weapon",
    stack_size = 50,
    
    -- Economy
    buy_price = 15,
    sell_price = 3,
    
    -- Production
    source = "fletcher",
    made_from = "wood",
    production_time = 20,
    
    -- Military
    equips_unit = "archer",
    
    description = "Basic ranged weapon"
}

resources.crossbow = {
    id = "crossbow",
    name = "Crossbow",
    category = "weapon",
    stack_size = 50,
    
    -- Economy
    buy_price = 25,
    sell_price = 5,
    
    -- Production
    source = "fletcher",
    made_from = "wood",
    production_time = 30,
    
    -- Military
    equips_unit = "crossbowman",
    
    description = "Armor-piercing ranged weapon"
}

resources.spear = {
    id = "spear",
    name = "Spear",
    category = "weapon",
    stack_size = 50,
    
    -- Economy
    buy_price = 10,
    sell_price = 2,
    
    -- Production
    source = "poleturner",
    made_from = "wood",
    production_time = 15,
    
    -- Military
    equips_unit = "spearman",
    
    description = "Basic melee weapon"
}

resources.pike = {
    id = "pike",
    name = "Pike",
    category = "weapon",
    stack_size = 50,
    
    -- Economy
    buy_price = 20,
    sell_price = 4,
    
    -- Production
    source = "poleturner",
    made_from = "wood",
    production_time = 25,
    
    -- Military
    equips_unit = "pikeman",
    
    description = "Long defensive weapon"
}

resources.mace = {
    id = "mace",
    name = "Mace",
    category = "weapon",
    stack_size = 50,
    
    -- Economy
    buy_price = 30,
    sell_price = 6,
    
    -- Production
    source = "blacksmith",
    made_from = "iron",
    production_time = 30,
    
    -- Military
    equips_unit = "maceman",
    
    description = "Fast attack weapon"
}

resources.sword = {
    id = "sword",
    name = "Sword",
    category = "weapon",
    stack_size = 50,
    
    -- Economy
    buy_price = 40,
    sell_price = 8,
    
    -- Production
    source = "blacksmith",
    made_from = "iron",
    production_time = 40,
    
    -- Military
    equips_unit = "swordsman",
    
    description = "Elite melee weapon"
}

resources.leather_armor = {
    id = "leather_armor",
    name = "Leather Armor",
    category = "armor",
    stack_size = 50,
    
    -- Economy
    buy_price = 25,
    sell_price = 5,
    
    -- Production
    source = "tanner",
    made_from = "cow_hide",
    production_time = 25,
    
    -- Military
    equips_unit = "maceman",
    
    description = "Light armor for fast units"
}

resources.metal_armor = {
    id = "metal_armor",
    name = "Metal Armor",
    category = "armor",
    stack_size = 50,
    
    -- Economy
    buy_price = 50,
    sell_price = 10,
    
    -- Production
    source = "armorer",
    made_from = "iron",
    production_time = 40,
    
    -- Military
    equips_units = {"pikeman", "swordsman", "knight"},
    
    description = "Heavy armor for elite units"
}

return resources