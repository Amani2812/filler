# Filler Docker Run Guide (Windows + PowerShell)

This project uses the assets extracted from `filler.zip` under:

- `filler_assets/docker_image`

Your bundle uses:

- `linux_game_engine` (not `game_engine`)
- `linux_robots` (not `robots`)

So the equivalent of:

```bash
./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator
```

is:

```bash
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator
```

---

## 1) Prerequisites

- Docker Desktop installed
- Docker Desktop running (`Engine running`)

Check:

```powershell
docker info
```

---

## 2) Build the image

From your workspace root (`C:\Users\amani\filler`), run:

```powershell
cd filler_assets/docker_image
docker build -t filler .
```

---

## 3) Run the exact engine sanity command inside container

Use this from `C:\Users\amani\filler`:

```powershell
docker run --rm -it -w /filler filler bash -lc "./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator"
```

---

## 4) Mount your student solution and build it in container
C:\Users\amani\filler
From ``:

```powershell
docker run --rm -it -v "C:/Users/amani/filler/filler_assets/docker_image/solution:/filler/solution" -w /filler/solution filler bash -lc "cargo build --release"
```

This creates:

- `/filler/solution/target/release/filler_bot`

---

## 5) Run student bot vs provided robots

### Example (map00 vs wall_e)

```powershell
docker run --rm -it -v "C:/Users/amani/filler/filler_assets/docker_image/solution:/filler/solution" -w /filler filler bash -lc "./linux_game_engine -f maps/map00 -p1 solution/target/release/filler_bot -p2 linux_robots/wall_e"
```

### Swap sides (student as p2)

```powershell
docker run --rm -it -v "C:/Users/amani/filler/filler_assets/docker_image/solution:/filler/solution" -w /filler filler bash -lc "./linux_game_engine -f maps/map00 -p1 linux_robots/wall_e -p2 solution/target/release/filler_bot"
```

---

## 6) Run the 5x audit loops

### map00 vs wall_e (both positions each iteration)

```powershell
docker run --rm -it -v "C:/Users/amani/filler/filler_assets/docker_image/solution:/filler/solution" -w /filler filler bash -lc "for i in 1 2 3 4 5; do echo RUN:$i p1=bot p2=wall_e; ./linux_game_engine -f maps/map00 -p1 solution/target/release/filler_bot -p2 linux_robots/wall_e; echo RUN:$i p1=wall_e p2=bot; ./linux_game_engine -f maps/map00 -p1 linux_robots/wall_e -p2 solution/target/release/filler_bot; done"
```

### map01 vs h2_d2

```powershell
docker run --rm -it -v "C:/Users/amani/filler/filler_assets/docker_image/solution:/filler/solution" -w /filler filler bash -lc "for i in 1 2 3 4 5; do echo RUN:$i p1=bot p2=h2_d2; ./linux_game_engine -f maps/map01 -p1 solution/target/release/filler_bot -p2 linux_robots/h2_d2; echo RUN:$i p1=h2_d2 p2=bot; ./linux_game_engine -f maps/map01 -p1 linux_robots/h2_d2 -p2 solution/target/release/filler_bot; done"
```

### map02 vs bender

```powershell
docker run --rm -it -v "C:/Users/amani/filler/filler_assets/docker_image/solution:/filler/solution" -w /filler filler bash -lc "for i in 1 2 3 4 5; do echo RUN:$i p1=bot p2=bender; ./linux_game_engine -f maps/map02 -p1 solution/target/release/filler_bot -p2 linux_robots/bender; echo RUN:$i p1=bender p2=bot; ./linux_game_engine -f maps/map02 -p1 linux_robots/bender -p2 solution/target/release/filler_bot; done"
```

---

## 7) Common issue

If you try this on host PowerShell:

```powershell
./linux_game_engine ...
```

you will get command-not-found.  
Reason: this is a Linux binary and must run **inside Docker container** as shown above.
