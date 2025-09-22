#!/bin/bash

# World Simulator - Complete Three-Tier Launch Script
# Automatically runs simulator, viewer, and opens web interface

# Show help if requested
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    echo "🌍 World Simulator Complete Launch Script"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help           Show this help message"
    echo "  --pack-path          Specify pack path for viewer"
    echo "  --viewer-port        Specify viewer WebSocket port (default: 11748)"
    echo "  --web-port           Specify web server port (default: 3000)"
    echo "  --no-browser         Don't open web browser automatically"
    echo "  --debug              Enable debug output"
    echo ""
    echo "Features:"
    echo "  • 🚀 Auto-starts complete three-tier architecture"
    echo "  • 🌐 Opens web browser to viewer interface"
    echo "  • 📦 Pack loading with visual definitions"
    echo "  • 🔌 IPC communication between simulator and viewer"
    echo "  • 📊 Real-time monitoring of all processes"
    echo ""
    echo "Architecture:"
    echo "  World Simulator (IPC) → Sim Viewer (WebSocket) → Web Browser"
    echo ""
    echo "Example:"
    echo "  $0                              # Full launch with browser"
    echo "  $0 --pack-path ./assets/packs/dev-world"
    echo "  $0 --no-browser                 # Launch without opening browser"
    echo ""
    exit 0
fi

# Parse command line options
PACK_PATH=""
VIEWER_PORT=11748
WEB_PORT=11721
NO_BROWSER=false
DEBUG=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --pack-path)
            PACK_PATH="$2"
            shift 2
            ;;
        --viewer-port)
            VIEWER_PORT="$2"
            shift 2
            ;;
        --web-port)
            WEB_PORT="$2"
            shift 2
            ;;
        --no-browser)
            NO_BROWSER=true
            shift
            ;;
        --debug)
            DEBUG=true
            shift
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

    # Create a pipe to connect simulator output to viewer input
    mkfifo /tmp/simulator_pipe 2>/dev/null || true

    # Start tailing simulator output and pipe to viewer background process
    tail -f /tmp/simulator_output.log > /tmp/simulator_pipe &
    TAIL_PID=$!
    echo "Pipe PID: $TAIL_PID"

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

    # Start viewer in background, reading from pipe
    cargo run -p world_sim_viewer -- --port $VIEWER_PORT --ipc-file /tmp/simulator_pipe $PACK_ARG > /tmp/viewer_output.log 2>&1 &
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

# Function to check if web viewer exists
check_web_viewer() {
    if [ ! -f "web-viewer/viewer.html" ]; then
        echo -e "${YELLOW}⚠️  Viewer not found at web-viewer/viewer.html${NC}"
        echo -e "${YELLOW}   You'll need to open the viewer manually and connect to: ws://localhost:$VIEWER_PORT${NC}"
        return 1
    fi
    return 0
}

# Function to start web server for the existing viewer
start_web_server() {
    echo -e "${CYAN}🌐 Starting web viewer server...${NC}"

    # Check if Python is available
    if ! command -v python3 &> /dev/null && ! command -v python &> /dev/null; then
        echo -e "${RED}❌ Python not found. Cannot start web server.${NC}"
        echo -e "${YELLOW}   Please open world_sim_simple/viewer.html manually or install Python.${NC}"
        return 1
    fi

    # Determine Python command
    PYTHON_CMD=""
    if command -v python3 &> /dev/null; then
        PYTHON_CMD="python3"
    elif command -v python &> /dev/null; then
        PYTHON_CMD="python"
    fi

    # Create a simple server script to serve the existing viewer
    cat > /tmp/serve_viewer.py << 'EOF'
#!/usr/bin/env python3
import http.server
import socketserver
import os
import sys
import webbrowser
import threading

class ViewerHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory="web-viewer", **kwargs)

    def log_message(self, format, *args):
        pass  # Reduce log noise

    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')
        super().end_headers()

def start_server(port):
    handler = ViewerHandler
    with socketserver.TCPServer(("", port), handler) as httpd:
        print(f"World Simulator Viewer server running at http://localhost:{port}/viewer.html")
        httpd.serve_forever()

if __name__ == "__main__":
    PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 11721

    print(f"Starting World Simulator Viewer server on port {PORT}")
    print(f"Open your browser to: http://localhost:{PORT}/viewer.html")

    # Start server in background
    server_thread = threading.Thread(target=start_server, args=(PORT,))
    server_thread.daemon = True
    server_thread.start()

    # Open browser after a short delay
    try:
        import time
        time.sleep(2)
        webbrowser.open(f'http://localhost:{PORT}/viewer.html')
        print(f"Opened browser to http://localhost:{PORT}/viewer.html")
    except Exception as e:
        print(f"Could not open browser automatically: {e}")
        print(f"Please manually open: http://localhost:{PORT}/viewer.html")

    # Keep the script running
    try:
        while True:
            import time
            time.sleep(1)
    except KeyboardInterrupt:
        print("\nShutting down server...")
        sys.exit(0)
EOF

    # Start web server in background
    $PYTHON_CMD /tmp/serve_viewer.py $WEB_PORT > /tmp/web_server.log 2>&1 &
    WEB_PID=$!

    echo "Web server PID: $WEB_PID on port $WEB_PORT"

    # Wait a moment for server to start
    sleep 2

    # Check if web server is running
    if ! ps -p $WEB_PID > /dev/null; then
        echo -e "${RED}❌ Failed to start web server${NC}"
        echo "Check logs: tail -f /tmp/web_server.log"
        return 1
    fi

    echo -e "${GREEN}✅ Web server started successfully${NC}"
    echo -e "${CYAN}   Viewer will be available at: http://localhost:$WEB_PORT/viewer.html${NC}"
    return 0
}

# Function to open web browser
open_browser() {
    if [ "$NO_BROWSER" = true ]; then
        return
    fi

    echo -e "${CYAN}🌍 Opening web browser...${NC}"

    # The Python server already opens the browser, but try anyway as backup
    if command -v google-chrome &> /dev/null; then
        google-chrome "http://localhost:$WEB_PORT" &> /dev/null &
    elif command -v chrome &> /dev/null; then
        chrome "http://localhost:$WEB_PORT" &> /dev/null &
    elif command -v firefox &> /dev/null; then
        firefox "http://localhost:$WEB_PORT" &> /dev/null &
    elif command -v safari &> /dev/null; then
        safari "http://localhost:$WEB_PORT" &> /dev/null &
    elif command -v open &> /dev/null; then
        open "http://localhost:$WEB_PORT" &> /dev/null &
    else
        echo -e "${YELLOW}⚠️  Could not detect web browser${NC}"
        echo -e "${YELLOW}   Please open your browser and navigate to: http://localhost:$WEB_PORT${NC}"
    fi
}

# Function to monitor all processes
monitor_processes() {
    echo ""
    echo -e "${CYAN}📊 Three-Tier Architecture Monitor${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo -e "${BLUE}🖥️  Simulator → IPC → 🌐 Viewer → WebSocket → 🌍 Browser${NC}"
    echo ""
    echo -e "${CYAN}WebSocket server: ws://localhost:$VIEWER_PORT${NC}"
    [ "$NO_BROWSER" = false ] && echo -e "${CYAN}Web interface: http://localhost:$WEB_PORT${NC}"
    echo ""

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
            if [ "$DEBUG" = true ]; then
                echo -e "${BLUE}📜 Recent Simulator Activity:${NC}"
                tail -3 /tmp/simulator_output.log | sed 's/^/  /'
            fi
        else
            echo -e "${RED}❌ Simulator - Not Running${NC}"
        fi

        echo ""

        # Check viewer status
        if [ ! -z "$VIEWER_PID" ] && ps -p $VIEWER_PID > /dev/null; then
            echo -e "${GREEN}✅ Viewer (PID: $VIEWER_PID) - Running on port $VIEWER_PORT${NC}"
            # Show last few lines of viewer output
            if [ "$DEBUG" = true ]; then
                echo -e "${PURPLE}📜 Recent Viewer Activity:${NC}"
                tail -3 /tmp/viewer_output.log | sed 's/^/  /'
            fi
        else
            echo -e "${RED}❌ Viewer - Not Running${NC}"
        fi

        # Check web server status
        if [ "$NO_BROWSER" = false ]; then
            if [ ! -z "$WEB_PID" ] && ps -p $WEB_PID > /dev/null; then
                echo -e "${GREEN}✅ Web Server (PID: $WEB_PID) - Running on port $WEB_PORT${NC}"
            else
                echo -e "${RED}❌ Web Server - Not Running${NC}"
            fi
        fi

        echo ""
        echo -e "${CYAN}🌐 Access your simulation:${NC}"
        echo "  WebSocket: ws://localhost:$VIEWER_PORT"
        [ "$NO_BROWSER" = false ] && echo "  Web Interface: http://localhost:$WEB_PORT"
        echo ""
        echo "Press Ctrl+C to stop all processes"
        echo "To kill processes manually: pkill -f 'world_sim'"
        echo ""

        sleep 2
    done
}

# Function to cleanup on exit
cleanup_on_exit() {
    echo -e "\n${YELLOW}🧹 Cleaning up processes...${NC}"
    [ ! -z "$TAIL_PID" ] && ps -p $TAIL_PID > /dev/null && kill $TAIL_PID 2>/dev/null
    [ ! -z "$SIM_PID" ] && ps -p $SIM_PID > /dev/null && kill $SIM_PID 2>/dev/null
    [ ! -z "$VIEWER_PID" ] && ps -p $VIEWER_PID > /dev/null && kill $VIEWER_PID 2>/dev/null
    [ ! -z "$WEB_PID" ] && ps -p $WEB_PID > /dev/null && kill $WEB_PID 2>/dev/null
    rm -f /tmp/simulator_pipe 2>/dev/null
    echo -e "${GREEN}✅ All processes stopped${NC}"
    exit 0
}

# Set up signal handlers
trap cleanup_on_exit SIGINT SIGTERM

# Main execution
echo "🌍 World Simulator Complete Three-Tier Launch Script"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check if web viewer exists
check_web_viewer

# Start all components
cleanup_processes
start_simulator
start_viewer

# Start web server if not disabled
if [ "$NO_BROWSER" = false ]; then
    start_web_server
    # Small delay to let web server start before opening browser
    sleep 1
    open_browser
fi

# Start monitoring
monitor_processes