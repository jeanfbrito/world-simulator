-- Enhanced Visual Definitions for Sim Viewer
-- This file provides comprehensive visual definitions for the web viewer

return {
    -- Tile visual definitions
    tiles = {
        grass = {
            name = "Grass",
            color = "#3a5f3a",
            emoji = "🌱",
            sprite = nil,
            animation = nil,
            variant_selector = nil,
            blocks_movement = false,
            blocks_sight = false,
        },

        water = {
            name = "Water",
            color = "#1e3a8a",
            emoji = "🌊",
            sprite = nil,
            animation = nil,
            variant_selector = nil,
            blocks_movement = true,
            blocks_sight = false,
        },

        sand = {
            name = "Sand",
            color = "#c2b280",
            emoji = "🏖️",
            sprite = nil,
            animation = nil,
            variant_selector = nil,
            blocks_movement = false,
            blocks_sight = false,
        },

        stone = {
            name = "Stone",
            color = "#696969",
            emoji = "🪨",
            sprite = nil,
            animation = nil,
            variant_selector = nil,
            blocks_movement = false,
            blocks_sight = false,
        },

        forest = {
            name = "Forest",
            color = "#166534",
            emoji = "🌲",
            sprite = nil,
            animation = nil,
            variant_selector = nil,
            blocks_movement = true,
            blocks_sight = false,
        },
    },

    -- Entity visual definitions
    entities = {
        -- Units
        peasant = {
            name = "Peasant",
            category = "unit",
            color = "#8B4513",
            emoji = "👨‍🌾",
            sprite = nil,
            size = {1.0, 1.0},
            animations = {},
            attachment_points = {},
            color_variations = true,
            visual_states = {
                idle = {emoji = "👨‍🌾", color = "#8B4513"},
                working = {emoji = "⚒️", color = "#DAA520"},
                sleeping = {emoji = "😴", color = "#4B0082"},
                eating = {emoji = "🍽️", color = "#FF6347"},
            },
        },

        -- Resources
        tree = {
            name = "Tree",
            category = "resource",
            color = "#228B22",
            emoji = "🌳",
            sprite = nil,
            size = {1.0, 1.0},
            animations = {},
            attachment_points = {},
            color_variations = false,
            visual_states = {
                mature = {emoji = "🌳", color = "#228B22"},
                young = {emoji = "🌱", color = "#90EE90"},
                old = {emoji = "🍂", color = "#8B4513"},
            },
        },

        berry_bush = {
            name = "Berry Bush",
            category = "resource",
            color = "#8B4513",
            emoji = "🫐",
            sprite = nil,
            size = {1.0, 1.0},
            animations = {},
            attachment_points = {},
            color_variations = false,
            visual_states = {
                fruiting = {emoji = "🫐", color = "#8B4513"},
                flowering = {emoji = "🌸", color = "#FFB6C1"},
                dormant = {emoji = "🌿", color = "#90EE90"},
            },
        },

        berry_bush_corner = {
            name = "Berry Bush (Corner)",
            category = "resource",
            color = "#8B4513",
            emoji = "🫐",
            sprite = nil,
            size = {1.0, 1.0},
            animations = {},
            attachment_points = {},
            color_variations = false,
            visual_states = {
                fruiting = {emoji = "🫐", color = "#8B4513"},
                flowering = {emoji = "🌸", color = "#FFB6C1"},
                dormant = {emoji = "🌿", color = "#90EE90"},
            },
        },

        stone_deposit = {
            name = "Stone Deposit",
            category = "resource",
            color = "#696969",
            emoji = "🪨",
            sprite = nil,
            size = {1.0, 1.0},
            animations = {},
            attachment_points = {},
            color_variations = false,
            visual_states = {
                full = {emoji = "🪨", color = "#696969"},
                partial = {emoji = "🪨", color = "#A9A9A9"},
                depleted = {emoji = "💨", color = "#D3D3D3"},
            },
        },

        iron_ore_deposit = {
            name = "Iron Ore Deposit",
            category = "resource",
            color = "#CD853F",
            emoji = "⛏️",
            sprite = nil,
            size = {1.0, 1.0},
            animations = {},
            attachment_points = {},
            color_variations = false,
            visual_states = {
                full = {emoji = "⛏️", color = "#CD853F"},
                partial = {emoji = "⛏️", color = "#DEB887"},
                depleted = {emoji = "💨", color = "#D3D3D3"},
            },
        },

        -- Buildings
        house = {
            name = "House",
            category = "building",
            color = "#8B4513",
            emoji = "🏠",
            sprite = nil,
            size = {2.0, 2.0},
            animations = {},
            attachment_points = {},
            color_variations = true,
            visual_states = {
                empty = {emoji = "🏠", color = "#8B4513"},
                occupied = {emoji = "🏡", color = "#D2691E"},
                damaged = {emoji = "🏚️", color = "#696969"},
            },
        },

        storage = {
            name = "Storage",
            category = "building",
            color = "#D2691E",
            emoji = "📦",
            sprite = nil,
            size = {2.0, 2.0},
            animations = {},
            attachment_points = {},
            color_variations = false,
            visual_states = {
                empty = {emoji = "📦", color = "#D2691E"},
                full = {emoji = "📚", color = "#8B4513"},
            },
        },

        -- Items
        wood = {
            name = "Wood",
            category = "item",
            color = "#8B4513",
            emoji = "🪵",
            sprite = nil,
            size = {0.5, 0.5},
            animations = {},
            attachment_points = {},
            color_variations = false,
            visual_states = {},
        },

        stone = {
            name = "Stone",
            category = "item",
            color = "#696969",
            emoji = "🪨",
            sprite = nil,
            size = {0.5, 0.5},
            animations = {},
            attachment_points = {},
            color_variations = false,
            visual_states = {},
        },

        berry = {
            name = "Berry",
            category = "item",
            color = "#DC143C",
            emoji = "🫐",
            sprite = nil,
            size = {0.3, 0.3},
            animations = {},
            attachment_points = {},
            color_variations = false,
            visual_states = {},
        },

        iron_ore = {
            name = "Iron Ore",
            category = "item",
            color = "#CD853F",
            emoji = "⛏️",
            sprite = nil,
            size = {0.5, 0.5},
            animations = {},
            attachment_points = {},
            color_variations = false,
            visual_states = {},
        },
    },

    -- UI themes
    ui_themes = {
        default = {
            name = "Default",
            background = "#1a1a1a",
            text = "#ffffff",
            accent = "#4CAF50",
            warning = "#FF9800",
            error = "#F44336",
            success = "#4CAF50",
        },
        dark = {
            name = "Dark",
            background = "#0d0d0d",
            text = "#e0e0e0",
            accent = "#66BB6A",
            warning = "#FFA726",
            error = "#EF5350",
            success = "#66BB6A",
        },
        light = {
            name = "Light",
            background = "#f5f5f5",
            text = "#212121",
            accent = "#4CAF50",
            warning = "#FF9800",
            error = "#F44336",
            success = "#4CAF50",
        },
    },

    -- Animation definitions
    animations = {
        walk = {
            name = "Walk",
            frames = {"🚶", "🚶‍♂️", "🚶‍♀️"},
            duration = 0.5,
            loop = true,
        },
        work = {
            name = "Work",
            frames = {"⚒️", "🔨", "⚒️"},
            duration = 0.3,
            loop = true,
        },
        harvest = {
            name = "Harvest",
            frames = {"🌳", "🪓", "🪵"},
            duration = 0.8,
            loop = false,
        },
        eat = {
            name = "Eat",
            frames = {"🍽️", "😋", "🍽️"},
            duration = 0.6,
            loop = false,
        },
    },
}