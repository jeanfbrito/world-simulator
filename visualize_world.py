#!/usr/bin/env python3
"""
Simple world map visualizer for the 32x32 test world
Simulates the world generation algorithm from world_generation.rs
"""

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from enum import Enum
import random

class BiomeType(Enum):
    FOREST = 1
    MOUNTAIN = 2
    PLAINS = 3
    DESERT = 4
    MIXED = 5

class TileType(Enum):
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

# Color mapping for tiles
TILE_COLORS = {
    TileType.GRASS: '#4a7c4e',
    TileType.STONE: '#8c8c8c',
    TileType.SAND: '#d4c4a0',
    TileType.WATER: '#4a90e2',
    TileType.TREE: '#2d5016',
    TileType.ORE_NODE: '#4a4a4a',
    TileType.BERRY_BUSH: '#8b3a3a',
    TileType.STORAGE: '#704214',
    TileType.WORKSHOP: '#8b6f47',
    TileType.FLOOR: '#8b7355',
}

def simple_noise(x, y, seed):
    """Simple pseudo-noise function"""
    value = np.sin(x * 12.9898 + y * 78.233 + seed * 0.1) * 43758.5453
    return abs(value - int(value))

def generate_world(width=32, height=32, seed=42, biome=BiomeType.MIXED, resource_density=0.2):
    """Generate a small test world"""
    world = np.zeros((height, width), dtype=int)
    resources = np.zeros((height, width), dtype=int)
    
    # Generate base terrain
    for y in range(height):
        for x in range(width):
            noise_value = simple_noise(x / 10.0, y / 10.0, seed)
            
            if biome == BiomeType.FOREST:
                world[y, x] = TileType.GRASS.value if noise_value > 0.3 else TileType.WATER.value
            elif biome == BiomeType.MOUNTAIN:
                world[y, x] = TileType.STONE.value if noise_value > 0.5 else TileType.GRASS.value
            elif biome == BiomeType.PLAINS:
                world[y, x] = TileType.GRASS.value if noise_value > 0.1 else TileType.WATER.value
            elif biome == BiomeType.DESERT:
                world[y, x] = TileType.SAND.value if noise_value > 0.2 else TileType.STONE.value
            else:  # MIXED
                if noise_value > 0.7:
                    world[y, x] = TileType.STONE.value
                elif noise_value > 0.3:
                    world[y, x] = TileType.GRASS.value
                elif noise_value > 0.1:
                    world[y, x] = TileType.SAND.value
                else:
                    world[y, x] = TileType.WATER.value
    
    # Add rivers (1-3 rivers)
    random.seed(seed)
    num_rivers = random.randint(1, 3)
    for _ in range(num_rivers):
        start_x = random.randint(0, width-1)
        current_x = start_x
        for y in range(height):
            world[y, current_x] = TileType.WATER.value
            # Meander left or right
            current_x = max(0, min(width-1, current_x + random.randint(-1, 1)))
    
    # Place resources
    num_resources = int(width * height * resource_density)
    for _ in range(num_resources):
        x = random.randint(0, width-1)
        y = random.randint(0, height-1)
        
        if world[y, x] == TileType.GRASS.value:
            resources[y, x] = TileType.TREE.value if random.random() > 0.3 else TileType.BERRY_BUSH.value
        elif world[y, x] == TileType.STONE.value:
            if random.random() > 0.7:
                resources[y, x] = TileType.ORE_NODE.value
    
    # Place starting settlement in center
    center_x, center_y = width // 2, height // 2
    
    # Clear area for settlement
    for dy in range(-3, 4):
        for dx in range(-3, 4):
            if 0 <= center_y + dy < height and 0 <= center_x + dx < width:
                world[center_y + dy, center_x + dx] = TileType.GRASS.value
                resources[center_y + dy, center_x + dx] = 0
    
    # Place buildings
    resources[center_y, center_x] = TileType.STORAGE.value
    resources[center_y, center_x + 2] = TileType.WORKSHOP.value
    
    # Place floor tiles around buildings
    for dy in range(-1, 2):
        for dx in range(-1, 2):
            if 0 <= center_y + dy < height and 0 <= center_x + dx < width:
                if resources[center_y + dy, center_x + dx] == 0:
                    resources[center_y + dy, center_x + dx] = TileType.FLOOR.value
            if 0 <= center_y + dy < height and 0 <= center_x + 2 + dx < width:
                if resources[center_y + dy, center_x + 2 + dx] == 0:
                    resources[center_y + dy, center_x + 2 + dx] = TileType.FLOOR.value
    
    return world, resources

def visualize_world(world, resources):
    """Create a visual representation of the world"""
    height, width = world.shape
    
    # Create RGB image
    img = np.zeros((height, width, 3))
    
    for y in range(height):
        for x in range(width):
            # Get base terrain color
            base_tile = TileType(world[y, x])
            color_hex = TILE_COLORS.get(base_tile, '#000000')
            color_rgb = tuple(int(color_hex[i:i+2], 16)/255.0 for i in (1, 3, 5))
            img[y, x] = color_rgb
            
            # Overlay resources/buildings if present
            if resources[y, x] > 0:
                resource_tile = TileType(resources[y, x])
                color_hex = TILE_COLORS.get(resource_tile, '#000000')
                color_rgb = tuple(int(color_hex[i:i+2], 16)/255.0 for i in (1, 3, 5))
                # Blend with base color
                img[y, x] = color_rgb
    
    # Create figure
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 7))
    
    # Show terrain
    ax1.imshow(img, interpolation='nearest')
    ax1.set_title('Small Test World (32x32) - Mixed Biome', fontsize=14, fontweight='bold')
    ax1.set_xlabel('X')
    ax1.set_ylabel('Y')
    ax1.grid(True, alpha=0.3, linewidth=0.5)
    
    # Show resource/building overlay
    overlay = np.zeros((height, width, 4))
    for y in range(height):
        for x in range(width):
            if resources[y, x] > 0:
                resource_tile = TileType(resources[y, x])
                color_hex = TILE_COLORS.get(resource_tile, '#000000')
                color_rgb = tuple(int(color_hex[i:i+2], 16)/255.0 for i in (1, 3, 5))
                overlay[y, x] = (*color_rgb, 1.0)
    
    ax2.imshow(img, interpolation='nearest', alpha=0.3)
    ax2.imshow(overlay, interpolation='nearest')
    ax2.set_title('Resources & Buildings Layer', fontsize=14, fontweight='bold')
    ax2.set_xlabel('X')
    ax2.set_ylabel('Y')
    ax2.grid(True, alpha=0.3, linewidth=0.5)
    
    # Create legend
    legend_elements = [
        mpatches.Patch(color='#4a7c4e', label='Grass'),
        mpatches.Patch(color='#8c8c8c', label='Stone'),
        mpatches.Patch(color='#d4c4a0', label='Sand'),
        mpatches.Patch(color='#4a90e2', label='Water'),
        mpatches.Patch(color='#2d5016', label='Tree'),
        mpatches.Patch(color='#8b3a3a', label='Berry Bush'),
        mpatches.Patch(color='#4a4a4a', label='Ore Node'),
        mpatches.Patch(color='#704214', label='Storage'),
        mpatches.Patch(color='#8b6f47', label='Workshop'),
        mpatches.Patch(color='#8b7355', label='Floor'),
    ]
    ax1.legend(handles=legend_elements, loc='upper left', bbox_to_anchor=(1.05, 1), fontsize=10)
    
    plt.suptitle('World Simulator - Tilemap Visualization', fontsize=16, fontweight='bold')
    plt.tight_layout()
    
    # Save the figure
    plt.savefig('/tmp/world_map.png', dpi=150, bbox_inches='tight')
    print("World map saved to /tmp/world_map.png")
    
    plt.show()

if __name__ == "__main__":
    print("Generating small test world (32x32)...")
    world, resources = generate_world(
        width=32, 
        height=32, 
        seed=12345,  # Using same seed as in test_world.rs
        biome=BiomeType.MIXED,
        resource_density=0.2
    )
    
    print("Visualizing world...")
    visualize_world(world, resources)
    print("\nWorld features:")
    print(f"- Size: 32x32 tiles")
    print(f"- Biome: Mixed (grass, stone, sand, water)")
    print(f"- Rivers: 1-3 meandering rivers")
    print(f"- Resources: Trees, berry bushes, ore nodes")
    print(f"- Settlement: Central location with storage and workshop")