# How to Run

Docker is the recommended way to run this on any platform — it's what the repo is built around and needs no local Rust install. Native/local runs are also possible, but only on Linux or Apple Silicon Mac, since the provided `game_engine`/`robots` binaries are platform-specific compiled executables (not something a flag or wrapper can get around).

## Recommended: Run with Docker (works on Linux, Mac, and Windows)

First-time setup:

```bash
cd docker_image
chmod +x linux_game_engine linux_robots/*
```

Build the image once, or whenever `Dockerfile` or the engine/robots/maps change:

```bash
docker build -t filler .
```

Run the container, mounting `solution/` so it's live-editable and built inside the container:

```bash
# macOS / Linux (bash)
docker run -v "$(pwd)/solution":/filler/solution -it filler

# Windows PowerShell
docker run -v "${PWD}/solution":/filler/solution -it filler

# Windows cmd.exe
docker run -v "%cd%/solution":/filler/solution -it filler
```

Inside the container shell:

```bash
cd solution
cargo build --release
cd ..
./linux_game_engine -f maps/map01 -p1 solution/target/release/filler_bot -p2 linux_robots/bender
```

## Alternative: Run natively without Docker (Linux, or Apple Silicon Mac only)

If you're on Linux with Rust installed, you can skip Docker entirely for faster iteration:

```bash
cd docker_image
chmod +x linux_game_engine linux_robots/*
cd solution
cargo build --release
cd ..
./linux_game_engine -f maps/map01 -p1 solution/target/release/filler_bot -p2 linux_robots/bender
```

On Apple Silicon (M1/M2/etc.) Mac, use `m1_game_engine` and `m1_robots/*` in place of the `linux_*` versions, same commands otherwise.

After the first build, just re-run `cargo build --release` after code changes, then re-run the engine command.

## Windows without Docker: not supported directly

`linux_game_engine` and `linux_robots/*` are Linux binaries and cannot run natively on Windows — there's no command-line flag or wrapper that changes this, it's a fundamental OS/binary-format limitation. If Docker isn't an option, the only alternative is installing **WSL2** (`wsl --install` from an admin PowerShell, then install Ubuntu from the Microsoft Store), installing Rust inside it via `rustup`, and following the native Linux steps above from within the WSL environment — at that point it behaves exactly like a real Linux machine.

## Available maps and robots

- Maps: `maps/map00`, `maps/map01`, `maps/map02`
- Robots: `bender`, `h2_d2`, `terminator`, `wall_e` (in `linux_robots/` or `m1_robots/`)
- `terminator` is a very strong robot — beating it is optional.

Swap `-p2 linux_robots/bender` for any other robot name to test against different opponents.

## Troubleshooting

**"Permission denied" on `.cargo-build-lock` or similar:**
This means `target/` was previously created by a different user (e.g. root, from a Docker build). Fix with:

```bash
sudo chown -R $USER:$USER docker_image/solution/target
```

Or simply delete and let Cargo recreate it:

```bash
sudo rm -rf docker_image/solution/target
cargo build --release
```

**Fresh clone of this repo:**

```bash
git clone <repo-url>
cd filler/docker_image
chmod +x linux_game_engine linux_robots/*
docker build -t filler .
docker run -v "$(pwd)/solution":/filler/solution -it filler
```
