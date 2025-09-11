#!/bin/bash

# Run simulation and capture berry-related events
echo "Testing berry foraging behavior..."
echo "================================="

# Run for 20 seconds and filter output
(RUST_LOG=info cargo run -p world_sim_simple 2>&1 &
PID=$!
sleep 20
kill $PID 2>/dev/null) | grep -E "Berry|berry|gather|Gather|harvest|Harvest|food|depleted|regenerated|respawn" | head -100