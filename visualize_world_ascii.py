#!/usr/bin/env python3
"""
ASCII world map visualizer for the 32x32 test world
Simulates the world generation algorithm from world_generation.rs
"""

import random
import math

class BiomeType:
    FOREST = 1
    MOUNTAIN = 2
    PLAINS = 3
    DESERT = 4
    MIXED = 5

class TileType:
    GRASS = 0
    STONE = 1
    SAND = 2
    WATER = 3
    TREE = 4
    ORE_NODE = 5
    BERRY_BUSH = 6
    STORAGE = 7
    WORKSHOP = 8
    FLOOR = 9

# ASCII representation for tiles
TILE_CHARS = {
    TileType.GRASS: '.',
    TileType.STONE: '#',
    TileType.SAND: '~',
    TileType.WATER: '≈',
    TileType.TREE: '♣',
    TileType.ORE_NODE: '◊',
    TileType.BERRY_BUSH: '∙',
    TileType.STORAGE: 'S',
    TileType.WORKSHOP: 'W',
    TileType.FLOOR: '_',
}

# Color codes for terminal
TILE_COLORS = {
    TileType.GRASS: '\033[92m',     # Green
    TileType.STONE: '\033[90m',     # Gray
    TileType.SAND: '\033[93m',      # Yellow
    TileType.WATER: '\033[94m',     # Blue
    TileType.TREE: '\033[32m',      # Dark Green
    TileType.ORE_NODE: '\033[37m',  # White
    TileType.BERRY_BUSH: '\033[91m', # Light Red
    TileType.STORAGE: '\033[33m',   # Brown/Orange
    TileType.WORKSHOP: '\033[35m',  # Magenta
    TileType.FLOOR: '\033[90m',     # Gray
}

RESET_COLOR = '\033[0m'

def simple_noise(x, y, seed):
    """Simple pseudo-noise function"""
    value = math.sin(x * 12.9898 + y * 78.233 + seed * 0.1) * 43758.5453
    return abs(value - int(value))

def generate_world(width=32, height=32, seed=42, biome=BiomeType.MIXED, resource_density=0.2):
    """Generate a small test world"""
    world = [[TileType.GRASS for _ in range(width)] for _ in range(height)]
    resources = [[None for _ in range(width)] for _ in range(height)]
    
    # Generate base terrain
    for y in range(height):
        for x in range(width):
            noise_value = simple_noise(x / 10.0, y / 10.0, seed)
            
            if biome == BiomeType.FOREST:
                world[y][x] = TileType.GRASS if noise_value > 0.3 else TileType.WATER
            elif biome == BiomeType.MOUNTAIN:
                world[y][x] = TileType.STONE if noise_value > 0.5 else TileType.GRASS
            elif biome == BiomeType.PLAINS:
                world[y][x] = TileType.GRASS if noise_value > 0.1 else TileType.WATER
            elif biome == BiomeType.DESERT:
                world[y][x] = TileType.SAND if noise_value > 0.2 else TileType.STONE
            else:  # MIXED
                if noise_value > 0.7:
                    world[y][x] = TileType.STONE
                elif noise_value > 0.3:
                    world[y][x] = TileType.GRASS
                elif noise_value > 0.1:
                    world[y][x] = TileType.SAND
                else:
                    world[y][x] = TileType.WATER
    
    # Add rivers (1-3 rivers)
    random.seed(seed)
    num_rivers = random.randint(1, 3)
    for _ in range(num_rivers):
        start_x = random.randint(0, width-1)
        current_x = start_x
        for y in range(height):
            world[y][current_x] = TileType.WATER
            # Meander left or right
            current_x = max(0, min(width-1, current_x + random.randint(-1, 1)))
    
    # Place resources
    num_resources = int(width * height * resource_density)
    for _ in range(num_resources):
        x = random.randint(0, width-1)
        y = random.randint(0, height-1)
        
        if world[y][x] == TileType.GRASS:
            resources[y][x] = TileType.TREE if random.random() > 0.3 else TileType.BERRY_BUSH
        elif world[y][x] == TileType.STONE:
            if random.random() > 0.7:
                resources[y][x] = TileType.ORE_NODE
    
    # Place starting settlement in center
    center_x, center_y = width // 2, height // 2
    
    # Clear area for settlement
    for dy in range(-3, 4):
        for dx in range(-3, 4):
            if 0 <= center_y + dy < height and 0 <= center_x + dx < width:
                world[center_y + dy][center_x + dx] = TileType.GRASS
                resources[center_y + dy][center_x + dx] = None
    
    # Place buildings
    resources[center_y][center_x] = TileType.STORAGE
    resources[center_y][center_x + 2] = TileType.WORKSHOP
    
    # Place floor tiles around buildings
    for dy in range(-1, 2):
        for dx in range(-1, 2):
            if 0 <= center_y + dy < height and 0 <= center_x + dx < width:
                if resources[center_y + dy][center_x + dx] is None and (dy != 0 or dx != 0):
                    resources[center_y + dy][center_x + dx] = TileType.FLOOR
            if 0 <= center_y + dy < height and 0 <= center_x + 2 + dx < width:
                if resources[center_y + dy][center_x + 2 + dx] is None and (dy != 0 or dx != 0):
                    resources[center_y + dy][center_x + 2 + dx] = TileType.FLOOR
    
    return world, resources

def print_world(world, resources, use_color=True):
    """Print the world in ASCII"""
    height = len(world)
    width = len(world[0])
    
    # Print header
    print("\n" + "=" * 70)
    print("WORLD SIMULATOR - 32x32 TEST WORLD (MIXED BIOME)")
    print("=" * 70)
    
    # Print column numbers
    print("    ", end="")
    for x in range(0, width, 2):
        print(f"{x:2} ", end="")
    print()
    
    # Print world
    for y in range(height):
        print(f"{y:2}: ", end="")
        for x in range(width):
            # Check for resource/building first
            if resources[y][x] is not None:
                tile = resources[y][x]
            else:
                tile = world[y][x]
            
            char = TILE_CHARS.get(tile, '?')
            
            if use_color:
                color = TILE_COLORS.get(tile, '')
                print(f"{color}{char}{RESET_COLOR}", end="")
            else:
                print(char, end="")
        print(f" :{y}")
    
    # Print legend
    print("\n" + "-" * 70)
    print("LEGEND:")
    print("  Terrain:  . = Grass   # = Stone   ~ = Sand    ≈ = Water")
    print("  Resources: ♣ = Tree   ∙ = Berry   ◊ = Ore")
    print("  Buildings: S = Storage W = Workshop _ = Floor")
    print("-" * 70)
    
    # Print statistics
    terrain_counts = {}
    resource_counts = {}
    
    for y in range(height):
        for x in range(width):
            terrain = world[y][x]
            terrain_counts[terrain] = terrain_counts.get(terrain, 0) + 1
            
            if resources[y][x] is not None:
                res = resources[y][x]
                resource_counts[res] = resource_counts.get(res, 0) + 1
    
    print("\nWORLD STATISTICS:")
    print(f"  Size: {width}x{height} tiles")
    print(f"  Total tiles: {width * height}")
    print("\n  Terrain distribution:")
    for tile_type, count in sorted(terrain_counts.items(), key=lambda x: x[1], reverse=True):
        name = ['Grass', 'Stone', 'Sand', 'Water'][tile_type] if tile_type < 4 else 'Unknown'
        percentage = (count / (width * height)) * 100
        print(f"    {name:8} {count:3} tiles ({percentage:5.1f}%)")
    
    print("\n  Resources & Buildings:")
    for tile_type, count in sorted(resource_counts.items(), key=lambda x: x[1], reverse=True):
        if tile_type == TileType.TREE:
            name = "Trees"
        elif tile_type == TileType.BERRY_BUSH:
            name = "Berries"
        elif tile_type == TileType.ORE_NODE:
            name = "Ore"
        elif tile_type == TileType.STORAGE:
            name = "Storage"
        elif tile_type == TileType.WORKSHOP:
            name = "Workshop"
        elif tile_type == TileType.FLOOR:
            name = "Floor"
        else:
            name = "Unknown"
        print(f"    {name:8} {count:3}")
    
    print("\n  Settlement: Located at center ({}, {})".format(width//2, height//2))
    print("=" * 70)

if __name__ == "__main__":
    print("Generating small test world (32x32)...")
    world, resources = generate_world(
        width=32, 
        height=32, 
        seed=12345,  # Using same seed as in test_world.rs
        biome=BiomeType.MIXED,
        resource_density=0.2
    )
    
    print_world(world, resources, use_color=True)
    
    # Save to file
    with open('/tmp/world_map.txt', 'w') as f:
        # Redirect print to file
        import sys
        old_stdout = sys.stdout
        sys.stdout = f
        print_world(world, resources, use_color=False)
        sys.stdout = old_stdout
    
    print("\nWorld map also saved to /tmp/world_map.txt (without colors)")