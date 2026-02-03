#!/bin/bash

# Test script to verify input parsing works correctly
# This creates sample input and tests the player's response

echo "================================"
echo "Filler AI - Input Parser Test"
echo "================================"
echo ""

# Create test input file
cat > /tmp/test_input.txt << 'EOF'
$$$ exec p1 : [solution/filler]
Anfield 15 10:
    0123456789012345
000 ...............
001 ...............
002 .........@.....
003 ...............
004 ...............
005 ...............
006 ...............
007 ...............
008 ...............
009 ...............
010 ...............
011 ...............
012 .........$.....
013 ...............
014 ...............
Piece 3 2:
.#.
###
EOF

echo "Test 1: Basic Input Parsing"
echo "----------------------------"
echo "Input file created at /tmp/test_input.txt"
echo ""
echo "Testing player response..."

if [ -f "solution/filler" ]; then
    PLAYER="solution/filler"
elif [ -f "solution/target/release/filler" ]; then
    PLAYER="solution/target/release/filler"
else
    echo "ERROR: Player binary not found!"
    echo "Please build first: cd solution && cargo build --release"
    exit 1
fi

# Run player with test input
OUTPUT=$(cat /tmp/test_input.txt | $PLAYER)

echo "Player output: $OUTPUT"
echo ""

# Validate output format
if echo "$OUTPUT" | grep -qE '^[0-9]+ [0-9]+$'; then
    echo "✓ Output format is correct (X Y)"
else
    echo "✗ Output format is incorrect!"
    echo "Expected: 'X Y' (two numbers separated by space)"
    exit 1
fi

# Extract coordinates
X=$(echo "$OUTPUT" | awk '{print $1}')
Y=$(echo "$OUTPUT" | awk '{print $2}')

echo "Parsed coordinates: X=$X, Y=$Y"
echo ""

# Validate coordinates are within bounds
if [ "$X" -ge 0 ] && [ "$X" -lt 15 ] && [ "$Y" -ge 0 ] && [ "$Y" -lt 10 ]; then
    echo "✓ Coordinates are within board bounds"
else
    echo "⚠ Coordinates might be out of bounds or invalid move (0 0)"
fi

echo ""
echo "================================"
echo "Test 2: Player 2 Input"
echo "================================"

cat > /tmp/test_input2.txt << 'EOF'
$$$ exec p2 : [solution/filler]
Anfield 15 10:
    0123456789012345
000 ...............
001 ...............
002 .........@.....
003 ...............
004 ...............
005 ...............
006 ...............
007 ...............
008 ...............
009 ...............
010 ...............
011 ...............
012 .........$s....
013 ...............
014 ...............
Piece 2 2:
##
##
EOF

OUTPUT2=$(cat /tmp/test_input2.txt | $PLAYER)
echo "Player 2 output: $OUTPUT2"

if echo "$OUTPUT2" | grep -qE '^[0-9]+ [0-9]+$'; then
    echo "✓ Player 2 output format is correct"
else
    echo "✗ Player 2 output format is incorrect!"
fi

echo ""
echo "================================"
echo "Test 3: Large Piece"
echo "================================"

cat > /tmp/test_input3.txt << 'EOF'
$$$ exec p1 : [solution/filler]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 ....................
004 ....................
005 ....................
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 ....................
012 .........$..........
013 ....................
014 ....................
Piece 5 4:
.##..
.##..
..#..
...#.
EOF

OUTPUT3=$(cat /tmp/test_input3.txt | $PLAYER)
echo "Large piece output: $OUTPUT3"

if echo "$OUTPUT3" | grep -qE '^[0-9]+ [0-9]+$'; then
    echo "✓ Large piece handling works"
else
    echo "✗ Large piece handling failed!"
fi

echo ""
echo "================================"
echo "Summary"
echo "================================"
echo "All basic input parsing tests completed."
echo "The player can read input and produce output."
echo ""
echo "Next steps:"
echo "1. Build the player in Docker environment"
echo "2. Run against actual game_engine"
echo "3. Test win rates against robots"

# Cleanup
rm -f /tmp/test_input.txt /tmp/test_input2.txt /tmp/test_input3.txt
