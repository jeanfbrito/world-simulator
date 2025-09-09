-- Stronghold Buildings: All structures and production buildings

local buildings = {}

-- CASTLE BUILDINGS

buildings.keep = {
    id = "keep",
    name = "Keep",
    category = "castle_core",
    
    -- Cost
    stone_cost = 0,  -- Free to place initially
    
    -- Properties
    size = {width = 4, height = 4},
    health = 2000,
    provides_housing = 8,
    
    -- Functions
    is_headquarters = true,
    storage_capacity = 50,
    generates_lord = true,
    rally_point = true,
    
    description = "Your castle's heart - lose it and you lose the game"
}

buildings.stockpile = {
    id = "stockpile",
    name = "Stockpile",
    category = "storage",
    
    -- Cost
    wood_cost = 0,  -- First is free
    
    -- Properties
    size = {width = 2, height = 2},
    health = 100,
    
    -- Storage
    storage_slots = 8,  -- Can store 8 different resource types
    stores = {"wood", "stone", "iron", "pitch", "wheat", "flour", "hops", "ale"},
    capacity_per_slot = 250,
    
    description = "Stores raw materials and goods"
}

buildings.granary = {
    id = "granary",
    name = "Granary",
    category = "storage",
    
    -- Cost
    wood_cost = 10,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 150,
    
    -- Storage
    stores_food = true,
    food_types = {"apples", "cheese", "meat", "bread"},
    capacity = 250,  -- per food type
    
    -- Ration distribution
    distributes_rations = true,
    
    description = "Stores and distributes food"
}

buildings.armory = {
    id = "armory",
    name = "Armory",
    category = "storage",
    
    -- Cost
    wood_cost = 15,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 200,
    
    -- Storage
    stores_weapons = true,
    weapon_capacity = 50,  -- per type
    armor_capacity = 50,
    
    description = "Stores weapons and armor for troops"
}

buildings.barracks = {
    id = "barracks",
    name = "Barracks",
    category = "military",
    
    -- Cost
    stone_cost = 15,
    
    -- Properties
    size = {width = 3, height = 3},
    health = 400,
    
    -- Military
    trains_units = true,
    rally_point = true,
    garrison_capacity = 10,
    
    description = "Recruits and houses soldiers"
}

buildings.mercenary_post = {
    id = "mercenary_post",
    name = "Mercenary Post",
    category = "military",
    
    -- Cost
    wood_cost = 10,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 150,
    
    -- Mercenaries
    hires_mercenaries = true,
    available_units = {"arabian_archer", "horse_archer", "assassin"},
    
    description = "Hire exotic mercenary units"
}

-- RESOURCE PRODUCTION

buildings.woodcutter = {
    id = "woodcutter",
    name = "Woodcutter's Hut",
    category = "resource_gathering",
    
    -- Cost
    wood_cost = 3,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 100,
    workers = 1,
    
    -- Production
    produces = "wood",
    production_rate = 8,  -- per minute
    requires_trees = true,
    
    description = "Harvests wood from nearby trees"
}

buildings.quarry = {
    id = "quarry",
    name = "Quarry",
    category = "resource_gathering",
    
    -- Cost
    wood_cost = 20,
    
    -- Properties
    size = {width = 3, height = 3},
    health = 200,
    workers = 3,
    
    -- Production
    produces = "stone",
    production_rate = 8,
    requires_stone_deposit = true,
    
    description = "Extracts stone from deposits"
}

buildings.ox_tether = {
    id = "ox_tether",
    name = "Ox Tether",
    category = "resource_support",
    
    -- Cost
    wood_cost = 10,
    
    -- Properties
    size = {width = 1, height = 1},
    health = 50,
    
    -- Function
    delivers_stone = true,
    delivery_speed_bonus = 2.0,
    serves_quarries = 1,
    
    description = "Ox delivers stone from quarry"
}

buildings.iron_mine = {
    id = "iron_mine",
    name = "Iron Mine",
    category = "resource_gathering",
    
    -- Cost
    wood_cost = 20,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 200,
    workers = 2,
    
    -- Production
    produces = "iron",
    production_rate = 4,
    requires_iron_deposit = true,
    
    description = "Mines iron ore from deposits"
}

buildings.pitch_rig = {
    id = "pitch_rig",
    name = "Pitch Rig",
    category = "resource_gathering",
    
    -- Cost
    wood_cost = 20,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 150,
    workers = 1,
    
    -- Production
    produces = "pitch",
    production_rate = 3,
    requires_marsh = true,
    
    description = "Extracts pitch from marsh"
}

-- FOOD PRODUCTION

buildings.hunter = {
    id = "hunter",
    name = "Hunter's Post",
    category = "food_production",
    
    -- Cost
    wood_cost = 5,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 100,
    workers = 1,
    
    -- Production
    produces = "meat",
    production_rate = 2,
    requires_deer = true,
    
    description = "Hunts deer for meat"
}

buildings.dairy_farm = {
    id = "dairy_farm",
    name = "Dairy Farm",
    category = "food_production",
    
    -- Cost
    wood_cost = 10,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 150,
    workers = 1,
    
    -- Production
    produces = "cheese",
    production_rate = 3,
    requires_cows = 3,
    
    description = "Produces cheese from cows"
}

buildings.apple_orchard = {
    id = "apple_orchard",
    name = "Apple Orchard",
    category = "food_production",
    
    -- Cost
    wood_cost = 5,
    
    -- Properties
    size = {width = 3, height = 3},
    health = 100,
    workers = 1,
    
    -- Production
    produces = "apples",
    production_rate = 5,
    harvest_time = 90,  -- Seasonal
    
    description = "Grows apples"
}

buildings.wheat_farm = {
    id = "wheat_farm",
    name = "Wheat Farm",
    category = "food_production",
    
    -- Cost
    wood_cost = 15,
    
    -- Properties
    size = {width = 4, height = 4},
    health = 100,
    workers = 1,
    
    -- Production
    produces = "wheat",
    production_rate = 8,
    requires_farmland = true,
    harvest_time = 60,
    
    description = "Grows wheat for bread production"
}

buildings.hops_farm = {
    id = "hops_farm",
    name = "Hops Farm",
    category = "food_production",
    
    -- Cost
    wood_cost = 15,
    
    -- Properties
    size = {width = 3, height = 3},
    health = 100,
    workers = 1,
    
    -- Production
    produces = "hops",
    production_rate = 6,
    requires_farmland = true,
    harvest_time = 60,
    
    description = "Grows hops for ale production"
}

buildings.mill = {
    id = "mill",
    name = "Mill",
    category = "food_processing",
    
    -- Cost
    wood_cost = 20,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 200,
    workers = 1,
    
    -- Processing
    processes = "wheat",
    produces = "flour",
    processing_rate = 10,  -- Fast
    
    description = "Grinds wheat into flour"
}

buildings.bakery = {
    id = "bakery",
    name = "Bakery",
    category = "food_processing",
    
    -- Cost
    wood_cost = 10,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 150,
    workers = 1,
    
    -- Processing
    processes = "flour",
    produces = "bread",
    processing_rate = 4,
    
    description = "Bakes bread from flour"
}

buildings.brewery = {
    id = "brewery",
    name = "Brewery",
    category = "food_processing",
    
    -- Cost
    wood_cost = 10,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 150,
    workers = 1,
    
    -- Processing
    processes = "hops",
    produces = "ale",
    processing_rate = 5,
    
    description = "Brews ale from hops"
}

buildings.inn = {
    id = "inn",
    name = "Inn",
    category = "happiness",
    
    -- Cost
    wood_cost = 20,
    gold_cost = 100,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 200,
    workers = 1,
    
    -- Function
    distributes = "ale",
    coverage_radius = 10,
    popularity_bonus = {
        none = 0,
        partial = 4,
        full = 8
    },
    
    description = "Distributes ale to boost popularity"
}

-- WEAPON PRODUCTION

buildings.fletcher = {
    id = "fletcher",
    name = "Fletcher's Workshop",
    category = "weapon_production",
    
    -- Cost
    wood_cost = 20,
    gold_cost = 100,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 200,
    workers = 1,
    
    -- Production
    produces = {"bow", "crossbow"},
    uses = "wood",
    production_time = {
        bow = 20,
        crossbow = 30
    },
    
    description = "Makes bows and crossbows"
}

buildings.poleturner = {
    id = "poleturner",
    name = "Poleturner's Workshop",
    category = "weapon_production",
    
    -- Cost
    wood_cost = 20,
    gold_cost = 100,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 200,
    workers = 1,
    
    -- Production
    produces = {"spear", "pike"},
    uses = "wood",
    production_time = {
        spear = 15,
        pike = 25
    },
    
    description = "Makes spears and pikes"
}

buildings.blacksmith = {
    id = "blacksmith",
    name = "Blacksmith's Workshop",
    category = "weapon_production",
    
    -- Cost
    wood_cost = 20,
    gold_cost = 200,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 250,
    workers = 1,
    
    -- Production
    produces = {"sword", "mace"},
    uses = "iron",
    production_time = {
        sword = 40,
        mace = 30
    },
    
    description = "Forges metal weapons"
}

buildings.armorer = {
    id = "armorer",
    name = "Armorer's Workshop",
    category = "weapon_production",
    
    -- Cost
    wood_cost = 20,
    gold_cost = 200,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 250,
    workers = 1,
    
    -- Production
    produces = "metal_armor",
    uses = "iron",
    production_time = 40,
    
    description = "Creates metal armor"
}

buildings.tanner = {
    id = "tanner",
    name = "Tanner's Workshop",
    category = "weapon_production",
    
    -- Cost
    wood_cost = 20,
    gold_cost = 100,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 200,
    workers = 1,
    
    -- Production
    produces = "leather_armor",
    uses = "cow_hide",
    production_time = 25,
    
    description = "Creates leather armor"
}

-- HAPPINESS & FEAR BUILDINGS

buildings.chapel = {
    id = "chapel",
    name = "Chapel",
    category = "happiness",
    
    -- Cost
    stone_cost = 10,
    gold_cost = 250,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 300,
    
    -- Effect
    popularity_bonus = 1,
    coverage_radius = "whole_castle",
    blessing_bonus = true,
    
    description = "Provides spiritual comfort (+1 popularity)"
}

buildings.church = {
    id = "church",
    name = "Church",
    category = "happiness",
    
    -- Cost
    stone_cost = 20,
    gold_cost = 500,
    
    -- Properties
    size = {width = 3, height = 3},
    health = 500,
    
    -- Effect
    popularity_bonus = 2,
    coverage_radius = "whole_castle",
    blessing_bonus = true,
    
    description = "Large church (+2 popularity)"
}

buildings.cathedral = {
    id = "cathedral",
    name = "Cathedral",
    category = "happiness",
    
    -- Cost
    stone_cost = 40,
    gold_cost = 1000,
    
    -- Properties
    size = {width = 4, height = 4},
    health = 800,
    
    -- Effect
    popularity_bonus = 3,
    coverage_radius = "whole_castle",
    blessing_bonus = true,
    monk_production = true,
    
    description = "Magnificent cathedral (+3 popularity)"
}

buildings.good_things = {
    id = "good_things",
    name = "Good Things",
    category = "happiness",
    
    types = {
        garden = {
            cost = {stone = 5, gold = 50},
            popularity = 1,
            description = "Beautiful garden"
        },
        statue = {
            cost = {stone = 10, gold = 100},
            popularity = 1,
            description = "Inspiring statue"
        },
        pond = {
            cost = {gold = 200},
            popularity = 2,
            description = "Peaceful pond"
        },
        maypole = {
            cost = {wood = 10, gold = 150},
            popularity = 2,
            description = "Dancing maypole"
        }
    }
}

buildings.bad_things = {
    id = "bad_things",
    name = "Bad Things",
    category = "fear",
    
    types = {
        gallows = {
            cost = {wood = 5},
            fear_factor = -1,
            description = "Execution device"
        },
        stocks = {
            cost = {wood = 5},
            fear_factor = -1,
            description = "Public punishment"
        },
        burning_stake = {
            cost = {wood = 10},
            fear_factor = -1,
            description = "Burning at stake"
        },
        dungeon = {
            cost = {stone = 10},
            fear_factor = -2,
            description = "Prison dungeon"
        },
        rack = {
            cost = {wood = 10},
            fear_factor = -2,
            description = "Torture device"
        },
        iron_maiden = {
            cost = {iron = 10},
            fear_factor = -2,
            description = "Deadly torture"
        },
        chopping_block = {
            cost = {wood = 5},
            fear_factor = -2,
            description = "Beheading block"
        }
    }
}

buildings.market = {
    id = "market",
    name = "Market",
    category = "economy",
    
    -- Cost
    wood_cost = 20,
    
    -- Properties
    size = {width = 3, height = 3},
    health = 200,
    
    -- Trading
    enables_trading = true,
    trader_visits = "monthly",
    buy_sell_resources = true,
    
    description = "Trade resources with merchants"
}

buildings.hovel = {
    id = "hovel",
    name = "Hovel",
    category = "housing",
    
    -- Cost
    wood_cost = 6,
    
    -- Properties
    size = {width = 1, height = 1},
    health = 50,
    
    -- Population
    provides_housing = 8,
    generates_peasants = true,
    
    description = "Houses for your population"
}

return buildings