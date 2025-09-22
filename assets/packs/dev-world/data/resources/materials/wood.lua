-- Wood Resource Definition
-- Basic building material harvested from trees

register_resource {
    id = "wood",
    name = "Wood",
    description = "Raw timber harvested from trees",
    category = "raw_material",

    properties = {
        weight = 1.5,
        stack_size = 100,
        base_value = 2,
    },

    -- Trees are separate entities that yield wood
    -- This defines the wood resource itself
}