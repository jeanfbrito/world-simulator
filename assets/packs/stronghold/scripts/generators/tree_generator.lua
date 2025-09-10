-- Tree Generation Script for Stronghold-style settlements
-- Generates trees in natural clusters and groves using Lua

tree_generator = {
    name = "Tree Generator",
    version = "1.0.0",
    author = "World Simulator",
    
    -- Configuration
    config = {
        -- Tree spawning parameters
        tree_density = 0.15,           -- Base tree density (0.0 to 1.0)
        cluster_size = 3,              -- Average trees per cluster
        cluster_variation = 2,         -- Random variation in cluster size
        grove_chance = 0.1,            -- Chance of creating large groves
        grove_size = 8,                -- Base size of groves
        
        -- Tree type distribution
        tree_types = {
            oak = { weight = 40, wood_yield = 15, growth_time = 60 },
            pine = { weight = 30, wood_yield = 12, growth_time = 45 },
            birch = { weight = 20, wood_yield = 8, growth_time = 30 },
            apple = { weight = 10, wood_yield = 6, growth_time = 90, fruit_yield = 5 }
        },
        
        -- Terrain preferences
        terrain_preference = {
            grass = 1.0,       -- Trees love grass
            dirt = 0.8,        -- Trees can grow on dirt
            forest = 2.0,      -- Forest terrain encourages more trees
            sand = 0.2,        -- Trees struggle in sand
            stone = 0.1,       -- Very few trees on stone
            water = 0.0,       -- No trees in water
            mountain = 0.3     -- Some hardy trees on mountains
        },
        
        -- Distance from settlements (tiles)
        min_distance_from_buildings = 3,
        preferred_distance = 5,
        max_distance = 30
    },
    
    -- Generate trees for a given map area
    generate = function(self, map_data)
        local generated_trees = {}
        local map_width = map_data.width or 64
        local map_height = map_data.height or 64
        
        -- Create tree clusters
        local clusters = self:create_clusters(map_width, map_height)
        
        for _, cluster in ipairs(clusters) do
            local cluster_trees = self:generate_cluster(cluster, map_data)
            
            -- Add trees to the result
            for _, tree in ipairs(cluster_trees) do
                table.insert(generated_trees, tree)
            end
        end
        
        return generated_trees
    end,
    
    -- Create cluster points across the map
    create_clusters = function(self, width, height)
        local clusters = {}
        local total_area = width * height
        local target_clusters = math.floor(total_area * self.config.tree_density / self.config.cluster_size)
        
        for i = 1, target_clusters do
            local cluster = {
                x = math.random(5, width - 5),
                y = math.random(5, height - 5),
                size = math.max(1, self.config.cluster_size + math.random(-self.config.cluster_variation, self.config.cluster_variation)),
                is_grove = math.random() < self.config.grove_chance
            }
            
            -- Make groves larger
            if cluster.is_grove then
                cluster.size = cluster.size + self.config.grove_size
            end
            
            table.insert(clusters, cluster)
        end
        
        return clusters
    end,
    
    -- Generate trees for a specific cluster
    generate_cluster = function(self, cluster, map_data)
        local trees = {}
        local attempts = 0
        local max_attempts = cluster.size * 3  -- Prevent infinite loops
        
        while #trees < cluster.size and attempts < max_attempts do
            attempts = attempts + 1
            
            -- Random position around cluster center
            local radius = math.random(0, 4)  -- Trees spread within 4 tiles of center
            local angle = math.random() * 2 * math.pi
            local x = math.floor(cluster.x + radius * math.cos(angle))
            local y = math.floor(cluster.y + radius * math.sin(angle))
            
            -- Check if position is valid
            if self:is_valid_tree_position(x, y, map_data) then
                local tree_type = self:select_tree_type(x, y, map_data)
                
                local tree = {
                    x = x,
                    y = y,
                    type = tree_type,
                    health = 100,
                    growth_stage = "mature",  -- Trees start mature for gameplay
                    wood_yield = self.config.tree_types[tree_type].wood_yield,
                    fruit_yield = self.config.tree_types[tree_type].fruit_yield or 0,
                    last_harvest = 0,
                    
                    -- Visual properties
                    sprite = tree_type .. "_tree",
                    color = { r = 0.18, g = 0.31, b = 0.09, a = 1.0 },
                    
                    -- Gameplay properties
                    blocks_movement = true,
                    blocks_building = true,
                    harvestable = true,
                    respawn_time = self.config.tree_types[tree_type].growth_time
                }
                
                table.insert(trees, tree)
            end
        end
        
        return trees
    end,
    
    -- Check if a position is valid for tree placement
    is_valid_tree_position = function(self, x, y, map_data)
        -- Check bounds
        if x < 0 or x >= (map_data.width or 64) or y < 0 or y >= (map_data.height or 64) then
            return false
        end
        
        -- Check terrain preference
        local terrain = map_data.terrain and map_data.terrain[y] and map_data.terrain[y][x] or "grass"
        local preference = self.config.terrain_preference[terrain] or 0.5
        
        if math.random() > preference then
            return false
        end
        
        -- Check distance from buildings
        if map_data.buildings then
            for _, building in ipairs(map_data.buildings) do
                local distance = math.sqrt((x - building.x)^2 + (y - building.y)^2)
                if distance < self.config.min_distance_from_buildings then
                    return false
                end
            end
        end
        
        -- Check for existing trees (no overlaps)
        if map_data.existing_trees then
            for _, existing in ipairs(map_data.existing_trees) do
                if existing.x == x and existing.y == y then
                    return false
                end
            end
        end
        
        return true
    end,
    
    -- Select tree type based on terrain and randomness
    select_tree_type = function(self, x, y, map_data)
        local terrain = map_data.terrain and map_data.terrain[y] and map_data.terrain[y][x] or "grass"
        
        -- Build weighted list based on terrain
        local weighted_types = {}
        local total_weight = 0
        
        for tree_type, data in pairs(self.config.tree_types) do
            local weight = data.weight
            
            -- Adjust weight based on terrain
            if terrain == "forest" then
                weight = weight * 1.5  -- More trees in forest
            elseif terrain == "mountain" then
                if tree_type == "pine" then
                    weight = weight * 2.0  -- Pines love mountains
                else
                    weight = weight * 0.5
                end
            elseif terrain == "grass" then
                if tree_type == "oak" or tree_type == "apple" then
                    weight = weight * 1.2  -- Oaks and fruit trees prefer grassland
                end
            end
            
            if weight > 0 then
                table.insert(weighted_types, { type = tree_type, weight = weight })
                total_weight = total_weight + weight
            end
        end
        
        -- Select random type based on weights
        local random_value = math.random() * total_weight
        local current_weight = 0
        
        for _, entry in ipairs(weighted_types) do
            current_weight = current_weight + entry.weight
            if random_value <= current_weight then
                return entry.type
            end
        end
        
        -- Fallback to oak
        return "oak"
    end,
    
    -- Generate a forest area (for testing)
    generate_forest = function(self, center_x, center_y, radius)
        local forest_data = {
            width = radius * 2 + 10,
            height = radius * 2 + 10,
            terrain = {},
            buildings = {},
            existing_trees = {}
        }
        
        -- Create forest terrain
        for y = 1, forest_data.height do
            forest_data.terrain[y] = {}
            for x = 1, forest_data.width do
                local distance = math.sqrt((x - radius)^2 + (y - radius)^2)
                if distance <= radius then
                    forest_data.terrain[y][x] = "forest"
                else
                    forest_data.terrain[y][x] = "grass"
                end
            end
        end
        
        return self:generate(forest_data)
    end
}

-- Export the tree generator for use by the game
return tree_generator