#!/bin/bash

echo "Starting World Simulator with Terminal Debug Mode"
echo "=================================================="
echo ""
echo "Debug Controls:"
echo "  In-game keys:"
echo "    F1 - Toggle stats display"
echo "    F2 - Toggle grid display" 
echo "    F3 - Toggle agents display"
echo "    F5 - Clear debug buffer"
echo ""
echo "  CLI commands (type in terminal):"
echo "    verbosity <level> - Set log level (error|warn|info|debug|trace)"
echo "    grid/g - Toggle grid display"
echo "    agents/a - Toggle agents display"
echo "    stats/s - Toggle stats display"
echo "    clear/c - Clear debug buffer"
echo "    help/h - Show help"
echo ""
echo "Starting with INFO log level..."
echo "=================================================="
echo ""

# Set log level and run
RUST_LOG=info cargo run --manifest-path world_sim_simple/Cargo.toml