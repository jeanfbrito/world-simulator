#!/bin/bash

# World Monitor - Three-tier Architecture with Sim Viewer
# Automatically manages both the simulator and sim viewer

# Show help if requested
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    echo "🌍 World Simulator Monitor Script - Three-Tier Architecture"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help       Show this help message"
    echo "  --viewer-only    Only show viewer status (no simulator)"
    echo "  --sim-only       Only run simulator (no viewer)"
    echo "  --pack-path      Specify pack path for viewer"
    echo "  --viewer-port    Specify viewer WebSocket port (default: 8080)"
    echo ""
    echo "Features:"
    echo "  • 🚀 Auto-starts both simulator and sim viewer"
    echo "  • 📦 Pack loading with visual definitions"
    echo "  • 🔌 IPC communication between simulator and viewer"
    echo "  • 🌐 WebSocket bridge for web clients"
    echo "  • 📊 Real-time monitoring of both processes"
    echo ""
    echo "Architecture:"
    echo "  World Simulator (IPC) → Sim Viewer (WebSocket) → Web Browser"
    echo ""
    echo "Example:"
    echo "  $0                    # Full three-tier monitoring"
    echo "  $0 --viewer-only      # Only viewer for existing simulator"
    echo "  $0 --pack-path ./assets/packs/dev-world"
    echo ""
    exit 0
fi

# Parse command line options
VIEWER_ONLY=false
SIM_ONLY=false
PACK_PATH=""
VIEWER_PORT=8080

while [[ $# -gt 0 ]]; do
    case $1 in
        --viewer-only)
            VIEWER_ONLY=true
            shift
            ;;
        --sim-only)
            SIM_ONLY=true
            shift
            ;;
        --pack-path)
            PACK_PATH="$2"
            shift 2
            ;;
        --viewer-port)
            VIEWER_PORT="$2"
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
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to check and cleanup processes
cleanup_processes() {
    echo -e "${YELLOW}🧹 Checking for existing processes...${NC}"

    # Check for running simulators
    RUNNING_SIMS=$(ps aux | grep "target/debug/world_sim_simple" | grep -v grep | awk '{print $2}')
    SIM_COUNT=$(echo "$RUNNING_SIMS" | wc -w)

    # Check for running viewers
    RUNNING_VIEWERS=$(ps aux | grep "target/debug/world_sim_viewer" | grep -v grep | awk '{print $2}')
    VIEWER_COUNT=$(echo "$RUNNING_VIEWERS" | wc -w)

    if [ $SIM_COUNT -gt 0 ] || [ $VIEWER_COUNT -gt 0 ]; then
        echo -e "${YELLOW}Found running processes:${NC}"
        [ $SIM_COUNT -gt 0 ] && echo -e "  ${BLUE}🖥️  Simulators: $SIM_COUNT (PIDs: $RUNNING_SIMS)${NC}"
        [ $VIEWER_COUNT -gt 0 ] && echo -e "  ${PURPLE}🌐 Viewers: $VIEWER_COUNT (PIDs: $RUNNING_VIEWERS)${NC}"

        if [ "$VIEWER_ONLY" = true ] && [ $SIM_COUNT -eq 0 ]; then
            echo -e "${RED}❌ Viewer-only mode requires a running simulator!${NC}"
            exit 1
        fi

        read -p "Kill existing processes? [y/N]: " kill_choice
        if [[ "$kill_choice" =~ ^[Yy]$ ]]; then
            [ $SIM_COUNT -gt 0 ] && echo -e "${RED}Stopping simulators...${NC}" && pkill -f "world_sim_simple"
            [ $VIEWER_COUNT -gt 0 ] && echo -e "${RED}Stopping viewers...${NC}" && pkill -f "world_sim_viewer"
            sleep 2
        else
            echo "Keeping existing processes..."
        fi
    fi
}

# Function to start simulator
start_simulator() {
    echo -e "${GREEN}🚀 Starting World Simulator...${NC}"

    # Start simulator in background with IPC output
    RUST_LOG=info cargo run -p world_sim_simple > /tmp/simulator_output.log 2>&1 &
    SIM_PID=$!
    echo "Simulator PID: $SIM_PID"

    # Wait for simulator to start
    echo "Waiting for simulator to initialize..."
    sleep 3

    # Check if simulator is running
    if ! ps -p $SIM_PID > /dev/null; then
        echo -e "${RED}❌ Failed to start simulator${NC}"
        echo "Check logs: tail -f /tmp/simulator_output.log"
        exit 1
    fi

    echo -e "${GREEN}✅ Simulator started successfully${NC}"
}

# Function to start viewer
start_viewer() {
    echo -e "${PURPLE}🌐 Starting Sim Viewer...${NC}"

    # Build viewer if needed
    if [ ! -f "target/debug/world_sim_viewer" ]; then
        echo "Building viewer..."
        cargo build -p world_sim_viewer
    fi

    # Set pack path
    PACK_ARG=""
    if [ ! -z "$PACK_PATH" ]; then
        PACK_ARG="--pack-path $PACK_PATH"
        echo "Using pack path: $PACK_PATH"
    else
        PACK_ARG="--pack-path ./assets/packs/dev-world"
        echo "Using default pack path: ./assets/packs/dev-world"
    fi

    # Start viewer in background
    cargo run -p world_sim_viewer -- --port $VIEWER_PORT $PACK_ARG > /tmp/viewer_output.log 2>&1 &
    VIEWER_PID=$!
    echo "Viewer PID: $VIEWER_PID on port $VIEWER_PORT"

    # Wait for viewer to start
    echo "Waiting for viewer to initialize..."
    sleep 3

    # Check if viewer is running
    if ! ps -p $VIEWER_PID > /dev/null; then
        echo -e "${RED}❌ Failed to start viewer${NC}"
        echo "Check logs: tail -f /tmp/viewer_output.log"
        exit 1
    fi

    echo -e "${GREEN}✅ Viewer started successfully${NC}"
}

# Function to monitor both processes
monitor_processes() {
    echo ""
    echo -e "${CYAN}📊 Three-Tier Architecture Monitor${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo -e "${BLUE}🖥️  Simulator → IPC → 🌐 Viewer → WebSocket → 🌍 Browser${NC}"
    echo ""

    if [ "$SIM_ONLY" = true ]; then
        echo -e "${YELLOW}📺 SIMULATOR-ONLY MODE${NC}"
        echo "Monitoring simulator output:"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        tail -f /tmp/simulator_output.log | grep -E "📦|✅|⚡|IPC|TICK|━━━|👤|📍|➡️|📝|⚠️|🌍|🌲|🫐|🌳|depleted|Hunger|Energy|Inventory|📏|🚶|🧍|💤|🤔|🔍|🌾|⚒️|🍽️|😴|🎯|🏠|📦|🔨|👀|Mind State|\[MIND\]|Entity spawning|Loaded"
    else
        echo -e "${PURPLE}🌐 THREE-TIER MODE${NC}"
        echo -e "${CYAN}WebSocket server: ws://localhost:$VIEWER_PORT${NC}"
        echo ""
        echo "Process Status:"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

        # Monitor loop
        while true; do
            clear
            echo -e "${CYAN}📊 Three-Tier Architecture Monitor${NC}"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo ""

            # Check simulator status
            if [ ! -z "$SIM_PID" ] && ps -p $SIM_PID > /dev/null; then
                echo -e "${GREEN}✅ Simulator (PID: $SIM_PID) - Running${NC}"
                # Show last few lines of simulator output
                echo -e "${BLUE}📜 Recent Simulator Activity:${NC}"
                tail -5 /tmp/simulator_output.log | sed 's/^/  /'
            else
                echo -e "${RED}❌ Simulator - Not Running${NC}"
            fi

            echo ""

            # Check viewer status
            if [ ! -z "$VIEWER_PID" ] && ps -p $VIEWER_PID > /dev/null; then
                echo -e "${GREEN}✅ Viewer (PID: $VIEWER_PID) - Running on port $VIEWER_PORT${NC}"
                # Show last few lines of viewer output
                echo -e "${PURPLE}📜 Recent Viewer Activity:${NC}"
                tail -5 /tmp/viewer_output.log | sed 's/^/  /'
            else
                echo -e "${RED}❌ Viewer - Not Running${NC}"
            fi

            echo ""
            echo -e "${CYAN}🌐 Access your simulation:${NC}"
            echo "  WebSocket: ws://localhost:$VIEWER_PORT"
            echo "  Web Viewer: Open your web viewer app and connect to ws://localhost:$VIEWER_PORT"
            echo ""
            echo "Press Ctrl+C to stop monitoring"
            echo "To kill processes: pkill -f 'world_sim'"

            sleep 2
        done
    fi
}

# Main execution
echo "🌍 World Simulator Three-Tier Monitor"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Handle different modes
if [ "$VIEWER_ONLY" = true ]; then
    echo -e "${PURPLE}🌐 VIEWER-ONLY MODE${NC}"
    echo "Starting viewer only (assuming simulator is already running)..."
    start_viewer
    monitor_processes
elif [ "$SIM_ONLY" = true ]; then
    echo -e "${BLUE}🖥️  SIMULATOR-ONLY MODE${NC}"
    echo "Starting simulator only..."
    cleanup_processes
    start_simulator
    monitor_processes
else
    echo -e "${CYAN}🚀 THREE-TIER MODE${NC}"
    echo "Starting both simulator and viewer..."
    cleanup_processes
    start_simulator
    start_viewer
    monitor_processes
fi