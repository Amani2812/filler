# Filler AI Player - Usage Guide

## Quick Start

### 1. Setup Docker Environment

```bash
# Download and extract docker_image folder
# Navigate to the docker_image directory

# Build the Docker image
docker build -t filler .

# Run the container with solution mounted
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

### 2. Build the Player (Inside Container)

```bash
cd solution
cargo build --release
cp target/release/filler ./filler
```

Or use the build script:
```bash
cd solution
chmod +x build.sh
./build.sh
```

Or use Make:
```bash
cd solution
make
```

### 3. Run a Game

```bash
# From the container root (/filler)
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender
```

## Testing

### Manual Testing

Test against each robot on their respective maps:

```bash
# Test 1: wall_e on map00
./game_engine -f maps/map00 -p1 solution/filler -p2 robots/wall_e

# Test 2: h2_d2 on map01
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/h2_d2

# Test 3: bender on map02
./game_engine -f maps/map02 -p1 solution/filler -p2 robots/bender

# Swap player positions
./game_engine -f maps/map00 -p1 robots/wall_e -p2 solution/filler
```

### Automated Testing

Use the test script to run all required tests:

```bash
chmod +x solution/test.sh
./solution/test.sh
```

This will run 5 games against each robot, alternating player positions.

## Game Engine Flags

- `-f, -file <path>`: Path to map file
- `-p1, -player1 <path>`: Path to player 1 AI
- `-p2, -player2 <path>`: Path to player 2 AI
- `-q, -quiet`: Quiet mode (less output)
- `-r, -refresh`: Throttling mode
- `-s, -seed <int>`: Use specific random seed
- `-t, -time <int>`: Set timeout in seconds (default: 10)

## Examples

### Watch a game with visual output
```bash
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender
```

### Run in quiet mode
```bash
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender -q
```

### Use specific seed for reproducibility
```bash
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender -s 42
```

### Set custom timeout
```bash
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender -t 5
```

## Understanding the Output

### Game Display
- `.` = Empty cell
- `@` or `a` = Player 1 territory (@ = old, a = last placed)
- `$` or `s` = Player 2 territory ($ = old, s = last placed)

### Win Conditions
The player with the most territory (cells occupied) wins when:
- Both players cannot place any more pieces
- One player times out or errors
- One player makes an invalid move

## Troubleshooting

### Build Issues

**Problem**: Cargo not found
```bash
# Install Rust in the container
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Problem**: Permission denied
```bash
chmod +x solution/filler
chmod +x solution/build.sh
chmod +x solution/test.sh
```

### Runtime Issues

**Problem**: Player returns 0 0 every turn
- Check that the input parsing is working correctly
- Verify the player can identify its own territory
- Ensure piece parsing is correct

**Problem**: Player times out
- The default timeout is 10 seconds
- Optimize the search algorithm if needed
- Check for infinite loops in the code

**Problem**: Invalid placement
- Verify exactly 1-cell overlap with own territory
- Check bounds validation
- Ensure no overlap with opponent

## Strategy Tips

The AI uses these strategies:
1. **Blocking**: Prioritizes positions near opponent territory
2. **Expansion**: Favors positions with adjacent empty cells
3. **Center Control**: Prefers central positions early game
4. **Greedy Growth**: Maximizes immediate territory gain

## Performance Benchmarks

Expected win rates (4/5 minimum):
- vs wall_e on map00: 80-100%
- vs h2_d2 on map01: 80-100%
- vs bender on map02: 80-100%

## Advanced Usage

### Custom Maps

Create your own map file:
```
..............................
..............................
..$...........................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
...........................@..
..............................
..............................
```

Rules:
- Use `.` for empty cells
- Use `@` for Player 1 starting position
- Use `$` for Player 2 starting position
- Rectangular grid (all rows same length)

### Debugging

Add debug output (modify main.rs):
```rust
eprintln!("Debug: Player {}, Position: ({}, {})", game.player_num, x, y);
```

Note: Use `eprintln!` for debug output (stderr), not `println!` (stdout is for game moves only)

## Competition Mode

To compete against terminator (bonus challenge):
```bash
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/terminator
```

Note: Beating terminator is a bonus objective and not required for project completion.
