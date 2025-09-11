#!/bin/bash

echo "🧪 Testing Harvest System..."
echo "=========================="

# Kill any existing processes
pkill -f world_sim_simple 2>/dev/null

# Run simulation for 30 seconds and capture output
echo "Running simulation for 30 seconds..."
timeout 30 cargo run --manifest-path world_sim_simple/Cargo.toml 2>&1 | tee /tmp/harvest_test.log &

# Wait a bit for it to start
sleep 5

# Monitor for harvesting messages
echo ""
echo "Monitoring for harvest activity..."
tail -f /tmp/harvest_test.log | grep -E "berry|Berry|harvest|Harvest|Work|WORK|gathered|inventory|🍓|🫐|🔨" &
TAIL_PID=$!

# Wait for timeout
wait

# Kill tail
kill $TAIL_PID 2>/dev/null

echo ""
echo "Test complete. Check /tmp/harvest_test.log for full output."
echo ""
echo "Summary of harvest-related messages:"
grep -E "berry|Berry|harvest|Harvest|Work|gathered|inventory" /tmp/harvest_test.log | tail -20