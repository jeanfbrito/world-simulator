#!/bin/bash
# Generate a simple placeholder tileset using ImageMagick or create a dummy file

OUTPUT_DIR="assets/textures"
OUTPUT_FILE="$OUTPUT_DIR/tileset.png"

# Check if ImageMagick is installed
if command -v convert &> /dev/null; then
    echo "Generating tileset with ImageMagick..."
    
    # Create a 512x512 tileset with colored tiles
    convert -size 512x512 xc:transparent \
        -fill '#4a7c4e' -draw "rectangle 0,0 31,31" \
        -fill '#8c8c8c' -draw "rectangle 32,0 63,31" \
        -fill '#d4c4a0' -draw "rectangle 64,0 95,31" \
        -fill '#4a90e2' -draw "rectangle 96,0 127,31" \
        -fill '#2e5d8b' -draw "rectangle 128,0 159,31" \
        -fill '#2d5016' -draw "rectangle 0,32 31,63" \
        -fill '#4a4a4a' -draw "rectangle 32,32 63,63" \
        -fill '#8b3a3a' -draw "rectangle 64,32 95,63" \
        -fill '#6b6b6b' -draw "rectangle 0,64 31,95" \
        -fill '#8b7355' -draw "rectangle 32,64 63,95" \
        -fill '#5d4e37' -draw "rectangle 64,64 95,95" \
        -fill '#704214' -draw "rectangle 96,64 127,95" \
        -fill '#8b6f47' -draw "rectangle 128,64 159,95" \
        "$OUTPUT_FILE"
    
    echo "Tileset generated at $OUTPUT_FILE"
else
    echo "ImageMagick not found. Creating a placeholder tileset file..."
    
    # Create a minimal 1x1 PNG as placeholder
    # This is a valid 1x1 transparent PNG
    printf '\x89\x50\x4e\x47\x0d\x0a\x1a\x0a\x00\x00\x00\x0d\x49\x48\x44\x52' > "$OUTPUT_FILE"
    printf '\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4' >> "$OUTPUT_FILE"
    printf '\x89\x00\x00\x00\x0d\x49\x44\x41\x54\x08\x5b\x63\x00\x01\x00\x00' >> "$OUTPUT_FILE"
    printf '\x05\x00\x01\x0d\x0a\x2d\xb4\x00\x00\x00\x00\x49\x45\x4e\x44\xae' >> "$OUTPUT_FILE"
    printf '\x42\x60\x82' >> "$OUTPUT_FILE"
    
    echo "Placeholder tileset created at $OUTPUT_FILE"
    echo "Note: This is just a placeholder. You'll need to replace it with a real tileset."
fi