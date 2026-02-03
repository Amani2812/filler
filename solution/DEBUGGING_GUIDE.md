# Filler AI Player - Debugging Guide

## 🔧 Available Debugging Tools

### 1. Debug Player (`debug_player.rs`)
A verbose version of the player that outputs detailed information to stderr.

**Build:**
```bash
rustc debug_player.rs -o debug_player
```

**Use:**
```bash
./game_engine -f maps/map01 -p1 debug_player -p2 robots/bender 2> debug.log
```

**Output includes:**
- Player number identification
- Board dimensions
- Territory counts
- Valid move counts
- Position scores
- Best move selection reasoning

### 2. Build Validator (`validate_build.sh`)
Comprehensive build verification script.

**Run:**
```bash
chmod +x validate_build.sh
./validate_build.sh
```

**Checks:**
- Rust installation
- Source files
- Compilation success
- Binary creation
- Code quality (clippy)
- Code formatting
- Documentation
- Test scripts

### 3. Input Parser Test (`test_input_parser.sh`)
Tests the player's ability to parse game input.

**Run:**
```bash
chmod +x test_input_parser.sh
./test_input_parser.sh
```

**Tests:**
- Basic input parsing
- Player 1 and Player 2 handling
- Different piece sizes
- Output format validation

### 4. Game Simulator (`simulate_game.sh`)
Simulates multiple game turns without game_engine.

**Run:**
```bash
chmod +x simulate_game.sh
./simulate_game.sh
```

**Scenarios:**
- Initial placement
- Territory expansion
- Complex pieces
- No valid moves

## 🐛 Common Issues and Solutions

### Issue 1: Compilation Errors

**Symptoms:**
```
error: could not compile `filler`
```

**Solutions:**
1. Check Rust version: `cargo --version`
2. Update Rust: `rustup update`
3. Clean build: `cargo clean && cargo build --release`
4. Check syntax in `src/main.rs`

### Issue 2: Player Returns "0 0" Every Turn

**Symptoms:**
- Player always outputs `0 0`
- Never places pieces

**Debug Steps:**
1. Use debug player to see what's happening:
   ```bash
   rustc debug_player.rs -o debug_player
   ./game_engine -f maps/map01 -p1 debug_player -p2 robots/bender 2> debug.log
   cat debug.log
   ```

2. Check for issues:
   - Is player number detected correctly?
   - Is territory being recognized?
   - Are valid moves being found?

**Common Causes:**
- Incorrect player character detection
- Wrong overlap counting
- Bounds checking too strict

### Issue 3: Invalid Placement

**Symptoms:**
- Game_engine rejects moves
- Player stops playing

**Debug Steps:**
1. Add debug output to `can_place_piece()`:
   ```rust
   eprintln!("Checking position ({}, {})", col, row);
   eprintln!("Overlap count: {}", overlap_count);
   ```

2. Verify:
   - Exactly 1 overlap with own territory
   - No overlap with opponent
   - Within board bounds

### Issue 4: Timeout

**Symptoms:**
- Player takes too long to respond
- Game_engine timeout error

**Debug Steps:**
1. Profile the code:
   ```bash
   cargo build --release
   time echo "test" | ./target/release/filler
   ```

2. Check search space:
   - Is the board very large?
   - Are there too many valid positions?

**Solutions:**
- Optimize search range
- Add early termination
- Reduce scoring complexity

### Issue 5: Player Loses Consistently

**Symptoms:**
- Win rate < 80%
- Poor strategic decisions

**Debug Steps:**
1. Watch games with debug player:
   ```bash
   ./game_engine -f maps/map01 -p1 debug_player -p2 robots/bender 2> debug.log
   ```

2. Analyze debug.log:
   - Are scores reasonable?
   - Is game phase detected correctly?
   - Are blocking opportunities found?

**Tuning:**
- Adjust scoring weights in `score_position()`
- Change early_game threshold
- Modify blocking bonus

## 📊 Debug Output Examples

### Good Debug Output
```
[DEBUG] We are Player 1 (@)
[DEBUG] Anfield size: 20x15
[DEBUG] Turn 1: Searching for best move...
[DEBUG] Piece size: 3x2
[DEBUG] Found 45 valid positions
[DEBUG] New best: (9, 1) with score 150
[DEBUG] Best move: (9, 1) with score 150
[DEBUG] Territory - Us: 1, Opponent: 1
[DEBUG] Sent move: 9 1
```

### Bad Debug Output (No Valid Moves)
```
[DEBUG] We are Player 1 (@)
[DEBUG] Anfield size: 20x15
[DEBUG] Turn 1: Searching for best move...
[DEBUG] Piece size: 3x2
[DEBUG] Found 0 valid positions
[DEBUG] No valid moves found! Returning (0, 0)
[DEBUG] Territory - Us: 0, Opponent: 1
[DEBUG] Sent move: 0 0
```

**Problem:** No territory found (Us: 0)
**Solution:** Check player character detection

## 🔍 Manual Testing Procedure

### Step 1: Verify Compilation
```bash
cd solution
cargo build --release
ls -la target/release/filler
```

### Step 2: Test Input Parsing
```bash
./test_input_parser.sh
```

### Step 3: Simulate Games
```bash
./simulate_game.sh
```

### Step 4: Test with Debug Player
```bash
rustc debug_player.rs -o debug_player
cd ..
./game_engine -f maps/map01 -p1 solution/debug_player -p2 robots/bender 2> debug.log
```

### Step 5: Analyze Results
```bash
cat debug.log | grep "Best move"
cat debug.log | grep "Territory"
```

### Step 6: Test Win Rate
```bash
# Run 5 games
for i in {1..5}; do
    echo "Game $i:"
    ./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender -q
done
```

## 🎯 Performance Optimization

### Profiling
```bash
# Build with debug symbols
cargo build --release

# Run with profiling
perf record ./target/release/filler < test_input.txt
perf report
```

### Optimization Tips
1. **Reduce search space:**
   - Skip obviously invalid positions early
   - Use better bounds for iteration

2. **Cache calculations:**
   - Pre-calculate territory counts
   - Store frequently accessed values

3. **Optimize hot paths:**
   - `can_place_piece()` is called most often
   - `score_position()` is second most called

## 📝 Adding Debug Output

### To Main Player
```rust
// Add to src/main.rs (use stderr!)
eprintln!("Debug: {}", message);
```

### To Specific Functions
```rust
fn can_place_piece(&self, row: i32, col: i32) -> bool {
    eprintln!("[DEBUG] Checking ({}, {})", col, row);
    // ... rest of function
}
```

### Conditional Debug Output
```rust
const DEBUG: bool = false; // Set to true for debugging

if DEBUG {
    eprintln!("Debug info: {}", value);
}
```

## 🧪 Unit Testing

### Add Tests to main.rs
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_detection() {
        let mut game = GameState::new();
        game.parse_player_line("$$$ exec p1 : [test]");
        assert_eq!(game.player_num, 1);
        assert_eq!(game.player_char, '@');
    }

    #[test]
    fn test_anfield_parsing() {
        let mut game = GameState::new();
        game.parse_anfield_header("Anfield 20 15:");
        assert_eq!(game.anfield_width, 20);
        assert_eq!(game.anfield_height, 15);
    }
}
```

### Run Tests
```bash
cargo test
```

## 🎓 Learning from Games

### Watch Games
```bash
# Don't use -q flag to see the game
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender
```

### Save Game Output
```bash
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender > game.log 2>&1
```

### Analyze Patterns
- When do we win?
- When do we lose?
- What moves lead to victory?
- What mistakes are made?

## 🚀 Quick Debug Checklist

Before reporting issues, verify:
- [ ] Code compiles without errors
- [ ] Binary exists and is executable
- [ ] Input parsing works (test_input_parser.sh)
- [ ] Game simulation works (simulate_game.sh)
- [ ] Debug player shows reasonable output
- [ ] Player responds within timeout
- [ ] Output format is correct (X Y\n)

## 📞 Getting Help

If stuck, check:
1. **Debug logs** - What is the player thinking?
2. **Game output** - What moves are being made?
3. **Error messages** - What is failing?
4. **Documentation** - Is there a guide for this?

Common documentation:
- `README.md` - Overview
- `USAGE.md` - How to use
- `PROJECT_OVERVIEW.md` - How it works
- `TESTING_CHECKLIST.md` - Testing procedures
- `DEBUGGING_GUIDE.md` - This file
