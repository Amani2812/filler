# Filler AI Player - Testing Checklist

## 📋 Pre-Testing Setup

### Environment Setup
- [ ] Docker installed and running
- [ ] docker_image.zip downloaded and extracted
- [ ] Navigate to docker_image directory
- [ ] Build Docker image: `docker build -t filler .`
- [ ] Run container: `docker run -v "$(pwd)/solution":/filler/solution -it filler`

### Build Verification
- [ ] Navigate to solution directory: `cd solution`
- [ ] Run build: `cargo build --release`
- [ ] Verify binary exists: `ls -la target/release/filler`
- [ ] Copy binary: `cp target/release/filler ./filler`
- [ ] Make executable: `chmod +x filler`
- [ ] Verify copy: `ls -la filler`

## 🔧 Compilation Tests

### Build Methods
- [ ] **Cargo build**: `cargo build --release` - Success?
- [ ] **Make**: `make` - Success?
- [ ] **Build script**: `./build.sh` - Success?
- [ ] **Clean build**: `cargo clean && cargo build --release` - Success?

### Build Verification
- [ ] No compilation errors
- [ ] No warnings (or acceptable warnings only)
- [ ] Binary size reasonable (~3-4MB)
- [ ] Binary is executable

## 🎮 Basic Functionality Tests

### Input/Output Tests
- [ ] Player responds to stdin
- [ ] Output format is "X Y\n"
- [ ] Handles player 1 correctly
- [ ] Handles player 2 correctly
- [ ] Returns "0 0" when no valid moves

### Game Mechanics
- [ ] Correctly identifies own territory
- [ ] Correctly identifies opponent territory
- [ ] Validates piece placement
- [ ] Ensures exactly 1-cell overlap
- [ ] Prevents opponent overlap
- [ ] Respects board boundaries

## 🏆 Required Win Rate Tests

### Test 1: wall_e on map00
```bash
cd /filler
./game_engine -f maps/map00 -p1 solution/filler -p2 robots/wall_e
```

Run 5 times, alternating positions:
- [ ] Game 1 (p1): Win/Loss
- [ ] Game 2 (p2): Win/Loss
- [ ] Game 3 (p1): Win/Loss
- [ ] Game 4 (p2): Win/Loss
- [ ] Game 5 (p1): Win/Loss

**Result**: ___/5 wins (Need 4+) ✅/❌

### Test 2: h2_d2 on map01
```bash
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/h2_d2
```

Run 5 times, alternating positions:
- [ ] Game 1 (p1): Win/Loss
- [ ] Game 2 (p2): Win/Loss
- [ ] Game 3 (p1): Win/Loss
- [ ] Game 4 (p2): Win/Loss
- [ ] Game 5 (p1): Win/Loss

**Result**: ___/5 wins (Need 4+) ✅/❌

### Test 3: bender on map02
```bash
./game_engine -f maps/map02 -p1 solution/filler -p2 robots/bender
```

Run 5 times, alternating positions:
- [ ] Game 1 (p1): Win/Loss
- [ ] Game 2 (p2): Win/Loss
- [ ] Game 3 (p1): Win/Loss
- [ ] Game 4 (p2): Win/Loss
- [ ] Game 5 (p1): Win/Loss

**Result**: ___/5 wins (Need 4+) ✅/❌

## 🔍 Detailed Validation Tests

### Piece Placement Validation
- [ ] Places pieces with exactly 1 overlap
- [ ] Never overlaps opponent
- [ ] Stays within board bounds
- [ ] Handles all piece shapes correctly
- [ ] Handles edge pieces correctly
- [ ] Handles corner pieces correctly

### Strategy Validation
- [ ] Controls center in early game
- [ ] Blocks opponent in late game
- [ ] Expands territory efficiently
- [ ] Adapts to game phase
- [ ] Makes valid moves consistently

### Edge Cases
- [ ] Handles very small pieces (1x1)
- [ ] Handles large pieces (5x5+)
- [ ] Handles irregular piece shapes
- [ ] Handles tight spaces
- [ ] Handles endgame scenarios
- [ ] Handles no valid moves gracefully

## ⚡ Performance Tests

### Response Time
- [ ] Responds within 1 second per move
- [ ] No timeouts (default 10s)
- [ ] Consistent performance throughout game
- [ ] Handles large boards efficiently

### Resource Usage
- [ ] Memory usage reasonable (< 50MB)
- [ ] No memory leaks
- [ ] CPU usage acceptable
- [ ] No crashes or panics

## 🎯 Automated Testing

### Test Script Execution
```bash
cd /filler
chmod +x solution/test.sh
./solution/test.sh
```

- [ ] Script runs without errors
- [ ] All tests execute
- [ ] Results are clear
- [ ] Pass/fail status correct

## 📊 Results Summary

### Overall Results
- **wall_e**: ___/5 wins (80%+ required) ✅/❌
- **h2_d2**: ___/5 wins (80%+ required) ✅/❌
- **bender**: ___/5 wins (80%+ required) ✅/❌

### Total Score
- **Total Wins**: ___/15 (12+ required)
- **Win Rate**: ___%  (80%+ required)
- **Status**: PASS ✅ / FAIL ❌

## 🐛 Troubleshooting Checklist

### If Build Fails
- [ ] Check Rust installation: `cargo --version`
- [ ] Update Rust: `rustup update`
- [ ] Clean and rebuild: `cargo clean && cargo build --release`
- [ ] Check Cargo.toml syntax
- [ ] Verify src/main.rs exists

### If Player Doesn't Respond
- [ ] Check binary exists and is executable
- [ ] Test with simple input
- [ ] Check for infinite loops
- [ ] Verify stdin/stdout handling
- [ ] Check for panics in code

### If Player Loses Consistently
- [ ] Review scoring algorithm
- [ ] Check game phase detection
- [ ] Verify move validation
- [ ] Test against easier opponents first
- [ ] Add debug output to understand decisions

### If Invalid Moves
- [ ] Check overlap counting logic
- [ ] Verify bounds checking
- [ ] Test opponent detection
- [ ] Review piece parsing
- [ ] Check coordinate output format

## 📝 Code Quality Checklist

### Code Review
- [ ] No compiler warnings
- [ ] No clippy warnings: `cargo clippy`
- [ ] Formatted correctly: `cargo fmt`
- [ ] Comments are clear
- [ ] Functions are well-named
- [ ] Logic is easy to follow

### Best Practices
- [ ] No unsafe code
- [ ] Proper error handling
- [ ] No unwrap() on user input
- [ ] Efficient algorithms
- [ ] Clean code structure

## 🎓 Documentation Checklist

### Documentation Files
- [ ] README.md exists and is complete
- [ ] USAGE.md provides clear instructions
- [ ] PROJECT_OVERVIEW.md explains implementation
- [ ] QUICK_START.md is helpful
- [ ] Code comments are adequate

### Documentation Quality
- [ ] Installation steps are clear
- [ ] Build instructions work
- [ ] Usage examples are correct
- [ ] Troubleshooting helps
- [ ] All features documented

## 🏅 Bonus Tests (Optional)

### Terminator Challenge
```bash
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/terminator
```
- [ ] Can compete against terminator
- [ ] Wins occasionally
- [ ] No crashes or errors

### Additional Maps
- [ ] Test on all available maps
- [ ] Consistent performance across maps
- [ ] Adapts to different board sizes

### Stress Tests
- [ ] 100 consecutive games without crash
- [ ] Different random seeds
- [ ] Various piece combinations

## ✅ Final Validation

### Pre-Submission Checklist
- [ ] All required tests passed (12+/15 wins)
- [ ] Code compiles without errors
- [ ] Documentation is complete
- [ ] No known bugs
- [ ] Code follows best practices
- [ ] Ready for evaluation

### Submission Ready
- [ ] All files in solution/ directory
- [ ] Binary can be built from source
- [ ] Works in Docker environment
- [ ] Meets all requirements
- [ ] Documentation is professional

## 📈 Performance Tracking

### Game Statistics
| Opponent | Map | P1 Wins | P2 Wins | Total | Win Rate |
|----------|-----|---------|---------|-------|----------|
| wall_e   | map00 | __/3 | __/2 | __/5 | ___% |
| h2_d2    | map01 | __/3 | __/2 | __/5 | ___% |
| bender   | map02 | __/3 | __/2 | __/5 | ___% |
| **TOTAL** | | | | **__/15** | **___%** |

### Notes
- Record any issues encountered
- Note any patterns in wins/losses
- Document any improvements made
- Track performance over time

---

## 🎯 Success Criteria

**PASS Requirements:**
- ✅ Compiles successfully
- ✅ Runs without errors
- ✅ Wins 4/5 vs wall_e
- ✅ Wins 4/5 vs h2_d2
- ✅ Wins 4/5 vs bender
- ✅ Code quality is good
- ✅ Documentation is complete

**Status**: ⬜ NOT TESTED | ✅ PASSED | ❌ FAILED

**Date Tested**: ___________

**Tester**: ___________

**Final Result**: ___________
