-- Clay Resource Definition
-- Malleable earth material used for pottery and bricks

register_resource {
    id = "clay",
    name = "Clay",
    description = "Malleable earth material that can be shaped and fired",
    category = "raw_material",

    properties = {
        weight = 1.2,
        stack_size = 100,
        base_value = 2,
    },

    spawn = {
        biomes = {"riverbank", "swamp", "plains"},
        frequency = 0.3,
        cluster_size = {min = 3, max = 8},
    },
}