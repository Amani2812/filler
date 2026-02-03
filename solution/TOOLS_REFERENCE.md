# Filler AI Player - Tools Reference

Quick reference guide for all debugging and testing tools.

## 📚 Tool Index

| Tool | Purpose | Command |
|------|---------|---------|
| validate_build.sh | Verify build is correct | `./validate_build.sh` |
| test_input_parser.sh | Test input parsing | `./test_input_parser.sh` |
| simulate_game.sh | Simulate game scenarios | `./simulate_game.sh` |
| debug_player.rs | Verbose debug output | `rustc debug_player.rs -o debug_player` |
| test.sh | Automated win rate testing | `./test.sh` |
| build.sh | Build the player | `./build.sh` |

## 🔨 Build Tools

### build.sh
**Purpose:** Compile the player with one command

**Usage:**
```bash
chmod +x build.sh
./build.sh
```

**Output:**
- Compiles in release mode
- Copies binary to `./filler`
- Makes it executable

**When to use:** First time setup, after code changes

---

### validate_build.sh
**Purpose:** Comprehensive build verification

**Usage:**
```bash
chmod +x validate_build.sh
./validate_build.sh
```

**Checks:**
- ✓ Rust installation
- ✓ Source files exist
- ✓ Compilation succeeds
- ✓ Binary created
- ✓ Code quality (clippy)
- ✓ Formatting (rustfmt)
- ✓ Documentation files
- ✓ Test scripts

**When to use:** Before submitting, after major changes

---

### Makefile
**Purpose:** Traditional build system

**Usage:**
```bash
make          # Build
make clean    # Clean build artifacts
make re       # Rebuild from scratch
```

**When to use:** If you prefer make over shell scripts

---

## 🧪 Testing Tools

### test_input_parser.sh
**Purpose:** Verify input parsing works correctly

**Usage:**
```bash
chmod +x test_input_parser.sh
./test_input_parser.sh
```

**Tests:**
1. Basic input parsing (Player 1)
2. Player 2 input handling
3. Large piece handling
4. Output format validation

**Output:**
- ✓ Pass/fail for each test
- Parsed coordinates
- Format validation

**When to use:** After changing input parsing code

---

### simulate_game.sh
**Purpose:** Test player without game_engine

**Usage:**
```bash
chmod +x simulate_game.sh
./simulate_game.sh
```

**Scenarios:**
1. Turn 1 - Initial placement
2. Turn 2 - Territory expansion
3. Turn 3 - Player 2 perspective
4. Turn 4 - Complex piece
5. Turn 5 - No valid moves

**When to use:** Quick functionality check, before Docker testing

---

### test.sh
**Purpose:** Automated win rate testing

**Usage:**
```bash
chmod +x test.sh
./test.sh
```

**Tests:**
- 5 games vs wall_e on map00
- 5 games vs h2_d2 on map01
- 5 games vs bender on map02
- Alternates player positions

**Requirements:**
- Docker environment
- game_engine available
- Robots available

**When to use:** Final validation, checking win rates

---

## 🐛 Debug Tools

### debug_player.rs
**Purpose:** Verbose version with detailed logging

**Build:**
```bash
rustc debug_player.rs -o debug_player
```

**Usage:**
```bash
# Run with game_engine
./game_engine -f maps/map01 -p1 debug_player -p2 robots/bender 2> debug.log

# View debug output
cat debug.log
```

**Debug Output:**
```
[DEBUG] We are Player 1 (@)
[DEBUG] Anfield size: 20x15
[DEBUG] Turn 1: Searching for best move...
[DEBUG] Piece size: 3x2
[DEBUG] Found 45 valid positions
[DEBUG] New best: (9, 1) with score 150
[DEBUG] Best move: (9, 1) with score 150
[DEBUG] Territory - Us: 5, Opponent: 3
[DEBUG] Sent move: 9 1
```

**When to use:** 
- Understanding player decisions
- Debugging strategy issues
- Analyzing losses

---

## 📊 Quick Commands

### Build and Test Workflow
```bash
# 1. Validate build
./validate_build.sh

# 2. Test input parsing
./test_input_parser.sh

# 3. Simulate games
./simulate_game.sh

# 4. Build debug version
rustc debug_player.rs -o debug_player

# 5. Test with game_engine (in Docker)
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender

# 6. Run full test suite
./test.sh
```

### Debug Workflow
```bash
# 1. Build debug player
rustc debug_player.rs -o debug_player

# 2. Run with logging
./game_engine -f maps/map01 -p1 debug_player -p2 robots/bender 2> debug.log

# 3. Analyze logs
cat debug.log | grep "Best move"
cat debug.log | grep "Territory"
cat debug.log | grep "Found.*valid"

# 4. Watch specific turn
cat debug.log | grep "Turn 5" -A 10
```

### Performance Testing
```bash
# Time a single move
time echo "test input" | ./filler

# Profile with perf (Linux)
perf record ./filler < test_input.txt
perf report

# Memory usage
valgrind --leak-check=full ./filler < test_input.txt
```

## 🎯 Tool Selection Guide

### "I want to..."

**...verify my build is correct**
→ Use `validate_build.sh`

**...test input parsing**
→ Use `test_input_parser.sh`

**...quickly test functionality**
→ Use `simulate_game.sh`

**...understand why player loses**
→ Use `debug_player.rs` with game_engine

**...check win rates**
→ Use `test.sh` (requires Docker)

**...debug a specific issue**
→ Use `debug_player.rs` and analyze logs

**...optimize performance**
→ Use profiling tools (perf, valgrind)

**...just build the player**
→ Use `build.sh` or `make`

## 🔧 Customization

### Modify Debug Output
Edit `debug_player.rs`:
```rust
// Add more debug info
eprintln!("[DEBUG] Custom info: {}", value);

// Conditional debugging
if some_condition {
    eprintln!("[DEBUG] Special case detected");
}
```

### Adjust Test Scenarios
Edit `simulate_game.sh`:
```bash
# Add new test case
cat << 'EOF' | $PLAYER
$$$ exec p1 : [solution/filler]
Anfield 10 10:
# ... your test scenario
EOF
```

### Modify Validation Checks
Edit `validate_build.sh`:
```bash
# Add custom check
echo "Check X: Custom Validation"
if [ condition ]; then
    echo -e "${GREEN}✓${NC} Check passed"
else
    echo -e "${RED}✗${NC} Check failed"
fi
```

## 📝 Output Interpretation

### validate_build.sh Output
```
✓ = Check passed (green)
✗ = Check failed (red)
⚠ = Warning (yellow)
```

### test_input_parser.sh Output
```
✓ Output format is correct
⚠ Coordinates might be out of bounds
✗ Output format is incorrect
```

### debug_player.rs Output
```
[DEBUG] = Information message
Turn X = Current turn number
Found N valid positions = Number of valid moves
Best move: (X, Y) with score N = Selected move
```

## 🚀 Best Practices

1. **Always validate build first**
   ```bash
   ./validate_build.sh
   ```

2. **Test input parsing before game_engine**
   ```bash
   ./test_input_parser.sh
   ```

3. **Use debug player to understand behavior**
   ```bash
   rustc debug_player.rs -o debug_player
   ```

4. **Save debug logs for analysis**
   ```bash
   ./game_engine ... 2> debug.log
   ```

5. **Run full test suite before submission**
   ```bash
   ./test.sh
   ```

## 📞 Troubleshooting

### Tool won't run
```bash
# Make executable
chmod +x tool_name.sh

# Check if file exists
ls -la tool_name.sh
```

### Build fails
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Check Rust version
cargo --version
rustup update
```

### Tests fail
```bash
# Check binary exists
ls -la target/release/filler

# Verify it's executable
chmod +x target/release/filler

# Test manually
echo "test" | ./target/release/filler
```

## 📚 Related Documentation

- **README.md** - Project overview
- **USAGE.md** - Detailed usage instructions
- **QUICK_START.md** - Quick reference
- **DEBUGGING_GUIDE.md** - Debugging procedures
- **TESTING_CHECKLIST.md** - Testing procedures
- **PROJECT_OVERVIEW.md** - Technical details

---

**Last Updated:** 2024
**Version:** 1.0
