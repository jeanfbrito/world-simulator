-- Stronghold Military Units: Complete roster of combat units

local units = {}

-- BARRACKS UNITS

units.archer = {
    id = "archer",
    name = "Archer",
    category = "ranged",
    
    -- Cost
    gold_cost = 12,
    weapon_required = "bow",
    armor_required = nil,
    
    -- Stats
    health = 30,
    speed = 6,  -- Fast
    attack_damage = 8,
    attack_range = 10,  -- Long range
    attack_speed = 2.0,  -- Attacks per second
    
    -- Combat properties
    armor_piercing = 0.1,  -- Poor vs armor
    defense = 1,  -- Very weak
    morale = 5,
    
    -- Special
    can_man_walls = true,
    good_vs = {"spearman", "maceman"},
    weak_vs = {"crossbowman", "knight"},
    
    description = "Cheap, fast ranged unit with good range"
}

units.spearman = {
    id = "spearman",
    name = "Spearman",
    category = "melee",
    
    -- Cost
    gold_cost = 8,
    weapon_required = "spear",
    armor_required = nil,
    
    -- Stats
    health = 40,
    speed = 5,
    attack_damage = 10,
    attack_range = 1,
    attack_speed = 1.0,
    
    -- Combat properties
    armor_piercing = 0.2,
    defense = 2,
    morale = 6,
    
    -- Special
    cheap_unit = true,
    good_vs = {"archer", "horse_units"},
    weak_vs = {"maceman", "swordsman"},
    
    description = "Cheapest melee unit, good for early defense"
}

units.maceman = {
    id = "maceman",
    name = "Maceman",
    category = "melee",
    
    -- Cost
    gold_cost = 20,
    weapon_required = "mace",
    armor_required = "leather_armor",
    
    -- Stats
    health = 60,
    speed = 8,  -- Very fast
    attack_damage = 20,
    attack_range = 1,
    attack_speed = 1.5,
    
    -- Combat properties
    armor_piercing = 0.4,
    defense = 4,
    morale = 8,
    
    -- Special
    raider = true,  -- Good for raids
    ladder_climber = true,
    good_vs = {"archer", "spearman", "buildings"},
    weak_vs = {"swordsman", "pikeman", "crossbowman"},
    
    description = "Fast assault unit, excellent for raids"
}

units.crossbowman = {
    id = "crossbowman",
    name = "Crossbowman",
    category = "ranged",
    
    -- Cost
    gold_cost = 20,
    weapon_required = "crossbow",
    armor_required = "leather_armor",
    
    -- Stats
    health = 50,
    speed = 3,  -- Slow
    attack_damage = 20,
    attack_range = 8,  -- Shorter than archer
    attack_speed = 0.5,  -- Slow reload
    
    -- Combat properties
    armor_piercing = 0.8,  -- Excellent vs armor
    defense = 3,
    morale = 7,
    
    -- Special
    can_man_walls = true,
    armor_killer = true,
    good_vs = {"pikeman", "swordsman", "knight"},
    weak_vs = {"archer", "maceman"},
    
    description = "Slow but powerful, penetrates armor"
}

units.pikeman = {
    id = "pikeman",
    name = "Pikeman",
    category = "melee",
    
    -- Cost
    gold_cost = 20,
    weapon_required = "pike",
    armor_required = "metal_armor",
    
    -- Stats
    health = 100,
    speed = 2,  -- Very slow
    attack_damage = 15,
    attack_range = 2,  -- Pike reach
    attack_speed = 0.8,
    
    -- Combat properties
    armor_piercing = 0.3,
    defense = 8,  -- High defense
    morale = 9,
    
    -- Special
    anti_cavalry = true,
    defensive_bonus = 2.0,  -- When stationary
    good_vs = {"cavalry", "maceman", "spearman"},
    weak_vs = {"archer", "crossbowman", "swordsman"},
    
    description = "Tough defensive unit with long reach"
}

units.swordsman = {
    id = "swordsman",
    name = "Swordsman",
    category = "melee",
    
    -- Cost
    gold_cost = 40,
    weapon_required = "sword",
    armor_required = "metal_armor",
    
    -- Stats
    health = 120,
    speed = 1,  -- Extremely slow
    attack_damage = 35,
    attack_range = 1,
    attack_speed = 1.2,
    
    -- Combat properties
    armor_piercing = 0.6,
    defense = 10,  -- Highest defense
    morale = 10,
    
    -- Special
    elite_unit = true,
    siege_breaker = true,
    good_vs = {"all_melee", "buildings"},
    weak_vs = {"crossbowman", "massed_archers"},
    
    description = "Elite heavy infantry, nearly unstoppable"
}

units.knight = {
    id = "knight",
    name = "Knight",
    category = "cavalry",
    
    -- Cost
    gold_cost = 40,
    weapon_required = "sword",
    armor_required = "metal_armor",
    horse_required = true,
    
    -- Stats
    health = 180,
    speed = 10,  -- Fastest unit
    attack_damage = 40,
    attack_range = 1,
    attack_speed = 1.0,
    charge_damage = 60,  -- Bonus on charge
    
    -- Combat properties
    armor_piercing = 0.7,
    defense = 8,
    morale = 10,
    
    -- Special
    elite_unit = true,
    cavalry = true,
    charge_ability = true,
    good_vs = {"archers", "macemen", "buildings"},
    weak_vs = {"pikemen", "massed_crossbows", "pitch_ditches"},
    
    description = "Ultimate shock cavalry, expensive but devastating"
}

-- SPECIAL UNITS (Engineers/Siege)

units.engineer = {
    id = "engineer",
    name = "Engineer",
    category = "support",
    
    -- Cost
    gold_cost = 30,
    weapon_required = nil,
    armor_required = nil,
    
    -- Stats
    health = 20,
    speed = 4,
    attack_damage = 0,
    
    -- Special abilities
    can_build = {
        "protective_shield",
        "rolling_logs",
        "catapult",
        "trebuchet",
        "ballista",
        "battering_ram",
        "siege_tower",
        "portable_shield",
        "fire_ballista"
    },
    can_pour_oil = true,
    can_operate_siege = true,
    
    description = "Builds and operates siege equipment"
}

units.tunneler = {
    id = "tunneler",
    name = "Tunneler",
    category = "support",
    
    -- Cost
    gold_cost = 30,
    weapon_required = nil,
    
    -- Stats
    health = 30,
    speed = 3,
    
    -- Special abilities
    can_dig_tunnel = true,
    tunnel_speed = 2,  -- tiles per minute
    wall_damage = 100,  -- damage when tunnel collapses
    
    description = "Digs tunnels to collapse enemy walls"
}

units.ladderman = {
    id = "ladderman",
    name = "Ladderman",
    category = "support",
    
    -- Cost
    gold_cost = 4,
    weapon_required = nil,
    
    -- Stats
    health = 15,
    speed = 6,
    
    -- Special abilities
    carries_ladder = true,
    wall_scaling_speed = 3,
    vulnerable_while_climbing = true,
    
    description = "Cheap unit for scaling walls"
}

-- MERCENARY UNITS (Arabian units from Crusader, but could appear as mercenaries)

units.arabian_archer = {
    id = "arabian_archer",
    name = "Arabian Archer",
    category = "mercenary_ranged",
    
    -- Cost
    gold_cost = 15,
    mercenary = true,
    
    -- Stats
    health = 25,
    speed = 7,
    attack_damage = 10,
    attack_range = 9,
    attack_speed = 2.5,  -- Faster than European archer
    
    description = "Fast-firing mercenary archer"
}

units.horse_archer = {
    id = "horse_archer",
    name = "Horse Archer",
    category = "mercenary_cavalry",
    
    -- Cost
    gold_cost = 25,
    mercenary = true,
    
    -- Stats
    health = 40,
    speed = 9,
    attack_damage = 12,
    attack_range = 7,
    attack_speed = 1.5,
    
    -- Special
    can_fire_while_moving = true,
    hit_and_run = true,
    
    description = "Mobile ranged cavalry"
}

units.assassin = {
    id = "assassin",
    name = "Assassin",
    category = "mercenary_special",
    
    -- Cost
    gold_cost = 60,
    mercenary = true,
    
    -- Stats
    health = 60,
    speed = 8,
    attack_damage = 50,
    
    -- Special
    invisible_at_distance = true,
    can_climb_walls = true,
    instant_kill_peasants = true,
    
    description = "Stealthy unit that can infiltrate castles"
}

-- PEASANT UNITS

units.peasant = {
    id = "peasant",
    name = "Peasant",
    category = "civilian",
    
    -- Cost
    gold_cost = 0,  -- Free, comes from population
    
    -- Stats
    health = 10,
    speed = 5,
    
    -- Work
    can_work = true,
    can_construct = true,
    can_repair = true,
    can_firefight = true,
    
    -- Combat (when drafted)
    attack_damage = 2,
    defense = 0,
    morale = 2,
    
    description = "Workers that can be drafted in emergencies"
}

-- Unit special abilities functions

function units.get_unit(id)
    return units[id]
end

function units.calculate_cost(unit_id, quantity)
    local unit = units[unit_id]
    if not unit then return 0 end
    
    local gold = unit.gold_cost * quantity
    local weapons = {}
    
    if unit.weapon_required then
        weapons[unit.weapon_required] = quantity
    end
    if unit.armor_required then
        weapons[unit.armor_required] = quantity
    end
    
    return gold, weapons
end

function units.get_counter_units(unit_id)
    -- Returns units that counter the given unit
    local counters = {}
    for id, unit in pairs(units) do
        if unit.good_vs then
            for _, target in ipairs(unit.good_vs) do
                if target == unit_id then
                    table.insert(counters, id)
                end
            end
        end
    end
    return counters
end

return units