# Filler AI - Quick Start Guide

## 🚀 Get Started in 3 Steps

### Step 1: Setup Docker
```bash
# In the docker_image directory
docker build -t filler .
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

### Step 2: Build the Player
```bash
# Inside the container
cd solution
cargo build --release
cp target/release/filler ./filler
```

### Step 3: Run a Game
```bash
# Back to /filler directory
cd ..
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender
```

## 📋 Quick Commands

### Build Commands
```bash
# Option 1: Cargo (recommended)
cargo build --release

# Option 2: Make
make

# Option 3: Build script
./build.sh
```

### Test Commands
```bash
# Run all tests
./solution/test.sh

# Test individual robots
./game_engine -f maps/map00 -p1 solution/filler -p2 robots/wall_e
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/h2_d2
./game_engine -f maps/map02 -p1 solution/filler -p2 robots/bender

# Swap positions
./game_engine -f maps/map00 -p1 robots/wall_e -p2 solution/filler
```

### Useful Flags
```bash
# Quiet mode (less output)
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender -q

# Custom timeout (5 seconds)
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender -t 5

# Specific seed (reproducible)
./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender -s 42
```

## 🎯 Expected Results

### Win Rates (Minimum 4/5)
- ✅ vs wall_e on map00: 80-100%
- ✅ vs h2_d2 on map01: 80-100%
- ✅ vs bender on map02: 80-100%

## 🐛 Troubleshooting

### Build Issues
```bash
# Cargo not found?
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Permission denied?
chmod +x solution/filler
chmod +x solution/build.sh
chmod +x solution/test.sh
```

### Runtime Issues
```bash
# Player not responding?
# Check that binary exists
ls -la solution/filler

# Verify it's executable
chmod +x solution/filler

# Test manually
echo "test" | solution/filler
```

## 📚 Documentation

- **README.md**: Project overview and features
- **USAGE.md**: Detailed usage instructions
- **PROJECT_OVERVIEW.md**: Implementation details
- **QUICK_START.md**: This file

## 🎮 Game Symbols

- `.` = Empty cell
- `@` / `a` = Player 1 (@ = old, a = last)
- `$` / `s` = Player 2 ($ = old, s = last)

## 🏆 Success Checklist

- [ ] Docker image built
- [ ] Container running
- [ ] Player compiled
- [ ] Beats wall_e (4/5)
- [ ] Beats h2_d2 (4/5)
- [ ] Beats bender (4/5)

## 💡 Pro Tips

1. **Test Early**: Run games frequently during development
2. **Watch Games**: Don't use `-q` flag to see strategy in action
3. **Vary Seeds**: Test with different `-s` values
4. **Swap Positions**: Always test as both p1 and p2
5. **Check Logs**: Use `eprintln!` for debug output (goes to stderr)

## 🔗 Quick Links

### Inside Container
```bash
# Game engine location
/filler/game_engine

# Maps directory
/filler/maps/

# Robots directory
/filler/robots/

# Your solution
/filler/solution/
```

### File Locations
```bash
# Source code
solution/src/main.rs

# Binary (after build)
solution/target/release/filler
solution/filler  # Copied for convenience
```

## ⚡ One-Liner Tests

```bash
# Test all three robots quickly
for robot in wall_e h2_d2 bender; do 
  echo "Testing $robot..."; 
  ./game_engine -f maps/map01 -p1 solution/filler -p2 robots/$robot -q; 
done
```

## 🎓 Learning Resources

1. Study the game output to understand piece placement
2. Watch how opponent robots play
3. Experiment with different strategies
4. Analyze winning vs losing games
5. Read the scoring algorithm in main.rs

---

**Need Help?** Check the detailed guides:
- Build issues → USAGE.md
- Strategy questions → PROJECT_OVERVIEW.md
- Feature details → README.md
