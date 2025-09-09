-- Stronghold Castle Defenses: Walls, towers, gates, and defensive structures

local defenses = {}

-- WALLS

defenses.wooden_wall = {
    id = "wooden_wall",
    name = "Wooden Wall",
    category = "wall",
    
    -- Cost
    wood_cost = 2,
    
    -- Properties
    health = 100,
    height = 2,
    walkable = true,  -- Units can walk on top
    
    -- Defense
    arrow_protection = 0.3,
    fire_vulnerable = true,
    climb_difficulty = 0.5,
    
    description = "Cheap but weak wooden palisade"
}

defenses.stone_wall = {
    id = "stone_wall",
    name = "Stone Wall",
    category = "wall",
    
    -- Cost
    stone_cost = 3,
    
    -- Properties
    health = 300,
    height = 3,
    walkable = true,
    
    -- Defense
    arrow_protection = 0.8,
    fire_vulnerable = false,
    climb_difficulty = 0.8,
    
    description = "Strong stone fortification"
}

defenses.crenellated_wall = {
    id = "crenellated_wall",
    name = "Crenellated Wall",
    category = "wall",
    
    -- Cost
    stone_cost = 4,
    
    -- Properties
    health = 350,
    height = 3,
    walkable = true,
    
    -- Defense
    arrow_protection = 0.9,
    archer_damage_bonus = 1.2,  -- Archers get bonus
    climb_difficulty = 0.9,
    
    description = "Wall with battlements for archers"
}

defenses.stairs = {
    id = "stairs",
    name = "Stairs",
    category = "wall_access",
    
    -- Cost
    stone_cost = 2,
    
    -- Properties
    allows_wall_access = true,
    width = 1,
    
    description = "Access to walls and towers"
}

-- TOWERS

defenses.square_tower = {
    id = "square_tower",
    name = "Square Tower",
    category = "tower",
    
    -- Cost
    stone_cost = 10,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 500,
    height = 4,
    
    -- Combat
    archer_capacity = 5,
    range_bonus = 2,
    vision_bonus = 5,
    
    description = "Basic defensive tower"
}

defenses.round_tower = {
    id = "round_tower",
    name = "Round Tower",
    category = "tower",
    
    -- Cost
    stone_cost = 15,
    
    -- Properties
    size = {width = 2, height = 2},
    health = 600,
    height = 4,
    
    -- Combat
    archer_capacity = 5,
    range_bonus = 2,
    vision_bonus = 5,
    deflects_projectiles = 0.2,  -- Round shape deflects
    
    description = "Stronger tower with better defense"
}

defenses.great_tower = {
    id = "great_tower",
    name = "Great Tower",
    category = "tower",
    
    -- Cost
    stone_cost = 30,
    
    -- Properties
    size = {width = 3, height = 3},
    health = 1000,
    height = 5,
    
    -- Combat
    archer_capacity = 10,
    range_bonus = 3,
    vision_bonus = 8,
    ballista_platform = true,
    
    description = "Massive tower for ballista placement"
}

defenses.lookout_tower = {
    id = "lookout_tower",
    name = "Lookout Tower",
    category = "tower",
    
    -- Cost
    wood_cost = 10,
    
    -- Properties
    size = {width = 1, height = 1},
    health = 150,
    height = 3,
    
    -- Scouting
    vision_bonus = 10,
    reveals_invisible = true,
    early_warning = true,
    
    description = "Provides early warning of attacks"
}

-- GATES

defenses.gatehouse = {
    id = "gatehouse",
    name = "Gatehouse",
    category = "gate",
    
    -- Cost
    stone_cost = 20,
    
    -- Properties
    size = {width = 3, height = 2},
    health = 800,
    
    -- Function
    allows_passage = true,
    can_close = true,
    close_speed = 2,  -- seconds
    archer_positions = 4,
    
    description = "Main entrance with portcullis"
}

defenses.drawbridge = {
    id = "drawbridge",
    name = "Drawbridge",
    category = "gate",
    
    -- Cost
    wood_cost = 20,
    
    -- Properties
    size = {width = 2, height = 3},
    health = 400,
    
    -- Function
    crosses_moat = true,
    can_raise = true,
    raise_speed = 5,  -- seconds
    
    description = "Bridge that can be raised"
}

defenses.small_gate = {
    id = "small_gate",
    name = "Small Gate",
    category = "gate",
    
    -- Cost
    stone_cost = 10,
    
    -- Properties
    size = {width = 1, height = 1},
    health = 400,
    
    -- Function
    allows_passage = true,
    can_close = true,
    close_speed = 1,
    
    description = "Secondary entrance"
}

-- DEFENSIVE STRUCTURES

defenses.moat = {
    id = "moat",
    name = "Moat",
    category = "obstacle",
    
    -- Cost
    gold_cost = 50,  -- per tile (digging cost)
    
    -- Properties
    width = 1,
    depth = 2,
    
    -- Defense
    blocks_movement = true,
    blocks_tunneling = true,
    requires_bridge = true,
    can_fill_with_water = true,
    
    description = "Water-filled ditch blocking access"
}

defenses.pitch_ditch = {
    id = "pitch_ditch",
    name = "Pitch Ditch",
    category = "trap",
    
    -- Cost
    pitch_cost = 2,
    
    -- Properties
    width = 1,
    
    -- Combat
    can_ignite = true,
    fire_damage = 50,
    fire_duration = 30,  -- seconds
    area_denial = true,
    
    description = "Flammable trap that burns enemies"
}

defenses.killing_pit = {
    id = "killing_pit",
    name = "Killing Pit",
    category = "trap",
    
    -- Cost
    gold_cost = 100,
    
    -- Properties
    size = {width = 3, height = 3},
    
    -- Combat
    instant_kill = true,
    capacity = 10,  -- units killed before filling
    reset_time = 60,
    
    description = "Hidden pit that kills units"
}

defenses.brazier = {
    id = "brazier",
    name = "Brazier",
    category = "defense_equipment",
    
    -- Cost
    iron_cost = 10,
    pitch_cost = 5,
    
    -- Properties
    health = 50,
    
    -- Combat
    ignites_pitch = true,
    throw_range = 3,
    engineer_operated = true,
    
    description = "Engineers throw fire from walls"
}

defenses.mantlet = {
    id = "mantlet",
    name = "Mantlet",
    category = "defense_equipment",
    
    -- Cost
    wood_cost = 5,
    
    -- Properties
    health = 100,
    mobile = true,
    
    -- Protection
    arrow_protection = 0.9,
    provides_cover = true,
    
    description = "Portable shield for archers"
}

-- SIEGE EQUIPMENT (Defender)

defenses.ballista_tower = {
    id = "ballista_tower",
    name = "Ballista Tower",
    category = "siege_defense",
    
    -- Cost
    wood_cost = 50,
    gold_cost = 200,
    
    -- Properties
    health = 300,
    requires_engineer = true,
    
    -- Combat
    damage = 100,
    range = 15,
    rate_of_fire = 0.2,  -- shots per second
    penetrates_units = true,
    
    description = "Giant crossbow on tower"
}

defenses.mangonels = {
    id = "mangonels",
    name = "Mangonels",
    category = "siege_defense",
    
    -- Cost
    wood_cost = 50,
    gold_cost = 150,
    
    -- Properties
    health = 200,
    requires_engineer = true,
    
    -- Combat
    damage = 80,
    range = 12,
    area_damage = true,
    damage_radius = 2,
    
    description = "Defensive catapult"
}

-- SIEGE EQUIPMENT (Attacker)

defenses.catapult = {
    id = "catapult",
    name = "Catapult",
    category = "siege_offense",
    
    -- Cost
    gold_cost = 150,
    
    -- Properties
    health = 200,
    requires_engineer = true,
    mobile = true,
    
    -- Combat
    damage = 150,
    range = 10,
    targets_walls = true,
    rate_of_fire = 0.1,
    
    description = "Destroys walls and buildings"
}

defenses.trebuchet = {
    id = "trebuchet",
    name = "Trebuchet",
    category = "siege_offense",
    
    -- Cost
    gold_cost = 300,
    
    -- Properties
    health = 250,
    requires_engineer = true,
    setup_time = 30,  -- seconds
    
    -- Combat
    damage = 300,
    range = 20,
    targets_walls = true,
    rate_of_fire = 0.05,
    can_throw_cows = true,  -- Disease warfare
    
    description = "Long-range siege engine"
}

defenses.battering_ram = {
    id = "battering_ram",
    name = "Battering Ram",
    category = "siege_offense",
    
    -- Cost
    gold_cost = 100,
    
    -- Properties
    health = 300,
    requires_units = 4,
    mobile = true,
    
    -- Combat
    gate_damage = 50,  -- per hit
    attack_speed = 1,  -- hits per second
    protects_operators = true,
    
    description = "Breaks down gates"
}

defenses.siege_tower = {
    id = "siege_tower",
    name = "Siege Tower",
    category = "siege_offense",
    
    -- Cost
    gold_cost = 250,
    
    -- Properties
    health = 400,
    height = 4,
    mobile = true,
    
    -- Function
    carries_units = 20,
    deploys_to_walls = true,
    arrow_protection = 0.8,
    
    description = "Mobile tower for scaling walls"
}

defenses.fire_ballista = {
    id = "fire_ballista",
    name = "Fire Ballista",
    category = "siege_offense",
    
    -- Cost
    gold_cost = 200,
    
    -- Properties
    health = 250,
    requires_engineer = true,
    
    -- Combat
    damage = 80,
    fire_damage = 30,
    range = 12,
    ignites_buildings = true,
    
    description = "Ballista with flaming bolts"
}

defenses.portable_shield = {
    id = "portable_shield",
    name = "Portable Shield",
    category = "siege_equipment",
    
    -- Cost
    gold_cost = 50,
    
    -- Properties
    health = 150,
    mobile = true,
    
    -- Protection
    protects_units = 10,
    arrow_protection = 0.9,
    
    description = "Mobile cover for advancing troops"
}

-- Helper functions

function defenses.get_defense(id)
    return defenses[id]
end

function defenses.calculate_wall_cost(wall_type, length)
    local wall = defenses[wall_type]
    if not wall then return 0 end
    
    local costs = {}
    if wall.wood_cost then
        costs.wood = wall.wood_cost * length
    end
    if wall.stone_cost then
        costs.stone = wall.stone_cost * length
    end
    
    return costs
end

return defenses