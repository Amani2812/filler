# Filler Audit Checklist (Step-by-Step)

This file tells you **exactly what to run and what to check** to answer each audit question.

---

## 0) Prerequisites

- Docker Desktop is running.
- You are in host PowerShell (not inside a container prompt).
- Project root expected: `C:\Users\amani\filler`
- Docker image exists: `filler` (or `filler:latest`)
- Student solution mounted from:
  - Host: `C:/Users/amani/filler/filler_assets/docker_image/solution`
  - Container: `/filler/solution`

---

## 1) Start a container shell correctly

Run from **host PowerShell**:

```powershell
docker run --rm -it -v "C:/Users/amani/filler/filler_assets/docker_image/solution:/filler/solution" -w /filler filler bash
```

You should see prompt similar to:

```bash
root@<container_id>:/filler#
```

---

## 2) Audit Q: image + container created correctly?

### Command (inside container):
```bash
pwd
ls -la
ls -la /filler
ls -la /filler/solution
```

### Pass criteria:
- `pwd` is `/filler`
- `/filler/linux_game_engine` exists
- `/filler/linux_robots` exists
- `/filler/maps` exists
- `/filler/solution` exists and contains `Cargo.toml`, `src/`

---

## 3) Why `linux_game_engine` and not `game_engine`?

The audit text uses generic names (`game_engine`, `robots/...`), but your extracted archive contains Linux-labeled binaries:

- `linux_game_engine`
- `linux_robots/...`

So this is only a **filename/packaging difference**, not a behavior difference.  
Inside your container, use the binaries that actually exist in `/filler`.

---

## 4) Audit Q: run engine sanity command

Requested question uses:
`./game_engine ... robots/...`

Equivalent in your bundle:
`./linux_game_engine ... linux_robots/...`

### Command:
```bash
chmod +x ./linux_game_engine ./linux_robots/*
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator
```

### Pass criteria:
- Command runs without “file not valid” path error.
- Engine completes a match and prints end/winner information.

---

## 5) Audit Q: project runs correctly (student bot build + run)

### Build student bot:
```bash
cd /filler/solution
cargo build --release
cd /filler
```

### Quick run vs one robot:
```bash
./linux_game_engine -f maps/map00 -p1 solution/target/release/filler_bot -p2 linux_robots/wall_e
./linux_game_engine -f maps/map00 -p1 linux_robots/wall_e -p2 solution/target/release/filler_bot
```

### Pass criteria:
- `cargo build --release` succeeds.
- Both game commands execute successfully.

---

## 6) Audit Q: overlapping of just one cell

## How to verify
- During game output, ensure no invalid placement errors.
- If engine supports verbose/validation output, check for illegal overlap messages.
- Your implementation logic already enforces:
  - exactly one overlap with own cells,
  - zero overlap with opponent,
  - no out-of-bounds placement.

### Evidence to keep
- Terminal output of at least 2 completed games with no placement-rule errors.

---

## 7) Audit Q: 5 runs with swaps vs wall_e (need >= 4/5)

Run this manually (inside container):

```bash
for i in 1 2 3 4 5; do
  echo "WALL_E RUN $i - BOT as P1"
  ./linux_game_engine -f maps/map00 -p1 solution/target/release/filler_bot -p2 linux_robots/wall_e

  echo "WALL_E RUN $i - BOT as P2"
  ./linux_game_engine -f maps/map00 -p1 linux_robots/wall_e -p2 solution/target/release/filler_bot
done
```

### Record table
Create a tally:
- BOT as P1 wins: __ /5
- BOT as P2 wins: __ /5
- Total wins: __ /10

For strict “at least 4 out of 5” interpretation per side, check each side separately.
For overall interpretation, at least 8/10.

---

## 8) Audit Q: 5 runs with swaps vs h2_d2 (need >= 4/5)

```bash
for i in 1 2 3 4 5; do
  echo "H2_D2 RUN $i - BOT as P1"
  ./linux_game_engine -f maps/map01 -p1 solution/target/release/filler_bot -p2 linux_robots/h2_d2

  echo "H2_D2 RUN $i - BOT as P2"
  ./linux_game_engine -f maps/map01 -p1 linux_robots/h2_d2 -p2 solution/target/release/filler_bot
done
```

Record wins the same way.

---

## 9) Audit Q: 5 runs with swaps vs bender (need >= 4/5)

```bash
for i in 1 2 3 4 5; do
  echo "BENDER RUN $i - BOT as P1"
  ./linux_game_engine -f maps/map02 -p1 solution/target/release/filler_bot -p2 linux_robots/bender

  echo "BENDER RUN $i - BOT as P2"
  ./linux_game_engine -f maps/map02 -p1 linux_robots/bender -p2 solution/target/release/filler_bot
done
```

Record wins the same way.

---

## 10) Unit test audit questions

From host PowerShell in project root (`C:\Users\amani\filler`):

```powershell
cargo test
```

### Pass criteria:
- All tests pass.
- Existing test coverage includes:
  - Input parsing
  - Placement validation
  - Opponent overlap rejection
  - Boundary detection
  - Output format

Test file:
- `tests/filler_tests.rs`

---

## 11) Basic code-quality audit questions

Check:
- Code structure: modular files in `src/` (`model`, `parser`, `logic`, `main`).
- Test file exists and runs.
- Good practices: clear separation of concerns and rule checks in logic.

---

## 12) Bonus audit questions

### Visualizer
- If not implemented, answer: **No visualizer was created.**

### Terminator 5x with swaps
```bash
for i in 1 2 3 4 5; do
  echo "TERMINATOR RUN $i - BOT as P1"
  ./linux_game_engine -f maps/map01 -p1 solution/target/release/filler_bot -p2 linux_robots/terminator

  echo "TERMINATOR RUN $i - BOT as P2"
  ./linux_game_engine -f maps/map01 -p1 linux_robots/terminator -p2 solution/target/release/filler_bot
done
```

Record wins and compare with required threshold.

---

## 13) Template to answer each audit question

Use this final report template after running all commands:

- Image/container created correctly: YES/NO
- Engine sanity command runs: YES/NO
- Student project runs correctly: YES/NO
- Overlap-one-cell rule respected: YES/NO
- map00 vs wall_e >= 4/5: YES/NO (wins: __/5 as P1, __/5 as P2)
- map01 vs h2_d2 >= 4/5: YES/NO (wins: __/5 as P1, __/5 as P2)
- map02 vs bender >= 4/5: YES/NO (wins: __/5 as P1, __/5 as P2)
- Unit tests pass: YES/NO
- Parsing tests exist: YES/NO
- Placement validation tests exist: YES/NO
- Boundary tests exist: YES/NO
- Good practices followed: YES/NO
- Test file exists: YES/NO
- Tests cover each possible case: PARTIAL/YES/NO
- Visualizer created: YES/NO
- Bonus terminator >= 4/5: YES/NO (wins: __/5 as P1, __/5 as P2)

---

## 14) Common mistakes and fixes

- If you see `bash: docker: command not found`:
  - You are inside container; type `exit` and run docker from host PowerShell.
- If you see `Error: file not valid`:
  - Verify binary exists:
    ```bash
    ls -la /filler/solution/target/release/filler_bot
    ```
  - Rebuild:
    ```bash
    cd /filler/solution && cargo build --release
    ```
- If `/filler/solution` missing:
  - Container was started without `-v ...:/filler/solution` mount.
