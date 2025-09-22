#!/bin/bash

# World Monitor - Shows only the living world status without debug logs
# Automatically checks for running simulations and manages instances
# Now supports IPC (Inter-Process Communication) output monitoring

# Show help if requested
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    echo "🌍 World Simulator Monitor Script"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help      Show this help message"
    echo "  --ipc-only     Only show IPC JSON output (clean)"
    echo "  --log-file     Specify custom log file path"
    echo "  --no-filter    Show all output without filtering"
    echo ""
    echo "Features:"
    echo "  • 🚀 Auto-starts simulation with pack system"
    echo "  • 📊 Filters and highlights relevant game events"
    echo "  • 🔌 Extracts and saves IPC JSON messages"
    echo "  • 🛡️ Manages multiple running instances"
    echo "  • 📝 Saves logs with timestamps"
    echo ""
    echo "Example:"
    echo "  $0              # Normal monitoring"
    echo "  $0 --ipc-only   # Only show IPC messages"
    echo ""
    exit 0
fi

# Parse command line options
IPC_ONLY=false
NO_FILTER=false
CUSTOM_LOG_FILE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --ipc-only)
            IPC_ONLY=true
            shift
            ;;
        --no-filter)
            NO_FILTER=true
            shift
            ;;
        --log-file)
            CUSTOM_LOG_FILE="$2"
            shift 2
            ;;
        *)
            # Unknown option
            shift
            ;;
    esac
done

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
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
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${YELLOW}🌍 World Simulator is running in headless mode with IPC output${NC}"
    echo -e "${BLUE}📊 Simulation Status:${NC}"
    echo -e "${PURPLE}🔌 IPC Protocol: JSON over stdout/stdin${NC}"
    echo ""
    echo -e "${YELLOW}Note: Showing new output only. Historical output not available.${NC}"
    echo "Press Ctrl+C to stop monitoring (simulation continues running)"
    echo "To kill simulation: pkill -f world_sim_simple"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
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

echo "🌍 Starting World Monitor with IPC Output..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Start simulation with output monitoring
# Use tee to both display and save output for potential later viewing
LOG_FILE="/tmp/world_sim_$(date +%Y%m%d_%H%M%S).log"
IPC_LOG_FILE="/tmp/world_sim_ipc_$(date +%Y%m%d_%H%M%S).json"
echo "📝 Simulation logging to: $LOG_FILE"
echo "🔌 IPC JSON output to: $IPC_LOG_FILE"
echo ""

echo -e "${GREEN}🚀 Starting World Simulator with Pack System & IPC Output${NC}"
echo -e "${BLUE}📦 Loading 37+ Lua entity definitions from packs...${NC}"
echo -e "${PURPLE}🔌 IPC Output: JSON messages via stdout/stdin${NC}"
echo ""

# Function to extract IPC JSON messages from output
extract_ipc_messages() {
    local input_file="$1"
    local output_file="$2"

    # Extract lines that look like JSON IPC messages (starting with {"version":)
    grep '{"version":[0-9]*,"timestamp":[0-9]*,' "$input_file" > "$output_file" 2>/dev/null

    if [ -s "$output_file" ]; then
        echo -e "${GREEN}✓ Extracted $(wc -l < "$output_file") IPC messages to $output_file${NC}"
    else
        echo -e "${YELLOW}⚠️  No IPC messages found in output${NC}"
        # Create empty file for consistency
        touch "$output_file"
    fi
}

# Use custom log file if specified
if [ ! -z "$CUSTOM_LOG_FILE" ]; then
    LOG_FILE="$CUSTOM_LOG_FILE"
fi

# Start simulation with output monitoring based on options
if [ "$IPC_ONLY" = true ]; then
    echo -e "${PURPLE}🔌 IPC-ONLY MODE: Showing only JSON messages${NC}"
    echo ""
    RUST_LOG=info cargo run -p world_sim_simple 2>&1 | \
    tee $LOG_FILE | \
    grep '{"version":[0-9]*,"timestamp":[0-9]*,' | \
    grep -v "warning:" | \
    grep -v "Compiling" | \
    grep -v "Finished" | \
    grep -v "Running"
elif [ "$NO_FILTER" = true ]; then
    echo -e "${YELLOW}📺 NO-FILTER MODE: Showing all output${NC}"
    echo ""
    RUST_LOG=info cargo run -p world_sim_simple 2>&1 | \
    tee $LOG_FILE
else
    echo -e "${GREEN}🚀 Starting World Simulator with Pack System & IPC Output${NC}"
    echo -e "${BLUE}📦 Loading 37+ Lua entity definitions from packs...${NC}"
    echo -e "${PURPLE}🔌 IPC Output: JSON messages via stdout/stdin${NC}"
    echo ""
    RUST_LOG=info cargo run -p world_sim_simple 2>&1 | \
    tee $LOG_FILE | \
    grep -E "━━━ TICK|👤 Peasant|📍|➡️|📝|✅|⚠️|🌍 World Resources|🌲|🫐|🌳|depleted|Hunger|Energy|Inventory|📏|🚶|🧍|━━━━|💤|🤔|🔍|🌾|⚒️|🍽️|😴|🎯|🏠|📦|🔨|👀|Mind State|\[MIND\]|📦 Entity spawning|🎯 Loaded|⚡ IPC|🔄 Tick|🗺️ World Map" | \
    grep -v "warning:" | \
    grep -v "Compiling" | \
    grep -v "Finished" | \
    grep -v "Running"
fi

# Extract IPC messages after simulation completes
if [ -f "$LOG_FILE" ]; then
    extract_ipc_messages "$LOG_FILE" "$IPC_LOG_FILE"

    # Show summary of IPC messages found
    if [ -s "$IPC_LOG_FILE" ]; then
        echo ""
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${BLUE}📊 IPC Output Summary:${NC}"
        echo -e "${GREEN}✓ Total IPC messages: $(wc -l < "$IPC_LOG_FILE")${NC}"
        echo -e "${PURPLE}📁 IPC log saved to: $IPC_LOG_FILE${NC}"

        # Show a sample IPC message
        echo ""
        echo -e "${YELLOW}🔍 Sample IPC message:${NC}"
        head -1 "$IPC_LOG_FILE" | head -c 200 | sed 's/$/.../' 2>/dev/null || echo "No IPC messages found"
        echo ""
    fi
fi