-- Custom World Generation Configuration
-- This configuration demonstrates advanced world generation features
-- including procedural terrain, custom biomes, and dynamic resource distribution

---@class WorldConfig
local config = {
    -- World Generation Parameters
    world = {
        width = 32,
        height = 32,
        seed = 1337, -- Random seed for reproducible generation
        tile_size = 1.0,

        -- Terrain Generation
        terrain = {
            -- Noise parameters for terrain generation
            noise_scale = 0.1,
            noise_octaves = 4,
            noise_persistence = 0.5,
            noise_lacunarity = 2.0,

            -- Elevation parameters
            sea_level = 0.3,
            mountain_level = 0.7,

            -- Feature distribution
            forest_density = 0.3,
            mountain_density = 0.2,
            water_density = 0.25,

            -- Biome definitions
            biomes = {
                {
                    name = "plains",
                    humidity_range = {0.3, 0.7},
                    temperature_range = {0.4, 0.8},
                    color = {0.2, 0.8, 0.2},
                    resources = {"wood", "food"},
                    features = {"trees", "grass"}
                },
                {
                    name = "forest",
                    humidity_range = {0.6, 1.0},
                    temperature_range = {0.3, 0.7},
                    color = {0.1, 0.5, 0.1},
                    resources = {"wood", "food"},
                    features = {"dense_trees", "wildlife"}
                },
                {
                    name = "mountains",
                    humidity_range = {0.0, 0.4},
                    temperature_range = {0.0, 0.4},
                    color = {0.5, 0.4, 0.3},
                    resources = {"stone", "ore"},
                    features = {"rocks", "caves"}
                },
                {
                    name = "desert",
                    humidity_range = {0.0, 0.2},
                    temperature_range = {0.7, 1.0},
                    color = {0.9, 0.8, 0.4},
                    resources = {"stone", "ore"},
                    features = {"dunes", "oasis"}
                },
                {
                    name = "tundra",
                    humidity_range = {0.2, 0.6},
                    temperature_range = {0.0, 0.3},
                    color = {0.8, 0.9, 0.9},
                    resources = {"stone"},
                    features = {"snow", "ice"}
                }
            }
        },

        -- Resource Generation
        resources = {
            -- Global resource settings
            regeneration_rate = 0.02,
            max_resources_per_tile = 3,

            -- Resource type definitions
            types = {
                wood = {
                    name = "wood",
                    max_amount = 100,
                    min_amount = 20,
                    regeneration_rate = 0.03,
                    biome_preference = {"forest", "plains"},
                    cluster_size = 3,
                    cluster_density = 0.4
                },
                stone = {
                    name = "stone",
                    max_amount = 80,
                    min_amount = 15,
                    regeneration_rate = 0.01,
                    biome_preference = {"mountains", "desert", "tundra"},
                    cluster_size = 4,
                    cluster_density = 0.3
                },
                food = {
                    name = "food",
                    max_amount = 60,
                    min_amount = 10,
                    regeneration_rate = 0.05,
                    biome_preference = {"plains", "forest"},
                    cluster_size = 2,
                    cluster_density = 0.5
                },
                ore = {
                    name = "ore",
                    max_amount = 40,
                    min_amount = 5,
                    regeneration_rate = 0.005,
                    biome_preference = {"mountains"},
                    cluster_size = 2,
                    cluster_density = 0.2
                }
            }
        },

        -- Feature Generation
        features = {
            -- Natural features
            rivers = {
                count = 2,
                width = 2,
                meander_strength = 0.3
            },
            lakes = {
                count = 3,
                min_size = 4,
                max_size = 12
            },
            forests = {
                count = 5,
                min_size = 6,
                max_size = 20
            },
            mountains = {
                count = 2,
                min_height = 3,
                max_height = 8
            }
        }
    },

    -- Entity Configuration
    entities = {
        -- Unit definitions for custom world
        units = {
            peasant = {
                name = "peasant",
                display_name = "Settler",
                description = "Basic settler unit for world development",

                -- Unit properties
                health = 100,
                max_health = 100,
                energy = 100,
                max_energy = 100,

                -- Movement (using ticks_per_tile as per project standard)
                ticks_per_tile = 2,

                -- Resource carrying capacity
                inventory_size = 50,

                -- Vision and detection
                vision_range = 8,
                detection_range = 6,

                -- AI configuration
                ai = {
                    personality = "balanced",
                    behavior_weights = {
                        exploration = 0.3,
                        gathering = 0.4,
                        building = 0.2,
                        social = 0.1
                    },
                    goals = {
                        "survive",
                        "gather_resources",
                        "expand_settlement",
                        "cooperate"
                    }
                },

                -- Special abilities
                abilities = {
                    "gather_wood",
                    "gather_stone",
                    "gather_food",
                    "build_basic",
                    "explore"
                },

                -- Spawn requirements
                spawn_requirements = {
                    resources = {
                        food = 10
                    },
                    buildings = {
                        house = 1
                    }
                },

                -- Appearance
                appearance = {
                    color = {0.4, 0.3, 0.2},
                    size = 1.0,
                    sprite = "peasant"
                }
            }
        },

        -- Building definitions
        buildings = {
            house = {
                name = "house",
                display_name = "Settler's House",
                description = "Basic housing for settlers",

                -- Building properties
                health = 200,
                max_health = 200,
                construction_time = 100,

                -- Resource requirements
                construction_cost = {
                    wood = 20,
                    stone = 10
                },

                -- Functionality
                provides = {
                    housing = 4,
                    comfort = 0.2
                },

                -- Placement requirements
                placement_requirements = {
                    terrain = "ground",
                    min_distance_from_water = 2,
                    max_slope = 0.3
                },

                -- Appearance
                appearance = {
                    color = {0.8, 0.6, 0.4},
                    size = {2, 2},
                    sprite = "house"
                }
            },

            workshop = {
                name = "workshop",
                display_name = "Crafting Workshop",
                description = "Basic crafting and production building",

                -- Building properties
                health = 150,
                max_health = 150,
                construction_time = 150,

                -- Resource requirements
                construction_cost = {
                    wood = 30,
                    stone = 20
                },

                -- Functionality
                provides = {
                    crafting = true,
                    storage = 20,
                    efficiency = 0.3
                },

                -- Placement requirements
                placement_requirements = {
                    terrain = "ground",
                    min_distance_from_water = 1,
                    max_slope = 0.2,
                    near_building = "house"
                },

                -- Appearance
                appearance = {
                    color = {0.6, 0.4, 0.2},
                    size = {3, 3},
                    sprite = "workshop"
                }
            },

            storage = {
                name = "storage",
                display_name = "Storage Warehouse",
                description = "Large storage facility for resources",

                -- Building properties
                health = 300,
                max_health = 300,
                construction_time = 200,

                -- Resource requirements
                construction_cost = {
                    wood = 40,
                    stone = 30
                },

                -- Functionality
                provides = {
                    storage = 100,
                    protection = 0.1,
                    organization = 0.2
                },

                -- Placement requirements
                placement_requirements = {
                    terrain = "ground",
                    max_slope = 0.1,
                    near_road = true
                },

                -- Appearance
                appearance = {
                    color = {0.5, 0.5, 0.5},
                    size = {4, 4},
                    sprite = "storage"
                }
            }
        }
    },

    -- Simulation Parameters
    simulation = {
        -- Game speed
        tick_rate = 30,
        day_length = 1000, -- ticks per day

        -- Economic parameters
        resource_spawn_rate = 0.02,
        building_efficiency = 1.0,
        unit_efficiency = 1.0,

        -- AI parameters
        ai_update_interval = 10,
        pathfinding_complexity = "medium",

        -- World evolution
        seasons_enabled = true,
        weather_enabled = true,
        natural_disasters = false,

        -- Victory conditions
        victory_conditions = {
            population = 50,
            buildings = 20,
            resources = 1000
        }
    },

    -- UI and Visualization
    ui = {
        -- Display settings
        show_grid = true,
        show_coordinates = false,
        show_resources = true,
        show_buildings = true,
        show_units = true,

        -- Camera settings
        camera_speed = 5.0,
        zoom_levels = {0.5, 1.0, 2.0, 4.0},

        -- Colors
        colors = {
            grid = {0.2, 0.2, 0.2},
            background = {0.1, 0.1, 0.15},
            selection = {1.0, 1.0, 0.0},
            highlight = {0.0, 1.0, 1.0}
        }
    }
}

-- Return the configuration
return config