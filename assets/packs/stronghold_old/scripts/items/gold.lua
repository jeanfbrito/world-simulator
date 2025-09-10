-- Stronghold-style Gold: Primary currency

return {
    id = "gold",
    name = "Gold",
    category = "currency",
    stack_size = 99999,  -- Essentially unlimited
    
    -- Pure currency
    is_currency = true,
    
    -- Sources of gold
    sources = {
        "taxation",
        "trade",
        "church_tithes",
        "ale_sales"
    },
    
    -- Uses
    pays_for = {
        "soldiers_wages",
        "castle_construction",
        "weapons",
        "siege_equipment",
        "bribes"
    },
    
    description = "The lifeblood of your kingdom"
}