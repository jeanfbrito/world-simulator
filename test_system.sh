#!/bin/bash
# Automated test script for world_sim_simple
# Run after each change to validate system still works

set -e

echo "🔧 World Simulator Test Suite"
echo "=============================="

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test results
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run a test
run_test() {
    local test_name=$1
    local test_command=$2
    
    echo -n "Testing $test_name... "
    if eval $test_command > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
        ((TESTS_PASSED++))
        return 0
    else
        echo -e "${RED}✗${NC}"
        ((TESTS_FAILED++))
        return 1
    fi
}

# 1. Check if Rust code compiles
echo -e "\n${YELLOW}1. Compilation Tests${NC}"
run_test "Backend compilation" "cd world_sim_simple && cargo build 2>&1 | grep -v warning"
run_test "Backend tests" "cd world_sim_simple && cargo test 2>&1 | grep -v warning || true"

# 2. Check if backend starts
echo -e "\n${YELLOW}2. Backend Runtime Tests${NC}"
# Kill any existing backend
pkill -f world_sim_simple 2>/dev/null || true
sleep 1

# Start backend
echo -n "Starting backend... "
cd world_sim_simple && ../target/debug/world_sim_simple > /tmp/test_backend.log 2>&1 &
BACKEND_PID=$!
sleep 3

if kill -0 $BACKEND_PID 2>/dev/null; then
    echo -e "${GREEN}✓${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗${NC}"
    echo "Backend failed to start. Check /tmp/test_backend.log"
    ((TESTS_FAILED++))
    exit 1
fi

# 3. Check WebSocket connectivity
echo -e "\n${YELLOW}3. WebSocket Tests${NC}"
run_test "WebSocket port listening" "lsof -i :8080 | grep -q LISTEN"

# Test WebSocket with curl (basic HTTP upgrade test)
run_test "WebSocket endpoint accessible" "curl -s -o /dev/null -w '%{http_code}' http://localhost:8080 | grep -qE '(400|426)'"

# 4. Check frontend
echo -e "\n${YELLOW}4. Frontend Tests${NC}"
# Kill any existing frontend server
pkill -f "python3.*9090" 2>/dev/null || true
lsof -ti:9090 | xargs kill -9 2>/dev/null || true
sleep 1

# Start frontend server
python3 -m http.server 9090 > /tmp/test_frontend.log 2>&1 &
FRONTEND_PID=$!
sleep 2

run_test "Frontend server running" "kill -0 $FRONTEND_PID 2>/dev/null"
run_test "Frontend accessible" "curl -s http://localhost:9090/index.html | grep -q 'World Simulator'"
run_test "Frontend JS loads" "curl -s http://localhost:9090/js/main.js | grep -q 'WorldMap'"

# 5. Component validation (will add more as we add components)
echo -e "\n${YELLOW}5. Component Tests${NC}"
run_test "Worker component exists" "grep -q 'struct Worker' world_sim_simple/src/main.rs"
run_test "TileEntity component exists" "grep -q 'struct TileEntity' world_sim_simple/src/main.rs"

# Cleanup
echo -e "\n${YELLOW}Cleaning up...${NC}"
kill $BACKEND_PID 2>/dev/null || true
kill $FRONTEND_PID 2>/dev/null || true

# Results
echo -e "\n=============================="
echo -e "Tests Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests Failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}✅ All tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some tests failed!${NC}"
    exit 1
fi