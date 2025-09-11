#!/bin/bash

# World Monitor - Shows only the living world status without debug logs
# No compilation output, no warnings, no debug messages - just the world state

echo "🌍 Starting World Monitor..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Run the simulation and filter only the AI monitor output
# This shows the peasant status, resources, and activities
RUST_LOG=info cargo run -p world_sim_simple 2>&1 | \
grep -E "━━━ TICK|👤 Peasant|📍|➡️|📝|✅|⚠️|🌍 World Resources|🌲|🫐|Hunger|Energy|Inventory|📏|🚶|🧍|━━━━" | \
grep -v "warning:" | \
grep -v "Compiling" | \
grep -v "Finished" | \
grep -v "Running"