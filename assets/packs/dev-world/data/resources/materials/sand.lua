-- Sand Resource Definition
-- Fine granular material used for making glass

register_resource {
    id = "sand",
    name = "Sand",
    description = "Fine granular material found in deserts and beaches",
    category = "raw_material",

    properties = {
        weight = 1.2,
        stack_size = 100,
        base_value = 1,
    },

    spawn = {
        biomes = {"desert", "beach"},
        frequency = 0.8,
        cluster_size = {min = 5, max = 15},
    },
}