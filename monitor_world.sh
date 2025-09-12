#!/bin/bash

# World Monitor - Shows only the living world status without debug logs
# Automatically checks for running simulations and manages instances

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check for any running simulations
RUNNING_PIDS=$(ps aux | grep "target/debug/world_sim_simple" | grep -v grep | awk '{print $2}')
PID_COUNT=$(echo "$RUNNING_PIDS" | wc -w)

if [ $PID_COUNT -gt 1 ]; then
    echo -e "${RED}⚠️  WARNING: Found $PID_COUNT simulations running!${NC}"
    echo "PIDs: $RUNNING_PIDS"
    echo ""
    echo "Options:"
    echo "  1) Kill all and start fresh (recommended)"
    echo "  2) Keep one and kill others"
    echo "  3) Exit without changes"
    echo ""
    read -p "Choice [1-3]: " choice
    
    case $choice in
        1)
            echo "Killing all simulations..."
            pkill -f "world_sim_simple"
            sleep 1
            ;;
        2)
            KEEP_PID=$(echo "$RUNNING_PIDS" | head -1)
            echo "Keeping PID $KEEP_PID, killing others..."
            for pid in $RUNNING_PIDS; do
                if [ "$pid" != "$KEEP_PID" ]; then
                    kill -9 $pid
                fi
            done
            ;;
        3)
            echo "Exiting..."
            exit 0
            ;;
        *)
            echo "Invalid choice, exiting..."
            exit 1
            ;;
    esac
fi

# Check again after potential cleanup
RUNNING_PID=$(ps aux | grep "target/debug/world_sim_simple" | grep -v grep | awk '{print $2}' | head -1)

if [ ! -z "$RUNNING_PID" ]; then
    echo -e "${GREEN}✓ Found running simulation (PID: $RUNNING_PID)${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${YELLOW}Note: This will show new output only. Historical output not available.${NC}"
    echo "Press Ctrl+C to stop monitoring (simulation continues running)"
    echo "To kill simulation: pkill -f world_sim_simple"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # Since we can't easily attach to existing stdout, we'll restart with monitoring
    echo -e "${YELLOW}Restarting simulation with monitoring...${NC}"
    pkill -f "world_sim_simple"
    sleep 1
fi

# Check if another process is compiling/starting
if ps aux | grep "cargo run.*world_sim_simple" | grep -v grep > /dev/null; then
    echo "⏳ Another process is starting the simulation..."
    echo "Waiting for it to be ready..."
    sleep 5
    # Recursively call ourselves to check again
    exec $0
fi

echo "🌍 Starting World Monitor..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Start simulation with output monitoring
# Use tee to both display and save output for potential later viewing
LOG_FILE="/tmp/world_sim_$(date +%Y%m%d_%H%M%S).log"
echo "📝 Logging to: $LOG_FILE"
echo ""

RUST_LOG=info cargo run -p world_sim_simple 2>&1 | \
tee $LOG_FILE | \
grep -E "━━━ TICK|👤 Peasant|📍|➡️|📝|✅|⚠️|🌍 World Resources|🌲|🫐|🌳|depleted|Hunger|Energy|Inventory|📏|🚶|🧍|━━━━" | \
grep -v "warning:" | \
grep -v "Compiling" | \
grep -v "Finished" | \
grep -v "Running"