#!/bin/bash

# Test script for Filler AI Player
# Tests the player against all required robots

PLAYER="./solution/filler"
GAME_ENGINE="./game_engine"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "================================"
echo "Filler AI Player Test Suite"
echo "================================"
echo ""

# Check if player binary exists
if [ ! -f "$PLAYER" ]; then
    echo -e "${RED}Error: Player binary not found at $PLAYER${NC}"
    echo "Please build the player first: cd solution && cargo build --release && cp target/release/filler ./filler"
    exit 1
fi

# Check if game_engine exists
if [ ! -f "$GAME_ENGINE" ]; then
    echo -e "${RED}Error: game_engine not found${NC}"
    echo "Make sure you're running this from the Docker container"
    exit 1
fi

# Function to run tests
run_test() {
    local map=$1
    local robot=$2
    local wins=0
    
    echo -e "${YELLOW}Testing against $robot on $map${NC}"
    echo "Running 5 games..."
    
    for i in {1..5}; do
        echo -n "  Game $i: "
        
        # Alternate player positions
        if [ $((i % 2)) -eq 1 ]; then
            # Player as p1
            result=$($GAME_ENGINE -f "$map" -p1 "$PLAYER" -p2 "$robot" -q 2>&1)
        else
            # Player as p2
            result=$($GAME_ENGINE -f "$map" -p1 "$robot" -p2 "$PLAYER" -q 2>&1)
        fi
        
        # Check if our player won (simple check - may need adjustment based on actual output)
        if echo "$result" | grep -q "filler.*won\|p1.*won.*filler\|p2.*won.*filler"; then
            echo -e "${GREEN}WIN${NC}"
            ((wins++))
        else
            echo -e "${RED}LOSS${NC}"
        fi
    done
    
    echo ""
    echo -e "Results: ${GREEN}$wins${NC}/5 wins"
    
    if [ $wins -ge 4 ]; then
        echo -e "${GREEN}✓ PASSED${NC} (4+ wins required)"
        return 0
    else
        echo -e "${RED}✗ FAILED${NC} (4+ wins required)"
        return 1
    fi
    
    echo ""
}

# Run all tests
total_passed=0
total_tests=3

echo "Test 1: Against wall_e on map00"
echo "--------------------------------"
if run_test "maps/map00" "robots/wall_e"; then
    ((total_passed++))
fi

echo ""
echo "Test 2: Against h2_d2 on map01"
echo "--------------------------------"
if run_test "maps/map01" "robots/h2_d2"; then
    ((total_passed++))
fi

echo ""
echo "Test 3: Against bender on map02"
echo "--------------------------------"
if run_test "maps/map02" "robots/bender"; then
    ((total_passed++))
fi

echo ""
echo "================================"
echo "Final Results"
echo "================================"
echo -e "Tests Passed: ${GREEN}$total_passed${NC}/$total_tests"

if [ $total_passed -eq $total_tests ]; then
    echo -e "${GREEN}All tests passed! ✓${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed ✗${NC}"
    exit 1
fi
