-- Dev World Pack Metadata
-- This is the base game content pack containing all standard resources, items, recipes, and entities

pack_metadata = {
    id = "dev-world",
    name = "Development World",
    version = "0.1.0",
    author = "World Simulator Team",
    description = "Base game content pack with all standard gameplay elements",

    -- Dependencies (empty for base pack)
    dependencies = {},

    -- Load order is important - resources must be loaded before items that reference them
    load_order = {
        "resources",  -- Raw materials, plants, etc.
        "items",      -- Tools, consumables, etc. that reference resources
        "recipes",    -- Crafting recipes that reference items
        "entities",   -- Units and buildings that may reference items
        "world",      -- World generation that references everything
    },

    -- Optional configuration
    config = {
        debug = true,                    -- Enable debug logging for this pack
        validate_strict = true,           -- Strict validation of data
        allow_hot_reload = true,          -- Allow hot-reloading during development
    },

    -- Pack features (for future compatibility checking)
    features = {
        "base_resources",
        "crafting_system",
        "unit_spawning",
        "building_system",
        "world_generation",
    },
}