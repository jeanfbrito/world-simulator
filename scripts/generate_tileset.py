#!/usr/bin/env python3
"""
Generate a simple placeholder tileset for the world simulator.
Creates a 512x512 PNG with 32x32 tiles based on the tileset_config.toml
"""

import os
import sys
import toml
from PIL import Image, ImageDraw, ImageFont

def hex_to_rgb(hex_color):
    """Convert hex color to RGB tuple."""
    hex_color = hex_color.lstrip('#')
    return tuple(int(hex_color[i:i+2], 16) for i in (0, 2, 4))

def create_tile(size, color, label=None):
    """Create a single tile with color and optional label."""
    tile = Image.new('RGBA', (size, size), color)
    draw = ImageDraw.Draw(tile)
    
    # Add a simple border
    border_color = tuple(max(0, c - 50) for c in color[:3]) + (255,)
    draw.rectangle([0, 0, size-1, size-1], outline=border_color, width=2)
    
    # Add some texture variation
    for i in range(0, size, 4):
        for j in range(0, size, 4):
            if (i + j) % 8 == 0:
                variation = tuple(max(0, min(255, c + 10)) for c in color[:3]) + (255,)
                draw.rectangle([i, j, i+2, j+2], fill=variation)
    
    return tile

def generate_tileset(config_path, output_path):
    """Generate tileset from configuration."""
    # Load configuration
    with open(config_path, 'r') as f:
        config = toml.load(f)
    
    metadata = config['metadata']
    tile_size = metadata['tile_size']
    texture_width = metadata['texture_width']
    texture_height = metadata['texture_height']
    tiles_per_row = metadata['tiles_per_row']
    
    # Create base image
    tileset = Image.new('RGBA', (texture_width, texture_height), (0, 0, 0, 0))
    
    # Generate each tile
    for tile_config in config['tiles']:
        index = tile_config['index']
        color = hex_to_rgb(tile_config['color']) + (255,)  # Add alpha
        name = tile_config['name']
        
        # Calculate position in tileset
        row = index // tiles_per_row
        col = index % tiles_per_row
        x = col * tile_size
        y = row * tile_size
        
        # Create and paste tile
        tile = create_tile(tile_size, color, name)
        tileset.paste(tile, (x, y))
        
        print(f"Generated tile '{name}' at index {index} ({x}, {y})")
    
    # Save tileset
    tileset.save(output_path)
    print(f"\nTileset saved to {output_path}")

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(script_dir)
    
    config_path = os.path.join(project_root, 'assets', 'textures', 'tileset_config.toml')
    output_path = os.path.join(project_root, 'assets', 'textures', 'tileset.png')
    
    if not os.path.exists(config_path):
        print(f"Error: Config file not found at {config_path}")
        sys.exit(1)
    
    try:
        import PIL
        import toml
    except ImportError as e:
        print("Missing dependencies. Install with:")
        print("pip install pillow toml")
        sys.exit(1)
    
    generate_tileset(config_path, output_path)

if __name__ == "__main__":
    main()