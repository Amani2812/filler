# Filler

> A Rust player for the **Filler / Anfield** board game.

The player reads each board state and piece from standard input, chooses a legal position, and writes its move as `x y`. A legal move must overlap **exactly one** of the player's existing cells without covering an opponent cell.

## Highlights

- Rust 2021, with no third-party runtime dependencies
- Docker image for a Linux grading environment
- Robust parsing for player, board, and piece input
- Automated coverage for parsing, placement validation, and board boundaries

## Quick start

### Build

```sh
cargo build --release
```

The player executable is produced at:

```text
target/release/solution
```

### Test

```sh
cargo test
```

Current suite: **7 passing tests**.

## Run a match

The game engine, maps, and robots are supplied by the grading package. From its Linux container, build the player and run a game:

```sh
cd /filler/solution
cargo build --release

cd /filler
PLAYER=./solution/target/release/solution
./linux_game_engine -q -f maps/map00 -p1 "$PLAYER" -p2 linux_robots/wall_e
```

To reverse the player order:

```sh
./linux_game_engine -q -f maps/map00 -p1 linux_robots/wall_e -p2 "$PLAYER"
```

For the complete Windows/Docker setup, five-game commands, result counting, and audit response templates, see [AUDIT_RUNBOOK.md](AUDIT_RUNBOOK.md).

## Docker

Build the standalone player image from PowerShell:

```powershell
docker build -t filler .
docker inspect filler --format '{{json .Config.Entrypoint}}'
```

Expected entrypoint:

```text
["/solution"]
```

The standalone image contains only the player. Use the supplied grader image when you need `linux_game_engine`, maps, and robots.

## Validation rules

`put_piece` accepts a position only when all of these are true:

1. The piece remains completely inside the board.
2. It overlaps exactly one of the player's cells.
3. It does not overlap an opponent cell.

## Test coverage

| Area | Tests |
| --- | --- |
| Input parsing | Player identity, Anfield dimensions/grid rows, piece shapes |
| Placement validation | One valid self-overlap, multiple self-overlaps, opponent overlap |
| Boundary detection | Horizontal and vertical placements that extend beyond the board |

Test files:

- [tests/input_parsing.rs](tests/input_parsing.rs)
- [tests/placement_validation.rs](tests/placement_validation.rs)

## Project layout

```text
src/
  main.rs          Game input/output loop
  parsing.rs       Pure, testable input parsers
  place_piece.rs   Move selection and validation
  lib.rs           Shared library exports
tests/             Integration tests
Dockerfile         Standalone Linux player image
AUDIT_RUNBOOK.md   Grader commands and audit evidence guide
```

## Audit evidence

[AUDIT.md](AUDIT.md) contains the audit questionnaire. Use [AUDIT_RUNBOOK.md](AUDIT_RUNBOOK.md) to generate reproducible command output before recording game results.
