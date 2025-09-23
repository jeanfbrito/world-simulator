-- Development World Pack
-- Basic pack for testing the three-tier architecture

return {
    id = "dev-world",
    name = "Development World",
    version = "0.1.0",
    author = "World Simulator Team",
    description = "Basic development pack for testing IPC and viewer functionality",
    dependencies = {},
    load_order = {
        "entities",
        "resources",
        "items",
        "recipes",
        "world"
    },

    config = {
        debug = true,
        validate_strict = false,
        allow_hot_reload = true
    }
}