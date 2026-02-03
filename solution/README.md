# Filler AI Player - Rust Implementation

A competitive AI player for the Filler game that uses strategic positioning and opponent blocking to maximize territory coverage.

## Features

- **Smart Piece Placement**: Finds all valid positions with exactly 1-cell overlap
- **Strategic Scoring**: Evaluates positions based on:
  - Proximity to opponent (blocking strategy)
  - Territory expansion potential
  - Center control early game
  - Adjacent empty cells for growth
- **Robust Input Parsing**: Handles game_engine stdin format correctly
- **Efficient Algorithm**: Evaluates all possible placements and selects optimal position

## Building

```bash
cd solution
cargo build --release
```

The compiled binary will be at `target/release/filler`

## Usage

### Running with game_engine

```bash
# Inside Docker container
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/bender
```

### Testing against different robots

```bash
# Against wall_e
./game_engine -f maps/map00 -p1 solution/target/release/filler -p2 robots/wall_e

# Against h2_d2
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/h2_d2

# Against bender
./game_engine -f maps/map02 -p1 solution/target/release/filler -p2 robots/bender
```

## Strategy

The AI uses a multi-factor scoring system:

1. **Opponent Blocking** (+50 points): Prioritizes positions adjacent to opponent territory
2. **Territory Expansion** (+10 points): Favors positions near empty cells
3. **Center Control** (negative distance): Prefers central positions to maximize options
4. **Valid Placement**: Ensures exactly 1-cell overlap with existing territory

## Algorithm

1. Parse game state (player number, anfield, piece)
2. Iterate through all possible positions
3. Validate each position:
   - Check bounds
   - Ensure no opponent overlap
   - Verify exactly 1-cell overlap with own territory
4. Score valid positions using strategic heuristics
5. Return highest-scoring position or (0, 0) if no valid moves

## Input Format

The player reads from stdin:
```
$$$ exec p1 : [robots/player]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
...
Piece 4 1:
.OO.
```

## Output Format

Coordinates in format: `X Y\n`

Example: `7 2`

## Performance

Designed to win 4/5 games against:
- wall_e
- h2_d2
- bender

## Docker Compatibility

The solution is designed to work within the provided Docker environment. Make sure to:
1. Build the image: `docker build -t filler .`
2. Run with mounted solution: `docker run -v "$(pwd)/solution":/filler/solution -it filler`
3. Compile inside container: `cd solution && cargo build --release`
