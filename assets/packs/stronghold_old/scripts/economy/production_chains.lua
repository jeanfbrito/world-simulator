-- Stronghold Production Chains: Economic relationships and ratios

local chains = {}

-- FOOD PRODUCTION CHAINS

chains.bread_chain = {
    id = "bread_chain",
    name = "Bread Production",
    category = "food",
    
    -- Chain: Wheat Farm -> Mill -> Bakery -> Granary
    steps = {
        {
            building = "wheat_farm",
            workers = 1,
            produces = "wheat",
            rate = 8,  -- per harvest
            cycle_time = 60  -- seconds
        },
        {
            building = "mill",
            workers = 1,
            consumes = "wheat",
            produces = "flour",
            ratio = 1.0,  -- 1:1 conversion
            processing_speed = 10
        },
        {
            building = "bakery", 
            workers = 1,
            consumes = "flour",
            produces = "bread",
            ratio = 1.0,
            processing_speed = 4
        }
    },
    
    -- Optimal ratios for continuous production
    optimal_ratio = {
        wheat_farms = 3,
        mills = 1,
        bakeries = 8
    },
    
    -- This setup feeds approximately 80 people
    output_capacity = 80,
    
    efficiency_notes = "Most efficient food chain. 3 farms keep 1 mill busy, which supplies 8 bakeries."
}

chains.cheese_chain = {
    id = "cheese_chain",
    name = "Cheese Production",
    category = "food",
    
    steps = {
        {
            building = "dairy_farm",
            workers = 1,
            produces = "cheese",
            rate = 3,
            requires = "cows"
        }
    },
    
    optimal_ratio = {
        dairy_farms = 4  -- for 30 people
    },
    
    output_capacity = 30,
    
    efficiency_notes = "Simple but requires initial cow investment"
}

chains.apple_chain = {
    id = "apple_chain",
    name = "Apple Production",
    category = "food",
    
    steps = {
        {
            building = "apple_orchard",
            workers = 1,
            produces = "apples",
            rate = 5,
            cycle_time = 90  -- Slower cycle
        }
    },
    
    optimal_ratio = {
        apple_orchards = 5  -- for 30 people
    },
    
    output_capacity = 30,
    
    efficiency_notes = "Slow but requires no processing"
}

chains.meat_chain = {
    id = "meat_chain",
    name = "Meat Production",
    category = "food",
    
    steps = {
        {
            building = "hunter",
            workers = 1,
            produces = "meat",
            rate = 2,
            requires = "deer"
        }
    },
    
    optimal_ratio = {
        hunters = 6  -- for 20 people
    },
    
    output_capacity = 20,
    
    efficiency_notes = "Least efficient, depends on deer availability"
}

chains.ale_chain = {
    id = "ale_chain",
    name = "Ale Production",
    category = "happiness",
    
    -- Chain: Hops Farm -> Brewery -> Inn
    steps = {
        {
            building = "hops_farm",
            workers = 1,
            produces = "hops",
            rate = 6,
            cycle_time = 60
        },
        {
            building = "brewery",
            workers = 1,
            consumes = "hops",
            produces = "ale",
            ratio = 1.0,
            processing_speed = 5
        },
        {
            building = "inn",
            workers = 1,
            consumes = "ale",
            distributes = true,
            coverage = 50  -- people
        }
    },
    
    optimal_ratio = {
        hops_farms = 2,
        breweries = 2,
        inns = 1
    },
    
    popularity_bonus = 8,
    
    efficiency_notes = "2 hops farms supply 2 breweries which keep 1 inn stocked"
}

-- WEAPON PRODUCTION CHAINS

chains.archer_equipment = {
    id = "archer_equipment",
    name = "Archer Equipment",
    category = "military",
    
    steps = {
        {
            building = "woodcutter",
            produces = "wood",
            rate = 8
        },
        {
            building = "fletcher",
            consumes = "wood",
            produces = "bow",
            consumption_rate = 2,  -- wood per bow
            production_time = 20
        }
    },
    
    optimal_ratio = {
        woodcutters = 1,
        fletchers = 2
    },
    
    output = "20 bows per minute with optimal setup"
}

chains.crossbow_equipment = {
    id = "crossbow_equipment",
    name = "Crossbowman Equipment",
    category = "military",
    
    steps = {
        {
            building = "fletcher",
            consumes = "wood",
            produces = "crossbow",
            consumption_rate = 3,
            production_time = 30
        },
        {
            building = "tanner",
            consumes = "cow_hide",
            produces = "leather_armor",
            production_time = 25
        }
    },
    
    equipment_per_unit = {
        crossbow = 1,
        leather_armor = 1
    }
}

chains.swordsman_equipment = {
    id = "swordsman_equipment",
    name = "Swordsman Equipment",
    category = "military",
    
    steps = {
        {
            building = "iron_mine",
            produces = "iron",
            rate = 4
        },
        {
            building = "blacksmith",
            consumes = "iron",
            produces = "sword",
            consumption_rate = 2,
            production_time = 40
        },
        {
            building = "armorer",
            consumes = "iron",
            produces = "metal_armor",
            consumption_rate = 3,
            production_time = 40
        }
    },
    
    optimal_ratio = {
        iron_mines = 2,
        blacksmiths = 1,
        armorers = 1
    },
    
    equipment_per_unit = {
        sword = 1,
        metal_armor = 1
    }
}

-- ECONOMY BALANCING

chains.fear_factor_production = {
    id = "fear_factor_production",
    name = "Fear Factor Economy",
    category = "advanced",
    
    description = "Using fear to boost production",
    
    fear_levels = {
        {
            level = -1,
            happiness_penalty = 1,
            production_bonus = 1.1
        },
        {
            level = -3,
            happiness_penalty = 3,
            production_bonus = 1.3
        },
        {
            level = -5,
            happiness_penalty = 5,
            production_bonus = 1.5,
            worker_health_penalty = 0.25
        }
    },
    
    optimal_setup = {
        fear_factor = -5,
        extra_rations = "double",  -- +8 happiness
        ale_coverage = "full",      -- +8 happiness
        church = true,              -- +2 happiness
        result_happiness = 13,      -- Net positive
        production_multiplier = 2.25  -- For bread chain
    },
    
    notes = "Fear factor -5 with happiness compensation gives 225% bread production"
}

chains.trade_economy = {
    id = "trade_economy",
    name = "Trading Strategy",
    category = "economy",
    
    profitable_trades = {
        {
            sell = "bread",
            sell_price = 3,
            production_cost = 0.5,
            profit_margin = 2.5
        },
        {
            sell = "ale",
            sell_price = 4,
            production_cost = 1,
            profit_margin = 3
        },
        {
            sell = "weapons",
            sell_price = 8,
            production_cost = 3,
            profit_margin = 5
        }
    },
    
    import_priorities = {
        "iron",  -- If no iron deposits
        "stone",  -- For quick building
        "pitch"   -- For defense
    }
}

-- OPTIMAL BUILD ORDERS

chains.early_game = {
    id = "early_game",
    name = "Early Game Build Order",
    category = "strategy",
    
    build_order = {
        {step = 1, building = "granary", reason = "Food storage"},
        {step = 2, building = "stockpile", reason = "Resource storage"},
        {step = 3, building = "woodcutter", count = 2, reason = "Wood income"},
        {step = 4, building = "hunter", reason = "Quick food"},
        {step = 5, building = "quarry", reason = "Stone for castle"},
        {step = 6, building = "wheat_farm", count = 3, reason = "Bread chain start"},
        {step = 7, building = "mill", reason = "Process wheat"},
        {step = 8, building = "bakery", count = 4, reason = "Bread production"},
        {step = 9, building = "hovel", count = 3, reason = "More workers"}
    }
}

chains.military_rush = {
    id = "military_rush",
    name = "Military Rush Strategy",
    category = "strategy",
    
    build_order = {
        {step = 1, building = "armory", reason = "Weapon storage"},
        {step = 2, building = "barracks", reason = "Train troops"},
        {step = 3, building = "fletcher", count = 2, reason = "Bow production"},
        {step = 4, building = "poleturner", reason = "Spear production"},
        {step = 5, recruit = "archer", count = 10},
        {step = 6, recruit = "spearman", count = 5},
        {step = 7, building = "blacksmith", reason = "Advanced weapons"},
        {step = 8, recruit = "maceman", count = 5}
    }
}

chains.castle_defense = {
    id = "castle_defense",
    name = "Defensive Strategy",
    category = "strategy",
    
    build_priorities = {
        {priority = 1, structure = "stone_wall", reason = "Basic defense"},
        {priority = 2, structure = "gatehouse", reason = "Controlled entry"},
        {priority = 3, structure = "square_tower", count = 4, reason = "Corner defense"},
        {priority = 4, structure = "moat", reason = "Slow attackers"},
        {priority = 5, structure = "pitch_ditch", reason = "Fire trap"},
        {priority = 6, structure = "ballista_tower", reason = "Anti-siege"}
    }
}

-- HELPER FUNCTIONS

function chains.calculate_food_required(population)
    -- Each person consumes 0.5 food units per minute
    return population * 0.5
end

function chains.calculate_workers_needed(chain_id, target_output)
    local chain = chains[chain_id]
    if not chain then return 0 end
    
    local workers = 0
    for _, step in ipairs(chain.steps) do
        workers = workers + (step.workers or 0)
    end
    
    local ratio = target_output / chain.output_capacity
    return math.ceil(workers * ratio)
end

function chains.get_production_bonus(fear_factor)
    if fear_factor == 0 then return 1.0 end
    if fear_factor == -1 then return 1.1 end
    if fear_factor == -2 then return 1.2 end
    if fear_factor == -3 then return 1.3 end
    if fear_factor == -4 then return 1.4 end
    if fear_factor == -5 then return 1.5 end
    return 1.0
end

return chains